<script lang="ts">
  import type { Button } from "$lib/client";

  import PhArrowsClockwise from "virtual:icons/ph/arrows-clockwise";

  import { onMount } from "svelte";

  import { listButtons } from "$lib/client";
  import { notify } from "$lib/utils";

  let buttons: Button[] = [];

  onMount(async () => {
    await loadButtons();
  });

  const loadButtons = async () => {
    const { data, error } = await listButtons();
    if (data) {
      buttons = data;
    } else {
      buttons = [];
    }
    if (error) {
      notify(`${error.error}`, "error");
    }
  };
</script>

<svelte:head>
  <title>LEDPlayr: Models</title>
</svelte:head>

<div class="p-5">
  <h1 class="text-2xl">View Models</h1>

  <div class="divider"></div>

  <button class="btn join-item my-4" onclick={loadButtons}>
    <PhArrowsClockwise /> Refresh Buttons
  </button>

  {#if buttons.length > 0}
    <table class="table">
      <thead class="hidden sm:table-header-group">
        <tr>
          <th>ID</th>
          <th>Status</th>
          <th>Error</th>
          <th>Battery</th>
          <th>Input</th>
          <th>Last</th>
          <th>Now</th>
        </tr>
      </thead>
      <tbody class="grid grid-cols-1 gap-2 sm:table-row-group">
        {#each buttons as row (row.id)}
          <tr class="hover card mb-4 flex flex-col border sm:table-row sm:border-none">
            <td class="flex flex-row sm:table-cell">
              <span class="flex-grow font-semibold sm:hidden">ID:</span>
              {row.id}
            </td>
            <td class="flex flex-row sm:table-cell">
              <span class="flex-grow font-semibold sm:hidden">Status:</span>
              {row.status}
            </td>
            <td class="flex flex-row sm:table-cell">
              <span class="flex-grow font-semibold sm:hidden">Error:</span>
              {row.error}
            </td>
            <td class="flex flex-row sm:table-cell">
              <span class="flex-grow font-semibold sm:hidden">Battery:</span>
              {row.battery}
            </td>
            <td class="flex flex-row sm:table-cell">
              <span class="flex-grow font-semibold sm:hidden">Input:</span>
              {row.input}
            </td>
            <td class="flex flex-row sm:table-cell">
              <span class="flex-grow font-semibold sm:hidden">Last:</span>
              {row.last}
            </td>
            <td class="flex flex-row sm:table-cell">
              <span class="flex-grow font-semibold sm:hidden">Now:</span>
              {row.now}
            </td>
          </tr>
        {/each}
      </tbody>
    </table>
  {/if}
</div>
