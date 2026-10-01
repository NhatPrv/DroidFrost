use crate::models::{ConnectionType, DeviceInfo, DeviceStatus};
use crate::services::adb_executor::AdbExecutor;
use regex::Regex;

pub struct DeviceDetectorService;

impl DeviceDetectorService {
    /// Quét và lấy danh sách toàn bộ thiết bị Android đang kết nối với máy tính
    pub async fn detect_devices() -> Result<Vec<DeviceInfo>, String> {
        let output = AdbExecutor::execute_raw(&["devices", "-l"]).await?;
        Ok(Self::parse_devices_output(&output))
    }

    /// Phân tích cú pháp chuỗi trả về từ `adb devices -l`
    pub fn parse_devices_output(output: &str) -> Vec<DeviceInfo> {
        lazy_static::lazy_static! {
            // Regex bóc tách Serial, Status, Product, Model
            static ref RE: Regex = Regex::new(
                r"(?m)^(?P<serial>[^\s]+)\s+(?P<status>device|unauthorized|offline|no permissions)(?:\s+product:(?P<product>[^\s]+))?(?:\s+model:(?P<model>[^\s]+))?"
            ).unwrap();
        }

        let mut devices = Vec::new();

        for cap in RE.captures_iter(output) {
            let serial = cap.name("serial").map_or("", |m| m.as_str()).to_string();
            
            // Bỏ qua dòng tiêu đề "List of devices attached"
            if serial == "List" || serial.is_empty() {
                continue;
            }

            let status_str = cap.name("status").map_or("unknown", |m| m.as_str());
            let status = match status_str {
                "device" => DeviceStatus::Device,
                "unauthorized" => DeviceStatus::Unauthorized,
                "offline" => DeviceStatus::Offline,
                _ => DeviceStatus::Unknown,
            };

            let product = cap.name("product").map_or("Android", |m| m.as_str()).to_string();
            let model = cap.name("model").map_or("Device", |m| m.as_str()).to_string();

            // Nhận diện kiểu kết nối: Nếu serial có dạng ip:port thì là Wireless ADB, ngược lại là USB
            let connection_type = if serial.contains(':') {
                ConnectionType::Wireless
            } else {
                ConnectionType::Usb
            };

            devices.push(DeviceInfo {
                serial,
                model,
                product,
                status,
                connection_type,
            });
        }

        devices
    }

    /// Kết nối tới thiết bị qua Wireless ADB (adb connect <ip>:<port>)
    pub async fn connect_wireless(ip: &str, port: u16) -> Result<String, String> {
        let addr = format!("{}:{}", ip.trim(), port);
        AdbExecutor::execute_raw(&["connect", &addr]).await
    }

    /// Ghép nối thiết bị qua Pairing Code trên Android 11+ (adb pair <ip>:<port> <code>)
    pub async fn pair_wireless(ip: &str, port: u16, pairing_code: &str) -> Result<String, String> {
        let addr = format!("{}:{}", ip.trim(), port);
        AdbExecutor::execute_raw(&["pair", &addr, pairing_code.trim()]).await
    }
}
