# DroidFrost - ADB Protocol, Command Reference & Regex Specs

Tài liệu này định nghĩa chi tiết tất cả các lệnh Android Debug Bridge (ADB), quy chuẩn định dạng chuỗi trả về, biểu thức chính quy (Regex) bóc tách dữ liệu và danh sách phân loại gói ứng dụng an toàn.

---

## 1. Core ADB Command Matrix

| Chức năng | Câu lệnh ADB | Quyền yêu cầu | Mô tả kỹ thuật |
| :--- | :--- | :--- | :--- |
| **Liệt kê thiết bị** | `adb devices -l` | ADB Shell thường | Lấy danh sách serial, product model, device id, trạng thái (device / unauthorized / offline). |
| **Giám sát kết nối** | `adb track-devices` | ADB Daemon | Luồng stream liên tục thông báo sự kiện cắm/rút thiết bị trong thời gian thực. |
| **Gói người dùng** | `pm list packages -3 -f` | Non-Root | Liệt kê các ứng dụng bên thứ 3 (User Apps) kèm đường dẫn APK. |
| **Gói hệ thống** | `pm list packages -s -f` | Non-Root | Liệt kê các ứng dụng được cài đặt mặc định trong ROM hệ thống (System Apps). |
| **Gói đã đóng băng** | `pm list packages -d` | Non-Root | Liệt kê các ứng dụng hiện đang ở trạng thái vô hiệu hóa (Disabled/Frozen). |
| **Buộc dừng app** | `am force-stop <pkg>` | Non-Root | Hủy ngay lập tức các tiến trình đang chạy của ứng dụng, giải phóng RAM. |
| **Dừng hàng loạt siêu tốc** | `sh -c "am force-stop p1; am force-stop p2; ..."` | Non-Root | Gộp chuỗi lệnh dừng trong 1 subprocess shell duy nhất (~100ms cho 30 apps). |
| **Đóng băng app** | `pm disable-user --user 0 <pkg>` | Non-Root (`MANAGE_USERS`) | Đóng băng ứng dụng hoàn toàn đối với User 0 (người dùng chính). |
| **Rã đông app** | `pm enable <pkg>` | Non-Root | Kích hoạt lại ứng dụng, đưa biểu tượng trở lại Launcher. |
| **Gỡ cài đặt User App** | `pm uninstall <pkg>` | Non-Root | Xóa bỏ hoàn toàn ứng dụng và dữ liệu liên quan khỏi thiết bị. |
| **Gỡ bỏ System Bloatware** | `pm uninstall -k --user 0 <pkg>` | Non-Root | Gỡ bỏ hoàn toàn ứng dụng hệ thống/bloatware khỏi User 0 (người dùng chính). |
| **Đọc RAM Kernel & Swap** | `cat /proc/meminfo` | Non-Root | Đọc trực tiếp từ Kernel Linux (~30ms) lấy MemTotal, MemAvailable, SwapTotal, SwapFree. |
| **Đọc RAM theo app (PSS)** | `dumpsys meminfo` | Non-Root | Trích xuất section `Total PSS by process:` và lọc trùng PID bằng `HashSet<u32>`. |
| **Fallback đọc RAM (RSS)**| `ps -A -o PID,NAME,RSS` | Non-Root | Đọc bảng tiến trình và RSS dự phòng khi dumpsys bị từ chối. |

---

## 2. Regular Expression & Output Parsing

### 2.1. Phân tích danh sách thiết bị (`adb devices -l`)
**Đầu ra mẫu:**
```
List of devices attached
RFCT40ABCDE            device product:r8qxxx model:SM_G780G device:r8q transport_id:1
192.168.1.55:5555      device product:husky model:Pixel_8_Pro device:husky transport_id:2
1234567890             unauthorized transport_id:3
```
**Regex Pattern:**
```regex
^(?P<serial>[^\s]+)\s+(?P<status>device|unauthorized|offline|no permissions)(?:\s+product:(?P<product>[^\s]+))?(?:\s+model:(?P<model>[^\s]+))?(?:\s+device:(?P<device>[^\s]+))?
```

### 2.2. Phân tích gói ứng dụng (`pm list packages -f`)
**Đầu ra mẫu:**
```
package:/data/app/~~aBc123xyz==/com.facebook.katana-123/base.apk=com.facebook.katana
package:/system/priv-app/SystemUI/SystemUI.apk=com.android.systemui
```
**Regex Pattern:**
```regex
^package:(?P<apk_path>.+?)=(?P<package_name>[a-zA-Z0-9_\.]+)$
```

### 2.3. Phân tích RAM hệ thống & Swap (`cat /proc/meminfo`)
**Đầu ra mẫu:**
```
MemTotal:        3872972 kB
MemFree:          150392 kB
MemAvailable:     701396 kB
Buffers:            1192 kB
Cached:           719572 kB
SwapTotal:       3145724 kB
SwapFree:         868964 kB
```
**Công thức tính toán:**
- `Total RAM (MB)` = `MemTotal / 1024`
- `Free/Available RAM (MB)` = `MemAvailable / 1024`
- `Used RAM (MB)` = `Total RAM - Free RAM`
- `Swap / RAM Plus Total (MB)` = `SwapTotal / 1024`
- `Swap Used (MB)` = `(SwapTotal - SwapFree) / 1024`

### 2.4. Phân tích RAM tiến trình & Chống lặp (`dumpsys meminfo`)
**Đầu ra mẫu (Section PSS Isolation):**
```
Total PSS by process:
    215,680K: com.facebook.katana (pid 14522)
    124,320K: com.zing.zalo (pid 18901)
     45,100K: com.android.systemui:screenshot (pid 2411)
Total PSS by OOM adjustment:
    215,680K: com.facebook.katana (pid 14522)
```
**Quy tắc Parse:**
1. Chỉ quét các dòng nằm giữa `Total PSS by process:` và tiêu đề section kế tiếp (`Total PSS by OOM adjustment:`).
2. Dùng Regex: `^\s*(?P<ram_kb>[\d,]+)K:\s+(?P<pkg>[a-zA-Z0-9_\.\:]+)(?:\s+\(pid\s+(?P<pid>\d+)\))?`
3. Lưu và kiểm tra `HashSet<u32>` theo PID để ngăn tuyệt đối tình trạng nhân lặp 4 lần bộ nhớ.

### 2.5. Fallback bảng tiến trình (`ps -A -o PID,NAME,RSS`)
**Đầu ra mẫu:**
```
PID NAME RSS
14091 com.facebook.katana 183000
14092 com.facebook.katana:service 12000
```
- Sử dụng khi `dumpsys meminfo` bị từ chối hoặc thiết bị chạy Android Go Edition. Hiển thị nhãn `RAM RSS` minh bạch trên UI.

---

## 3. Whitelist & Safety Boundary Matrix

Để ngăn chặn tối đa việc vô hiệu hóa nhầm ứng dụng cốt lõi của hệ thống (có thể dẫn đến Brick/Loop):

### 3.1. Critical System Whitelist (BẢO VỆ TUYỆT ĐỐI - KHÔNG CHO PHÉP FREEZE/KILL)
- `android` (Core Android Framework)
- `com.android.systemui` (Thanh trạng thái, Navigation bar, Notification shade)
- `com.android.phone` (Dịch vụ gọi thoại, Modem)
- `com.android.server.telecom` (Quản lý cuộc gọi)
- `com.android.providers.telephony` (Dữ liệu tin nhắn & danh bạ)
- `com.android.settings` (Cài đặt hệ điều hành)
- `com.google.android.gms` (Google Play Services cốt lõi)
- `com.google.android.gsf` (Google Services Framework)
- `com.android.vending` (Google Play Store)
- `com.android.inputmethod.latin` (Bàn phím ảo mặc định AOSP)
- `com.google.android.inputmethod.latin` (Gboard)

### 3.2. Recommended Safe Freeze Candidates (Ứng dụng ngốn RAM nên đóng băng khi không dùng)
- Mạng xã hội: `com.facebook.katana`, `com.facebook.orca`, `com.instagram.android`, `com.zhiliaoapp.musically` (TikTok).
- Ứng dụng thương mại điện tử: `com.shopee.vn`, `com.lazada.android`, `com.tiki.app.tikiandroid`.
- Game và ứng dụng giải trí nặng: Các gói game đồ họa cao tiêu thụ bộ nhớ nền.
- Bloatware OEM: Các ứng dụng quảng cáo hoặc ứng dụng rác được nhà mạng/OEM cài sẵn (Samsung Free, Xiaomi Mi Pay,...).
