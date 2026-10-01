use regex::Regex;
use std::process::Stdio;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use tokio::process::Command;
use tokio::time::timeout;

static MOCK_MODE: AtomicBool = AtomicBool::new(false);

pub struct AdbExecutor;

impl AdbExecutor {
    pub fn set_mock_mode(enabled: bool) {
        MOCK_MODE.store(enabled, Ordering::SeqCst);
    }

    pub fn is_mock_mode() -> bool {
        MOCK_MODE.load(Ordering::SeqCst)
    }

    /// Kiểm tra tính hợp lệ của package name nhằm ngăn chặn Command Injection
    pub fn sanitize_package_name(pkg: &str) -> bool {
        lazy_static::lazy_static! {
            static ref RE: Regex = Regex::new(r"^[a-zA-Z0-9_\.]+$").unwrap();
        }
        !pkg.trim().is_empty() && RE.is_match(pkg)
    }

    /// Thực thi lệnh ADB chung với cơ chế Timeout 5000ms
    pub async fn execute_raw(args: &[&str]) -> Result<String, String> {
        if Self::is_mock_mode() {
            return Self::mock_execute_raw(args).await;
        }

        let mut cmd = Command::new("adb");
        cmd.args(args);
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());

        #[cfg(target_os = "windows")]
        {
            // CREATE_NO_WINDOW = 0x08000000 để ngăn hiện cửa sổ console đen nhấp nháy trên Windows
            cmd.creation_flags(0x08000000);
        }

        let future = async {
            match cmd.spawn() {
                Ok(child) => match child.wait_with_output().await {
                    Ok(output) => {
                        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
                        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
                        if output.status.success() {
                            Ok(stdout)
                        } else {
                            Err(format!("ADB Error ({}): {}", output.status, stderr.trim()))
                        }
                    }
                    Err(e) => Err(format!("Lỗi chờ tiến trình ADB: {}", e)),
                },
                Err(e) => Err(format!("Không thể khởi chạy lệnh adb: {}", e)),
            }
        };

        match timeout(Duration::from_millis(5000), future).await {
            Ok(result) => result,
            Err(_) => Err("Hết thời gian chờ lệnh ADB (Timeout 5000ms)".to_string()),
        }
    }

    /// Thực thi lệnh shell trên thiết bị cụ thể: adb -s <serial> shell <command...>
    pub async fn execute_shell(serial: &str, shell_args: &[&str]) -> Result<String, String> {
        if Self::is_mock_mode() {
            return Self::mock_execute_shell(serial, shell_args).await;
        }

        let mut args = vec!["-s", serial, "shell"];
        args.extend_from_slice(shell_args);
        Self::execute_raw(&args).await
    }

    // Mock handler phục vụ kiểm thử UI không cần thiết bị thật
    async fn mock_execute_raw(args: &[&str]) -> Result<String, String> {
        if args.contains(&"devices") {
            Ok(
                "List of devices attached\nRFCT40MOCK1            device product:galaxy_s24 model:SM_S928B device:e3q transport_id:1\n192.168.1.88:5555      device product:pixel8 model:Pixel_8_Pro device:husky transport_id:2\n"
                    .to_string(),
            )
        } else {
            Ok("Mock command executed successfully".to_string())
        }
    }

    async fn mock_execute_shell(_serial: &str, shell_args: &[&str]) -> Result<String, String> {
        let first = shell_args.first().copied().unwrap_or("");
        match first {
            "pm" => {
                if shell_args.contains(&"-d") {
                    Ok("package:com.facebook.katana\npackage:com.shopee.vn\n".to_string())
                } else if shell_args.contains(&"-3") {
                    Ok("package:/data/app/com.facebook.katana/base.apk=com.facebook.katana\npackage:/data/app/com.shopee.vn/base.apk=com.shopee.vn\npackage:/data/app/com.zing.zalo/base.apk=com.zing.zalo\npackage:/data/app/com.zhiliaoapp.musically/base.apk=com.zhiliaoapp.musically\npackage:/data/app/com.dts.freefireth/base.apk=com.dts.freefireth\n".to_string())
                } else {
                    Ok("Success\n".to_string())
                }
            }
            "dumpsys" => {
                if shell_args.contains(&"meminfo") {
                    Ok("Total RAM: 8,192,000K\n Free RAM: 3,450,000K (  850,000K cached pss + 1,900,000K cached kernel +   700,000K free)\n Used RAM: 4,742,000K\n".to_string())
                } else {
                    Ok("Mock dumpsys result".to_string())
                }
            }
            "am" => Ok("Complete\n".to_string()),
            _ => Ok("Mock shell response\n".to_string()),
        }
    }
}
