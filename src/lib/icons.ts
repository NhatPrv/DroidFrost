// DroidFrost - Comprehensive Vector App Icon Engine & Brand Assets

export interface AppIconDefinition {
  svg: string;
  bg?: string;
}

// Bộ icon SVG chính xác cho các ứng dụng phổ biến trên Android
const BRAND_ICONS: Record<string, string> = {
  // Google Chrome
  'com.android.chrome': `
    <svg viewBox="0 0 48 48" fill="none" xmlns="http://www.w3.org/2000/svg">
      <circle cx="24" cy="24" r="20" fill="#EA4335"/>
      <path d="M24 14H42.5C38.5 7.5 31.5 4 24 4C18.5 4 13.5 6.5 10 10.5L19 26L24 14Z" fill="#EA4335"/>
      <path d="M24 34L15 18L5.5 18C4.5 20 4 22 4 24C4 32.5 9.5 39.5 17 42.5L24 34Z" fill="#34A853"/>
      <path d="M24 34L33 18H42.5C43.5 20 44 22 44 24C44 35 35 44 24 44L17 42.5L24 34Z" fill="#FBBC05"/>
      <circle cx="24" cy="24" r="9" fill="#FFFFFF"/>
      <circle cx="24" cy="24" r="7" fill="#4285F4"/>
    </svg>`,

  // YouTube
  'com.google.android.youtube': `
    <svg viewBox="0 0 48 48" fill="none" xmlns="http://www.w3.org/2000/svg">
      <rect width="48" height="48" rx="12" fill="#FF0000"/>
      <path d="M38 18C38 15 36 13 33 13H15C12 13 10 15 10 18V30C10 33 12 35 15 35H33C36 35 38 33 38 30V18Z" fill="#FF0000"/>
      <path d="M20 18L30 24L20 30V18Z" fill="#FFFFFF"/>
    </svg>`,

  // Facebook
  'com.facebook.katana': `
    <svg viewBox="0 0 48 48" fill="none" xmlns="http://www.w3.org/2000/svg">
      <rect width="48" height="48" rx="12" fill="#1877F2"/>
      <path d="M32 25H27V42H20V25H16V19H20V15C20 11.7 22.7 9 26 9H31V15H28C26.9 15 26 15.9 26 17V19H32L31 25Z" fill="#FFFFFF"/>
    </svg>`,

  // Messenger
  'com.facebook.orca': `
    <svg viewBox="0 0 48 48" fill="none" xmlns="http://www.w3.org/2000/svg">
      <rect width="48" height="48" rx="12" fill="url(#msg-grad)"/>
      <defs>
        <linearGradient id="msg-grad" x1="0" y1="0" x2="48" y2="48" gradientUnits="userSpaceOnUse">
          <stop stop-color="#00C6FF"/>
          <stop offset="0.5" stop-color="#0078FF"/>
          <stop offset="1" stop-color="#A033FF"/>
        </linearGradient>
      </defs>
      <path d="M24 10C16.3 10 10 15.8 10 23C10 27 12 30.5 15.3 32.7V37.5L20 35C21.3 35.3 22.6 35.5 24 35.5C31.7 35.5 38 29.7 38 22.5C38 15.3 31.7 10 24 10ZM25.5 27.5L21.5 23.2L13.8 27.5L22.2 18.5L26.5 22.8L33.8 18.5L25.5 27.5Z" fill="#FFFFFF"/>
    </svg>`,

  // TikTok
  'com.zhiliaoapp.musically': `
    <svg viewBox="0 0 48 48" fill="none" xmlns="http://www.w3.org/2000/svg">
      <rect width="48" height="48" rx="12" fill="#000000"/>
      <path d="M33 16C31 16 29 14.5 28.5 12.5H24V31C24 34.3 21.3 37 18 37C14.7 37 12 34.3 12 31C12 27.7 14.7 25 18 25C19 25 20 25.3 20.8 25.8V21.1C19.9 20.8 19 20.6 18 20.6C12.2 20.6 7.6 25.3 7.6 31C7.6 36.8 12.3 41.5 18 41.5C23.8 41.5 28.5 36.8 28.5 31V20.5C30.6 22 33.2 22.9 36 22.9V18.5C34.9 18.5 33.9 17.5 33 16Z" fill="#25F4EE"/>
      <path d="M34 17C32 17 30 15.5 29.5 13.5H25V32C25 35.3 22.3 38 19 38C15.7 38 13 35.3 13 32C13 28.7 15.7 26 19 26C20 26 21 26.3 21.8 26.8V22.1C20.9 21.8 20 21.6 19 21.6C13.2 21.6 8.6 26.3 8.6 32C8.6 37.8 13.3 42.5 19 42.5C24.8 42.5 29.5 37.8 29.5 32V21.5C31.6 23 34.2 23.9 37 23.9V19.5C35.9 19.5 34.9 18.5 34 17Z" fill="#FE2C55"/>
      <path d="M33.5 16.5C31.5 16.5 29.5 15 29 13H24.5V31.5C24.5 34.8 21.8 37.5 18.5 37.5C15.2 37.5 12.5 34.8 12.5 31.5C12.5 28.2 15.2 25.5 18.5 25.5C19.5 25.5 20.5 25.8 21.3 26.3V21.6C20.4 21.3 19.5 21.1 18.5 21.1C12.7 21.1 8.1 25.8 8.1 31.5C8.1 37.3 12.8 42 18.5 42C24.3 42 29 37.3 29 31.5V21C31.1 22.5 33.7 23.4 36.5 23.4V19C35.4 19 34.4 18 33.5 16.5Z" fill="#FFFFFF"/>
    </svg>`,

  // Zalo
  'com.zing.zalo': `
    <svg viewBox="0 0 48 48" fill="none" xmlns="http://www.w3.org/2000/svg">
      <rect width="48" height="48" rx="12" fill="#0068FF"/>
      <path d="M12 16H23L12 30H24V34H11V30L22 16H12V12H24V16H12Z" fill="#FFFFFF"/>
      <circle cx="31" cy="22" r="8" fill="#FFFFFF"/>
      <circle cx="31" cy="22" r="5" fill="#0068FF"/>
    </svg>`,

  // Shopee
  'com.shopee.vn': `
    <svg viewBox="0 0 48 48" fill="none" xmlns="http://www.w3.org/2000/svg">
      <rect width="48" height="48" rx="12" fill="#EE4D2D"/>
      <path d="M24 10C21.2 10 19 12.2 19 15V17H14L16 38H32L34 17H29V15C29 12.2 26.8 10 24 10ZM21 15C21 13.3 22.3 12 24 12C25.7 12 27 13.3 27 15V17H21V15ZM24 23C27 23 28 24.5 28 26C28 28.5 25 29 23 29.5C21 30 20 30.5 20 31.5C20 32.5 21 33.5 24 33.5C27 33.5 28 32.5 28 32.5V34.5C28 34.5 26.5 35.5 24 35.5C20.5 35.5 18 34 18 31.5C18 29 21 28.5 23 28C25 27.5 26 27 26 26C26 25 25 24.5 23.5 24.5C21 24.5 20 25.5 20 25.5V23.5C20 23.5 21.5 23 24 23Z" fill="#FFFFFF"/>
    </svg>`,

  // Spotify
  'com.spotify.music': `
    <svg viewBox="0 0 48 48" fill="none" xmlns="http://www.w3.org/2000/svg">
      <circle cx="24" cy="24" r="22" fill="#1DB954"/>
      <path d="M33.5 19C26.5 14.8 15 14.5 8.5 16.5C7.5 16.8 6.5 16.2 6.2 15.2C5.9 14.2 6.5 13.2 7.5 12.9C15.2 10.6 28 11 36 15.8C36.9 16.3 37.2 17.5 36.7 18.4C36.2 19.2 34.4 19.5 33.5 19ZM32.8 24.8C32.3 25.6 31.2 25.8 30.4 25.3C24.8 21.8 16.4 20.8 9.9 22.8C9 23.1 8 22.5 7.7 21.6C7.4 20.7 8 19.7 8.9 19.4C16.4 17.1 25.7 18.2 32.3 22.2C33.1 22.6 33.3 24 32.8 24.8ZM30.2 30.5C29.8 31.1 29 31.3 28.4 30.9C23.6 28 17.5 27.3 10.3 28.9C9.6 29.1 8.9 28.6 8.7 27.9C8.5 27.2 9 26.5 9.7 26.3C17.6 24.5 24.3 25.3 29.7 28.6C30.4 29 30.6 29.8 30.2 30.5Z" fill="#000000"/>
    </svg>`,

  // Netflix
  'com.netflix.mediaclient': `
    <svg viewBox="0 0 48 48" fill="none" xmlns="http://www.w3.org/2000/svg">
      <rect width="48" height="48" rx="12" fill="#000000"/>
      <path d="M14 8H20V40H14V8Z" fill="#E50914"/>
      <path d="M28 8H34V40H28V8Z" fill="#E50914"/>
      <path d="M14 8L34 40H28L14 18V8Z" fill="#B81D24"/>
    </svg>`,

  // Telegram
  'org.telegram.messenger': `
    <svg viewBox="0 0 48 48" fill="none" xmlns="http://www.w3.org/2000/svg">
      <circle cx="24" cy="24" r="22" fill="#24A1DE"/>
      <path d="M35 13L9 23.5L16.5 26.5L19.5 35L24 30L29.5 34L35 13Z" fill="#FFFFFF"/>
      <path d="M16.5 26.5L31 16L21 28L19.5 35L16.5 26.5Z" fill="#D2EBF7"/>
    </svg>`,

  // Google Play Store
  'com.android.vending': `
    <svg viewBox="0 0 48 48" fill="none" xmlns="http://www.w3.org/2000/svg">
      <rect width="48" height="48" rx="12" fill="#202124"/>
      <path d="M10 7L27 24L10 41V7Z" fill="#00E676"/>
      <path d="M32 19L27 24L10 7L32 19Z" fill="#00B0FF"/>
      <path d="M32 29L10 41L27 24L32 29Z" fill="#FF1744"/>
      <path d="M37 24L32 19L27 24L32 29L37 24Z" fill="#FFEA00"/>
    </svg>`,

  // System UI / Android System
  'com.android.systemui': `
    <svg viewBox="0 0 48 48" fill="none" xmlns="http://www.w3.org/2000/svg">
      <rect width="48" height="48" rx="12" fill="#1E293B"/>
      <circle cx="24" cy="24" r="14" stroke="#38BDF8" stroke-width="3"/>
      <circle cx="24" cy="24" r="5" fill="#38BDF8"/>
      <path d="M24 6V10M24 38V42M6 24H10M38 24H42" stroke="#38BDF8" stroke-width="3" stroke-linecap="round"/>
    </svg>`,

  // Android Framework Core
  'android': `
    <svg viewBox="0 0 48 48" fill="none" xmlns="http://www.w3.org/2000/svg">
      <rect width="48" height="48" rx="12" fill="#3DDC84"/>
      <path d="M15 15L12 10M33 15L36 10" stroke="#FFFFFF" stroke-width="2.5" stroke-linecap="round"/>
      <path d="M11 26C11 18.8 16.8 13 24 13C31.2 13 37 18.8 37 26H11Z" fill="#FFFFFF"/>
      <circle cx="18" cy="20" r="2" fill="#3DDC84"/>
      <circle cx="30" cy="20" r="2" fill="#3DDC84"/>
    </svg>`,

  // Google Play Services
  'com.google.android.gms': `
    <svg viewBox="0 0 48 48" fill="none" xmlns="http://www.w3.org/2000/svg">
      <rect width="48" height="48" rx="12" fill="#F8FAFC"/>
      <path d="M24 10C16.3 10 10 16.3 10 24C10 31.7 16.3 38 24 38C31.7 38 38 31.7 38 24H24V29H32.5C31.3 32.5 28 34.5 24 34.5C18.2 34.5 13.5 29.8 13.5 24C13.5 18.2 18.2 13.5 24 13.5C26.8 13.5 29.3 14.5 31.2 16.3L34.7 12.8C31.9 10.2 28.1 10 24 10Z" fill="#4285F4"/>
    </svg>`,

  // Phone / Dialer
  'com.android.phone': `
    <svg viewBox="0 0 48 48" fill="none" xmlns="http://www.w3.org/2000/svg">
      <rect width="48" height="48" rx="12" fill="#10B981"/>
      <path d="M33 29L28.5 27.5L25 31C20.5 28.5 19.5 27.5 17 23L20.5 19.5L19 15H15C13.5 15 12 16.5 12.5 18C14 27 21 34 30 35.5C31.5 36 33 34.5 33 33V29Z" fill="#FFFFFF"/>
    </svg>`,

  // Settings
  'com.android.settings': `
    <svg viewBox="0 0 48 48" fill="none" xmlns="http://www.w3.org/2000/svg">
      <rect width="48" height="48" rx="12" fill="#475569"/>
      <path d="M24 18C20.7 18 18 20.7 18 24C18 27.3 20.7 30 24 30C27.3 30 30 27.3 30 24C30 20.7 27.3 18 24 18ZM36.5 22.5L34.2 21.8C34 21 33.7 20.2 33.2 19.5L34.2 17.3C34.5 16.6 34.3 15.8 33.7 15.3L32.7 14.3C32.2 13.7 31.4 13.5 30.7 13.8L28.5 14.8C27.8 14.3 27 14 26.2 13.8L25.5 11.5C25.3 10.7 24.6 10.2 23.8 10.2H22.2C21.4 10.2 20.7 10.7 20.5 11.5L19.8 13.8C19 14 18.2 14.3 17.5 14.8L15.3 13.8C14.6 13.5 13.8 13.7 13.3 14.3L12.3 15.3C11.7 15.8 11.5 16.6 11.8 17.3L12.8 19.5C12.3 20.2 12 21 11.8 21.8L9.5 22.5C8.7 22.7 8.2 23.4 8.2 24.2V25.8C8.2 26.6 8.7 27.3 9.5 27.5L11.8 28.2C12 29 12.3 29.8 12.8 30.5L11.8 32.7C11.5 33.4 11.7 34.2 12.3 34.7L13.3 35.7C13.8 36.3 14.6 36.5 15.3 36.2L17.5 35.2C18.2 35.7 19 36 19.8 36.2L20.5 38.5C20.7 39.3 21.4 39.8 22.2 39.8H23.8C24.6 39.8 25.3 39.3 25.5 38.5L26.2 36.2C27 36 27.8 35.7 28.5 35.2L30.7 36.2C31.4 36.5 32.2 36.3 32.7 35.7L33.7 34.7C34.3 34.2 34.5 33.4 34.2 32.7L33.2 30.5C33.7 29.8 34 29 34.2 28.2L36.5 27.5C37.3 27.3 37.8 26.6 37.8 25.8V24.2C37.8 23.4 37.3 22.7 36.5 22.5Z" fill="#F1F5F9"/>
    </svg>`,
};

/**
 * Trả về SVG icon vector chính hãng nếu có sẵn, hoặc tạo Adaptive Vector Icon chuẩn Material You
 */
export function getAppIconSvg(packageName: string, appName: string): string {
  // 1. Kiểm tra chính xác package
  if (BRAND_ICONS[packageName]) {
    return BRAND_ICONS[packageName];
  }

  // 2. Tìm kiếm theo tiền tố hoặc tên phổ biến
  const pkgLower = packageName.toLowerCase();
  for (const [key, svg] of Object.entries(BRAND_ICONS)) {
    if (pkgLower.includes(key.replace('com.', '').replace('.android', ''))) {
      return svg;
    }
  }

  // 3. Tạo Adaptive Vector Icon chuẩn Android 14 / Material You
  // Tạo màu thương hiệu theo hash chuỗi package
  let hash = 0;
  for (let i = 0; i < packageName.length; i++) {
    hash = packageName.charCodeAt(i) + ((hash << 5) - hash);
  }

  const hue1 = Math.abs(hash) % 360;
  const hue2 = (hue1 + 40) % 360;
  const initial = (appName || packageName.split('.').pop() || '?').charAt(0).toUpperCase();

  return `
    <svg viewBox="0 0 48 48" fill="none" xmlns="http://www.w3.org/2000/svg">
      <defs>
        <linearGradient id="grad-${Math.abs(hash)}" x1="0" y1="0" x2="48" y2="48" gradientUnits="userSpaceOnUse">
          <stop stop-color="hsl(${hue1}, 70%, 55%)"/>
          <stop offset="1" stop-color="hsl(${hue2}, 80%, 42%)"/>
        </linearGradient>
      </defs>
      <rect width="48" height="48" rx="12" fill="url(#grad-${Math.abs(hash)})"/>
      <rect x="1" y="1" width="46" height="46" rx="11" stroke="rgba(255,255,255,0.2)" stroke-width="1.5"/>
      <text x="24" y="31" font-family="-apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif" font-size="20" font-weight="700" fill="#FFFFFF" text-anchor="middle" filter="drop-shadow(0 2px 4px rgba(0,0,0,0.3))">
        ${initial}
      </text>
    </svg>
  `;
}
