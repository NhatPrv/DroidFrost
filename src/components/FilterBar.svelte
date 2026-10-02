<script lang="ts">
  import type { TabFilter } from '../types';

  export let currentTab: TabFilter = 'all';
  export let showSystemApps: boolean = false;
  export let searchQuery: string = '';
  export let countAll: number = 0;
  export let countRunning: number = 0;
  export let countFrozen: number = 0;
  export let countScheduled: number = 0;
  export let isBoosting: boolean = false;
  export let isStoppingAll: boolean = false;

  export let onTabChange: (tab: TabFilter) => void;
  export let onToggleSystem: () => void;
  export let onOneClickBoost: () => void;
  export let onStopAllRunning: () => void;
</script>

<div class="toolbar">
  <!-- Segmented Tab Group -->
  <div class="segmented-control">
    <button
      class="segment-btn"
      class:active={currentTab === 'all'}
      on:click={() => onTabChange('all')}
    >
      Tất cả <span class="badge-num">{countAll}</span>
    </button>

    <button
      class="segment-btn"
      class:active={currentTab === 'running'}
      on:click={() => onTabChange('running')}
    >
      <span class="dot running"></span>
      Đang chạy <span class="badge-num">{countRunning}</span>
    </button>

    <button
      class="segment-btn"
      class:active={currentTab === 'frozen'}
      on:click={() => onTabChange('frozen')}
    >
      <span class="dot frozen"></span>
      Đã đóng băng <span class="badge-num">{countFrozen}</span>
    </button>

    <button
      class="segment-btn"
      class:active={currentTab === 'scheduled'}
      on:click={() => onTabChange('scheduled')}
    >
      <span class="dot scheduled"></span>
      Hẹn giờ <span class="badge-num">{countScheduled}</span>
    </button>
  </div>

  <!-- Right Actions -->
  <div class="actions-dock">
    <!-- Clean Search Input -->
    <div class="search-box">
      <svg class="search-svg" viewBox="0 0 24 24" width="13" height="13" fill="none" stroke="currentColor" stroke-width="2">
        <circle cx="11" cy="11" r="8"/>
        <path d="m21 21-4.35-4.35"/>
      </svg>
      <input
        type="text"
        placeholder="Lọc theo tên hoặc package..."
        bind:value={searchQuery}
      />
      {#if searchQuery}
        <button class="clear-text" on:click={() => (searchQuery = '')}>✕</button>
      {/if}
    </div>

    <!-- System Apps Checkbox -->
    <label class="system-toggle" title="Hiển thị hoặc ẩn các gói hệ điều hành cài sẵn">
      <input
        type="checkbox"
        checked={showSystemApps}
        on:change={onToggleSystem}
      />
      <span>App hệ thống</span>
    </label>

    <!-- Stop All Running Action -->
    <button
      class="btn btn-danger stop-all-action"
      on:click={onStopAllRunning}
      disabled={isStoppingAll || countRunning === 0}
      title="Buộc dừng ngay lập tức toàn bộ {countRunning} ứng dụng đang chạy nền"
    >
      <svg viewBox="0 0 24 24" width="13" height="13" fill="none" stroke="currentColor" stroke-width="2">
        <rect x="5" y="5" width="14" height="14" rx="2"/>
      </svg>
      {isStoppingAll ? 'Đang dừng...' : `Dừng tất cả (${countRunning})`}
    </button>

    <!-- Clean Boost Action -->
    <button
      class="btn btn-primary boost-action"
      on:click={onOneClickBoost}
      disabled={isBoosting}
      title="Buộc dừng toàn bộ ứng dụng người dùng chạy nền để thu hồi RAM"
    >
      <svg viewBox="0 0 24 24" width="13" height="13" fill="none" stroke="currentColor" stroke-width="2">
        <polygon points="13 2 3 14 12 14 11 22 21 10 12 10 13 2"/>
      </svg>
      {isBoosting ? 'Đang giải phóng...' : 'One-Click Boost'}
    </button>
  </div>
</div>

<style>
  .toolbar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 10px 20px;
    background: var(--bg-app);
    border-bottom: 1px solid var(--border-subtle);
    gap: 16px;
    flex-wrap: wrap;
  }

  .segmented-control {
    display: flex;
    background: var(--bg-surface);
    border: 1px solid var(--border-subtle);
    border-radius: var(--radius-sm);
    padding: 3px;
    gap: 2px;
  }

  .segment-btn {
    display: flex;
    align-items: center;
    gap: 6px;
    background: transparent;
    border: none;
    color: var(--text-secondary);
    padding: 5px 12px;
    border-radius: 4px;
    font-size: 12px;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .segment-btn:hover {
    color: var(--text-primary);
  }

  .segment-btn.active {
    background: var(--bg-surface-elevated);
    color: var(--text-primary);
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.4);
  }

  .badge-num {
    background: rgba(255, 255, 255, 0.08);
    padding: 1px 5px;
    border-radius: 3px;
    font-size: 10px;
    font-family: var(--font-mono);
  }

  .dot {
    width: 6px;
    height: 6px;
    border-radius: 50%;
  }

  .dot.running { background: var(--status-running); }
  .dot.frozen { background: var(--status-frozen); }
  .dot.scheduled { background: var(--status-scheduled); }

  .actions-dock {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .search-box {
    display: flex;
    align-items: center;
    background: var(--bg-surface);
    border: 1px solid var(--border-subtle);
    border-radius: var(--radius-sm);
    padding: 5px 10px;
    width: 240px;
    gap: 6px;
  }

  .search-box:focus-within {
    border-color: var(--border-focus);
  }

  .search-svg {
    color: var(--text-muted);
  }

  .search-box input {
    background: transparent;
    border: none;
    outline: none;
    color: var(--text-primary);
    font-size: 12px;
    width: 100%;
    font-family: inherit;
  }

  .clear-text {
    background: none;
    border: none;
    color: var(--text-muted);
    cursor: pointer;
    font-size: 11px;
    padding: 0;
  }

  .system-toggle {
    display: flex;
    align-items: center;
    gap: 6px;
    font-size: 12px;
    color: var(--text-secondary);
    cursor: pointer;
  }

  .stop-all-action {
    padding: 6px 14px;
    font-size: 12px;
    font-weight: 600;
  }

  .stop-all-action:disabled {
    opacity: 0.4;
    cursor: not-allowed;
  }

  .boost-action {
    padding: 6px 14px;
    font-size: 12px;
    font-weight: 600;
  }
</style>
