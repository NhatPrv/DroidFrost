use crate::models::{OperationResult, ScheduledTask};
use crate::services::device_detector::DeviceDetectorService;
use crate::services::process_controller::ProcessController;
use crate::services::storage::StorageManager;
use chrono::Utc;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};

#[derive(Clone)]
pub struct FreezeSchedulerService {
    storage: StorageManager,
    is_running: Arc<Mutex<bool>>,
}

impl FreezeSchedulerService {
    pub fn new(storage: StorageManager) -> Self {
        Self {
            storage,
            is_running: Arc::new(Mutex::new(false)),
        }
    }

    /// Lên lịch đóng băng ứng dụng kèm hẹn giờ tự động rã đông
    pub async fn schedule_freeze(
        &self,
        serial: &str,
        pkg: &str,
        duration_seconds: i64,
    ) -> OperationResult {
        // 1. Tiến hành đóng băng ứng dụng trước
        let freeze_res = ProcessController::freeze_app(serial, pkg).await;
        if !freeze_res.success {
            return freeze_res;
        }

        let now = Utc::now().timestamp();
        let target_time = now + duration_seconds;
        let task_id = format!("{}-{}-{}", serial, pkg, now);

        let task = ScheduledTask {
            id: task_id,
            package_name: pkg.to_string(),
            device_serial: serial.to_string(),
            scheduled_at: now,
            duration_seconds,
            target_time,
            is_pending: false,
        };

        if let Err(e) = self.storage.save_task(task) {
            return OperationResult::err(&format!("Không thể lưu lịch hẹn: {}", e));
        }

        OperationResult::ok(&format!(
            "Đã đóng băng và đặt lịch tự động rã đông cho {} sau {} giây",
            pkg, duration_seconds
        ))
    }

    /// Hủy lịch hẹn và rã đông ứng dụng ngay lập tức
    pub async fn cancel_schedule(&self, serial: &str, pkg: &str) -> OperationResult {
        let _ = self.storage.remove_task(serial, pkg);
        ProcessController::unfreeze_app(serial, pkg).await
    }

    /// Lấy danh sách map các package đang có hẹn giờ và số giây còn lại
    pub fn get_remaining_seconds_map(&self, serial: &str) -> HashMap<String, i64> {
        let now = Utc::now().timestamp();
        let mut map = HashMap::new();
        for task in self.storage.get_tasks() {
            if task.device_serial == serial {
                let remaining = (task.target_time - now).max(0);
                map.insert(task.package_name, remaining);
            }
        }
        map
    }

    /// Khởi chạy vòng lặp giám sát hẹn giờ nền (Background Scheduler Engine Loop)
    pub fn start_scheduler_loop(&self) {
        let storage = self.storage.clone();
        let is_running = self.is_running.clone();
        tauri::async_runtime::spawn(async move {
            {
                let mut running = is_running.lock().await;
                if *running {
                    return;
                }
                *running = true;
            }

            loop {
                sleep(Duration::from_secs(1)).await;
                let now = Utc::now().timestamp();

                // Quét danh sách thiết bị đang online
                let online_serials: Vec<String> = DeviceDetectorService::detect_devices()
                    .await
                    .unwrap_or_default()
                    .into_iter()
                    .filter(|d| d.status == crate::models::DeviceStatus::Device)
                    .map(|d| d.serial)
                    .collect();

                let tasks = storage.get_tasks();

                for task in tasks {
                    let is_online = online_serials.contains(&task.device_serial);

                    if is_online {
                        // Thiết bị đang kết nối: Kiểm tra xem đã đến giờ rã đông chưa
                        if now >= task.target_time {
                            // Thực thi lệnh rã đông
                            let _ = ProcessController::unfreeze_app(&task.device_serial, &task.package_name).await;
                            // Xóa task hoàn tất
                            let _ = storage.remove_task(&task.device_serial, &task.package_name);
                        } else if task.is_pending {
                            // Thiết bị vừa cắm lại nhưng chưa hết giờ -> Hủy cờ pending để tiếp tục đếm
                            let _ = storage.mark_task_pending(&task.device_serial, &task.package_name, false);
                        }
                    } else {
                        // Thiết bị ngắt kết nối (Disconnect Resilience):
                        // Đánh dấu pending để khi cắm lại sẽ kiểm tra và unfreeze ngay nếu đã quá hạn
                        if !task.is_pending {
                            let _ = storage.mark_task_pending(&task.device_serial, &task.package_name, true);
                        }
                    }
                }
            }
        });
    }
}
