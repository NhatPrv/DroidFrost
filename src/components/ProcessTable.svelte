<script lang="ts">
  import type { ProcessInfo } from '../types';
  import { getAppIconSvg } from '../lib/icons';

  export let processes: ProcessInfo[] = [];
  export let onKill: (pkg: string) => void;
  export let onFreeze: (pkg: string) => void;
  export let onUnfreeze: (pkg: string) => void;
  export let onSchedule: (pkg: string, durationSeconds: number) => void;
  export let onCancelSchedule: (pkg: string) => void;
  export let onBatchKill: (packages: string[]) => void;
  export let onBatchFreeze: (packages: string[]) => void;

  let selectedPkgs: Set<string> = new Set();
  let sortField: 'ram' | 'name' = 'ram';
  let sortAsc: boolean = false;
  let activeDropdownPkg: string | null = null;

  function toggleSelectAll() {
    if (selectedPkgs.size === processes.length) {
      selectedPkgs = new Set();
    } else {
      selectedPkgs = new Set(processes.map((p) => p.package_name));
    }
  }

  function toggleSelectRow(pkg: string) {
    const next = new Set(selectedPkgs);
    if (next.has(pkg)) {
      next.delete(pkg);
    } else {
      next.add(pkg);
    }
    selectedPkgs = next;
  }

  function handleSort(field: 'ram' | 'name') {
    if (sortField === field) {
      sortAsc = !sortAsc;
    } else {
      sortField = field;
      sortAsc = false;
    }
  }

  $: sortedProcesses = [...processes].sort((a, b) => {
    if (sortField === 'ram') {
      return sortAsc ? a.ram_mb - b.ram_mb : b.ram_mb - a.ram_mb;
    } else {
      return sortAsc
        ? a.app_name.localeCompare(b.app_name)
        : b.app_name.localeCompare(a.app_name);
    }
  });

  function toggleDropdown(pkg: string) {
    activeDropdownPkg = activeDropdownPkg === pkg ? null : pkg;
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
</script>

<div class="table-wrapper">
  <table class="native-table">
    <thead>
      <tr>
        <th class="col-check">
          <input
            type="checkbox"
            checked={processes.length > 0 && selectedPkgs.size === processes.length}
            on:change={toggleSelectAll}
          />
        </th>
        <th class="col-app sortable" on:click={() => handleSort('name')}>
          Ứng dụng {sortField === 'name' ? (sortAsc ? '▲' : '▼') : ''}
        </th>
        <th class="col-type">Phân loại</th>
        <th class="col-ram sortable" on:click={() => handleSort('ram')}>
          Bộ nhớ RAM {sortField === 'ram' ? (sortAsc ? '▲' : '▼') : ''}
        </th>
        <th class="col-status">Trạng thái</th>
        <th class="col-actions">Tác vụ</th>
      </tr>
    </thead>
    <tbody>
      {#if sortedProcesses.length === 0}
        <tr>
          <td colspan="6" class="empty-cell">
            <div class="empty-state">
              <svg viewBox="0 0 24 24" width="36" height="36" fill="none" stroke="currentColor" stroke-width="1.5" color="#64748b">
                <rect x="2" y="3" width="20" height="14" rx="2" ry="2"/>
                <line x1="8" y1="21" x2="16" y2="21"/>
                <line x1="12" y1="17" x2="12" y2="21"/>
              </svg>
              <span class="empty-title">Không tìm thấy ứng dụng</span>
              <span class="empty-desc">Kiểm tra kết nối cáp USB hoặc điều chỉnh lại thanh tìm kiếm</span>
            </div>
          </td>
        </tr>
      {:else}
        {#each sortedProcesses as p (p.package_name)}
          <tr
            class="data-row"
            class:selected-row={selectedPkgs.has(p.package_name)}
            class:frozen-opacity={p.is_frozen}
          >
            <!-- Checkbox -->
            <td class="col-check">
              <input
                type="checkbox"
                checked={selectedPkgs.has(p.package_name)}
                on:change={() => toggleSelectRow(p.package_name)}
              />
            </td>

            <!-- App Logo + Name -->
            <td class="col-app">
              <div class="app-cell">
                <div class="app-icon-box">
                  {@html getAppIconSvg(p.package_name, p.app_name)}
                </div>
                <div class="app-info">
                  <span class="app-title">{p.app_name}</span>
                  <span class="app-package">{p.package_name}</span>
                </div>
              </div>
            </td>

            <!-- Category Pill -->
            <td class="col-type">
              {#if p.is_whitelisted}
                <span class="pill pill-whitelist" title="Hệ thống cốt lõi bảo vệ tuyệt đối">
                  Whitelist
                </span>
              {:else if p.is_system}
                <span class="pill pill-system">ROM System</span>
              {:else}
                <span class="pill pill-running">User App</span>
              {/if}
            </td>

            <!-- RAM Metric -->
            <td class="col-ram">
              {#if p.ram_mb > 0}
                <div class="ram-box">
                  <span class="ram-text">{p.ram_mb.toFixed(1)} MB</span>
                  <div class="ram-track">
                    <div
                      class="ram-fill"
                      style="width: {Math.min(100, (p.ram_mb / 600) * 100)}%"
                    ></div>
                  </div>
                </div>
              {:else}
                <span class="ram-idle">0 MB</span>
              {/if}
            </td>

            <!-- Status Pill -->
            <td class="col-status">
              {#if p.is_frozen}
                <span class="pill pill-frozen">Đã đóng băng</span>
              {:else if p.is_running}
                <span class="pill pill-running">Đang chạy</span>
              {:else}
                <span class="pill pill-whitelist">Chờ</span>
              {/if}

              {#if p.is_scheduled}
                <span class="pill pill-scheduled" title="Tự động unfreeze khi hết giờ">
                  {formatTimeRemaining(p.scheduled_remaining_seconds)}
                </span>
              {/if}
            </td>

            <!-- Action Buttons -->
            <td class="col-actions">
              <div class="action-dock">
                {#if p.is_running && !p.is_whitelisted}
                  <button
                    class="btn btn-danger btn-action"
                    on:click={() => onKill(p.package_name)}
                    title="Buộc dừng ngay lập tức"
                  >
                    Kill
                  </button>
                {/if}

                {#if p.is_frozen}
                  <button
                    class="btn btn-unfreeze btn-action"
                    on:click={() => onUnfreeze(p.package_name)}
                    title="Kích hoạt lại ứng dụng"
                  >
                    Unfreeze
                  </button>
                {:else if !p.is_whitelisted}
                  <button
                    class="btn btn-freeze btn-action"
                    on:click={() => onFreeze(p.package_name)}
                    title="Đóng băng ứng dụng hoàn toàn"
                  >
                    Freeze
                  </button>
                {/if}

                {#if p.is_scheduled}
                  <button
                    class="btn btn-action"
                    on:click={() => onCancelSchedule(p.package_name)}
                    title="Hủy lịch hẹn"
                  >
                    Hủy hẹn
                  </button>
                {:else if !p.is_whitelisted}
                  <div class="dropdown-anchor">
                    <button
                      class="btn btn-action"
                      on:click={() => toggleDropdown(p.package_name)}
                      title="Hẹn giờ đóng băng"
                    >
                      Hẹn giờ ▾
                    </button>

                    {#if activeDropdownPkg === p.package_name}
                      <div class="dropdown-pane">
                        <button on:click={() => handleScheduleSelect(p.package_name, 900)}>15 phút</button>
                        <button on:click={() => handleScheduleSelect(p.package_name, 3600)}>1 giờ</button>
                        <button on:click={() => handleScheduleSelect(p.package_name, 7200)}>2 giờ</button>
                        <button on:click={() => handleScheduleSelect(p.package_name, 28800)}>8 giờ</button>
                        <button on:click={() => handleScheduleSelect(p.package_name, 86400)}>24 giờ</button>
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

  <!-- Floating Batch Action Bar -->
  {#if selectedPkgs.size > 0}
    <div class="floating-batch-bar">
      <span class="batch-count">Đã chọn <strong>{selectedPkgs.size}</strong> ứng dụng</span>
      <div class="batch-buttons">
        <button
          class="btn btn-danger"
          on:click={() => {
            onBatchKill(Array.from(selectedPkgs));
            selectedPkgs = new Set();
          }}
        >
          Buộc dừng đã chọn
        </button>
        <button
          class="btn btn-freeze"
          on:click={() => {
            onBatchFreeze(Array.from(selectedPkgs));
            selectedPkgs = new Set();
          }}
        >
          Đóng băng đã chọn
        </button>
        <button class="btn" on:click={() => (selectedPkgs = new Set())}>
          Bỏ chọn
        </button>
      </div>
    </div>
  {/if}
</div>

<style>
  .table-wrapper {
    flex: 1;
    overflow-y: auto;
    position: relative;
    background: var(--bg-app);
  }

  .native-table {
    width: 100%;
    border-collapse: collapse;
    text-align: left;
    font-size: 12px;
  }

  thead {
    position: sticky;
    top: 0;
    background: var(--bg-surface);
    z-index: 10;
    border-bottom: 1px solid var(--border-medium);
  }

  th {
    padding: 10px 14px;
    font-size: 11px;
    font-weight: 600;
    color: var(--text-muted);
    letter-spacing: 0.3px;
  }

  th.sortable {
    cursor: pointer;
  }

  th.sortable:hover {
    color: var(--text-primary);
  }

  td {
    padding: 9px 14px;
    border-bottom: 1px solid var(--border-subtle);
    vertical-align: middle;
  }

  .data-row {
    transition: background 0.1s ease;
  }

  .data-row:hover {
    background: var(--bg-surface-elevated);
  }

  .selected-row {
    background: rgba(56, 189, 248, 0.05);
  }

  .frozen-opacity {
    opacity: 0.75;
  }

  .col-check {
    width: 38px;
    text-align: center;
  }

  .col-app {
    min-width: 250px;
  }

  .app-cell {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .app-icon-box {
    width: 32px;
    height: 32px;
    flex-shrink: 0;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  :global(.app-icon-box svg) {
    width: 32px;
    height: 32px;
    border-radius: 8px;
  }

  .app-info {
    display: flex;
    flex-direction: column;
    gap: 1px;
    overflow: hidden;
  }

  .app-title {
    font-weight: 600;
    color: var(--text-primary);
    font-size: 13px;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .app-package {
    font-size: 11px;
    color: var(--text-muted);
    font-family: var(--font-mono);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .ram-box {
    display: flex;
    flex-direction: column;
    gap: 4px;
    width: 110px;
  }

  .ram-text {
    font-family: var(--font-mono);
    font-size: 11px;
    color: var(--text-primary);
    font-weight: 600;
  }

  .ram-track {
    width: 100%;
    height: 3px;
    background: rgba(255, 255, 255, 0.05);
    border-radius: 2px;
    overflow: hidden;
  }

  .ram-fill {
    height: 100%;
    background: var(--accent-frost);
    border-radius: 2px;
  }

  .ram-idle {
    font-size: 11px;
    color: var(--text-muted);
    font-family: var(--font-mono);
  }

  .col-status {
    min-width: 130px;
  }

  .action-dock {
    display: flex;
    align-items: center;
    gap: 5px;
  }

  .btn-action {
    padding: 3px 8px;
    font-size: 11px;
    font-weight: 500;
  }

  .dropdown-anchor {
    position: relative;
  }

  .dropdown-pane {
    position: absolute;
    right: 0;
    top: 100%;
    margin-top: 4px;
    background: var(--bg-surface-elevated);
    border: 1px solid var(--border-medium);
    border-radius: var(--radius-sm);
    padding: 4px;
    display: flex;
    flex-direction: column;
    z-index: 50;
    width: 100px;
    box-shadow: 0 8px 20px rgba(0, 0, 0, 0.5);
  }

  .dropdown-pane button {
    background: transparent;
    border: none;
    color: var(--text-secondary);
    padding: 5px 8px;
    font-size: 11px;
    text-align: left;
    cursor: pointer;
    border-radius: 3px;
  }

  .dropdown-pane button:hover {
    background: var(--accent-frost);
    color: #000;
  }

  .empty-cell {
    text-align: center;
    padding: 60px 20px;
  }

  .empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 8px;
  }

  .empty-title {
    font-weight: 600;
    color: var(--text-secondary);
  }

  .empty-desc {
    font-size: 11px;
    color: var(--text-muted);
  }

  .floating-batch-bar {
    position: fixed;
    bottom: 20px;
    left: 50%;
    transform: translateX(-50%);
    background: var(--bg-surface-elevated);
    border: 1px solid var(--border-medium);
    border-radius: var(--radius-md);
    padding: 8px 16px;
    display: flex;
    align-items: center;
    gap: 20px;
    box-shadow: 0 10px 30px rgba(0, 0, 0, 0.6);
    z-index: 100;
  }

  .batch-count {
    font-size: 12px;
    color: var(--text-secondary);
  }

  .batch-count strong {
    color: var(--accent-frost);
  }

  .batch-buttons {
    display: flex;
    gap: 8px;
  }
</style>
