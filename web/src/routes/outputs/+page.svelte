<script lang="ts">
  import type { Channels, Universe } from "$lib/client";

  import PhArrowsClockwise from "virtual:icons/ph/arrows-clockwise";
  import PhBackspace from "virtual:icons/ph/backspace";
  import PhFloppyDisk from "virtual:icons/ph/floppy-disk";
  import PhNotePencil from "virtual:icons/ph/note-pencil";

  import { onMount } from "svelte";
  import { getOutputs, uploadOutputs } from "$lib/client";
  import Delete from "$lib/components/Delete.svelte";
  import { notify } from "$lib/utils";

  let emptyOutput: Universe = {
    id: 1,
    description: "",
    active: true,
    address: "127.0.0.1",
    startChannel: 1,
    channelCount: 0,
    deDuplicate: false,
    monitor: true,
    priority: 0,
    type: 4,
  };
  let outputToAdd: Universe = $state({ ...emptyOutput });
  let outputs: Universe[] = $state([]);

  onMount(async () => {
    await loadOutputs();
  });

  const loadOutputs = async () => {
    const { data, error } = await getOutputs();
    if (data) {
      outputs = data.channelOutputs[0].universes;
    } else {
      outputs = [];
    }
    if (error) {
      notify(`${error.error}`, "error");
    }
  };

  const saveOutputs = async () => {
    const channels: Channels = {
      channelOutputs: [
        {
          type: "universes",
          startChannel: 1,
          enabled: true,
          timeout: 1000,
          channelCount: -1,
          universes: outputs,
        },
      ],
    };
    const { error } = await uploadOutputs({ body: channels });
    if (error) {
      notify(`${error.error}`, "error");
    }
    await loadOutputs();
  };

  const clearOutput = () => {
    outputToAdd = { ...emptyOutput };
  };

  const editOutput = (u: Universe) => {
    outputToAdd = { ...u };
  };

  const removeOutput = async (i: number) => {
    outputs.splice(i, 1);
    await saveOutputs();
  };
</script>

<svelte:head>
  <title>LEDPlayr: Outputs</title>
</svelte:head>

<div class="p-5">
  <h1 class="text-2xl">Manage Outputs</h1>

  <div class="divider"></div>

  <h2 class="text-xl">Create / Update Outputs</h2>

  <label class="form-control w-full max-w-xl">
    <div class="label">
      <span class="label-text">ID:</span>
    </div>
    <input
      type="number"
      bind:value={outputToAdd.id}
      class="input input-bordered w-full max-w-xl" />
  </label>

  <label class="form-control w-full max-w-xl">
    <div class="label">
      <span class="label-text">Description:</span>
    </div>
    <input
      type="text"
      bind:value={outputToAdd.description}
      class="input input-bordered w-full max-w-xl"
      placeholder="Description" />
  </label>

  <div class="form-control w-full max-w-xl">
    <label class="label cursor-pointer">
      <span class="label-text">Active?</span>
      <input type="checkbox" bind:checked={outputToAdd.active} class="toggle" />
    </label>
  </div>

  <label class="form-control w-full max-w-xl">
    <div class="label">
      <span class="label-text">Description:</span>
    </div>
    <input
      type="text"
      bind:value={outputToAdd.address}
      class="input input-bordered w-full max-w-xl"
      placeholder="127.0.0.1" />
  </label>

  <label class="form-control w-full max-w-xl">
    <div class="label">
      <span class="label-text">Start Channel:</span>
    </div>
    <input
      type="number"
      bind:value={outputToAdd.startChannel}
      class="input input-bordered w-full max-w-xl" />
  </label>

  <label class="form-control w-full max-w-xl">
    <div class="label">
      <span class="label-text">Channel Count:</span>
    </div>
    <input
      type="number"
      bind:value={outputToAdd.channelCount}
      class="input input-bordered w-full max-w-xl" />
  </label>

  <div class="my-3 grid w-full max-w-xl grid-cols-2 gap-4">
    <button onclick={saveOutputs} class="btn btn-primary">
      <PhFloppyDisk /> Save Outputs
    </button>
    <button onclick={clearOutput} class="btn btn-neutral">
      <PhBackspace /> Clear Outputs
    </button>
  </div>

  <div class="divider"></div>

  <h2 class="text-xl">Edit Outputs</h2>

  <button class="btn join-item my-4" onclick={loadOutputs}>
    <PhArrowsClockwise /> Refresh Outputs
  </button>

  {#if outputs.length > 0}
    <table class="table">
      <thead class="hidden sm:table-header-group">
        <tr>
          <th>ID</th>
          <th>Description</th>
          <th>Active</th>
          <th>IP Address</th>
          <th>Start Channel</th>
          <th>End Channel</th>
          <th>Channel Count</th>
          <th></th>
        </tr>
      </thead>
      <tbody class="grid grid-cols-1 gap-2 sm:table-row-group">
        {#each outputs as row, index (row.id)}
          <tr class="hover card mb-4 flex flex-col border sm:table-row sm:border-none">
            <td class="flex flex-row sm:table-cell">
              <span class="flex-grow font-semibold sm:hidden">ID:</span>
              {row.id}
            </td>
            <td class="flex flex-row sm:table-cell">
              <span class="flex-grow font-semibold sm:hidden">Description:</span>
              {row.description}
            </td>
            <td class="flex flex-row sm:table-cell">
              <span class="flex-grow font-semibold sm:hidden">Active:</span>
              {row.active ? "Yes" : "No"}
            </td>
            <td class="flex flex-row sm:table-cell">
              <span class="flex-grow font-semibold sm:hidden">IP Address:</span>
              {row.address}
            </td>
            <td class="flex flex-row sm:table-cell">
              <span class="flex-grow font-semibold sm:hidden">Start Channel:</span>
              {row.startChannel}
            </td>
            <td class="flex flex-row sm:table-cell">
              <span class="flex-grow font-semibold sm:hidden">End Channel:</span>
              {row.startChannel + row.channelCount - 1}
            </td>
            <td class="flex flex-row sm:table-cell">
              <span class="flex-grow font-semibold sm:hidden">Channel Count:</span>
              {row.channelCount}
            </td>
            <td class="grid grid-cols-2 gap-2 sm:table-cell sm:w-36">
              <button
                onclick={() => {
                  editOutput(row);
                }}
                class="btn btn-primary sm:mx-2 sm:h-8 sm:min-h-8 sm:w-8 sm:rounded-full sm:p-0 sm:text-sm">
                <PhNotePencil />
              </button>
              <Delete
                showText={false}
                callback={async () => {
                  await removeOutput(index);
                }}
                class="sm:mx-2 sm:h-8 sm:min-h-8 sm:w-8 sm:rounded-full sm:p-0 sm:text-sm" />
            </td>
          </tr>
        {/each}
      </tbody>
    </table>
  {:else}
    <div><span class="italic">No outputs. Trying creating one</span></div>
  {/if}
</div>
