# DroidFrost - Production Desktop Bundler Script
Write-Host "❄️ Bắt đầu quy trình đóng gói DroidFrost Desktop Application (.exe)..." -ForegroundColor Cyan

# 1. Kiểm tra Node & Rust
Write-Host "1. Kiểm tra công cụ..." -ForegroundColor Yellow
node -v
cargo -v

# 2. Cài đặt npm dependencies nếu chưa có
if (-not (Test-Path "node_modules")) {
    Write-Host "2. Cài đặt node dependencies..." -ForegroundColor Yellow
    npm install
}

# 3. Build Frontend
Write-Host "3. Biên dịch giao diện Frontend (Svelte/Vite)..." -ForegroundColor Yellow
npm run build

# 4. Build Tauri Desktop Binary
Write-Host "4. Đóng gói ứng dụng desktop qua Tauri CLI..." -ForegroundColor Yellow
npm run tauri build

Write-Host "✅ Hoàn tất! File thực thi nằm trong: src-tauri/target/release/bundle/msi hoặc nsis" -ForegroundColor Green
