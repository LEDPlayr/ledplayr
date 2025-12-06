<script lang="ts">
  import { onDestroy, onMount } from "svelte";

  import { getCurrentTimeAndTimezone, listTimezones, setTimezone } from "$lib/client";
  import { notify } from "$lib/utils";

  let time = $state("");
  let timezone = $state("");
  let timezones: string[] = $state([]);
  let useCurrentTime = $state(true);

  let browserTime = $state("");
  let browserTimezone = $state("");
  let serverTime = $state("");
  let serverTimezone = $state("");

  let pollInterval: ReturnType<typeof setInterval>;

  async function fetchServerTime() {
    const { data: serverData, error: serverError } = await getCurrentTimeAndTimezone();

    if (serverData) {
      let t = serverData.time;
      serverTime = t.slice(0, 19);
      serverTimezone = serverData.timezone;
    } else if (serverError) {
      if (!serverTime) {
        notify(`Error loading server time: ${serverError.error}`, "error");
      }
    }

    const now = new Date();
    browserTime = now.toISOString().slice(0, 19);
  }

  onMount(async () => {
    updateTime();
    browserTimezone = Intl.DateTimeFormat().resolvedOptions().timeZone;
    timezone = browserTimezone;

    const { data, error } = await listTimezones();
    if (data) {
      timezones = data;
    } else if (error) {
      notify(`Error loading timezones: ${error.error}`, "error");
    }

    await fetchServerTime();

    pollInterval = setInterval(fetchServerTime, 1000);
  });

  onDestroy(() => {
    if (pollInterval) {
      clearInterval(pollInterval);
    }
  });

  function updateTime() {
    const now = new Date();
    // For datetime-local format
    time = now.toISOString().slice(0, 16);
  }

  const handleSubmit = async () => {
    let isoTime = time;
    let tz = timezone;

    if (useCurrentTime) {
      const date = new Date();
      isoTime = date.toISOString();
      tz = browserTimezone;
    } else {
      if (time.length === 16) {
        // datetime-local format YYYY-MM-DDTHH:MM
        isoTime = `${time}:00`;
      }

      const date = new Date(isoTime);
      isoTime = date.toISOString();
    }

    const { error } = await setTimezone({
      body: { time: isoTime, timezone: tz },
    });
    if (error) {
      notify(`Error: ${error.error}`, "error");
    } else {
      notify("Time and timezone updated successfully", "success");
    }
  };
</script>

<svelte:head>
  <title>LEDPlayr: Configuration</title>
</svelte:head>

<div class="p-5">
  <h1 class="text-2xl">Configuration</h1>

  <div class="divider"></div>

  <div class="mb-4">
    <p class="text-sm text-gray-600">Server Time: {serverTime} ({serverTimezone})</p>
  </div>

  <div class="mb-4">
    <p class="text-sm text-gray-600">Browser Time: {browserTime} ({browserTimezone})</p>
  </div>

  <div class="form-control mt-2 w-full max-w-xs">
    <label class="label cursor-pointer" for="current-time-checkbox">
      <span class="label-text">Use current browser time</span>
      <input
        id="current-time-checkbox"
        type="checkbox"
        class="checkbox"
        bind:checked={useCurrentTime} />
    </label>
  </div>

  <div class="form-control w-full max-w-xs">
    <label class="label" for="time-input">
      <span class="label-text">Time</span>
    </label>
    <input
      id="time-input"
      type="datetime-local"
      class="input input-bordered w-full max-w-xs"
      bind:value={time}
      disabled={useCurrentTime} />
  </div>

  <div class="form-control mt-4 w-full max-w-xs">
    <label class="label" for="timezone-select">
      <span class="label-text">Timezone</span>
    </label>
    <select
      id="timezone-select"
      class="select select-bordered w-full max-w-xs"
      bind:value={timezone}
      disabled={useCurrentTime}>
      {#each timezones as tz (tz)}
        <option value={tz}>{tz}</option>
      {/each}
    </select>
  </div>

  <button class="btn btn-primary mt-4" onclick={handleSubmit}> Set Time and Timezone </button>
</div>
