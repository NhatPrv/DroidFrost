<script lang="ts">
  export let isOpen: boolean = false;
  export let onClose: () => void;
  export let onConnect: (ip: string, port: number) => Promise<string>;
  export let onPair: (ip: string, port: number, code: string) => Promise<string>;

  let ip: string = '';
  let port: number = 5555;
  let pairingCode: string = '';
  let isPairingMode: boolean = false;
  let isLoading: boolean = false;
  let statusMessage: string = '';
  let isError: boolean = false;

  async function handleAction() {
    if (!ip.trim()) {
      statusMessage = 'Vui lòng nhập địa chỉ IP thiết bị!';
      isError = true;
      return;
    }

    isLoading = true;
    statusMessage = 'Đang xử lý kết nối...';
    isError = false;

    try {
      if (isPairingMode) {
        if (!pairingCode.trim()) {
          statusMessage = 'Vui lòng nhập mã ghép nối (Pairing Code)!';
          isError = true;
          isLoading = false;
          return;
        }
        const res = await onPair(ip, port, pairingCode);
        statusMessage = res;
      } else {
        const res = await onConnect(ip, port);
        statusMessage = res;
      }
    } catch (e: any) {
      statusMessage = 'Lỗi kết nối: ' + (e.message || String(e));
      isError = true;
    } finally {
      isLoading = false;
    }
  }
</script>

{#if isOpen}
  <div class="modal-backdrop">
    <div class="modal-box glass-panel">
      <div class="modal-header">
        <div class="title-row">
          <span class="modal-icon">📶</span>
          <h3>Kết nối Wireless ADB</h3>
        </div>
        <button class="close-btn" on:click={onClose}>✕</button>
      </div>

      <div class="modal-body">
        <p class="modal-desc">
          Kết nối tới điện thoại qua Wi-Fi nội bộ mà không cần cắm dây cáp USB.
        </p>

        <div class="tabs-sub">
          <button
            class="subtab-btn"
            class:active={!isPairingMode}
            on:click={() => { isPairingMode = false; statusMessage = ''; }}
          >
            Kết nối trực tiếp (Port 5555)
          </button>
          <button
            class="subtab-btn"
            class:active={isPairingMode}
            on:click={() => { isPairingMode = true; statusMessage = ''; }}
          >
            Ghép nối Android 11+ (Pair Code)
          </button>
        </div>

        <div class="form-group">
          <label for="ip-input">Địa chỉ IP thiết bị</label>
          <input
            id="ip-input"
            type="text"
            placeholder="Ví dụ: 192.168.1.15"
            bind:value={ip}
          />
        </div>

        <div class="form-group">
          <label for="port-input">Cổng (Port)</label>
          <input
            id="port-input"
            type="number"
            placeholder="5555"
            bind:value={port}
          />
        </div>

        {#if isPairingMode}
          <div class="form-group">
            <label for="code-input">Mã ghép nối (6 chữ số)</label>
            <input
              id="code-input"
              type="text"
              placeholder="Ví dụ: 123456"
              bind:value={pairingCode}
            />
          </div>
        {/if}

        {#if statusMessage}
          <div class="status-box" class:error={isError}>
            {statusMessage}
          </div>
        {/if}
      </div>

      <div class="modal-footer">
        <button class="btn btn-secondary" on:click={onClose}>Đóng</button>
        <button
          class="btn btn-primary"
          on:click={handleAction}
          disabled={isLoading}
        >
          {#if isLoading}
            Đang kết nối...
          {:else}
            {isPairingMode ? 'Ghép nối' : 'Kết nối'}
          {/if}
        </button>
      </div>
    </div>
  </div>
{/if}

<style>
  .modal-backdrop {
    position: fixed;
    top: 0;
    left: 0;
    width: 100vw;
    height: 100vh;
    background: rgba(0, 0, 0, 0.7);
    backdrop-filter: blur(8px);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 100;
  }

  .modal-box {
    width: 440px;
    border-radius: var(--radius-lg);
    border: 1px solid var(--border-active);
    box-shadow: 0 20px 40px rgba(0, 0, 0, 0.7);
    overflow: hidden;
  }

  .modal-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 16px 20px;
    border-bottom: 1px solid var(--border-subtle);
  }

  .title-row {
    display: flex;
    align-items: center;
    gap: 10px;
  }

  .title-row h3 {
    font-size: 16px;
    font-weight: 700;
  }

  .modal-icon {
    font-size: 18px;
  }

  .close-btn {
    background: none;
    border: none;
    color: var(--text-muted);
    font-size: 16px;
    cursor: pointer;
  }

  .modal-body {
    padding: 20px;
    display: flex;
    flex-direction: column;
    gap: 16px;
  }

  .modal-desc {
    font-size: 13px;
    color: var(--text-secondary);
    line-height: 1.4;
  }

  .tabs-sub {
    display: flex;
    gap: 6px;
    background: var(--bg-surface-elevated);
    padding: 4px;
    border-radius: var(--radius-sm);
  }

  .subtab-btn {
    flex: 1;
    background: transparent;
    border: none;
    color: var(--text-muted);
    font-size: 11px;
    font-weight: 600;
    padding: 6px 0;
    border-radius: 4px;
    cursor: pointer;
  }

  .subtab-btn.active {
    background: var(--bg-surface);
    color: var(--text-primary);
  }

  .form-group {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .form-group label {
    font-size: 12px;
    font-weight: 600;
    color: var(--text-secondary);
  }

  .form-group input {
    background: var(--bg-surface-elevated);
    border: 1px solid var(--border-subtle);
    border-radius: var(--radius-sm);
    padding: 10px 14px;
    color: var(--text-primary);
    font-size: 13px;
    outline: none;
  }

  .form-group input:focus {
    border-color: var(--accent-cyan);
  }

  .status-box {
    background: rgba(16, 185, 129, 0.1);
    border: 1px solid rgba(16, 185, 129, 0.3);
    color: #34d399;
    padding: 10px;
    border-radius: var(--radius-sm);
    font-size: 12px;
  }

  .status-box.error {
    background: rgba(239, 68, 68, 0.1);
    border-color: rgba(239, 68, 68, 0.3);
    color: #f87171;
  }

  .modal-footer {
    display: flex;
    justify-content: flex-end;
    gap: 10px;
    padding: 14px 20px;
    background: rgba(0, 0, 0, 0.2);
    border-top: 1px solid var(--border-subtle);
  }
</style>
