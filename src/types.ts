export type DeviceStatus = 'Device' | 'Unauthorized' | 'Offline' | 'Unknown';
export type ConnectionType = 'Usb' | 'Wireless';

export interface DeviceInfo {
  serial: string;
  model: string;
  product: string;
  status: DeviceStatus;
  connection_type: ConnectionType;
}

export interface ProcessInfo {
  pid: number | null;
  package_name: string;
  app_name: string;
  ram_mb: number;
  is_running: bool;
  is_frozen: bool;
  is_scheduled: bool;
  is_system: bool;
  is_whitelisted: bool;
  scheduled_remaining_seconds: number | null;
}

export interface SystemMemoryInfo {
  total_ram_mb: number;
  used_ram_mb: number;
  free_ram_mb: number;
  cached_ram_mb: number;
}

export interface OneClickBoostResult {
  killed_count: number;
  frozen_count: number;
  freed_ram_mb: number;
  packages_affected: string[];
}

export interface OperationResult {
  success: boolean;
  message: string;
  data?: any;
}

export type TabFilter = 'all' | 'running' | 'frozen' | 'scheduled';
