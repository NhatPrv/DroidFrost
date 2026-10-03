use crate::models::{ProcessInfo, SystemMemoryInfo};
use regex::Regex;
use std::collections::{HashMap, HashSet};

pub const CRITICAL_WHITELIST: &[&str] = &[
    "android",
    "com.android.systemui",
    "com.android.phone",
    "com.android.server.telecom",
    "com.android.providers.telephony",
    "com.android.settings",
    "com.google.android.gms",
    "com.google.android.gsf",
    "com.android.vending",
    "com.android.inputmethod.latin",
    "com.google.android.inputmethod.latin",
];

pub struct ProcessParser;

impl ProcessParser {
    /// Kiểm tra xem package có nằm trong danh sách hệ thống tối quan trọng không
    pub fn is_whitelisted(pkg: &str) -> bool {
        CRITICAL_WHITELIST.contains(&pkg)
    }

    /// Định dạng tên hiển thị thân thiện từ tên package
    pub fn format_friendly_name(pkg: &str) -> String {
        match pkg {
            "com.facebook.katana" => "Facebook".to_string(),
            "com.facebook.orca" => "Messenger".to_string(),
            "com.instagram.android" => "Instagram".to_string(),
            "com.zhiliaoapp.musically" => "TikTok".to_string(),
            "com.zing.zalo" => "Zalo".to_string(),
            "com.shopee.vn" => "Shopee".to_string(),
            "com.lazada.android" => "Lazada".to_string(),
            "com.tiki.app.tikiandroid" => "Tiki".to_string(),
            "com.google.android.youtube" => "YouTube".to_string(),
            "com.spotify.music" => "Spotify".to_string(),
            "com.netflix.mediaclient" => "Netflix".to_string(),
            "com.android.chrome" => "Google Chrome".to_string(),
            "com.dts.freefireth" => "Free Fire".to_string(),
            "com.tencent.ig" => "PUBG Mobile".to_string(),
            _ => {
                // Tự động lấy segment cuối cùng và viết hoa chữ cái đầu
                let parts: Vec<&str> = pkg.split('.').collect();
                if let Some(last) = parts.last() {
                    let mut chars = last.chars();
                    match chars.next() {
                        None => pkg.to_string(),
                        Some(f) => f.to_uppercase().collect::<String>() + chars.as_str(),
                    }
                } else {
                    pkg.to_string()
                }
            }
        }
    }

    /// Phân tích cú pháp đầu ra `pm list packages -f`
    pub fn parse_package_list(output: &str, is_system: bool) -> Vec<(String, bool)> {
        lazy_static::lazy_static! {
            static ref RE: Regex = Regex::new(r"^package:(?:.*=)?([a-zA-Z0-9_\.]+)").unwrap();
        }

        let mut list = Vec::new();
        for line in output.lines() {
            let line = line.trim();
            if let Some(caps) = RE.captures(line) {
                if let Some(pkg) = caps.get(1) {
                    list.push((pkg.as_str().to_string(), is_system));
                }
            }
        }
        list
    }

    /// Phân tích cú pháp đầu ra `pm list packages -d` để lấy danh sách app đang bị đóng băng
    pub fn parse_disabled_packages(output: &str) -> HashSet<String> {
        let mut disabled = HashSet::new();
        for line in output.lines() {
            let line = line.trim();
            if let Some(pkg) = line.strip_prefix("package:") {
                disabled.insert(pkg.trim().to_string());
            }
        }
        disabled
    }

    /// Read physical RAM from /proc/meminfo. MemAvailable includes reclaimable cache.
    pub fn parse_system_memory(output: &str) -> SystemMemoryInfo {
        let field = |name: &str| -> f64 {
            output.lines().find_map(|line| {
                let value = line.strip_prefix(name)?.trim().split_whitespace().next()?;
                value.parse::<f64>().ok().map(|kb| kb / 1024.0)
            }).unwrap_or(0.0)
        };
        let total_ram_mb = field("MemTotal:");
        let free_ram_mb = field("MemAvailable:");
        let cached_ram_mb = field("Cached:") + field("Buffers:") + field("SReclaimable:");
        let used_ram_mb = (total_ram_mb - free_ram_mb).max(0.0);
        let swap_total_mb = field("SwapTotal:");
        let swap_used_mb = (swap_total_mb - field("SwapFree:")).max(0.0);

        SystemMemoryInfo {
            total_ram_mb,
            used_ram_mb,
            free_ram_mb,
            cached_ram_mb,
            swap_total_mb,
            swap_used_mb,
            process_metric: "PSS".to_string(),
        }
    }

    /// Phân tích chi tiết RAM và tiến trình từ `dumpsys meminfo`
    pub fn parse_process_ram_table(output: &str) -> HashMap<String, (Option<u32>, f64)> {
        lazy_static::lazy_static! {
            // Định dạng: 215,680K: com.facebook.katana (pid 14522)
            static ref RE_LINE: Regex = Regex::new(
                r"^\s*([\d,]+)K:\s+([a-zA-Z0-9_\.\:]+)(?:\s+\(pid\s+(\d+)\))?"
            ).unwrap();
        }

        let mut map = HashMap::new();
        let mut in_process_section = false;
        let mut seen_pids = HashSet::new();
        for line in output.lines() {
            if line.trim() == "Total PSS by process:" {
                in_process_section = true;
                continue;
            }
            if in_process_section && (line.trim().is_empty() || line.trim().starts_with("Total PSS by ")) {
                break;
            }
            if !in_process_section { continue; }
            if let Some(caps) = RE_LINE.captures(line) {
                let ram_kb_str = caps.get(1).map_or("0", |m| m.as_str()).replace(',', "");
                let full_pkg = caps.get(2).map_or("", |m| m.as_str());
                let pid = caps.get(3).and_then(|m| m.as_str().parse::<u32>().ok());
                if let Some(pid) = pid {
                    if !seen_pids.insert(pid) { continue; }
                }

                // Tách package chính nếu có sub-process dạng "com.pkg:service"
                let pkg_name = full_pkg.split(':').next().unwrap_or(full_pkg).to_string();

                if let Ok(kb) = ram_kb_str.parse::<f64>() {
                    let mb = (kb / 1024.0 * 10.0).round() / 10.0;
                    map.entry(pkg_name)
                        .and_modify(|entry: &mut (Option<u32>, f64)| {
                            entry.1 += mb;
                            if entry.0.is_none() && pid.is_some() {
                                entry.0 = pid;
                            }
                        })
                        .or_insert((pid, mb));
                }
            }
        }
        map
    }

    /// ps RSS is a fallback: shared pages are counted in every process.
    pub fn parse_ps_rss(output: &str) -> HashMap<String, (Option<u32>, f64)> {
        let mut map = HashMap::new();
        for line in output.lines().skip(1) {
            let fields: Vec<&str> = line.split_whitespace().collect();
            if fields.len() < 3 { continue; }
            let (Ok(pid), Ok(kb)) = (fields[0].parse::<u32>(), fields[fields.len()-1].parse::<f64>()) else { continue; };
            let name = fields[1].split(':').next().unwrap_or(fields[1]);
            if !name.contains('.') { continue; }
            map.entry(name.to_string())
                .and_modify(|entry: &mut (Option<u32>, f64)| entry.1 += kb / 1024.0)
                .or_insert((Some(pid), kb / 1024.0));
        }
        map
    }

    /// Hợp nhất tất cả các nguồn thông tin thành danh sách `ProcessInfo` đầy đủ
    pub fn build_process_list(
        packages: &[(String, bool)], // (package, is_system)
        disabled_set: &HashSet<String>,
        ram_map: &HashMap<String, (Option<u32>, f64)>,
        scheduled_tasks: &HashMap<String, i64>, // package -> remaining_seconds
    ) -> Vec<ProcessInfo> {
        let mut results = Vec::new();
        let mut seen = HashSet::new();

        for (pkg, is_system) in packages {
            if seen.contains(pkg) {
                continue;
            }
            seen.insert(pkg.clone());

            let is_frozen = disabled_set.contains(pkg);
            let (pid, ram_mb) = ram_map.get(pkg).cloned().unwrap_or((None, 0.0));
            let is_running = pid.is_some() || ram_mb > 0.0;
            let is_whitelisted = Self::is_whitelisted(pkg);
            let remaining = scheduled_tasks.get(pkg).copied();
            let is_scheduled = remaining.is_some();

            results.push(ProcessInfo {
                pid,
                package_name: pkg.clone(),
                app_name: Self::format_friendly_name(pkg),
                ram_mb,
                is_running: is_running && !is_frozen,
                is_frozen,
                is_scheduled,
                is_system: *is_system,
                is_whitelisted,
                scheduled_remaining_seconds: remaining,
            });
        }

        // Sắp xếp: App chạy chiếm nhiều RAM nhất lên trước, tiếp đến app người dùng, app đóng băng
        results.sort_by(|a, b| {
            b.ram_mb
                .partial_cmp(&a.ram_mb)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        results
    }
}
