<script lang="ts">
  import type { DeviceInfo, SystemMemoryInfo } from '../types';

  export let devices: DeviceInfo[] = [];
  export let selectedSerial: string = '';
  export let memory: SystemMemoryInfo = {
    total_ram_mb: 8192,
    used_ram_mb: 4096,
    free_ram_mb: 4096,
    cached_ram_mb: 0,
  };
  export let onSelectDevice: (serial: string) => void;
  export let onRefresh: () => void;
  export let onOpenWirelessModal: () => void;

  $: currentDevice = devices.find((d) => d.serial === selectedSerial);
  $: usedPercentage = memory.total_ram_mb > 0
    ? Math.min(100, Math.round((memory.used_ram_mb / memory.total_ram_mb) * 100))
    : 0;

  function formatMb(mb: number): string {
    if (mb >= 1024) {
      return (mb / 1024).toFixed(1) + ' GB';
    }
    return Math.round(mb) + ' MB';
  }
</script>

<header class="header glass-panel">
  <div class="brand-section">
    <div class="logo-box">
      <span class="snowflake-icon">❄️</span>
      <div class="title-wrap">
        <h1 class="app-title">DroidFrost</h1>
        <span class="version-tag">v1.0 • Non-Root</span>
      </div>
    </div>

    <!-- Device Selector Dropdown -->
    <div class="device-selector-wrap">
      {#if devices.length > 0}
        <div class="custom-select">
          <span class="status-dot online"></span>
          <select
            value={selectedSerial}
            on:change={(e) => onSelectDevice(e.currentTarget.value)}
          >
            {#each devices as d}
              <option value={d.serial}>
                [{d.connection_type === 'Usb' ? 'USB' : 'WiFi'}] {d.model} ({d.serial})
              </option>
            {/each}
          </select>
        </div>
      {:else}
        <div class="device-empty-state">
          <span class="status-dot offline"></span>
          <span class="no-device-text">Chưa kết nối thiết bị nào</span>
        </div>
      {/if}

      <button class="btn btn-secondary btn-icon" on:click={onRefresh} title="Quét lại thiết bị">
        🔄
      </button>
      <button class="btn btn-secondary" on:click={onOpenWirelessModal} title="Kết nối qua Wi-Fi">
        📶 Wireless ADB
      </button>
    </div>
  </div>

  <!-- RAM Visualizer Bar -->
  <div class="memory-section">
    <div class="mem-labels">
      <span class="mem-title">Bộ nhớ RAM thiết bị</span>
      <span class="mem-stats">
        <strong class="used-text">{formatMb(memory.used_ram_mb)}</strong> / {formatMb(memory.total_ram_mb)} ({usedPercentage}%)
      </span>
    </div>

    <div class="progress-track">
      <div
        class="progress-fill"
        style="width: {usedPercentage}%"
        class:warning={usedPercentage > 75}
        class:danger={usedPercentage > 90}
      ></div>
    </div>

    <div class="mem-subtext">
      <span>Trống: <strong>{formatMb(memory.free_ram_mb)}</strong></span>
      <span>Đệm: <strong>{formatMb(memory.cached_ram_mb)}</strong></span>
    </div>
  </div>
</header>

<style>
  .header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 16px 24px;
    border-bottom: 1px solid var(--border-subtle);
    gap: 32px;
  }

  .brand-section {
    display: flex;
    align-items: center;
    gap: 24px;
  }

  .logo-box {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .snowflake-icon {
    font-size: 28px;
    filter: drop-shadow(0 0 10px rgba(0, 210, 255, 0.6));
    animation: spinSlow 30s linear infinite;
  }

  @keyframes spinSlow {
    from { transform: rotate(0deg); }
    to { transform: rotate(360deg); }
  }

  .app-title {
    font-size: 20px;
    font-weight: 800;
    letter-spacing: -0.5px;
    background: var(--accent-gradient);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
  }

  .version-tag {
    font-size: 11px;
    color: var(--text-muted);
    font-weight: 600;
  }

  .device-selector-wrap {
    display: flex;
    align-items: center;
    gap: 10px;
  }

  .custom-select {
    display: flex;
    align-items: center;
    background: var(--bg-surface-elevated);
    border: 1px solid var(--border-subtle);
    border-radius: var(--radius-sm);
    padding: 2px 10px;
    gap: 8px;
  }

  .custom-select select {
    background: transparent;
    border: none;
    color: var(--text-primary);
    font-size: 13px;
    font-weight: 600;
    font-family: inherit;
    outline: none;
    cursor: pointer;
    padding: 6px 0;
  }

  .custom-select select option {
    background: var(--bg-surface);
    color: var(--text-primary);
  }

  .device-empty-state {
    display: flex;
    align-items: center;
    gap: 8px;
    background: rgba(239, 68, 68, 0.1);
    border: 1px solid rgba(239, 68, 68, 0.2);
    padding: 6px 12px;
    border-radius: var(--radius-sm);
  }

  .no-device-text {
    font-size: 12px;
    color: #f87171;
    font-weight: 500;
  }

  .status-dot {
    width: 8px;
    height: 8px;
    border-radius: 50%;
  }

  .status-dot.online {
    background: var(--status-running);
    box-shadow: 0 0 8px var(--status-running);
  }

  .status-dot.offline {
    background: var(--status-danger);
    box-shadow: 0 0 8px var(--status-danger);
  }

  .btn-icon {
    padding: 8px 10px;
  }

  .memory-section {
    display: flex;
    flex-direction: column;
    width: 320px;
    gap: 6px;
  }

  .mem-labels {
    display: flex;
    justify-content: space-between;
    font-size: 12px;
  }

  .mem-title {
    color: var(--text-secondary);
    font-weight: 500;
  }

  .used-text {
    color: var(--accent-cyan);
  }

  .progress-track {
    width: 100%;
    height: 8px;
    background: var(--bg-surface-elevated);
    border-radius: var(--radius-full);
    overflow: hidden;
    border: 1px solid var(--border-subtle);
  }

  .progress-fill {
    height: 100%;
    background: var(--accent-gradient);
    border-radius: var(--radius-full);
    transition: width 0.4s cubic-bezier(0.4, 0, 0.2, 1);
  }

  .progress-fill.warning {
    background: linear-gradient(135deg, #f59e0b 0%, #d97706 100%);
  }

  .progress-fill.danger {
    background: linear-gradient(135deg, #ef4444 0%, #b91c1c 100%);
  }

  .mem-subtext {
    display: flex;
    justify-content: space-between;
    font-size: 11px;
    color: var(--text-muted);
  }

  .mem-subtext strong {
    color: var(--text-secondary);
  }
</style>
