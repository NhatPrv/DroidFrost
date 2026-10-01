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

  export let onTabChange: (tab: TabFilter) => void;
  export let onToggleSystem: () => void;
  export let onOneClickBoost: () => void;
</script>

<div class="filter-bar glass-panel">
  <!-- Tabs Selector -->
  <div class="tabs-group">
    <button
      class="tab-btn"
      class:active={currentTab === 'all'}
      on:click={() => onTabChange('all')}
    >
      Tất cả <span class="counter">{countAll}</span>
    </button>

    <button
      class="tab-btn"
      class:active={currentTab === 'running'}
      on:click={() => onTabChange('running')}
    >
      <span class="dot-indicator green"></span>
      Đang chạy <span class="counter">{countRunning}</span>
    </button>

    <button
      class="tab-btn"
      class:active={currentTab === 'frozen'}
      on:click={() => onTabChange('frozen')}
    >
      <span class="dot-indicator blue"></span>
      Đã đóng băng <span class="counter">{countFrozen}</span>
    </button>

    <button
      class="tab-btn"
      class:active={currentTab === 'scheduled'}
      on:click={() => onTabChange('scheduled')}
    >
      <span class="dot-indicator amber"></span>
      Hẹn giờ <span class="counter">{countScheduled}</span>
    </button>
  </div>

  <!-- Actions & Search Right Side -->
  <div class="controls-group">
    <!-- Live Search -->
    <div class="search-wrap">
      <span class="search-icon">🔍</span>
      <input
        type="text"
        placeholder="Tìm theo tên app hoặc package..."
        bind:value={searchQuery}
      />
      {#if searchQuery}
        <button class="clear-btn" on:click={() => (searchQuery = '')}>✕</button>
      {/if}
    </div>

    <!-- System Apps Toggle -->
    <label class="toggle-wrap" title="Hiện hoặc ẩn ứng dụng cài sẵn trong ROM">
      <input
        type="checkbox"
        checked={showSystemApps}
        on:change={onToggleSystem}
      />
      <span class="toggle-slider"></span>
      <span class="toggle-label">App hệ thống</span>
    </label>

    <!-- One-Click Boost Button -->
    <button
      class="btn btn-primary boost-btn"
      on:click={onOneClickBoost}
      disabled={isBoosting}
      title="Giải phóng RAM ngay lập tức, tắt các app ngầm ngoài danh sách bảo vệ"
    >
      {#if isBoosting}
        <span class="spinner"></span> Đang tối ưu...
      {:else}
        ⚡ One-Click Boost
      {/if}
    </button>
  </div>
</div>

<style>
  .filter-bar {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 12px 24px;
    border-bottom: 1px solid var(--border-subtle);
    gap: 16px;
    flex-wrap: wrap;
  }

  .tabs-group {
    display: flex;
    align-items: center;
    background: var(--bg-surface-elevated);
    border-radius: var(--radius-md);
    padding: 4px;
    gap: 4px;
    border: 1px solid var(--border-subtle);
  }

  .tab-btn {
    display: flex;
    align-items: center;
    gap: 8px;
    background: transparent;
    border: none;
    color: var(--text-secondary);
    padding: 6px 14px;
    border-radius: var(--radius-sm);
    font-size: 13px;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.2s;
  }

  .tab-btn:hover {
    color: var(--text-primary);
    background: rgba(255, 255, 255, 0.04);
  }

  .tab-btn.active {
    background: var(--bg-surface);
    color: var(--text-primary);
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.3);
  }

  .counter {
    background: rgba(255, 255, 255, 0.08);
    padding: 1px 6px;
    border-radius: var(--radius-full);
    font-size: 11px;
    font-weight: 700;
  }

  .dot-indicator {
    width: 6px;
    height: 6px;
    border-radius: 50%;
  }

  .dot-indicator.green { background: var(--status-running); }
  .dot-indicator.blue { background: var(--status-frozen); }
  .dot-indicator.amber { background: var(--status-scheduled); }

  .controls-group {
    display: flex;
    align-items: center;
    gap: 16px;
  }

  .search-wrap {
    display: flex;
    align-items: center;
    background: var(--bg-surface-elevated);
    border: 1px solid var(--border-subtle);
    border-radius: var(--radius-sm);
    padding: 6px 12px;
    width: 260px;
    gap: 8px;
  }

  .search-wrap:focus-within {
    border-color: var(--accent-cyan);
    box-shadow: 0 0 10px rgba(0, 210, 255, 0.2);
  }

  .search-wrap input {
    background: transparent;
    border: none;
    outline: none;
    color: var(--text-primary);
    font-size: 13px;
    width: 100%;
    font-family: inherit;
  }

  .search-icon {
    font-size: 12px;
    color: var(--text-muted);
  }

  .clear-btn {
    background: none;
    border: none;
    color: var(--text-muted);
    cursor: pointer;
    font-size: 11px;
    padding: 0 4px;
  }

  .toggle-wrap {
    display: flex;
    align-items: center;
    gap: 8px;
    cursor: pointer;
    font-size: 13px;
    color: var(--text-secondary);
    font-weight: 500;
  }

  .toggle-wrap input {
    display: none;
  }

  .toggle-slider {
    width: 32px;
    height: 18px;
    background: var(--bg-surface-elevated);
    border: 1px solid var(--border-subtle);
    border-radius: var(--radius-full);
    position: relative;
    transition: all 0.3s;
  }

  .toggle-slider::before {
    content: '';
    position: absolute;
    width: 12px;
    height: 12px;
    left: 2px;
    top: 2px;
    background: var(--text-muted);
    border-radius: 50%;
    transition: all 0.3s;
  }

  .toggle-wrap input:checked + .toggle-slider {
    background: var(--accent-cyan);
    border-color: var(--accent-cyan);
  }

  .toggle-wrap input:checked + .toggle-slider::before {
    transform: translateX(14px);
    background: #000;
  }

  .boost-btn {
    padding: 8px 18px;
    font-size: 13px;
    letter-spacing: 0.3px;
    text-transform: uppercase;
  }

  .spinner {
    width: 12px;
    height: 12px;
    border: 2px solid rgba(255, 255, 255, 0.3);
    border-top-color: white;
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
  }

  @keyframes spin {
    to { transform: rotate(360deg); }
  }
</style>
