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
  is_running: boolean;
  is_frozen: boolean;
  is_scheduled: boolean;
  is_system: boolean;
  is_whitelisted: boolean;
  scheduled_remaining_seconds: number | null;
}

export interface SystemMemoryInfo {
  total_ram_mb: number;
  used_ram_mb: number;
  free_ram_mb: number;
  cached_ram_mb: number;
  swap_total_mb: number;
  swap_used_mb: number;
  process_metric: 'PSS' | 'RSS';
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

export interface AppStorageInfo {
  package_name: string;
  apk_size_mb: number;
  data_size_mb: number;
  cache_size_mb: number;
  total_storage_mb: number;
}

export type TabFilter = 'all' | 'running' | 'frozen' | 'scheduled';
