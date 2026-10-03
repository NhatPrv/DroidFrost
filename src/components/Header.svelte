<script lang="ts">
  import type { DeviceInfo, SystemMemoryInfo } from '../types';

  export let devices: DeviceInfo[] = [];
  export let selectedSerial: string = '';
  export let memory: SystemMemoryInfo = {
    total_ram_mb: 0,
    used_ram_mb: 0,
    free_ram_mb: 0,
    cached_ram_mb: 0,
    swap_total_mb: 0,
    swap_used_mb: 0,
  };
  export let onSelectDevice: (serial: string) => void;
  export let onRefresh: () => void;
  export let onOpenWirelessModal: () => void;

  $: currentDevice = devices.find((d) => d.serial === selectedSerial);
  $: usedPercentage = memory.total_ram_mb > 0
    ? Math.min(100, Math.round((memory.used_ram_mb / memory.total_ram_mb) * 100))
    : 0;

  function formatMb(mb: number): string {
    if (mb <= 0) return '0 MB';
    if (mb >= 1024) {
      return (mb / 1024).toFixed(1) + ' GB';
    }
    return Math.round(mb) + ' MB';
  }
</script>

<header class="app-header">
  <div class="brand-row">
    <!-- Clean Vector Logo -->
    <div class="brand-identity">
      <svg class="brand-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
        <path d="M12 2v20M2 12h20M4.93 4.93l14.14 14.14M19.07 4.93L4.93 19.07"/>
      </svg>
      <div class="brand-meta">
        <span class="brand-name">DroidFrost</span>
        <span class="brand-sub">Non-Root Device Optimizer</span>
      </div>
    </div>

    <!-- Active Device Selector -->
    <div class="device-dock">
      {#if devices.length > 0}
        <div class="device-pill">
          <span class="status-indicator online"></span>
          <select
            value={selectedSerial}
            on:change={(e) => onSelectDevice(e.currentTarget.value)}
          >
            {#each devices as d}
              <option value={d.serial}>
                {d.model} ({d.serial}) • {d.connection_type}
              </option>
            {/each}
          </select>
        </div>
      {:else}
        <div class="device-pill offline">
          <span class="status-indicator offline"></span>
          <span class="device-none">Không có thiết bị kết nối</span>
        </div>
      {/if}

      <button class="btn btn-tool" on:click={onRefresh} title="Quét lại thiết bị (Refresh)">
        <svg viewBox="0 0 24 24" width="14" height="14" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M21.5 2v6h-6M21.34 15.57a10 10 0 1 1-.57-8.38l5.67-5.67"/>
        </svg>
        Quét lại
      </button>

      <button class="btn btn-tool" on:click={onOpenWirelessModal} title="Kết nối qua Wi-Fi ADB">
        <svg viewBox="0 0 24 24" width="14" height="14" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M5 12.55a11 11 0 0 1 14.08 0M1.42 9a16 16 0 0 1 21.16 0M8.53 16.11a6 6 0 0 1 6.95 0M12 20h.01"/>
        </svg>
        Wireless ADB
      </button>
    </div>
  </div>

  <!-- Memory Metric Strip -->
  {#if memory.total_ram_mb > 0}
    <div class="memory-strip">
      <div class="metric-block">
        <span class="metric-label">RAM ĐÃ DÙNG</span>
        <span class="metric-val">{formatMb(memory.used_ram_mb)} <small>({usedPercentage}%)</small></span>
      </div>

      <div class="metric-bar-container">
        <div class="metric-bar-track">
          <div
            class="metric-bar-fill"
            style="width: {usedPercentage}%"
            class:high-load={usedPercentage > 85}
          ></div>
        </div>
        <div class="metric-sub-stats">
          <span>Khả dụng: {formatMb(memory.free_ram_mb)}</span>
          <span>Đệm: {formatMb(memory.cached_ram_mb)}</span>
          <span>RAM vật lý: {formatMb(memory.total_ram_mb)}</span>
          {#if memory.swap_total_mb > 0}
            <span class="swap-stat" title="Bộ nhớ ảo Swap / RAM Plus của Android">
              Swap (RAM Plus): {formatMb(memory.swap_used_mb)} / {formatMb(memory.swap_total_mb)}
            </span>
          {/if}
        </div>
      </div>
    </div>
  {/if}
</header>

<style>
  .app-header {
    background: var(--bg-surface);
    border-bottom: 1px solid var(--border-subtle);
    padding: 14px 20px;
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .brand-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 16px;
  }

  .brand-identity {
    display: flex;
    align-items: center;
    gap: 10px;
  }

  .brand-icon {
    width: 24px;
    height: 24px;
    color: var(--accent-frost);
  }

  .brand-meta {
    display: flex;
    flex-direction: column;
  }

  .brand-name {
    font-size: 15px;
    font-weight: 700;
    color: var(--text-primary);
    letter-spacing: -0.3px;
  }

  .brand-sub {
    font-size: 10px;
    color: var(--text-muted);
    font-weight: 500;
  }

  .device-dock {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .device-pill {
    display: flex;
    align-items: center;
    background: var(--bg-surface-elevated);
    border: 1px solid var(--border-medium);
    border-radius: var(--radius-sm);
    padding: 4px 10px;
    gap: 8px;
  }

  .device-pill select {
    background: transparent;
    border: none;
    color: var(--text-primary);
    font-size: 12px;
    font-weight: 600;
    font-family: inherit;
    outline: none;
    cursor: pointer;
  }

  .device-pill select option {
    background: var(--bg-surface);
  }

  .device-none {
    font-size: 12px;
    color: var(--text-muted);
  }

  .status-indicator {
    width: 6px;
    height: 6px;
    border-radius: 50%;
  }

  .status-indicator.online {
    background: var(--status-running);
    box-shadow: 0 0 6px rgba(34, 197, 94, 0.4);
  }

  .status-indicator.offline {
    background: var(--status-danger);
  }

  .btn-tool {
    padding: 5px 10px;
  }

  .memory-strip {
    display: flex;
    align-items: center;
    gap: 20px;
    background: var(--bg-surface-elevated);
    padding: 8px 14px;
    border-radius: var(--radius-sm);
    border: 1px solid var(--border-subtle);
  }

  .metric-block {
    display: flex;
    flex-direction: column;
    min-width: 130px;
  }

  .metric-label {
    font-size: 9px;
    font-weight: 700;
    color: var(--text-muted);
    letter-spacing: 0.5px;
  }

  .metric-val {
    font-size: 13px;
    font-weight: 700;
    color: var(--accent-frost);
    font-family: var(--font-mono);
  }

  .metric-val small {
    font-size: 11px;
    color: var(--text-secondary);
    font-weight: 500;
  }

  .metric-bar-container {
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 5px;
  }

  .metric-bar-track {
    width: 100%;
    height: 6px;
    background: rgba(255, 255, 255, 0.05);
    border-radius: 3px;
    overflow: hidden;
  }

  .metric-bar-fill {
    height: 100%;
    background: var(--accent-frost);
    border-radius: 3px;
    transition: width 0.3s ease;
  }

  .metric-bar-fill.high-load {
    background: var(--status-danger);
  }

  .metric-sub-stats {
    display: flex;
    justify-content: space-between;
    font-size: 10px;
    color: var(--text-muted);
    font-family: var(--font-mono);
  }

  .swap-stat {
    color: var(--accent-frost);
    font-weight: 600;
  }
</style>
