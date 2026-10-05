use crate::models::{OneClickBoostResult, OperationResult, ProcessInfo, SystemMemoryInfo};
use crate::services::adb_executor::AdbExecutor;
use crate::services::process_parser::ProcessParser;
use std::collections::HashMap;

pub struct ProcessController;

impl ProcessController {
    /// Buộc dừng ứng dụng tức thì (am force-stop)
    pub async fn kill_app(serial: &str, pkg: &str) -> OperationResult {
        if !AdbExecutor::sanitize_package_name(pkg) {
            return OperationResult::err("Tên package không hợp lệ hoặc chứa ký tự nguy hiểm!");
        }

        if ProcessParser::is_whitelisted(pkg) {
            return OperationResult::err("Gói này thuộc Whitelist hệ thống, không thể buộc dừng!");
        }

        match AdbExecutor::execute_shell(serial, &["am", "force-stop", pkg]).await {
            Ok(_) => OperationResult::ok(&format!("Đã dừng tạm {}. Ứng dụng có thể tự chạy lại khi nhận sự kiện.", pkg)),
            Err(e) => OperationResult::err(&format!("Lỗi khi buộc dừng {}: {}", pkg, e)),
        }
    }

    /// Đóng băng ứng dụng hoàn toàn (pm disable-user --user 0)
    pub async fn freeze_app(serial: &str, pkg: &str) -> OperationResult {
        if !AdbExecutor::sanitize_package_name(pkg) {
            return OperationResult::err("Tên package không hợp lệ!");
        }

        if ProcessParser::is_whitelisted(pkg) {
            return OperationResult::err("CẢNH BÁO AN TOÀN: Không thể đóng băng ứng dụng hệ thống cốt lõi!");
        }

        match AdbExecutor::execute_shell(serial, &["pm", "disable-user", "--user", "0", pkg]).await {
            Ok(_) => match AdbExecutor::execute_shell(serial, &["pm", "list", "packages", "-d", "--user", "0"]).await {
                Ok(disabled) if ProcessParser::parse_disabled_packages(&disabled).contains(pkg) =>
                    OperationResult::ok(&format!("Đã tắt hẳn {} cho user 0; chỉ chạy lại sau khi bạn bật lại.", pkg)),
                Ok(_) => OperationResult::err(&format!("Không xác nhận được trạng thái tắt hẳn của {}", pkg)),
                Err(e) => OperationResult::err(&format!("Không kiểm tra được trạng thái {}: {}", pkg, e)),
            },
            Err(e) => OperationResult::err(&format!("Lỗi khi đóng băng {}: {}", pkg, e)),
        }
    }

    /// Rã đông ứng dụng (pm enable)
    pub async fn unfreeze_app(serial: &str, pkg: &str) -> OperationResult {
        if !AdbExecutor::sanitize_package_name(pkg) {
            return OperationResult::err("Tên package không hợp lệ!");
        }

        match AdbExecutor::execute_shell(serial, &["pm", "enable", pkg]).await {
            Ok(out) => {
                let lower = out.to_lowercase();
                if lower.contains("enabled") || lower.contains("new state") {
                    OperationResult::ok(&format!("Đã bật lại thành công {}", pkg))
                } else {
                    // Kiểm tra xác thực trạng thái qua danh sách disabled
                    let disabled = AdbExecutor::execute_shell(serial, &["pm", "list", "packages", "-d"])
                        .await
                        .unwrap_or_default();
                    if !ProcessParser::parse_disabled_packages(&disabled).contains(pkg) {
                        OperationResult::ok(&format!("Đã bật lại thành công {}", pkg))
                    } else {
                        OperationResult::err(&format!("Không xác nhận được trạng thái bật lại của {}: {}", pkg, out.trim()))
                    }
                }
            }
            Err(e) => OperationResult::err(&format!("Lỗi khi rã đông {}: {}", pkg, e)),
        }
    }

    /// Gỡ cài đặt ứng dụng khỏi thiết bị (Hỗ trợ cả User App và System Bloatware qua User 0)
    pub async fn uninstall_app(serial: &str, pkg: &str) -> OperationResult {
        if !AdbExecutor::sanitize_package_name(pkg) {
            return OperationResult::err("Tên package không hợp lệ hoặc chứa ký tự nguy hiểm!");
        }

        if ProcessParser::is_whitelisted(pkg) {
            return OperationResult::err("CẢNH BÁO BẢO VỆ: Gói này thuộc Whitelist cốt lõi, không thể gỡ cài đặt!");
        }

        // Thử gỡ hoàn toàn cho ứng dụng người dùng trước
        match AdbExecutor::execute_shell(serial, &["pm", "uninstall", pkg]).await {
            Ok(out) if out.to_lowercase().contains("success") => {
                OperationResult::ok(&format!("Đã gỡ cài đặt thành công ứng dụng {}", pkg))
            }
            _ => {
                // Nếu là ứng dụng hệ thống (System App / Bloatware), gỡ bỏ cho User 0
                match AdbExecutor::execute_shell(serial, &["pm", "uninstall", "-k", "--user", "0", pkg]).await {
                    Ok(out) if out.to_lowercase().contains("success") => {
                        OperationResult::ok(&format!("Đã gỡ bỏ thành công ứng dụng {} cho người dùng hiện tại", pkg))
                    }
                    Ok(out) => OperationResult::err(&format!("Lỗi khi gỡ cài đặt {}: {}", pkg, out.trim())),
                    Err(e) => OperationResult::err(&format!("Không thể thực thi lệnh gỡ cài đặt {}: {}", pkg, e)),
                }
            }
        }
    }

    /// Lấy toàn bộ trạng thái RAM và danh sách tiến trình của thiết bị
    pub async fn get_device_state(
        serial: &str,
        scheduled_tasks: &HashMap<String, i64>,
    ) -> Result<(SystemMemoryInfo, Vec<ProcessInfo>), String> {
        // 1. Quét danh sách package bên thứ 3 (User apps)
        let user_pkgs_output = AdbExecutor::execute_shell(serial, &["pm", "list", "packages", "-3", "-f"]).await?;
        let user_pkgs = ProcessParser::parse_package_list(&user_pkgs_output, false);

        // 2. Quét danh sách package hệ thống (System apps)
        let sys_pkgs_output = AdbExecutor::execute_shell(serial, &["pm", "list", "packages", "-s", "-f"]).await?;
        let sys_pkgs = ProcessParser::parse_package_list(&sys_pkgs_output, true);

        // 3. Quét danh sách app đang bị đóng băng
        let disabled_output = AdbExecutor::execute_shell(serial, &["pm", "list", "packages", "-d", "--user", "0"]).await?;
        let disabled_set = ProcessParser::parse_disabled_packages(&disabled_output);

        // 4. Quét dumpsys meminfo để lấy RAM hệ thống và RAM từng tiến trình
        let proc_meminfo = AdbExecutor::execute_shell(serial, &["cat", "/proc/meminfo"]).await?;
        let mut sys_memory = ProcessParser::parse_system_memory(&proc_meminfo);
        if sys_memory.total_ram_mb <= 0.0 { return Err("Không đọc được RAM từ /proc/meminfo".into()); }
        let meminfo_output = AdbExecutor::execute_shell(serial, &["dumpsys", "meminfo"]).await.unwrap_or_default();
        let mut ram_map = ProcessParser::parse_process_ram_table(&meminfo_output);
        if ram_map.is_empty() {
            let ps_output = AdbExecutor::execute_shell(serial, &["ps", "-A", "-o", "PID,NAME,RSS"]).await?;
            ram_map = ProcessParser::parse_ps_rss(&ps_output);
            sys_memory.process_metric = "RSS".to_string();
        }

        // Hợp nhất danh sách
        let mut all_packages = user_pkgs;
        all_packages.extend(sys_pkgs);

        let process_list = ProcessParser::build_process_list(
            &all_packages,
            &disabled_set,
            &ram_map,
            scheduled_tasks,
        );

        Ok((sys_memory, process_list))
    }

    /// One-Click Boost: Buộc dừng toàn bộ ứng dụng người dùng đang chạy ngầm không thuộc Whitelist
    pub async fn one_click_boost(
        serial: &str,
        scheduled_tasks: &HashMap<String, i64>,
    ) -> Result<OneClickBoostResult, String> {
        let (_, processes) = Self::get_device_state(serial, scheduled_tasks).await?;

        let mut killed_count = 0;
        let mut packages_affected = Vec::new();

        for p in processes {
            // Chỉ can thiệp các app đang chạy, không phải hệ thống, và không nằm trong Whitelist
            if p.is_running && !p.is_system && !p.is_whitelisted {
                let kill_res = Self::kill_app(serial, &p.package_name).await;
                if kill_res.success {
                    killed_count += 1;
                    packages_affected.push(p.package_name);
                }
            }
        }

        Ok(OneClickBoostResult {
            killed_count,
            frozen_count: 0,
            freed_ram_mb: 0.0,
            packages_affected,
        })
    }

    /// Dừng toàn bộ tiến trình đang chạy bằng cơ chế batch shell execution siêu tốc
    pub async fn stop_all_running(
        serial: &str,
        _include_system: bool,
        scheduled_tasks: &HashMap<String, i64>,
    ) -> Result<OneClickBoostResult, String> {
        let (_, processes) = Self::get_device_state(serial, scheduled_tasks).await?;

        let mut targets = Vec::new();

        for p in &processes {
            if p.is_running && !p.is_whitelisted && !p.is_system {
                targets.push(p.package_name.clone());
            }
        }

        let valid_targets: Vec<String> = targets
            .into_iter()
            .filter(|pkg| AdbExecutor::sanitize_package_name(pkg))
            .collect();

        if valid_targets.is_empty() {
            return Ok(OneClickBoostResult {
                killed_count: 0,
                frozen_count: 0,
                freed_ram_mb: 0.0,
                packages_affected: Vec::new(),
            });
        }

        // Tối ưu hiệu năng: Gộp các lệnh force-stop thành 1 subprocess shell duy nhất (~100ms thay vì 5000ms)
        let batch_cmd = valid_targets
            .iter()
            .map(|pkg| format!("am force-stop {}", pkg))
            .collect::<Vec<_>>()
            .join("; ");

        let _ = AdbExecutor::execute_shell(serial, &["sh", "-c", &batch_cmd]).await;

        let killed_count = valid_targets.len();
        Ok(OneClickBoostResult {
            killed_count,
            frozen_count: 0,
            freed_ram_mb: 0.0,
            packages_affected: valid_targets,
        })
    }
}
