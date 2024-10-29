<script lang="ts">
  import PhPause from "virtual:icons/ph/pause";
  import PhPlay from "virtual:icons/ph/play";
  import PhSpinner from "virtual:icons/ph/spinner";

  import { startScheduler, stopScheduler } from "$lib/client";
  import { playerStatus, sysInfo } from "$lib/stores";
  import { updateStatus } from "$lib/utils";

  const toggleScheduler = async () => {
    if ($playerStatus == "Started" || $playerStatus == "Testing") {
      await stopScheduler();
      await updateStatus();
    } else if ($playerStatus == "Stopped") {
      await startScheduler();
      await updateStatus();
    }
  };
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
    <span class="flex-grow font-semibold">Scheduler:</span>{$playerStatus}
  </div>
  <button type="button" class="btn btn-neutral btn-sm m-2" onclick={toggleScheduler}>
    {#if $playerStatus == "Started" || $playerStatus == "Testing"}
      <PhPause /> Stop Scheduler
    {:else if $playerStatus == "Stopped"}
      <PhPlay /> Start Scheduler
    {:else}
      <PhSpinner class="animate-spin" /> Pending
    {/if}
  </button>
</div>
