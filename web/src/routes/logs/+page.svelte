<script lang="ts">
  import PhArrowsClockwise from "virtual:icons/ph/arrows-clockwise";

  import { AnsiUp } from "ansi_up";
  import { onMount, tick } from "svelte";

  import { getLog, listLogs } from "$lib/client";
  import { notify } from "$lib/utils";

  let logfiles: string[] = $state([]);
  let selectedLogfile: string | undefined = $state();
  let log = $state("");
  let lastRefresh = $state("");
  let logDiv: HTMLDivElement | undefined = $state();
  let scrollBottom = $state(true);

  onMount(() => {
    loadLogfiles();

    const invtl = setInterval(() => {
      loadLog();
    }, 1000);

    return () => {
      clearInterval(invtl);
    };
  });

  const loadLogfiles = async () => {
    const { data, error } = await listLogs();
    if (data) {
      logfiles = data;
    } else {
      logfiles = [];
    }
    if (error) {
      notify(`${error.error}`, "error");
    }
  };

  const loadLog = async () => {
    if (!selectedLogfile) return;

    const { data, error } = await getLog({ path: { name: selectedLogfile } });

    const text = data || "";
    const ansi = new AnsiUp();
    log = ansi.ansi_to_html(text);
    lastRefresh = new Date().toISOString();

    if (error) {
      notify(`${error.error}`, "error");
    }

    await tick();

    if (scrollBottom && logDiv) {
      logDiv.scrollTop = logDiv.scrollHeight - logDiv.clientHeight;
    }
  };

  $effect(() => {
    if (selectedLogfile) {
      loadLog();
    }
  });

  const scrolled = () => {
    if (!logDiv) return;
    scrollBottom = logDiv.scrollTop === logDiv.scrollHeight - logDiv.clientHeight;
  };
</script>

<svelte:head>
  <title>LEDPlayr: Logs</title>
</svelte:head>

<div class="p-5">
  <h1 class="text-2xl">View Server Logs</h1>

  <div class="divider"></div>

  <label class="form-control w-full max-w-xl">
    <div class="label">
      <span class="label-text">Select a log file</span>
    </div>

    <div class="join w-full">
      <select bind:value={selectedLogfile} class="join-item select select-bordered flex-grow">
        {#each logfiles as lf}
          <option>{lf}</option>
        {/each}
      </select>

      <button class="btn join-item" onclick={loadLogfiles}>
        <PhArrowsClockwise />
      </button>
    </div>
  </label>

  {#if selectedLogfile}
    <p class="my-4">Refreshed at: {lastRefresh}</p>
    <div
      class="my-4 max-h-96 overflow-y-auto rounded border p-4"
      onscroll={scrolled}
      bind:this={logDiv}>
      <pre>{@html log}</pre>
    </div>
  {/if}
</div>
