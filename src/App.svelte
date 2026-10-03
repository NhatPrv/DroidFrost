<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import Header from './components/Header.svelte';
  import FilterBar from './components/FilterBar.svelte';
  import ProcessTable from './components/ProcessTable.svelte';
  import WirelessModal from './components/WirelessModal.svelte';
  import { api } from './lib/api';
  import type {
    DeviceInfo,
    ProcessInfo,
    SystemMemoryInfo,
    TabFilter,
  } from './types';

  let devices: DeviceInfo[] = [];
  let selectedSerial: string = '';
  let memory: SystemMemoryInfo = {
    total_ram_mb: 0,
    used_ram_mb: 0,
    free_ram_mb: 0,
    cached_ram_mb: 0,
    process_metric: 'PSS',
  };
  let processes: ProcessInfo[] = [];

  let currentTab: TabFilter = 'all';
  let showSystemApps: boolean = false;
  let searchQuery: string = '';
  let isBoosting: boolean = false;
  let isStoppingAll: boolean = false;
  let isWirelessModalOpen: boolean = false;

  let toastMessage: string = '';
  let toastTimeout: any = null;
  let pollInterval: any = null;
  let refreshInFlight = false;

  function showToast(msg: string) {
    toastMessage = msg;
    if (toastTimeout) clearTimeout(toastTimeout);
    toastTimeout = setTimeout(() => {
      toastMessage = '';
    }, 3000);
  }

  async function loadDevices() {
    try {
      devices = await api.getDevices();
      if (devices.length > 0 && (!selectedSerial || !devices.some(d => d.serial === selectedSerial))) {
        selectedSerial = devices[0].serial;
        await refreshState();
      }
    } catch (e: any) {
      console.error('Lỗi nạp thiết bị:', e);
    }
  }

  async function refreshState() {
    if (!selectedSerial || refreshInFlight) return;
    const serial = selectedSerial;
    refreshInFlight = true;
    try {
      const [mem, procs] = await api.getDeviceState(serial);
      if (serial === selectedSerial) {
        memory = mem;
        processes = procs;
      }
    } catch (e: any) {
      console.error('Lỗi cập nhật trạng thái:', e);
      showToast(`Không đọc được trạng thái thiết bị: ${e}`);
    } finally {
      refreshInFlight = false;
    }
  }

  // Thao tác đơn lẻ
  async function handleKill(pkg: string) {
    const res = await api.killApp(selectedSerial, pkg);
    showToast(res.message);
    await refreshState();
  }

  async function handleFreeze(pkg: string) {
    const res = await api.freezeApp(selectedSerial, pkg);
    showToast(res.message);
    await refreshState();
  }

  async function handleUnfreeze(pkg: string) {
    const res = await api.unfreezeApp(selectedSerial, pkg);
    showToast(res.message);
    await refreshState();
  }

  async function handleSchedule(pkg: string, seconds: number) {
    const res = await api.scheduleFreeze(selectedSerial, pkg, seconds);
    showToast(res.message);
    await refreshState();
  }

  async function handleCancelSchedule(pkg: string) {
    const res = await api.cancelSchedule(selectedSerial, pkg);
    showToast(res.message);
    await refreshState();
  }

  // Thao tác hàng loạt (Batch Operations)
  async function handleBatchKill(pkgs: string[]) {
    let successCount = 0;
    for (const pkg of pkgs) {
      const res = await api.killApp(selectedSerial, pkg);
      if (res.success) successCount++;
    }
    showToast(`Đã buộc dừng thành công ${successCount}/${pkgs.length} ứng dụng.`);
    await refreshState();
  }

  async function handleBatchFreeze(pkgs: string[]) {
    let successCount = 0;
    for (const pkg of pkgs) {
      const res = await api.freezeApp(selectedSerial, pkg);
      if (res.success) successCount++;
    }
    showToast(`Đã đóng băng thành công ${successCount}/${pkgs.length} ứng dụng.`);
    await refreshState();
  }

  async function handleOneClickBoost() {
    isBoosting = true;
    try {
      const res = await api.oneClickBoost(selectedSerial);
      showToast(`Đã gửi lệnh dừng cho ${res.killed_count} ứng dụng. RAM đang được đo lại.`);
      await refreshState();
    } catch (e: any) {
      showToast('Lỗi khi thực hiện boost: ' + e);
    } finally {
      isBoosting = false;
    }
  }

  async function handleStopAllRunning() {
    if (!selectedSerial) return;
    isStoppingAll = true;
    try {
      const res = await api.stopAllRunning(selectedSerial, false);
      showToast(`Đã gửi lệnh dừng cho ${res.killed_count} ứng dụng. RAM đang được đo lại.`);
      await refreshState();
    } catch (e: any) {
      showToast('Lỗi khi dừng các tiến trình: ' + e);
    } finally {
      isStoppingAll = false;
    }
  }

  // Lọc danh sách tiến trình theo tab và search
  $: filteredProcesses = processes.filter((p) => {
    if (!showSystemApps && p.is_system) {
      return false;
    }

    if (currentTab === 'running' && !p.is_running) return false;
    if (currentTab === 'frozen' && !p.is_frozen) return false;
    if (currentTab === 'scheduled' && !p.is_scheduled) return false;

    if (searchQuery.trim()) {
      const q = searchQuery.toLowerCase().trim();
      const matchName = p.app_name.toLowerCase().includes(q);
      const matchPkg = p.package_name.toLowerCase().includes(q);
      return matchName || matchPkg;
    }

    return true;
  });

  $: countAll = processes.filter(p => showSystemApps || !p.is_system).length;
  $: countRunning = processes.filter(p => p.is_running && (showSystemApps || !p.is_system)).length;
  $: countStoppable = processes.filter(p => p.is_running && !p.is_system && !p.is_whitelisted).length;
  $: countFrozen = processes.filter(p => p.is_frozen && (showSystemApps || !p.is_system)).length;
  $: countScheduled = processes.filter(p => p.is_scheduled && (showSystemApps || !p.is_system)).length;

  onMount(() => {
    loadDevices();
    pollInterval = setInterval(() => {
      refreshState();
    }, 4000);
  });

  onDestroy(() => {
    if (pollInterval) clearInterval(pollInterval);
    if (toastTimeout) clearTimeout(toastTimeout);
  });
</script>

<main class="app-layout">
  <!-- Top Navigation Header -->
  <Header
    {devices}
    {selectedSerial}
    {memory}
    onSelectDevice={(serial) => {
      selectedSerial = serial;
      refreshState();
    }}
    onRefresh={() => {
      loadDevices();
      refreshState();
    }}
    onOpenWirelessModal={() => (isWirelessModalOpen = true)}
  />

  <!-- Filter & Actions Sub-bar -->
  <FilterBar
    {currentTab}
    {showSystemApps}
    bind:searchQuery
    {countAll}
    {countRunning}
    {countStoppable}
    {countFrozen}
    {countScheduled}
    {isBoosting}
    {isStoppingAll}
    onTabChange={(tab) => (currentTab = tab)}
    onToggleSystem={() => (showSystemApps = !showSystemApps)}
    onOneClickBoost={handleOneClickBoost}
    onStopAllRunning={handleStopAllRunning}
  />

  <!-- Main Content Process Table -->
  <ProcessTable
    processes={filteredProcesses}
    processMetric={memory.process_metric}
    onKill={handleKill}
    onFreeze={handleFreeze}
    onUnfreeze={handleUnfreeze}
    onSchedule={handleSchedule}
    onCancelSchedule={handleCancelSchedule}
    onBatchKill={handleBatchKill}
    onBatchFreeze={handleBatchFreeze}
  />

  <!-- Wireless ADB Modal -->
  <WirelessModal
    isOpen={isWirelessModalOpen}
    onClose={() => {
      isWirelessModalOpen = false;
      loadDevices();
    }}
    onConnect={async (ip, port) => {
      const res = await api.connectWireless(ip, port);
      await loadDevices();
      return res;
    }}
    onPair={async (ip, port, code) => {
      const res = await api.pairWireless(ip, port, code);
      await loadDevices();
      return res;
    }}
  />

  <!-- Studio Toast Notification -->
  {#if toastMessage}
    <div class="toast-card">
      <span class="toast-dot"></span>
      <span class="toast-text">{toastMessage}</span>
    </div>
  {/if}
</main>

<style>
  .app-layout {
    display: flex;
    flex-direction: column;
    height: 100vh;
    width: 100vw;
    background: var(--bg-app);
    position: relative;
  }

  .toast-card {
    position: fixed;
    bottom: 20px;
    right: 20px;
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 8px 14px;
    border-radius: var(--radius-sm);
    background: var(--bg-surface-elevated);
    border: 1px solid var(--border-medium);
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.4);
    z-index: 200;
    animation: fadeIn 0.2s ease;
  }

  @keyframes fadeIn {
    from { opacity: 0; transform: translateY(6px); }
    to { opacity: 1; transform: translateY(0); }
  }

  .toast-dot {
    width: 6px;
    height: 6px;
    border-radius: 50%;
    background: var(--accent-frost);
  }

  .toast-text {
    font-size: 12px;
    color: var(--text-primary);
  }
</style>
