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
    total_ram_mb: 8192,
    used_ram_mb: 4096,
    free_ram_mb: 4096,
    cached_ram_mb: 0,
  };
  let processes: ProcessInfo[] = [];

  let currentTab: TabFilter = 'all';
  let showSystemApps: boolean = false;
  let searchQuery: string = '';
  let isBoosting: boolean = false;
  let isWirelessModalOpen: boolean = false;

  let toastMessage: string = '';
  let toastTimeout: any = null;

  let pollInterval: any = null;

  function showToast(msg: string) {
    toastMessage = msg;
    if (toastTimeout) clearTimeout(toastTimeout);
    toastTimeout = setTimeout(() => {
      toastMessage = '';
    }, 3500);
  }

  async function loadDevices() {
    try {
      devices = await api.getDevices();
      if (devices.length > 0 && (!selectedSerial || !devices.some(d => d.serial === selectedSerial))) {
        selectedSerial = devices[0].serial;
        await refreshState();
      }
    } catch (e: any) {
      console.error('Lỗi tải danh sách thiết bị:', e);
    }
  }

  async function refreshState() {
    if (!selectedSerial) return;
    try {
      const [mem, procs] = await api.getDeviceState(selectedSerial);
      memory = mem;
      processes = procs;
    } catch (e: any) {
      console.error('Lỗi cập nhật trạng thái thiết bị:', e);
    }
  }

  // Thao tác điều khiển
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

  async function handleOneClickBoost() {
    isBoosting = true;
    try {
      const res = await api.oneClickBoost(selectedSerial);
      showToast(`⚡ One-Click Boost: Đã dừng ${res.killed_count} ứng dụng, giải phóng ${res.freed_ram_mb} MB RAM!`);
      await refreshState();
    } catch (e: any) {
      showToast('Lỗi khi thực hiện boost: ' + e);
    } finally {
      isBoosting = false;
    }
  }

  // Lọc danh sách tiến trình theo tab và search
  $: filteredProcesses = processes.filter((p) => {
    // 1. Lọc theo System Apps
    if (!showSystemApps && p.is_system) {
      return false;
    }

    // 2. Lọc theo Tab
    if (currentTab === 'running' && !p.is_running) return false;
    if (currentTab === 'frozen' && !p.is_frozen) return false;
    if (currentTab === 'scheduled' && !p.is_scheduled) return false;

    // 3. Lọc theo Search Query
    if (searchQuery.trim()) {
      const q = searchQuery.toLowerCase().trim();
      const matchName = p.app_name.toLowerCase().includes(q);
      const matchPkg = p.package_name.toLowerCase().includes(q);
      return matchName || matchPkg;
    }

    return true;
  });

  // Đếm số lượng theo trạng thái
  $: countAll = processes.filter(p => showSystemApps || !p.is_system).length;
  $: countRunning = processes.filter(p => p.is_running && (showSystemApps || !p.is_system)).length;
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
    {countFrozen}
    {countScheduled}
    {isBoosting}
    onTabChange={(tab) => (currentTab = tab)}
    onToggleSystem={() => (showSystemApps = !showSystemApps)}
    onOneClickBoost={handleOneClickBoost}
  />

  <!-- Main Content Process Table -->
  <ProcessTable
    processes={filteredProcesses}
    onKill={handleKill}
    onFreeze={handleFreeze}
    onUnfreeze={handleUnfreeze}
    onSchedule={handleSchedule}
    onCancelSchedule={handleCancelSchedule}
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

  <!-- Toast Notification Overlay -->
  {#if toastMessage}
    <div class="toast-notification glass-panel">
      <span class="toast-icon">✨</span>
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
    background: radial-gradient(circle at 50% 0%, rgba(0, 210, 255, 0.04) 0%, transparent 60%);
    position: relative;
  }

  .toast-notification {
    position: fixed;
    bottom: 24px;
    right: 24px;
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 12px 20px;
    border-radius: var(--radius-md);
    border: 1px solid var(--border-active);
    box-shadow: 0 10px 30px rgba(0, 0, 0, 0.6);
    z-index: 200;
    animation: slideUp 0.3s cubic-bezier(0.16, 1, 0.3, 1);
  }

  @keyframes slideUp {
    from {
      opacity: 0;
      transform: translateY(20px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }

  .toast-icon {
    font-size: 16px;
  }

  .toast-text {
    font-size: 13px;
    font-weight: 600;
    color: var(--text-primary);
  }
</style>
