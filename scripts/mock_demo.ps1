# DroidFrost - Mock ADB Demo Runner
Write-Host "❄️ =================================================== ❄️" -ForegroundColor Cyan
Write-Host "          DROIDFROST MOCK ADB DEMO ENVIRONMENT         " -ForegroundColor White
Write-Host "❄️ =================================================== ❄️" -ForegroundColor Cyan
Write-Host "Khởi chạy DroidFrost ở chế độ Giả lập (Mock Mode)..." -ForegroundColor Yellow
Write-Host "Bạn có thể trải nghiệm toàn bộ giao diện, tính năng đóng băng, hẹn giờ mà không cần cắm điện thoại!" -ForegroundColor Green

# Thiết lập biến môi trường MOCK
$env:VITE_MOCK_MODE = "true"
$env:DROIDFROST_MOCK = "1"

# Khởi chạy Vite dev server
npm run dev
