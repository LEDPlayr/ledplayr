<script lang="ts">
  import type { Channels, Universe } from "$lib/client";

  import PhArrowsClockwise from "~icons/ph/arrows-clockwise";
  import PhBackspaceDuotone from "~icons/ph/backspace-duotone";
  import PhFloppyDiskDuotone from "~icons/ph/floppy-disk-duotone";
  import PhNotePencilDuotone from "~icons/ph/note-pencil-duotone";

  import { onMount } from "svelte";

  import { getOutputs, uploadOutputs } from "$lib/client";
  import Delete from "$lib/components/Delete.svelte";
  import { entries, notify } from "$lib/utils";

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
  let outputs: Record<number, Universe> = $state({});

  onMount(async () => {
    await loadOutputs();
  });

  const loadOutputs = async () => {
    const { data, error } = await getOutputs();
    if (data) {
      outputs = Object.fromEntries(
        data.channelOutputs[0].universes
          .sort((a, b) => {
            return a.id - b.id;
          })
          .map((u) => [u.id, u]),
      );
    } else {
      outputs = {};
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
          universes: Object.values(outputs),
        },
      ],
    };
    const { error } = await uploadOutputs({ body: channels });
    if (error) {
      notify(`${error.error}`, "error");
    }
    await loadOutputs();
  };

  const addOrUpdateOutput = async () => {
    outputs[outputToAdd.id] = outputToAdd;
    await saveOutputs();
  };

  const clearOutput = () => {
    outputToAdd = { ...emptyOutput };
  };

  const editOutput = (u: Universe) => {
    outputToAdd = { ...u };
  };

  const removeOutput = async (i: number) => {
    delete outputs[i];
    await saveOutputs();
  };
</script>

<svelte:head>
  <title>LEDPlayr: Outputs</title>
</svelte:head>

<div class="p-5">
  <h1 class="text-2xl">Manage Outputs</h1>

  <div class="divider"></div>

  <fieldset class="fieldset max-w-xl gap-2">
    <legend class="fieldset-legend text-xl">Create / Update Outputs</legend>

    <label class="input input-bordered w-full max-w-xl">
      <span class="label">ID:</span>
      <input type="number" bind:value={outputToAdd.id} />
    </label>

    <label class="input input-bordered w-full max-w-xl">
      <span class="label">Description:</span>
      <input type="text" bind:value={outputToAdd.description} placeholder="Description" />
    </label>

    <label class="label">
      Active?
      <input type="checkbox" bind:checked={outputToAdd.active} class="toggle" />
    </label>

    <label class="input input-bordered w-full max-w-xl">
      <span class="label">Description:</span>
      <input type="text" bind:value={outputToAdd.address} placeholder="127.0.0.1" />
    </label>

    <label class="input input-bordered w-full max-w-xl">
      <span class="label">Start Channel:</span>
      <input type="number" bind:value={outputToAdd.startChannel} />
    </label>

    <label class="input input-bordered w-full max-w-xl">
      <span class="label">Channel Count:</span>
      <input type="number" bind:value={outputToAdd.channelCount} />
    </label>
  </fieldset>

  <div class="my-3 grid w-full max-w-xl grid-cols-2 gap-4">
    <button onclick={addOrUpdateOutput} class="btn btn-primary">
      <PhFloppyDiskDuotone />
      {outputToAdd.id in outputs ? "Update" : "Add"} Output
    </button>
    <button onclick={clearOutput} class="btn btn-neutral">
      <PhBackspaceDuotone /> Clear Outputs
    </button>
  </div>

  <div class="divider"></div>

  <h2 class="text-xl">Edit Outputs</h2>

  <button class="btn join-item my-4" onclick={loadOutputs}>
    <PhArrowsClockwise /> Refresh Outputs
  </button>

  {#if Object.keys(outputs).length > 0}
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
        {#each entries(outputs) as [id, row] (id)}
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
                <PhNotePencilDuotone />
              </button>
              <Delete
                showText={false}
                callback={async () => {
                  await removeOutput(id);
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
