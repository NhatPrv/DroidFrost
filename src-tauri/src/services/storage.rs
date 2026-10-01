use crate::models::ScheduledTask;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppStateStorage {
    pub custom_whitelist: HashSet<String>,
    pub scheduled_tasks: Vec<ScheduledTask>,
}

#[derive(Clone)]
pub struct StorageManager {
    file_path: PathBuf,
    state: Arc<Mutex<AppStateStorage>>,
}

impl StorageManager {
    pub fn new() -> Self {
        let mut path = dirs_or_fallback();
        path.push("droidfrost_storage.json");

        let initial_state = if path.exists() {
            match fs::read_to_string(&path) {
                Ok(content) => serde_json::from_str(&content).unwrap_or_default(),
                Err(_) => AppStateStorage::default(),
            }
        } else {
            AppStateStorage::default()
        };

        Self {
            file_path: path,
            state: Arc::new(Mutex::new(initial_state)),
        }
    }

    /// Lưu trạng thái hiện tại xuống ổ cứng
    pub fn persist(&self) -> Result<(), String> {
        let state = self.state.lock().map_err(|e| e.to_string())?;
        let json = serde_json::to_string_pretty(&*state).map_err(|e| e.to_string())?;
        fs::write(&self.file_path, json).map_err(|e| e.to_string())
    }

    /// Thêm package vào Custom Whitelist
    pub fn add_to_whitelist(&self, pkg: &str) -> Result<(), String> {
        {
            let mut state = self.state.lock().map_err(|e| e.to_string())?;
            state.custom_whitelist.insert(pkg.to_string());
        }
        self.persist()
    }

    /// Xóa package khỏi Custom Whitelist
    pub fn remove_from_whitelist(&self, pkg: &str) -> Result<(), String> {
        {
            let mut state = self.state.lock().map_err(|e| e.to_string())?;
            state.custom_whitelist.remove(pkg);
        }
        self.persist()
    }

    /// Lấy toàn bộ Custom Whitelist
    pub fn get_custom_whitelist(&self) -> HashSet<String> {
        self.state
            .lock()
            .map(|s| s.custom_whitelist.clone())
            .unwrap_or_default()
    }

    /// Thêm hoặc cập nhật task hẹn giờ
    pub fn save_task(&self, task: ScheduledTask) -> Result<(), String> {
        {
            let mut state = self.state.lock().map_err(|e| e.to_string())?;
            state.scheduled_tasks.retain(|t| !(t.device_serial == task.device_serial && t.package_name == task.package_name));
            state.scheduled_tasks.push(task);
        }
        self.persist()
    }

    /// Xóa task hẹn giờ
    pub fn remove_task(&self, serial: &str, pkg: &str) -> Result<(), String> {
        {
            let mut state = self.state.lock().map_err(|e| e.to_string())?;
            state.scheduled_tasks.retain(|t| !(t.device_serial == serial && t.package_name == pkg));
        }
        self.persist()
    }

    /// Lấy danh sách tất cả các task hẹn giờ
    pub fn get_tasks(&self) -> Vec<ScheduledTask> {
        self.state
            .lock()
            .map(|s| s.scheduled_tasks.clone())
            .unwrap_or_default()
    }

    /// Cập nhật cờ Pending cho task khi thiết bị bị ngắt kết nối
    pub fn mark_task_pending(&self, serial: &str, pkg: &str, is_pending: bool) -> Result<(), String> {
        {
            let mut state = self.state.lock().map_err(|e| e.to_string())?;
            if let Some(task) = state.scheduled_tasks.iter_mut().find(|t| t.device_serial == serial && t.package_name == pkg) {
                task.is_pending = is_pending;
            }
        }
        self.persist()
    }
}

fn dirs_or_fallback() -> PathBuf {
    if let Some(mut path) = std::env::var_os("APPDATA").map(PathBuf::from) {
        path.push("DroidFrost");
        let _ = fs::create_dir_all(&path);
        path
    } else {
        PathBuf::from(".")
    }
}
