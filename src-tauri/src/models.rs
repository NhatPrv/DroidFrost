use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum DeviceStatus {
    Device,
    Unauthorized,
    Offline,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ConnectionType {
    Usb,
    Wireless,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceInfo {
    pub serial: String,
    pub model: String,
    pub product: String,
    pub status: DeviceStatus,
    pub connection_type: ConnectionType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessInfo {
    pub pid: Option<u32>,
    pub package_name: String,
    pub app_name: String,
    pub ram_mb: f64,
    pub is_running: bool,
    pub is_frozen: bool,
    pub is_scheduled: bool,
    pub is_system: bool,
    pub is_whitelisted: bool,
    pub scheduled_remaining_seconds: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMemoryInfo {
    pub total_ram_mb: f64,
    pub used_ram_mb: f64,
    pub free_ram_mb: f64,
    pub cached_ram_mb: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduledTask {
    pub id: String,
    pub package_name: String,
    pub device_serial: String,
    pub scheduled_at: i64,
    pub duration_seconds: i64,
    pub target_time: i64,
    pub is_pending: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OneClickBoostResult {
    pub killed_count: usize,
    pub frozen_count: usize,
    pub freed_ram_mb: f64,
    pub packages_affected: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationResult {
    pub success: bool,
    pub message: String,
    pub data: Option<serde_json::Value>,
}

impl OperationResult {
    pub fn ok(message: &str) -> Self {
        Self {
            success: true,
            message: message.to_string(),
            data: None,
        }
    }

    pub fn ok_with_data(message: &str, data: serde_json::Value) -> Self {
        Self {
            success: true,
            message: message.to_string(),
            data: Some(data),
        }
    }

    pub fn err(message: &str) -> Self {
        Self {
            success: false,
            message: message.to_string(),
            data: None,
        }
    }
}
