import type {
  DeviceInfo,
  OneClickBoostResult,
  OperationResult,
  ProcessInfo,
  SystemMemoryInfo,
} from '../types';

let tauriInvoke: any = null;

async function getInvoke() {
  if (typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window) {
    try {
      const { invoke } = await import('@tauri-apps/api/core');
      tauriInvoke = invoke;
      return tauriInvoke;
    } catch {
      return null;
    }
  }
  return null;
}

// Dữ liệu Mock thông minh khi chạy trên trình duyệt hoặc chế độ Test
const MOCK_DEVICES: DeviceInfo[] = [
  {
    serial: 'RFCT40MOCK1',
    model: 'Galaxy S24 Ultra',
    product: 'e3q',
    status: 'Device',
    connection_type: 'Usb',
  },
  {
    serial: '192.168.1.88:5555',
    model: 'Pixel 8 Pro',
    product: 'husky',
    status: 'Device',
    connection_type: 'Wireless',
  },
];

let mockProcesses: ProcessInfo[] = [
  {
    pid: 14522,
    package_name: 'com.facebook.katana',
    app_name: 'Facebook',
    ram_mb: 320.5,
    is_running: true,
    is_frozen: false,
    is_scheduled: false,
    is_system: false,
    is_whitelisted: false,
    scheduled_remaining_seconds: null,
  },
  {
    pid: 18902,
    package_name: 'com.zhiliaoapp.musically',
    app_name: 'TikTok',
    ram_mb: 480.2,
    is_running: true,
    is_frozen: false,
    is_scheduled: false,
    is_system: false,
    is_whitelisted: false,
    scheduled_remaining_seconds: null,
  },
  {
    pid: 8812,
    package_name: 'com.zing.zalo',
    app_name: 'Zalo',
    ram_mb: 210.0,
    is_running: true,
    is_frozen: false,
    is_scheduled: false,
    is_system: false,
    is_whitelisted: false,
    scheduled_remaining_seconds: null,
  },
  {
    pid: null,
    package_name: 'com.shopee.vn',
    app_name: 'Shopee',
    ram_mb: 0.0,
    is_running: false,
    is_frozen: true,
    is_scheduled: true,
    is_system: false,
    is_whitelisted: false,
    scheduled_remaining_seconds: 1420,
  },
  {
    pid: 2400,
    package_name: 'com.android.systemui',
    app_name: 'SystemUI',
    ram_mb: 185.0,
    is_running: true,
    is_frozen: false,
    is_scheduled: false,
    is_system: true,
    is_whitelisted: true,
    scheduled_remaining_seconds: null,
  },
  {
    pid: 3100,
    package_name: 'com.google.android.gms',
    app_name: 'Google Play Services',
    ram_mb: 260.4,
    is_running: true,
    is_frozen: false,
    is_scheduled: false,
    is_system: true,
    is_whitelisted: true,
    scheduled_remaining_seconds: null,
  },
  {
    pid: 9540,
    package_name: 'com.dts.freefireth',
    app_name: 'Free Fire',
    ram_mb: 850.6,
    is_running: true,
    is_frozen: false,
    is_scheduled: false,
    is_system: false,
    is_whitelisted: false,
    scheduled_remaining_seconds: null,
  },
];

let mockMemory: SystemMemoryInfo = {
  total_ram_mb: 8192.0,
  used_ram_mb: 4850.0,
  free_ram_mb: 3342.0,
  cached_ram_mb: 1250.0,
};

export const api = {
  async getDevices(): Promise<DeviceInfo[]> {
    const inv = await getInvoke();
    if (inv) {
      try {
        return await inv('get_devices');
      } catch (e) {
        console.warn('Fallback to mock devices', e);
      }
    }
    return MOCK_DEVICES;
  },

  async getDeviceState(serial: string): Promise<[SystemMemoryInfo, ProcessInfo[]]> {
    const inv = await getInvoke();
    if (inv) {
      try {
        return await inv('get_device_state', { serial });
      } catch (e) {
        console.warn('Fallback to mock state', e);
      }
    }
    return [mockMemory, [...mockProcesses]];
  },

  async killApp(serial: string, pkg: string): Promise<OperationResult> {
    const inv = await getInvoke();
    if (inv) {
      return await inv('kill_app', { serial, pkg });
    }
    // Mock
    mockProcesses = mockProcesses.map((p) =>
      p.package_name === pkg ? { ...p, is_running: false, pid: null, ram_mb: 0 } : p
    );
    return { success: true, message: `[Mock] Đã buộc dừng ${pkg}` };
  },

  async freezeApp(serial: string, pkg: string): Promise<OperationResult> {
    const inv = await getInvoke();
    if (inv) {
      return await inv('freeze_app', { serial, pkg });
    }
    // Mock
    mockProcesses = mockProcesses.map((p) =>
      p.package_name === pkg
        ? { ...p, is_frozen: true, is_running: false, pid: null, ram_mb: 0 }
        : p
    );
    return { success: true, message: `[Mock] Đã đóng băng ${pkg}` };
  },

  async unfreezeApp(serial: string, pkg: string): Promise<OperationResult> {
    const inv = await getInvoke();
    if (inv) {
      return await inv('unfreeze_app', { serial, pkg });
    }
    // Mock
    mockProcesses = mockProcesses.map((p) =>
      p.package_name === pkg
        ? { ...p, is_frozen: false, is_scheduled: false, scheduled_remaining_seconds: null }
        : p
    );
    return { success: true, message: `[Mock] Đã rã đông ${pkg}` };
  },

  async scheduleFreeze(
    serial: string,
    pkg: string,
    durationSeconds: number
  ): Promise<OperationResult> {
    const inv = await getInvoke();
    if (inv) {
      return await inv('schedule_freeze', { serial, pkg, durationSeconds });
    }
    // Mock
    mockProcesses = mockProcesses.map((p) =>
      p.package_name === pkg
        ? {
            ...p,
            is_frozen: true,
            is_running: false,
            pid: null,
            ram_mb: 0,
            is_scheduled: true,
            scheduled_remaining_seconds: durationSeconds,
          }
        : p
    );
    return {
      success: true,
      message: `[Mock] Đã đóng băng và hẹn giờ rã đông ${pkg} sau ${durationSeconds}s`,
    };
  },

  async cancelSchedule(serial: string, pkg: string): Promise<OperationResult> {
    const inv = await getInvoke();
    if (inv) {
      return await inv('cancel_schedule', { serial, pkg });
    }
    return this.unfreezeApp(serial, pkg);
  },

  async oneClickBoost(serial: string): Promise<OneClickBoostResult> {
    const inv = await getInvoke();
    if (inv) {
      return await inv('one_click_boost', { serial });
    }
    let freed = 0;
    let count = 0;
    const pkgs: string[] = [];
    mockProcesses = mockProcesses.map((p) => {
      if (p.is_running && !p.is_system && !p.is_whitelisted) {
        freed += p.ram_mb;
        count += 1;
        pkgs.push(p.package_name);
        return { ...p, is_running: false, pid: null, ram_mb: 0 };
      }
      return p;
    });
    mockMemory.free_ram_mb += freed;
    mockMemory.used_ram_mb -= freed;
    return {
      killed_count: count,
      frozen_count: 0,
      freed_ram_mb: Math.round(freed * 10) / 10,
      packages_affected: pkgs,
    };
  },

  async connectWireless(ip: string, port: number): Promise<string> {
    const inv = await getInvoke();
    if (inv) {
      return await inv('connect_wireless', { ip, port });
    }
    return `[Mock] Đã kết nối thành công tới ${ip}:${port}`;
  },

  async pairWireless(ip: string, port: number, code: string): Promise<string> {
    const inv = await getInvoke();
    if (inv) {
      return await inv('pair_wireless', { ip, port, code });
    }
    return `[Mock] Đã ghép nối thành công thiết bị ${ip}:${port}`;
  },
};
