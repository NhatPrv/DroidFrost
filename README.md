<p align="center">
  <img src="https://raw.githubusercontent.com/NhatPrv/DroidFrost/main/docs/assets/banner.png" alt="DroidFrost Banner" width="700" onerror="this.style.display='none'"/>
</p>

# ❄️ DroidFrost

<p align="center">
  <strong>Autonomous Non-Root Android App Freezer, Process Killer & Task Scheduler for Desktop</strong>
</p>

<p align="center">
  <a href="https://github.com/NhatPrv/DroidFrost/releases"><img src="https://img.shields.io/github/v/release/NhatPrv/DroidFrost?style=for-the-badge&color=00d2ff" alt="Latest Release"/></a>
  <a href="https://github.com/NhatPrv/DroidFrost/blob/main/LICENSE"><img src="https://img.shields.io/badge/License-Apache_2.0-blue.svg?style=for-the-badge" alt="License"/></a>
  <a href="https://tauri.app/"><img src="https://img.shields.io/badge/Tauri-v2-FFC131?style=for-the-badge&logo=tauri&logoColor=white" alt="Tauri v2"/></a>
  <a href="https://www.rust-lang.org/"><img src="https://img.shields.io/badge/Backend-Rust-dea584?style=for-the-badge&logo=rust&logoColor=white" alt="Rust"/></a>
  <a href="https://svelte.dev/"><img src="https://img.shields.io/badge/Frontend-Svelte_5-ff3e00?style=for-the-badge&logo=svelte&logoColor=white" alt="Svelte 5"/></a>
</p>

---

## 📌 Giới thiệu

**DroidFrost** là công cụ máy tính (Desktop Application) hiệu năng cao cho phép kiểm soát toàn diện các tiến trình trên thiết bị Android **hoàn toàn không cần quyền Root**. Ứng dụng tận dụng trực tiếp giao thức Android Debug Bridge (ADB) để đo lường RAM/CPU, đóng băng triệt để các ứng dụng chạy ngầm ngốn pin, và tự động khôi phục theo bộ hẹn giờ thông minh.

Repo chính thức: [https://github.com/NhatPrv/DroidFrost](https://github.com/NhatPrv/DroidFrost)

---

## ⚡ So sánh cơ chế hoạt động

| Tiêu chí | Greenify (Không Root) | KillApps (Accessibility) | Shizuku / Hail | ❄️ DroidFrost |
| :--- | :--- | :--- | :--- | :--- |
| **Cơ chế can thiệp** | Mô phỏng bấm "Force Stop" qua Accessibility | Duyệt từng app bấm "Buộc dừng" trong Settings | Khởi tạo server daemon trên thiết bị qua adb | **Gọi ADB trực tiếp từ PC qua USB/WiFi** |
| **Tiêu tốn pin thiết bị** | Chạy background service liên tục trên điện thoại | Tốn pin khi quét danh sách | Nhẹ, nhưng vẫn giữ socket ngầm trên Android | **0% pin điện thoại** (Tất cả logic xử lý ở PC) |
| **Độ triệt để** | Chỉ dừng tạm thời (App dễ tự restart lại) | Chỉ dừng tạm thời, không ngăn tự kích hoạt | Đóng băng thật (`pm disable`) | **Đóng băng tuyệt đối (`pm disable-user --user 0`)** |
| **Tự động Unfreeze (Hẹn giờ)** | Không hỗ trợ | Không hỗ trợ | Giới hạn | **Scheduler Engine linh hoạt (15m, 1h, 2h, 8h, 24h)** |
| **Chống mất kết nối (Resilience)** | N/A | N/A | Lỗi nếu daemon bị kill | **Lưu cờ pending, tự động unfreeze ngay khi cắm lại** |
| **An toàn hệ điều hành** | Dựa vào blacklist người dùng | Dễ bấm nhầm service nếu không để ý | Đòi hỏi người dùng hiểu biết | **Tích hợp Hardcoded Safe Whitelist & One-Click Boost** |

---

## 🚀 Tính năng nổi bật

1. **Giao tiếp ADB USB & Wireless Độc Lập:** Tự động phát hiện thiết bị khi cắm cáp hoặc ghép nối qua Wireless Debugging (Port 5555 / Pairing Code Android 11+).
2. **Đo Lường RAM Realtime:** Phân tích `dumpsys meminfo` và `dumpsys activity processes` với tốc độ cao, hiển thị RAM tổng và RAM từng ứng dụng cụ thể.
3. **Hai Chế Độ Tối Ưu:**
   - **Kill (`am force-stop`):** Dọn dẹp RAM tức thì, thích hợp khi cần giải phóng bộ nhớ chơi game nặng.
   - **Freeze (`pm disable-user --user 0`):** Đóng băng ứng dụng 100%, ẩn hoàn toàn khỏi Launcher, chặn broadcast receiver đánh thức máy.
4. **Desktop Scheduler Engine:** Hẹn giờ rã đông tự động theo các mốc 15 phút, 1 giờ, 2 giờ, 8 giờ, 24 giờ.
5. **Disconnect Resilience (Chống Rút Cáp Đột Ngột):** Tự động phát hiện khi thiết bị kết nối lại và xử lý các tác vụ rã đông bị quá hạn một cách an toàn.
6. **Smart Whitelist:** Bảo vệ tuyệt đối các gói hệ điều hành cốt lõi (`SystemUI`, `Telephony`, `GMS`, `Phone`) ngăn ngừa nguy cơ bootloop.
7. **One-Click Boost:** Một chạm giải phóng RAM tối đa, đóng băng toàn bộ app không thuộc Whitelist.

---

## 🔌 Hướng dẫn kết nối thiết bị

### 1. Bật gỡ lỗi USB (USB Debugging)
1. Trên điện thoại Android, vào **Cài đặt** -> **Thông tin điện thoại** (About Phone).
2. Chạm 7 lần liên tiếp vào **Số bản dựng** (Build Number) để bật chế độ nhà phát triển.
3. Quay lại **Tùy chọn nhà phát triển** (Developer Options) -> Bật **Gỡ lỗi USB** (USB Debugging).
4. Cắm cáp vào máy tính, trên điện thoại sẽ xuất hiện hộp thoại: Tích chọn *"Luôn cho phép từ máy tính này"* -> Bấm **Cho phép** (Allow).

### 2. Kết nối không dây (Wireless Debugging - Android 11+)
1. Đảm bảo máy tính và điện thoại kết nối chung một mạng Wi-Fi.
2. Trong **Tùy chọn nhà phát triển**, bật **Gỡ lỗi qua Wi-Fi** (Wireless Debugging).
3. Sử dụng cổng hiển thị trên màn hình hoặc lệnh:
   ```bash
   adb tcpip 5555
   adb connect <IP_CỦA_ĐIỆN_THOẠI>:5555
   ```
4. DroidFrost sẽ tự động phát hiện thiết bị và chuyển sang trạng thái xanh (Ready).

---

## 🛠️ Cài đặt & Phát triển cục bộ

### Yêu cầu môi trường
- **Rust:** `rustc` và `cargo` (phiên bản ổn định mới nhất).
- **Node.js:** Node 18+ & npm.
- **Android Platform Tools:** Lệnh `adb` khả dụng trong PATH hệ thống (hoặc đặt cùng thư mục ứng dụng).

### Khởi chạy chế độ phát triển
```bash
# Cài đặt dependencies
npm install

# Khởi chạy Tauri Dev Server
npm run tauri dev
```

### Đóng gói ứng dụng Desktop (.exe / installer)
```bash
npm run tauri build
```

---

## 📄 Bản quyền (License)

### Cách đọc số liệu RAM

- RAM tổng, khả dụng và đã dùng lấy từ `/proc/meminfo` (`MemTotal` và `MemAvailable`). RAM đã dùng = tổng trừ khả dụng. Bộ nhớ đệm là `Cached + Buffers + SReclaimable` để tham khảo, đã nằm trong các nhóm trên.
- Cột ứng dụng ghi **PSS** khi Android cung cấp `dumpsys meminfo`; nếu dịch vụ đó không hoạt động, cột ghi **RSS** lấy từ `ps`. RSS tính cả trang chia sẻ nên không cộng các ứng dụng để so với RAM toàn máy.
- Lệnh buộc dừng không bảo đảm RAM giảm đúng bằng RAM trước khi dừng. Ứng dụng tự khởi chạy lại và bộ nhớ đệm có thể thay đổi; hãy xem số đo sau khi quét lại.
- Dừng tất cả chỉ tác động ứng dụng người dùng đang chạy. Hiển thị app hệ thống là bộ lọc xem, không mở rộng phạm vi dừng tất cả.


Dự án được phân phối dưới giấy phép mã nguồn mở **Apache License 2.0**. Xem chi tiết tại file [LICENSE](LICENSE).
