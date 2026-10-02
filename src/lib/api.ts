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

export const api = {
  async getDevices(): Promise<DeviceInfo[]> {
    const inv = await getInvoke();
    if (!inv) return [];
    return await inv('get_devices');
  },

  async getDeviceState(serial: string): Promise<[SystemMemoryInfo, ProcessInfo[]]> {
    const inv = await getInvoke();
    if (!inv) {
      return [
        { total_ram_mb: 0, used_ram_mb: 0, free_ram_mb: 0, cached_ram_mb: 0 },
        [],
      ];
    }
    return await inv('get_device_state', { serial });
  },

  async killApp(serial: string, pkg: string): Promise<OperationResult> {
    const inv = await getInvoke();
    if (!inv) return { success: false, message: 'Chưa kết nối Tauri Backend' };
    return await inv('kill_app', { serial, pkg });
  },

  async freezeApp(serial: string, pkg: string): Promise<OperationResult> {
    const inv = await getInvoke();
    if (!inv) return { success: false, message: 'Chưa kết nối Tauri Backend' };
    return await inv('freeze_app', { serial, pkg });
  },

  async unfreezeApp(serial: string, pkg: string): Promise<OperationResult> {
    const inv = await getInvoke();
    if (!inv) return { success: false, message: 'Chưa kết nối Tauri Backend' };
    return await inv('unfreeze_app', { serial, pkg });
  },

  async scheduleFreeze(
    serial: string,
    pkg: string,
    durationSeconds: number
  ): Promise<OperationResult> {
    const inv = await getInvoke();
    if (!inv) return { success: false, message: 'Chưa kết nối Tauri Backend' };
    return await inv('schedule_freeze', { serial, pkg, durationSeconds });
  },

  async cancelSchedule(serial: string, pkg: string): Promise<OperationResult> {
    const inv = await getInvoke();
    if (!inv) return { success: false, message: 'Chưa kết nối Tauri Backend' };
    return await inv('cancel_schedule', { serial, pkg });
  },

  async oneClickBoost(serial: string): Promise<OneClickBoostResult> {
    const inv = await getInvoke();
    if (!inv) {
      return {
        killed_count: 0,
        frozen_count: 0,
        freed_ram_mb: 0,
        packages_affected: [],
      };
    }
    return await inv('one_click_boost', { serial });
  },

  async stopAllRunning(serial: string, includeSystem: boolean = false): Promise<OneClickBoostResult> {
    const inv = await getInvoke();
    if (!inv) {
      return {
        killed_count: 0,
        frozen_count: 0,
        freed_ram_mb: 0,
        packages_affected: [],
      };
    }
    return await inv('stop_all_running', { serial, includeSystem });
  },

  async connectWireless(ip: string, port: number): Promise<string> {
    const inv = await getInvoke();
    if (!inv) return 'Chưa kết nối Tauri Backend';
    return await inv('connect_wireless', { ip, port });
  },

  async pairWireless(ip: string, port: number, code: string): Promise<string> {
    const inv = await getInvoke();
    if (!inv) return 'Chưa kết nối Tauri Backend';
    return await inv('pair_wireless', { ip, port, code });
  },
};
