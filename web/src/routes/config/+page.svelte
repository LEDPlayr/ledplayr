<script lang="ts">
  import { onMount } from "svelte";

  import { listTimezones, setTimezone } from "$lib/client";
  import { notify } from "$lib/utils";

  let time = "";
  let timezone = "";
  let timezones: string[] = [];
  let useCurrentTime = false;

  onMount(async () => {
    // Get current browser time and timezone
    updateTime();
    timezone = Intl.DateTimeFormat().resolvedOptions().timeZone;

    // Load available timezones
    const { data, error } = await listTimezones();
    if (data) {
      timezones = data;
    } else if (error) {
      notify(`Error loading timezones: ${error.error}`, "error");
    }
  });

  function updateTime() {
    const now = new Date();
    // For datetime-local format
    time = now.toISOString().slice(0, 16);
  }

  const handleSubmit = async () => {
    let isoTime = time;

    if (useCurrentTime) {
      const date = new Date();
      isoTime = date.toISOString();
    } else {
      if (time.length === 16) {
        // datetime-local format YYYY-MM-DDTHH:MM
        isoTime = `${time}:00`;
      }

      // Parse and reformat to ensure valid ISO
      const date = new Date(isoTime);
      isoTime = date.toISOString();
    }

    const { error } = await setTimezone({
      body: { time: isoTime, timezone },
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

  <div class="form-control mt-2 w-full max-w-xs">
    <label class="label cursor-pointer" for="current-time-checkbox">
      <span class="label-text">Use current browser time</span>
      <input
        id="current-time-checkbox"
        type="checkbox"
        class="checkbox"
        bind:checked={useCurrentTime}
        onchange={() => {
          if (useCurrentTime) {
            updateTime();
          }
        }} />
    </label>
  </div>

  <div class="form-control mt-4 w-full max-w-xs">
    <label class="label" for="timezone-select">
      <span class="label-text">Timezone</span>
    </label>
    <select
      id="timezone-select"
      class="select select-bordered w-full max-w-xs"
      bind:value={timezone}>
      {#each timezones as tz (tz)}
        <option value={tz}>{tz}</option>
      {/each}
    </select>
  </div>

  <button class="btn btn-primary mt-4" onclick={handleSubmit}> Set Time and Timezone </button>
</div>
