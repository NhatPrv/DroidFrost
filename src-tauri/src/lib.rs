pub mod models;
pub mod services;

use models::{DeviceInfo, OneClickBoostResult, OperationResult, ProcessInfo, SystemMemoryInfo};
use services::adb_executor::AdbExecutor;
use services::device_detector::DeviceDetectorService;
use services::process_controller::ProcessController;
use services::scheduler::FreezeSchedulerService;
use services::storage::StorageManager;
use std::sync::Arc;
use tauri::State;

pub struct AppState {
    pub storage: StorageManager,
    pub scheduler: FreezeSchedulerService,
}

#[tauri::command]
async fn get_devices() -> Result<Vec<DeviceInfo>, String> {
    DeviceDetectorService::detect_devices().await
}

#[tauri::command]
async fn connect_wireless(ip: String, port: u16) -> Result<String, String> {
    DeviceDetectorService::connect_wireless(&ip, port).await
}

#[tauri::command]
async fn pair_wireless(ip: String, port: u16, code: String) -> Result<String, String> {
    DeviceDetectorService::pair_wireless(&ip, port, &code).await
}

#[tauri::command]
async fn get_device_state(
    serial: String,
    state: State<'_, Arc<AppState>>,
) -> Result<(SystemMemoryInfo, Vec<ProcessInfo>), String> {
    let remaining_map = state.scheduler.get_remaining_seconds_map(&serial);
    ProcessController::get_device_state(&serial, &remaining_map).await
}

#[tauri::command]
async fn kill_app(serial: String, pkg: String) -> OperationResult {
    ProcessController::kill_app(&serial, &pkg).await
}

#[tauri::command]
async fn freeze_app(serial: String, pkg: String) -> OperationResult {
    ProcessController::freeze_app(&serial, &pkg).await
}

#[tauri::command]
async fn unfreeze_app(serial: String, pkg: String) -> OperationResult {
    ProcessController::unfreeze_app(&serial, &pkg).await
}

#[tauri::command]
async fn schedule_freeze(
    serial: String,
    pkg: String,
    duration_seconds: i64,
    state: State<'_, Arc<AppState>>,
) -> OperationResult {
    state.scheduler.schedule_freeze(&serial, &pkg, duration_seconds).await
}

#[tauri::command]
async fn cancel_schedule(
    serial: String,
    pkg: String,
    state: State<'_, Arc<AppState>>,
) -> OperationResult {
    state.scheduler.cancel_schedule(&serial, &pkg).await
}

#[tauri::command]
async fn one_click_boost(
    serial: String,
    state: State<'_, Arc<AppState>>,
) -> Result<OneClickBoostResult, String> {
    let remaining_map = state.scheduler.get_remaining_seconds_map(&serial);
    ProcessController::one_click_boost(&serial, &remaining_map).await
}

#[tauri::command]
fn set_mock_mode(enabled: bool) -> bool {
    AdbExecutor::set_mock_mode(enabled);
    enabled
}

#[tauri::command]
fn is_mock_mode() -> bool {
    AdbExecutor::is_mock_mode()
}

#[tauri::command]
fn get_custom_whitelist(state: State<'_, Arc<AppState>>) -> Vec<String> {
    state.storage.get_custom_whitelist().into_iter().collect()
}

#[tauri::command]
fn add_to_whitelist(pkg: String, state: State<'_, Arc<AppState>>) -> Result<(), String> {
    state.storage.add_to_whitelist(&pkg)
}

#[tauri::command]
fn remove_from_whitelist(pkg: String, state: State<'_, Arc<AppState>>) -> Result<(), String> {
    state.storage.remove_from_whitelist(&pkg)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let storage = StorageManager::new();
    let scheduler = FreezeSchedulerService::new(storage.clone());

    // Kích hoạt scheduler background loop
    scheduler.start_scheduler_loop();

    let app_state = Arc::new(AppState { storage, scheduler });

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            get_devices,
            connect_wireless,
            pair_wireless,
            get_device_state,
            kill_app,
            freeze_app,
            unfreeze_app,
            schedule_freeze,
            cancel_schedule,
            one_click_boost,
            set_mock_mode,
            is_mock_mode,
            get_custom_whitelist,
            add_to_whitelist,
            remove_from_whitelist
        ])
        .run(tauri::generate_context!())
        .expect("error while running DroidFrost desktop application");
}
