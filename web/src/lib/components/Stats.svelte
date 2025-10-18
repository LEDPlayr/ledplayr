<script lang="ts">
  import PhPlayDuotone from "~icons/ph/play-duotone";
  import PhSpinner from "~icons/ph/spinner";
  import PhStopDutone from "~icons/ph/stop-duotone";

  import { startScheduler, stop } from "$lib/client";
  import { playerStatus, sysInfo } from "$lib/stores";
  import { isPlaying, updateStatus } from "$lib/utils";

  const toggleScheduler = async () => {
    const playing = isPlaying($playerStatus);

    if (playing === true) {
      await stop();
      await updateStatus();
    } else if (playing === false) {
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
    <span class="flex-grow font-semibold">CPU:</span>{$sysInfo?.Utilization.CPU.toFixed(2)}%
  </div>
  <div class="flex w-full flex-row">
    <span class="flex-grow font-semibold">Memory:</span>{(
      ($sysInfo ? $sysInfo.Utilization.Memory : 0) * 100
    ).toFixed(2)}%
  </div>
  <div class="flex w-full flex-row">
    <span class="flex-grow font-semibold">Status:</span>
    <span class="capitalize">{$playerStatus}</span>
  </div>
  <button type="button" class="btn btn-neutral btn-sm m-2" onclick={toggleScheduler}>
    {#if isPlaying($playerStatus) === true}
      <PhStopDutone /> Stop Scheduler
    {:else if isPlaying($playerStatus) === false}
      <PhPlayDuotone /> Start Scheduler
    {:else}
      <PhSpinner class="animate-spin" /> Pending
    {/if}
  </button>
</div>
