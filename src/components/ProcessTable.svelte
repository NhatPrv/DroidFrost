<script lang="ts">
  import type { ProcessInfo } from '../types';

  export let processes: ProcessInfo[] = [];
  export let onKill: (pkg: string) => void;
  export let onFreeze: (pkg: string) => void;
  export let onUnfreeze: (pkg: string) => void;
  export let onSchedule: (pkg: string, durationSeconds: number) => void;
  export let onCancelSchedule: (pkg: string) => void;

  let activeDropdownPkg: string | null = null;

  function toggleDropdown(pkg: string) {
    if (activeDropdownPkg === pkg) {
      activeDropdownPkg = null;
    } else {
      activeDropdownPkg = pkg;
    }
  }

  function handleScheduleSelect(pkg: string, seconds: number) {
    activeDropdownPkg = null;
    onSchedule(pkg, seconds);
  }

  function formatTimeRemaining(seconds: number | null): string {
    if (seconds === null || seconds <= 0) return '0s';
    if (seconds < 60) return `${seconds}s`;
    if (seconds < 3600) return `${Math.floor(seconds / 60)}p ${seconds % 60}s`;
    const h = Math.floor(seconds / 3600);
    const m = Math.floor((seconds % 3600) / 60);
    return `${h}h ${m}p`;
  }

  function getAvatarColor(pkg: string): string {
    const colors = [
      '#00d2ff', '#3a7bd5', '#8b5cf6', '#ec4899',
      '#10b981', '#f59e0b', '#06b6d4', '#6366f1'
    ];
    let hash = 0;
    for (let i = 0; i < pkg.length; i++) {
      hash = pkg.charCodeAt(i) + ((hash << 5) - hash);
    }
    return colors[Math.abs(hash) % colors.length];
  }
</script>

<div class="table-container">
  <table class="process-table">
    <thead>
      <tr>
        <th class="col-app">Ứng dụng</th>
        <th class="col-category">Phân loại</th>
        <th class="col-ram">Bộ nhớ RAM</th>
        <th class="col-status">Trạng thái</th>
        <th class="col-actions">Thao tác</th>
      </tr>
    </thead>
    <tbody>
      {#if processes.length === 0}
        <tr>
          <td colspan="5" class="empty-state-cell">
            <div class="empty-box">
              <span class="empty-icon">🔍</span>
              <p>Không tìm thấy ứng dụng nào phù hợp với bộ lọc hiện tại</p>
            </div>
          </td>
        </tr>
      {:else}
        {#each processes as p (p.package_name)}
          <tr class="table-row" class:frozen-row={p.is_frozen}>
            <!-- App Identity -->
            <td class="col-app">
              <div class="app-identity">
                <div
                  class="app-avatar"
                  style="background: {getAvatarColor(p.package_name)}"
                >
                  {p.app_name ? p.app_name.charAt(0).toUpperCase() : '?'}
                </div>
                <div class="app-meta">
                  <span class="app-display-name">{p.app_name}</span>
                  <span class="app-pkg-name">{p.package_name}</span>
                </div>
              </div>
            </td>

            <!-- Category -->
            <td class="col-category">
              {#if p.is_whitelisted}
                <span class="badge badge-system" title="Hệ thống cốt lõi được bảo vệ">
                  🛡️ Whitelist
                </span>
              {:else if p.is_system}
                <span class="badge badge-system">Hệ thống</span>
              {:else}
                <span class="badge badge-running">Người dùng</span>
              {/if}
            </td>

            <!-- RAM Usage -->
            <td class="col-ram">
              {#if p.ram_mb > 0}
                <div class="ram-metric">
                  <span class="ram-val">{p.ram_mb.toFixed(1)} MB</span>
                  <div class="ram-bar-track">
                    <div
                      class="ram-bar-fill"
                      style="width: {Math.min(100, (p.ram_mb / 600) * 100)}%"
                    ></div>
                  </div>
                </div>
              {:else}
                <span class="ram-zero">0 MB</span>
              {/if}
            </td>

            <!-- Status -->
            <td class="col-status">
              {#if p.is_frozen}
                <span class="badge badge-frozen">❄️ Đã đóng băng</span>
              {:else if p.is_running}
                <span class="badge badge-running">● Đang chạy</span>
              {:else}
                <span class="badge badge-system">Nghỉ</span>
              {/if}

              {#if p.is_scheduled}
                <span class="badge badge-scheduled" title="Tự động rã đông khi hết giờ">
                  ⏳ {formatTimeRemaining(p.scheduled_remaining_seconds)}
                </span>
              {/if}
            </td>

            <!-- Actions -->
            <td class="col-actions">
              <div class="actions-group">
                <!-- Kill Action -->
                {#if p.is_running && !p.is_whitelisted}
                  <button
                    class="btn btn-danger btn-sm"
                    on:click={() => onKill(p.package_name)}
                    title="Buộc dừng ngay lập tức"
                  >
                    Kill
                  </button>
                {/if}

                <!-- Freeze / Unfreeze Action -->
                {#if p.is_frozen}
                  <button
                    class="btn btn-unfreeze btn-sm"
                    on:click={() => onUnfreeze(p.package_name)}
                    title="Rã đông ứng dụng này"
                  >
                    Rã đông
                  </button>
                {:else if !p.is_whitelisted}
                  <button
                    class="btn btn-freeze btn-sm"
                    on:click={() => onFreeze(p.package_name)}
                    title="Đóng băng ứng dụng 100%"
                  >
                    Đóng băng
                  </button>
                {/if}

                <!-- Schedule Menu -->
                {#if p.is_scheduled}
                  <button
                    class="btn btn-secondary btn-sm"
                    on:click={() => onCancelSchedule(p.package_name)}
                    title="Hủy lịch hẹn"
                  >
                    Hủy hẹn
                  </button>
                {:else if !p.is_whitelisted}
                  <div class="dropdown-wrapper">
                    <button
                      class="btn btn-secondary btn-sm dropdown-trigger"
                      on:click={() => toggleDropdown(p.package_name)}
                      title="Hẹn giờ đóng băng"
                    >
                      Hẹn giờ ▾
                    </button>

                    {#if activeDropdownPkg === p.package_name}
                      <div class="dropdown-menu glass-panel">
                        <button on:click={() => handleScheduleSelect(p.package_name, 900)}>
                          15 phút
                        </button>
                        <button on:click={() => handleScheduleSelect(p.package_name, 3600)}>
                          1 giờ
                        </button>
                        <button on:click={() => handleScheduleSelect(p.package_name, 7200)}>
                          2 giờ
                        </button>
                        <button on:click={() => handleScheduleSelect(p.package_name, 28800)}>
                          8 giờ
                        </button>
                        <button on:click={() => handleScheduleSelect(p.package_name, 86400)}>
                          24 giờ
                        </button>
                      </div>
                    {/if}
                  </div>
                {/if}
              </div>
            </td>
          </tr>
        {/each}
      {/if}
    </tbody>
  </table>
</div>

<style>
  .table-container {
    flex: 1;
    overflow-y: auto;
    padding: 0 24px 24px 24px;
  }

  .process-table {
    width: 100%;
    border-collapse: collapse;
    text-align: left;
    font-size: 13px;
  }

  thead {
    position: sticky;
    top: 0;
    background: var(--bg-primary);
    z-index: 10;
  }

  th {
    padding: 14px 16px;
    font-size: 12px;
    font-weight: 600;
    color: var(--text-muted);
    text-transform: uppercase;
    letter-spacing: 0.5px;
    border-bottom: 1px solid var(--border-subtle);
  }

  td {
    padding: 12px 16px;
    border-bottom: 1px solid rgba(255, 255, 255, 0.04);
    vertical-align: middle;
  }

  .table-row {
    transition: background 0.15s;
  }

  .table-row:hover {
    background: var(--bg-surface-elevated);
  }

  .table-row.frozen-row {
    opacity: 0.8;
  }

  .app-identity {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .app-avatar {
    width: 34px;
    height: 34px;
    border-radius: var(--radius-sm);
    display: flex;
    align-items: center;
    justify-content: center;
    font-weight: 700;
    color: white;
    font-size: 14px;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.3);
  }

  .app-meta {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .app-display-name {
    font-weight: 600;
    color: var(--text-primary);
  }

  .app-pkg-name {
    font-size: 11px;
    color: var(--text-muted);
    font-family: var(--font-mono);
  }

  .ram-metric {
    display: flex;
    flex-direction: column;
    gap: 4px;
    width: 120px;
  }

  .ram-val {
    font-family: var(--font-mono);
    font-size: 12px;
    color: var(--accent-cyan);
    font-weight: 600;
  }

  .ram-bar-track {
    width: 100%;
    height: 4px;
    background: var(--bg-surface-elevated);
    border-radius: var(--radius-full);
    overflow: hidden;
  }

  .ram-bar-fill {
    height: 100%;
    background: var(--accent-gradient);
    border-radius: var(--radius-full);
  }

  .ram-zero {
    font-size: 12px;
    color: var(--text-muted);
    font-family: var(--font-mono);
  }

  .col-status {
    display: flex;
    align-items: center;
    gap: 6px;
    flex-wrap: wrap;
    min-height: 48px;
  }

  .actions-group {
    display: flex;
    align-items: center;
    gap: 6px;
  }

  .btn-sm {
    padding: 5px 10px;
    font-size: 11px;
    border-radius: 4px;
  }

  .dropdown-wrapper {
    position: relative;
  }

  .dropdown-menu {
    position: absolute;
    right: 0;
    top: 100%;
    margin-top: 4px;
    border-radius: var(--radius-sm);
    padding: 4px;
    display: flex;
    flex-direction: column;
    gap: 2px;
    z-index: 50;
    width: 110px;
    box-shadow: 0 10px 25px rgba(0, 0, 0, 0.5);
  }

  .dropdown-menu button {
    background: transparent;
    border: none;
    color: var(--text-secondary);
    padding: 6px 10px;
    font-size: 12px;
    text-align: left;
    cursor: pointer;
    border-radius: 4px;
    transition: all 0.15s;
  }

  .dropdown-menu button:hover {
    background: var(--accent-cyan);
    color: #000;
    font-weight: 600;
  }

  .empty-state-cell {
    text-align: center;
    padding: 64px 20px;
  }

  .empty-box {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 12px;
    color: var(--text-muted);
  }

  .empty-icon {
    font-size: 32px;
  }
</style>
