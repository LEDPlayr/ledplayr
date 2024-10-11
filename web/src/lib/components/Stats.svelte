<script lang="ts">
  import type { SchedulerStatus } from "$lib/client";

  import PhPause from "virtual:icons/ph/pause";
  import PhPlay from "virtual:icons/ph/play";
  import PhSpinner from "virtual:icons/ph/spinner";

  import { onMount } from "svelte";
  import { getSchedulerStatus, startScheduler, stopScheduler, systemInfo } from "$lib/client";
  import { sysInfo } from "$lib/stores";

  type PlayerStatus = "Unknown" | "Pending" | "Started" | "Stopped";

  let player_state: SchedulerStatus | undefined = $state();
  let loading = $state(true);
  let player_status: PlayerStatus = $derived.by(() => {
    if (loading) {
      return "Pending";
    } else if (player_state?.status == "Start") {
      return "Started";
    } else if (player_state?.status == "Stop") {
      return "Stopped";
    } else {
      return "Unknown";
    }
  });

  const update = async () => {
    try {
      $sysInfo = (await systemInfo()).data;
      player_state = (await getSchedulerStatus()).data;
    } catch (_err) {
      $sysInfo = undefined;
      player_state = undefined;
    }
    loading = false;
  };

  const toggleScheduler = async () => {
    if (player_status == "Started") {
      loading = true;
      await stopScheduler();
      await update();
    } else if (player_status == "Stopped") {
      loading = true;
      await startScheduler();
      await update();
    }
  };

  onMount(() => {
    const intvl = setInterval(update, 5000);
    update();
    return () => {
      clearInterval(intvl);
    };
  });
</script>

<div class="w-full">
  <div class="flex w-full flex-row">
    <span class="flex-grow font-semibold">Hostname:</span>{$sysInfo?.HostName}
  </div>
  <div class="flex w-full flex-row">
    <span class="flex-grow font-semibold">Version:</span>{$sysInfo?.LocalGitVersion}
  </div>
  <div class="flex w-full flex-row">
    <span class="flex-grow font-semibold">CPU:</span>{$sysInfo?.Utilization.CPU}%
  </div>
  <div class="flex w-full flex-row">
    <span class="flex-grow font-semibold">Memory:</span>{(
      ($sysInfo ? $sysInfo.Utilization.Memory : 0) * 100
    ).toFixed(2)}%
  </div>
  <div class="flex w-full flex-row">
    <span class="flex-grow font-semibold">Scheduler:</span>{player_status}
  </div>
  <button type="button" class="btn btn-neutral btn-sm m-2" onclick={toggleScheduler}>
    {#if player_status == "Started"}
      <PhPause /> Stop Scheduler
    {:else if player_status == "Stopped"}
      <PhPlay /> Start Scheduler
    {:else}
      <PhSpinner class="animate-spin" /> Pending
    {/if}
  </button>
</div>
