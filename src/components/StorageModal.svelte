<script lang="ts">
  import { onMount } from 'svelte';
  import type { AppStorageInfo, ProcessInfo } from '../types';
  import { api } from '../lib/api';
  import { getAppIconSvg } from '../lib/icons';

  export let process: ProcessInfo | null = null;
  export let serial: string = '';
  export let onClose: () => void;
  export let onNotify: (msg: string) => void = () => {};

  let storageInfo: AppStorageInfo | null = null;
  let isLoading: boolean = true;
  let isClearing: boolean = false;
  let errorMsg: string = '';

  // Confirm dialog state
  let confirmAction: 'cache' | 'data' | null = null;

  async function loadStorage() {
    if (!process || !serial) return;
    isLoading = true;
    errorMsg = '';
    try {
      const data = await api.getAppStorage(serial, process.package_name);
      storageInfo = data;
    } catch (e: any) {
      errorMsg = typeof e === 'string' ? e : e?.message || 'Không thể lấy thông tin bộ nhớ';
    } finally {
      isLoading = false;
    }
  }

  onMount(() => {
    loadStorage();
  });

  async function executeClearCache() {
    if (!process || !serial) return;
    isClearing = true;
    confirmAction = null;
    try {
      const res = await api.clearAppCache(serial, process.package_name);
      onNotify(res.message);
      await loadStorage();
    } catch (e: any) {
      onNotify(`Lỗi dọn cache: ${e}`);
    } finally {
      isClearing = false;
    }
  }

  async function executeClearData() {
    if (!process || !serial) return;
    isClearing = true;
    confirmAction = null;
    try {
      const res = await api.clearAppData(serial, process.package_name);
      onNotify(res.message);
      await loadStorage();
    } catch (e: any) {
      onNotify(`Lỗi xóa dữ liệu: ${e}`);
    } finally {
      isClearing = false;
    }
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Escape') {
      if (confirmAction) {
        confirmAction = null;
      } else {
        onClose();
      }
    }
  }
</script>

<svelte:window on:keydown={handleKeydown} />

{#if process}
  <div class="storage-backdrop" role="presentation" on:click|self={onClose}>
    <div class="storage-card" role="dialog" aria-modal="true">
      <!-- Header -->
      <div class="storage-header">
        <div class="app-profile">
          <div class="app-icon-large">
            {@html getAppIconSvg(process.package_name, process.app_name)}
          </div>
          <div class="app-meta">
            <h3 class="app-title">{process.app_name}</h3>
            <span class="app-pkg">{process.package_name}</span>
            <div class="app-tags">
              {#if process.is_whitelisted}
                <span class="badge badge-whitelist">Whitelist Cốt lõi</span>
              {:else if process.is_system}
                <span class="badge badge-system">Hệ thống ROM</span>
              {:else}
                <span class="badge badge-user">Ứng dụng người dùng</span>
              {/if}
              {#if process.ram_mb > 0}
                <span class="badge badge-ram">RAM: {process.ram_mb.toFixed(1)} MB</span>
              {/if}
            </div>
          </div>
        </div>

        <button class="btn-close" on:click={onClose} title="Đóng (Esc)">
          <svg viewBox="0 0 24 24" width="18" height="18" fill="none" stroke="currentColor" stroke-width="2">
            <line x1="18" y1="6" x2="6" y2="18"></line>
            <line x1="6" y1="6" x2="18" y2="18"></line>
          </svg>
        </button>
      </div>

      <!-- Body -->
      <div class="storage-body">
        {#if isLoading}
          <div class="loading-state">
            <div class="spinner"></div>
            <span class="loading-text">Đang phân tích dung lượng chiếm dụng...</span>
            <span class="loading-sub">Truy vấn apk path, dữ liệu nội bộ và bộ nhớ đệm Android</span>
          </div>
        {:else if errorMsg}
          <div class="error-box">
            <svg viewBox="0 0 24 24" width="24" height="24" fill="none" stroke="#f43f5e" stroke-width="2">
              <circle cx="12" cy="12" r="10"/>
              <line x1="12" y1="8" x2="12" y2="12"/>
              <line x1="12" y1="16" x2="12.01" y2="16"/>
            </svg>
            <p>{errorMsg}</p>
            <button class="btn btn-action" on:click={loadStorage}>Thử lại</button>
          </div>
        {:else if storageInfo}
          <!-- Total Storage Big Banner -->
          <div class="total-banner">
            <div class="total-info">
              <span class="total-label">Tổng dung lượng chiếm dụng</span>
              <span class="total-val">
                {storageInfo.total_storage_mb.toFixed(1)} <span class="unit">MB</span>
              </span>
            </div>
            <button
              class="btn btn-refresh"
              on:click={loadStorage}
              disabled={isClearing}
              title="Quét lại dung lượng"
            >
              <svg viewBox="0 0 24 24" width="14" height="14" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M21.5 2v6h-6M21.34 15.57a10 10 0 1 1-.57-8.38l5.67-5.67"/>
              </svg>
              Đo lại
            </button>
          </div>

          <!-- Visual Bar Breakdown -->
          {#if storageInfo.total_storage_mb > 0}
            <div class="bar-container">
              <div class="bar-track">
                <div
                  class="bar-segment bar-apk"
                  style="width: {(storageInfo.apk_size_mb / storageInfo.total_storage_mb) * 100}%"
                  title="Ứng dụng: {storageInfo.apk_size_mb} MB"
                ></div>
                <div
                  class="bar-segment bar-data"
                  style="width: {(storageInfo.data_size_mb / storageInfo.total_storage_mb) * 100}%"
                  title="Dữ liệu: {storageInfo.data_size_mb} MB"
                ></div>
                <div
                  class="bar-segment bar-cache"
                  style="width: {(storageInfo.cache_size_mb / storageInfo.total_storage_mb) * 100}%"
                  title="Bộ nhớ đệm: {storageInfo.cache_size_mb} MB"
                ></div>
              </div>
              <div class="bar-legend">
                <div class="legend-item">
                  <span class="legend-dot dot-apk"></span>
                  <span>Ứng dụng ({storageInfo.apk_size_mb.toFixed(1)} MB)</span>
                </div>
                <div class="legend-item">
                  <span class="legend-dot dot-data"></span>
                  <span>Dữ liệu ({storageInfo.data_size_mb.toFixed(1)} MB)</span>
                </div>
                <div class="legend-item">
                  <span class="legend-dot dot-cache"></span>
                  <span>Cache ({storageInfo.cache_size_mb.toFixed(1)} MB)</span>
                </div>
              </div>
            </div>
          {/if}

          <!-- Metric Cards Grid -->
          <div class="storage-grid">
            <!-- APK Card -->
            <div class="metric-card">
              <div class="metric-head">
                <div class="metric-icon-box apk-icon">
                  <svg viewBox="0 0 24 24" width="16" height="16" fill="none" stroke="currentColor" stroke-width="2">
                    <rect x="2" y="7" width="20" height="14" rx="2" ry="2"/>
                    <path d="M16 21V5a2 2 0 0 0-2-2h-4a2 2 0 0 0-2 2v16"/>
                  </svg>
                </div>
                <span class="metric-title">Kích thước APK</span>
              </div>
              <div class="metric-body">
                <span class="metric-num">{storageInfo.apk_size_mb.toFixed(1)} <small>MB</small></span>
                <span class="metric-sub">Tệp nhị phân cài đặt gốc</span>
              </div>
            </div>

            <!-- Data Card -->
            <div class="metric-card">
              <div class="metric-head">
                <div class="metric-icon-box data-icon">
                  <svg viewBox="0 0 24 24" width="16" height="16" fill="none" stroke="currentColor" stroke-width="2">
                    <ellipse cx="12" cy="5" rx="9" ry="3"/>
                    <path d="M21 12c0 1.66-4 3-9 3s-9-1.34-9-3"/>
                    <path d="M3 5v14c0 1.66 4 3 9 3s9-1.34 9-3V5"/>
                  </svg>
                </div>
                <span class="metric-title">Dữ liệu người dùng</span>
              </div>
              <div class="metric-body">
                <span class="metric-num">{storageInfo.data_size_mb.toFixed(1)} <small>MB</small></span>
                <span class="metric-sub">Tài khoản, database, thiết lập</span>
              </div>
            </div>

            <!-- Cache Card -->
            <div class="metric-card">
              <div class="metric-head">
                <div class="metric-icon-box cache-icon">
                  <svg viewBox="0 0 24 24" width="16" height="16" fill="none" stroke="currentColor" stroke-width="2">
                    <path d="M12 2v4M12 18v4M4.93 4.93l2.83 2.83M16.24 16.24l2.83 2.83M2 12h4M18 12h4M4.93 19.07l2.83-2.83M16.24 7.76l2.83-2.83"/>
                  </svg>
                </div>
                <span class="metric-title">Bộ nhớ đệm (Cache)</span>
              </div>
              <div class="metric-body">
                <span class="metric-num">{storageInfo.cache_size_mb.toFixed(1)} <small>MB</small></span>
                <span class="metric-sub">Tệp tạm thời, ảnh nạp trước</span>
              </div>
            </div>
          </div>

          <!-- Actions Bar -->
          <div class="storage-action-box">
            <div class="action-desc-col">
              <span class="action-box-title">Dọn dẹp dung lượng lưu trữ</span>
              <span class="action-box-sub">
                Giải phóng không gian nhớ trên điện thoại Android của bạn
              </span>
            </div>

            <div class="action-btns-group">
              <!-- Clear Cache Button -->
              <button
                class="btn btn-warning"
                disabled={isClearing || storageInfo.cache_size_mb === 0}
                on:click={() => (confirmAction = 'cache')}
                title="Xóa tệp tạm thời không ảnh hưởng tài khoản"
              >
                <svg viewBox="0 0 24 24" width="14" height="14" fill="none" stroke="currentColor" stroke-width="2">
                  <path d="M3 3l18 18M9 9v10a2 2 0 0 0 2 2h2a2 2 0 0 0 2-2V9"/>
                </svg>
                Xóa bộ nhớ đệm
              </button>

              <!-- Clear Data Button -->
              <button
                class="btn btn-danger"
                disabled={isClearing || process.is_whitelisted}
                on:click={() => (confirmAction = 'data')}
                title={process.is_whitelisted ? 'Ứng dụng Whitelist cốt lõi được bảo vệ an toàn' : 'Xóa toàn bộ dữ liệu người dùng và reset app'}
              >
                <svg viewBox="0 0 24 24" width="14" height="14" fill="none" stroke="currentColor" stroke-width="2">
                  <path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"/>
                  <line x1="10" y1="11" x2="10" y2="17"/>
                  <line x1="14" y1="11" x2="14" y2="17"/>
                </svg>
                Xóa tất cả dữ liệu
              </button>
            </div>
          </div>
        {/if}
      </div>

      <!-- Footer -->
      <div class="storage-footer">
        <span class="footer-tip">
          💡 DroidFrost luôn kiểm tra quyền an toàn và bảo vệ các gói dịch vụ hệ thống cốt lõi.
        </span>
        <button class="btn btn-ghost" on:click={onClose}>Đóng</button>
      </div>
    </div>

    <!-- Confirmation Modal: Clear Cache -->
    {#if confirmAction === 'cache'}
      <div
        class="confirm-overlay"
        role="presentation"
        on:click|self={() => (confirmAction = null)}
      >
        <div class="confirm-dialog" role="alertdialog">
          <div class="confirm-header">
            <div class="confirm-icon-wrap warning">
              <svg viewBox="0 0 24 24" width="22" height="22" fill="none" stroke="currentColor" stroke-width="2">
                <circle cx="12" cy="12" r="10"/>
                <line x1="12" y1="8" x2="12" y2="12"/>
                <line x1="12" y1="16" x2="12.01" y2="16"/>
              </svg>
            </div>
            <div class="confirm-head-text">
              <h4>Xác nhận xóa bộ nhớ đệm</h4>
              <p>{process.app_name} <small>({process.package_name})</small></p>
            </div>
          </div>
          <div class="confirm-content">
            <p>
              Bạn có muốn dọn dẹp khoảng <strong>{storageInfo?.cache_size_mb.toFixed(1) || 0} MB</strong> bộ nhớ đệm (Cache) của ứng dụng này không?
            </p>
            <div class="safe-note">
              ✓ Thao tác này an toàn 100%, chỉ loại bỏ các tệp tin lưu tạm mà không làm mất thông tin đăng nhập hay dữ liệu của bạn.
            </div>
          </div>
          <div class="confirm-actions">
            <button class="btn btn-ghost" on:click={() => (confirmAction = null)}>Hủy bỏ</button>
            <button class="btn btn-warning-solid" on:click={executeClearCache}>
              Xóa bộ nhớ đệm ngay
            </button>
          </div>
        </div>
      </div>
    {/if}

    <!-- Confirmation Modal: Clear All Data -->
    {#if confirmAction === 'data'}
      <div
        class="confirm-overlay"
        role="presentation"
        on:click|self={() => (confirmAction = null)}
      >
        <div class="confirm-dialog" role="alertdialog">
          <div class="confirm-header">
            <div class="confirm-icon-wrap danger">
              <svg viewBox="0 0 24 24" width="22" height="22" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z"/>
                <line x1="12" y1="9" x2="12" y2="13"/>
                <line x1="12" y1="17" x2="12.01" y2="17"/>
              </svg>
            </div>
            <div class="confirm-head-text">
              <h4 class="text-danger">Cảnh báo: Xóa toàn bộ dữ liệu!</h4>
              <p>{process.app_name} <small>({process.package_name})</small></p>
            </div>
          </div>
          <div class="confirm-content">
            <p>
              Hành động này sẽ xóa vĩnh viễn toàn bộ <strong>dữ liệu người dùng, cơ sở dữ liệu, file cấu hình và các tài khoản đăng nhập</strong> của ứng dụng <strong>{process.app_name}</strong>.
            </p>
            <div class="danger-note">
              ⚠️ Ứng dụng sẽ được đưa về trạng thái nguyên bản như khi mới tải về. Bạn sẽ phải đăng nhập lại từ đầu.
            </div>
          </div>
          <div class="confirm-actions">
            <button class="btn btn-ghost" on:click={() => (confirmAction = null)}>Hủy bỏ</button>
            <button class="btn btn-danger-solid" on:click={executeClearData}>
              Xác nhận xóa sạch dữ liệu
            </button>
          </div>
        </div>
      </div>
    {/if}
  </div>
{/if}

<style>
  .storage-backdrop {
    position: fixed;
    top: 0;
    left: 0;
    width: 100vw;
    height: 100vh;
    background: rgba(0, 0, 0, 0.72);
    backdrop-filter: blur(6px);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
    animation: fadeIn 0.15s ease-out;
  }

  .storage-card {
    background: var(--bg-surface);
    border: 1px solid var(--border-medium);
    border-radius: var(--radius-lg);
    width: 90%;
    max-width: 580px;
    box-shadow: 0 20px 45px rgba(0, 0, 0, 0.65), 0 0 0 1px rgba(255, 255, 255, 0.05);
    display: flex;
    flex-direction: column;
    overflow: hidden;
    animation: scaleUp 0.15s cubic-bezier(0.16, 1, 0.3, 1);
  }

  /* Header */
  .storage-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 16px 20px;
    background: var(--bg-surface-elevated);
    border-bottom: 1px solid var(--border-subtle);
  }

  .app-profile {
    display: flex;
    align-items: center;
    gap: 14px;
  }

  .app-icon-large {
    width: 44px;
    height: 44px;
    border-radius: var(--radius-md);
    background: var(--bg-surface);
    display: flex;
    align-items: center;
    justify-content: center;
    border: 1px solid var(--border-subtle);
    flex-shrink: 0;
  }

  .app-icon-large :global(svg) {
    width: 28px;
    height: 28px;
  }

  .app-meta {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .app-title {
    font-size: 15px;
    font-weight: 600;
    color: var(--text-primary);
  }

  .app-pkg {
    font-size: 11px;
    color: var(--text-muted);
    font-family: var(--font-mono);
  }

  .app-tags {
    display: flex;
    align-items: center;
    gap: 6px;
    margin-top: 4px;
  }

  .badge {
    font-size: 10px;
    padding: 1px 7px;
    border-radius: 10px;
    font-weight: 500;
  }

  .badge-whitelist {
    background: rgba(148, 163, 184, 0.15);
    color: #94a3b8;
    border: 1px solid rgba(148, 163, 184, 0.25);
  }

  .badge-system {
    background: rgba(129, 140, 248, 0.15);
    color: #a5b4fc;
    border: 1px solid rgba(129, 140, 248, 0.3);
  }

  .badge-user {
    background: rgba(56, 189, 248, 0.12);
    color: #38bdf8;
    border: 1px solid rgba(56, 189, 248, 0.25);
  }

  .badge-ram {
    background: rgba(34, 197, 94, 0.12);
    color: #4ade80;
    border: 1px solid rgba(34, 197, 94, 0.25);
  }

  .btn-close {
    background: transparent;
    border: none;
    color: var(--text-muted);
    cursor: pointer;
    padding: 6px;
    border-radius: var(--radius-sm);
    display: flex;
    align-items: center;
    justify-content: center;
    transition: all 0.15s ease;
  }

  .btn-close:hover {
    color: var(--text-primary);
    background: var(--bg-surface-hover);
  }

  /* Body */
  .storage-body {
    padding: 20px;
    display: flex;
    flex-direction: column;
    gap: 18px;
    min-height: 240px;
  }

  .loading-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 40px 20px;
    gap: 12px;
  }

  .spinner {
    width: 32px;
    height: 32px;
    border: 3px solid rgba(56, 189, 248, 0.2);
    border-top-color: var(--accent-frost);
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
  }

  .loading-text {
    font-size: 13px;
    font-weight: 500;
    color: var(--text-primary);
  }

  .loading-sub {
    font-size: 11px;
    color: var(--text-muted);
  }

  .error-box {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 30px;
    gap: 12px;
    color: #f43f5e;
    text-align: center;
  }

  /* Total Banner */
  .total-banner {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 14px 18px;
    background: var(--bg-surface-elevated);
    border: 1px solid var(--border-subtle);
    border-radius: var(--radius-md);
  }

  .total-info {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .total-label {
    font-size: 11px;
    color: var(--text-muted);
    text-transform: uppercase;
    letter-spacing: 0.05em;
    font-weight: 600;
  }

  .total-val {
    font-size: 22px;
    font-weight: 700;
    color: var(--text-primary);
  }

  .total-val .unit {
    font-size: 14px;
    font-weight: 500;
    color: var(--text-secondary);
  }

  .btn-refresh {
    gap: 6px;
    font-size: 11px;
    padding: 6px 12px;
  }

  /* Progress Bar Breakdown */
  .bar-container {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .bar-track {
    height: 8px;
    background: var(--bg-surface-hover);
    border-radius: 4px;
    display: flex;
    overflow: hidden;
  }

  .bar-segment {
    height: 100%;
    transition: width 0.3s ease;
  }

  .bar-apk {
    background: #38bdf8;
  }

  .bar-data {
    background: #818cf8;
  }

  .bar-cache {
    background: #fbbf24;
  }

  .bar-legend {
    display: flex;
    align-items: center;
    gap: 16px;
    font-size: 11px;
    color: var(--text-secondary);
  }

  .legend-item {
    display: flex;
    align-items: center;
    gap: 6px;
  }

  .legend-dot {
    width: 8px;
    height: 8px;
    border-radius: 2px;
  }

  .dot-apk { background: #38bdf8; }
  .dot-data { background: #818cf8; }
  .dot-cache { background: #fbbf24; }

  /* Metric Grid */
  .storage-grid {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 12px;
  }

  .metric-card {
    background: var(--bg-surface-elevated);
    border: 1px solid var(--border-subtle);
    border-radius: var(--radius-md);
    padding: 12px 14px;
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .metric-head {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .metric-icon-box {
    width: 24px;
    height: 24px;
    border-radius: var(--radius-xs);
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .apk-icon {
    background: rgba(56, 189, 248, 0.15);
    color: #38bdf8;
  }

  .data-icon {
    background: rgba(129, 140, 248, 0.15);
    color: #818cf8;
  }

  .cache-icon {
    background: rgba(251, 191, 36, 0.15);
    color: #fbbf24;
  }

  .metric-title {
    font-size: 11px;
    color: var(--text-secondary);
    font-weight: 500;
  }

  .metric-body {
    display: flex;
    flex-direction: column;
  }

  .metric-num {
    font-size: 16px;
    font-weight: 600;
    color: var(--text-primary);
  }

  .metric-num small {
    font-size: 11px;
    font-weight: normal;
    color: var(--text-muted);
  }

  .metric-sub {
    font-size: 10px;
    color: var(--text-muted);
    margin-top: 2px;
  }

  /* Actions Bar */
  .storage-action-box {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 14px 16px;
    background: var(--bg-surface-elevated);
    border: 1px dashed var(--border-medium);
    border-radius: var(--radius-md);
    margin-top: 4px;
    gap: 14px;
  }

  .action-desc-col {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .action-box-title {
    font-size: 12px;
    font-weight: 600;
    color: var(--text-primary);
  }

  .action-box-sub {
    font-size: 11px;
    color: var(--text-muted);
  }

  .action-btns-group {
    display: flex;
    align-items: center;
    gap: 8px;
    flex-shrink: 0;
  }

  .btn-warning {
    background: rgba(245, 158, 11, 0.12);
    border-color: rgba(245, 158, 11, 0.3);
    color: #fbbf24;
  }

  .btn-warning:hover:not(:disabled) {
    background: rgba(245, 158, 11, 0.22);
    border-color: rgba(245, 158, 11, 0.5);
  }

  .btn-warning-solid {
    background: #d97706;
    color: #ffffff;
    border: 1px solid #b45309;
  }

  .btn-warning-solid:hover {
    background: #b45309;
  }

  .btn-danger {
    background: rgba(244, 63, 94, 0.12);
    border-color: rgba(244, 63, 94, 0.3);
    color: #f43f5e;
  }

  .btn-danger:hover:not(:disabled) {
    background: rgba(244, 63, 94, 0.22);
    border-color: rgba(244, 63, 94, 0.5);
  }

  .btn-danger-solid {
    background: #e11d48;
    color: #ffffff;
    border: 1px solid #be123c;
  }

  .btn-danger-solid:hover {
    background: #be123c;
  }

  /* Footer */
  .storage-footer {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 12px 20px;
    background: var(--bg-surface-elevated);
    border-top: 1px solid var(--border-subtle);
  }

  .footer-tip {
    font-size: 11px;
    color: var(--text-muted);
  }

  .btn-ghost {
    background: transparent;
    border: 1px solid var(--border-subtle);
    color: var(--text-secondary);
  }

  .btn-ghost:hover {
    color: var(--text-primary);
    background: var(--bg-surface-hover);
  }

  /* Confirm Dialogs */
  .confirm-overlay {
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    background: rgba(0, 0, 0, 0.7);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1100;
    animation: fadeIn 0.1s ease-out;
  }

  .confirm-dialog {
    background: var(--bg-surface);
    border: 1px solid var(--border-medium);
    border-radius: var(--radius-lg);
    width: 90%;
    max-width: 440px;
    box-shadow: 0 16px 36px rgba(0, 0, 0, 0.7);
    padding: 20px;
    display: flex;
    flex-direction: column;
    gap: 16px;
    animation: scaleUp 0.15s cubic-bezier(0.16, 1, 0.3, 1);
  }

  .confirm-header {
    display: flex;
    align-items: flex-start;
    gap: 12px;
  }

  .confirm-icon-wrap {
    width: 38px;
    height: 38px;
    border-radius: var(--radius-md);
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
  }

  .confirm-icon-wrap.warning {
    background: rgba(245, 158, 11, 0.15);
    color: #fbbf24;
    border: 1px solid rgba(245, 158, 11, 0.3);
  }

  .confirm-icon-wrap.danger {
    background: rgba(244, 63, 94, 0.15);
    color: #f43f5e;
    border: 1px solid rgba(244, 63, 94, 0.3);
  }

  .confirm-head-text h4 {
    font-size: 14px;
    font-weight: 600;
    color: var(--text-primary);
  }

  .confirm-head-text .text-danger {
    color: #f43f5e;
  }

  .confirm-head-text p {
    font-size: 11px;
    color: var(--text-muted);
    margin-top: 2px;
  }

  .confirm-content {
    display: flex;
    flex-direction: column;
    gap: 10px;
    font-size: 12.5px;
    color: var(--text-secondary);
    line-height: 1.5;
  }

  .safe-note {
    background: rgba(34, 197, 94, 0.08);
    border: 1px solid rgba(34, 197, 94, 0.2);
    border-radius: var(--radius-sm);
    padding: 8px 10px;
    font-size: 11.5px;
    color: #4ade80;
  }

  .danger-note {
    background: rgba(244, 63, 94, 0.08);
    border: 1px solid rgba(244, 63, 94, 0.2);
    border-radius: var(--radius-sm);
    padding: 8px 10px;
    font-size: 11.5px;
    color: #f43f5e;
  }

  .confirm-actions {
    display: flex;
    justify-content: flex-end;
    gap: 10px;
    margin-top: 6px;
  }

  @keyframes fadeIn {
    from { opacity: 0; }
    to { opacity: 1; }
  }

  @keyframes scaleUp {
    from { transform: scale(0.96); opacity: 0; }
    to { transform: scale(1); opacity: 1; }
  }

  @keyframes spin {
    to { transform: rotate(360deg); }
  }
</style>
