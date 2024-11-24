<script lang="ts">
  import type { Model } from "$lib/client";

  import { onMount } from "svelte";

  import { listModels } from "$lib/client";
  import { notify } from "$lib/utils";

  let models: Model[] = [];

  onMount(async () => {
    await loadModels();
  });

  const loadModels = async () => {
    const { data, error } = await listModels();
    if (data) {
      models = data;
    } else {
      models = [];
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

  {#if models.length > 0}
    <table class="table">
      <thead class="hidden sm:table-header-group">
        <tr>
          <th>Name</th>
          <th>Start Channel</th>
          <th>End Channel</th>
          <th>Node Count</th>
          <th>Channels per Node</th>
          <th>Channel Count</th>
          <th>String Count</th>
          <th>Strands per String</th>
        </tr>
      </thead>
      <tbody class="grid grid-cols-1 gap-2 sm:table-row-group">
        {#each models as row (row.Name)}
          <tr class="hover card mb-4 flex flex-col border sm:table-row sm:border-none">
            <td class="flex flex-row sm:table-cell">
              <span class="flex-grow font-semibold sm:hidden">Name:</span>
              {row.Name}
            </td>
            <td class="flex flex-row sm:table-cell">
              <span class="flex-grow font-semibold sm:hidden">Start Channel:</span>
              {row.StartChannel}
            </td>
            <td class="flex flex-row sm:table-cell">
              <span class="flex-grow font-semibold sm:hidden">End Channel:</span>
              {row.StartChannel + row.ChannelCount - 1}
            </td>
            <td class="flex flex-row sm:table-cell">
              <span class="flex-grow font-semibold sm:hidden">Node Count:</span>
              {row.ChannelCount / row.ChannelCountPerNode}
            </td>
            <td class="flex flex-row sm:table-cell">
              <span class="flex-grow font-semibold sm:hidden">Channels per Node:</span>
              {row.ChannelCountPerNode}
            </td>
            <td class="flex flex-row sm:table-cell">
              <span class="flex-grow font-semibold sm:hidden">Channel Count:</span>
              {row.ChannelCount}
            </td>
            <td class="flex flex-row sm:table-cell">
              <span class="flex-grow font-semibold sm:hidden">String Node:</span>
              {row.StringCount}
            </td>
            <td class="flex flex-row sm:table-cell">
              <span class="flex-grow font-semibold sm:hidden">Strands per String:</span>
              {row.StrandsPerString}
            </td>
          </tr>
        {/each}
      </tbody>
    </table>
  {/if}
</div>
