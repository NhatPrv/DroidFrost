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
| **Đóng băng app** | `pm disable-user --user 0 <pkg>` | Non-Root (`android.permission.MANAGE_USERS`) | Đóng băng ứng dụng hoàn toàn đối với User 0 (người dùng chính). |
| **Rã đông app** | `pm enable <pkg>` | Non-Root | Kích hoạt lại ứng dụng, đưa biểu tượng trở lại Launcher. |
| **Đọc RAM tổng thể** | `dumpsys meminfo` | Non-Root | Trích xuất bảng phân bổ RAM (Total RAM, Free RAM, Used RAM, PSS, Buffers/Cached). |
| **Đọc RAM theo app** | `dumpsys meminfo <pkg>` | Non-Root | Trích xuất chi tiết PSS, Private Dirty, Heap Alloc của từng tiến trình cụ thể. |
| **Tiến trình đang chạy** | `dumpsys activity processes` | Non-Root | Quét danh sách các Process Records (PID, UID, Process Name, OOM Adj Level). |

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

### 2.3. Phân tích RAM từ `dumpsys meminfo` (System-wide)
**Đầu ra mẫu:**
```
Total RAM: 7,842,120K (status normal)
 Free RAM: 3,214,560K (  984,200K cached pss + 1,820,360K cached kernel +   410,000K free)
 Used RAM: 4,627,560K (3,520,120K used pss + 1,107,440K kernel)
```
**Regex Pattern:**
```regex
Total RAM:\s*(?P<total_ram>[\d,]+)K
Free RAM:\s*(?P<free_ram>[\d,]+)K
Used RAM:\s*(?P<used_ram>[\d,]+)K
```

### 2.4. Phân tích RAM theo tiến trình (`dumpsys meminfo <pkg>`)
**Đầu ra mẫu:**
```
** MEMINFO in pid 14522 [com.facebook.katana] **
                   Pss  Private  Private  SwapPss     Heap     Heap     Heap
                 Total    Dirty    Clean    Dirty     Size    Alloc     Free
                ------   ------   ------   ------   ------   ------   ------
  Native Heap    45120    44892        0        0    98304    75230    23074
  Dalvik Heap    62340    61200        0        0    84210    58120    26090
        TOTAL   215680   185200     4200     1200   182514   133350    49164
```
**Regex Pattern:**
```regex
TOTAL\s+(?P<total_pss>\d+)\s+(?P<private_dirty>\d+)
```

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
