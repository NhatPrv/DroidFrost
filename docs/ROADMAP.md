# DroidFrost - Comprehensive Development Roadmap

Lộ trình phát triển toàn diện của DroidFrost theo chuẩn Autonomous Full-Cycle Engineering.

---

## Progress Overview

| Phase | Milestone | Scope | Trạng thái |
| :---: | :--- | :--- | :---: |
| **0** | **Repo & Environment** | Khởi tạo Git repo, cấu hình Remote origin, .gitignore | ✅ Hoàn thành |
| **1** | **Documentation & Specs**| Architecture, ADB Matrix, Regex specs, Roadmap, README, License | 🔄 Đang triển khai |
| **2** | **Core ADB Engine** | Tauri v2 + Rust Subprocess, Device Detector, Parser, Controller | ⏳ Kế tiếp |
| **3** | **Scheduler & Persistence**| Background Countdown Timer, Persistent DB, Disconnect Resilience | ⏳ Kế hoạch |
| **4** | **Desktop UI & State** | Svelte/TS Reactive UI, Process Table, Filters, One-Click Boost | ⏳ Kế hoạch |
| **5** | **Test Suite & Packaging** | Unit Test Suite, Mock ADB Provider, Production Bundle (.exe) | ⏳ Kế hoạch |

---

## Detailed Task Checklist

### Phase 0: Repo Initialization & Environment Setup
- [x] **Task 0.1:** Khởi tạo Git repo, thiết lập branch `main`, cấu hình remote origin `https://github.com/NhatPrv/DroidFrost.git`.
- [x] **Task 0.2:** Tạo `.gitignore` chuẩn cho Rust, Node, Tauri và IDE.

### Phase 1: Documentation & System Specifications
- [x] **Task 1.1:** Tạo `docs/ARCHITECTURE.md` (System design, State Machine, Auto-resume).
- [x] **Task 1.2:** Tạo `docs/ADB_COMMANDS.md` (Bảng lệnh ADB, Regex patterns, Safe Whitelist).
- [x] **Task 1.3:** Tạo `docs/ROADMAP.md` (Checklist tiến độ chi tiết từng giai đoạn).
- [ ] **Task 1.4:** Tạo `README.md` (Branding, kiến trúc so sánh, hướng dẫn kết nối) và `LICENSE` (Apache-2.0).

### Phase 2: Project Scaffolding & Core ADB Engine
- [ ] **Task 2.1:** Khởi tạo cấu trúc dự án (Tauri v2 + Rust backend + Svelte/Vite/TS frontend), cấu hình dependencies và scripts.
- [ ] **Task 2.2:** `DeviceDetectorService` (Tự động nhận diện thiết bị cắm qua USB / Wireless ADB, phân loại trạng thái `device`/`unauthorized`).
- [ ] **Task 2.3:** `AdbExecutor` (Safe subprocess wrapper, stream output, non-blocking, timeout guard 5000ms).
- [ ] **Task 2.4:** `ProcessParser` (Zero-copy regex parsing từ `dumpsys meminfo` và `dumpsys activity processes` ra danh sách struct app).
- [ ] **Task 2.5:** `ProcessController` (Cung cấp `killApp`, `freezeApp`, `unfreezeApp` tích hợp kiểm tra Whitelist an toàn).

### Phase 3: Scheduler Engine & Local Storage
- [ ] **Task 3.1:** Local Database / Storage (Persistent JSON / SQLite lưu Whitelist và Scheduled Tasks).
- [ ] **Task 3.2:** `FreezeSchedulerService` (Xử lý hàng đợi đếm ngược 15p, 1h, 2h, 8h, 24h; phát event unfreeze khi hết giờ).
- [ ] **Task 3.3:** Disconnect Resilience (Lưu cờ pending khi rút cáp, tự động khôi phục và unfreeze ngay khi cắm lại).

### Phase 4: Desktop UI & State Management
- [ ] **Task 4.1:** Layout chính (Header hiển thị thông số điện thoại, thanh RAM tổng trực quan, trạng thái kết nối).
- [ ] **Task 4.2:** Table danh sách ứng dụng (Icon placeholder, App Name, Package, RAM usage, Status badge).
- [ ] **Task 4.3:** Bộ lọc thông minh (Tabs: All, Running, Frozen, Scheduled; Toggle System vs User apps; Live Search).
- [ ] **Task 4.4:** Action Buttons trên từng dòng (Nút Kill, Freeze, Unfreeze, Menu dropdown chọn mốc giờ hẹn).
- [ ] **Task 4.5:** Nút "One-Click Boost" (Giải phóng RAM tối đa, đóng băng toàn bộ app không nằm trong Whitelist).

### Phase 5: Test Suite & Packaging
- [ ] **Task 5.1:** Unit tests cho bộ Regex Parser và Scheduler Logic.
- [ ] **Task 5.2:** Kịch bản Mock ADB (Cho phép chạy và trải nghiệm đầy đủ giao diện không cần điện thoại thật).
- [ ] **Task 5.3:** Cấu hình build script và kiểm thử đóng gói desktop executable.
