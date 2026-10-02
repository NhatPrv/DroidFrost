use droidfrost_lib::models::{ConnectionType, DeviceStatus, ScheduledTask};
use droidfrost_lib::services::adb_executor::AdbExecutor;
use droidfrost_lib::services::device_detector::DeviceDetectorService;
use droidfrost_lib::services::process_parser::ProcessParser;

#[test]
fn test_parse_devices_output() {
    let sample_output = r#"
List of devices attached
RFCT40ABCDE            device product:r8qxxx model:SM_G780G device:r8q transport_id:1
192.168.1.55:5555      device product:husky model:Pixel_8_Pro device:husky transport_id:2
1234567890             unauthorized transport_id:3
"#;

    let devices = DeviceDetectorService::parse_devices_output(sample_output);
    assert_eq!(devices.len(), 3);

    // Kiểm tra thiết bị USB
    assert_eq!(devices[0].serial, "RFCT40ABCDE");
    assert_eq!(devices[0].model, "SM_G780G");
    assert_eq!(devices[0].status, DeviceStatus::Device);
    assert_eq!(devices[0].connection_type, ConnectionType::Usb);

    // Kiểm tra thiết bị Wireless
    assert_eq!(devices[1].serial, "192.168.1.55:5555");
    assert_eq!(devices[1].model, "Pixel_8_Pro");
    assert_eq!(devices[1].connection_type, ConnectionType::Wireless);

    // Kiểm tra thiết bị Chưa cấp quyền
    assert_eq!(devices[2].serial, "1234567890");
    assert_eq!(devices[2].status, DeviceStatus::Unauthorized);
}

#[test]
fn test_parse_system_memory() {
    let sample_meminfo = r#"
Total RAM: 7,842,120K (status normal)
 Free RAM: 3,214,560K (  984,200K cached pss + 1,820,360K cached kernel +   410,000K free)
 Used RAM: 4,627,560K (3,520,120K used pss + 1,107,440K kernel)
"#;

    let mem = ProcessParser::parse_system_memory(sample_meminfo);
    assert!(mem.total_ram_mb > 7600.0 && mem.total_ram_mb < 7700.0);
    assert!(mem.free_ram_mb > 3100.0 && mem.free_ram_mb < 3200.0);
    assert!(mem.used_ram_mb > 4500.0 && mem.used_ram_mb < 4600.0);
}

#[test]
fn test_parse_process_ram_table() {
    let sample_procs = r#"
Total PSS by process:
    215,680K: com.facebook.katana (pid 14522)
    124,320K: com.zing.zalo (pid 18901)
     45,100K: com.android.systemui:screenshot (pid 2411)
"#;

    let map = ProcessParser::parse_process_ram_table(sample_procs);
    assert_eq!(map.len(), 3);

    let fb = map.get("com.facebook.katana").unwrap();
    assert_eq!(fb.0, Some(14522));
    assert!(fb.1 > 210.0 && fb.1 < 211.0);

    let zalo = map.get("com.zing.zalo").unwrap();
    assert_eq!(zalo.0, Some(18901));
    assert!(zalo.1 > 121.0 && zalo.1 < 122.0);
}

#[test]
fn test_parse_disabled_packages() {
    let sample_disabled = r#"
package:com.facebook.katana
package:com.shopee.vn
"#;

    let set = ProcessParser::parse_disabled_packages(sample_disabled);
    assert_eq!(set.len(), 2);
    assert!(set.contains("com.facebook.katana"));
    assert!(set.contains("com.shopee.vn"));
    assert!(!set.contains("com.zing.zalo"));
}

#[test]
fn test_safe_whitelist_boundary() {
    // Các gói quan trọng không được phép freeze
    assert!(ProcessParser::is_whitelisted("android"));
    assert!(ProcessParser::is_whitelisted("com.android.systemui"));
    assert!(ProcessParser::is_whitelisted("com.google.android.gms"));
    assert!(ProcessParser::is_whitelisted("com.android.phone"));

    // Các gói người dùng bình thường phải trả về false
    assert!(!ProcessParser::is_whitelisted("com.facebook.katana"));
    assert!(!ProcessParser::is_whitelisted("com.zing.zalo"));
}

#[test]
fn test_sanitize_package_name() {
    // Tên hợp lệ
    assert!(AdbExecutor::sanitize_package_name("com.facebook.katana"));
    assert!(AdbExecutor::sanitize_package_name("com.example.app_123"));

    // Tên chứa ký tự nguy hiểm nhằm command injection
    assert!(!AdbExecutor::sanitize_package_name("com.app; rm -rf /"));
    assert!(!AdbExecutor::sanitize_package_name("com.app && reboot"));
    assert!(!AdbExecutor::sanitize_package_name("com.app | cat /etc/passwd"));
    assert!(!AdbExecutor::sanitize_package_name(""));
    assert!(!AdbExecutor::sanitize_package_name("   "));
}

#[test]
fn test_scheduled_task_logic() {
    let now = chrono::Utc::now().timestamp();
    let task = ScheduledTask {
        id: "task-1".to_string(),
        package_name: "com.facebook.katana".to_string(),
        device_serial: "SERIAL123".to_string(),
        scheduled_at: now,
        duration_seconds: 900,
        target_time: now + 900,
        is_pending: false,
    };

    assert_eq!(task.duration_seconds, 900);
    assert_eq!(task.target_time, now + 900);
    assert!(!task.is_pending);
}

#[test]
fn test_stop_all_running_filter() {
    use droidfrost_lib::models::ProcessInfo;

    let procs = vec![
        ProcessInfo {
            pid: Some(101),
            package_name: "com.facebook.katana".to_string(),
            app_name: "Facebook".to_string(),
            ram_mb: 320.0,
            is_running: true,
            is_frozen: false,
            is_scheduled: false,
            is_system: false,
            is_whitelisted: false,
            scheduled_remaining_seconds: None,
        },
        ProcessInfo {
            pid: Some(102),
            package_name: "com.android.systemui".to_string(),
            app_name: "SystemUI".to_string(),
            ram_mb: 200.0,
            is_running: true,
            is_frozen: false,
            is_scheduled: false,
            is_system: true,
            is_whitelisted: true, // Whitelisted!
            scheduled_remaining_seconds: None,
        },
        ProcessInfo {
            pid: None,
            package_name: "com.shopee.vn".to_string(),
            app_name: "Shopee".to_string(),
            ram_mb: 0.0,
            is_running: false, // Không chạy!
            is_frozen: true,
            is_scheduled: false,
            is_system: false,
            is_whitelisted: false,
            scheduled_remaining_seconds: None,
        },
    ];

    // Lọc theo điều kiện dừng tất cả
    let targets: Vec<String> = procs
        .into_iter()
        .filter(|p| p.is_running && !p.is_whitelisted && !p.is_system)
        .map(|p| p.package_name)
        .collect();

    assert_eq!(targets.len(), 1);
    assert_eq!(targets[0], "com.facebook.katana");
}
