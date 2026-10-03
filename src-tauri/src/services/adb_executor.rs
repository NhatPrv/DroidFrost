use regex::Regex;
use std::process::Stdio;
use std::time::Duration;
use tokio::process::Command;
use tokio::time::timeout;

pub struct AdbExecutor;

impl AdbExecutor {
    /// Kiểm tra tính hợp lệ của package name nhằm ngăn chặn Command Injection
    pub fn sanitize_package_name(pkg: &str) -> bool {
        lazy_static::lazy_static! {
            static ref RE: Regex = Regex::new(r"^[a-zA-Z0-9_\.]+$").unwrap();
        }
        !pkg.trim().is_empty() && RE.is_match(pkg)
    }

    /// Thực thi lệnh ADB nguyên bản với cơ chế Timeout 5000ms
    pub async fn execute_raw(args: &[&str]) -> Result<String, String> {
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

        match timeout(Duration::from_millis(15000), future).await {
            Ok(result) => result,
            Err(_) => Err("Hết thời gian chờ lệnh ADB (Timeout 15000ms)".to_string()),
        }
    }

    /// Thực thi lệnh shell trên thiết bị cụ thể: adb -s <serial> shell <command...>
    pub async fn execute_shell(serial: &str, shell_args: &[&str]) -> Result<String, String> {
        let mut args = vec!["-s", serial, "shell"];
        args.extend_from_slice(shell_args);
        Self::execute_raw(&args).await
    }
}
