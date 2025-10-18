<script lang="ts">
  import type { NumberedPlaylist, Schedule } from "$lib/client";

  import PhArrowsClockwise from "~icons/ph/arrows-clockwise";
  import PhBackspaceDuotone from "~icons/ph/backspace-duotone";
  import PhFloppyDiskDuotone from "~icons/ph/floppy-disk-duotone";
  import PhNotePencilDuotone from "~icons/ph/note-pencil-duotone";

  import { onMount } from "svelte";

  import { delSchedule, listPlaylistsNumbered, listSchedules, newSchedule } from "$lib/client";
  import Delete from "$lib/components/Delete.svelte";
  import { notify } from "$lib/utils";

  let emptySchedule: Schedule = {
    name: "",
    enabled: true,
    playlist_id: 0,
    start_date: "1970-01-01",
    end_date: "1970-01-01",
    start_time: "00:00",
    end_time: "00:00",
    monday: false,
    tuesday: false,
    wednesday: false,
    thursday: false,
    friday: false,
    saturday: false,
    sunday: false,
  };

  let scheduleToAdd: Schedule = $state({ ...emptySchedule });
  let schedules: Schedule[] = $state([]);
  let playlists: NumberedPlaylist[] = $state([]);

  onMount(async () => {
    await loadSchedules();
    await loadPlaylists();
  });

  const loadSchedules = async () => {
    const { data, error } = await listSchedules();
    if (data) {
      schedules = data;
    } else {
      schedules = [];
    }
    if (error) {
      notify(`${error.error}`, "error");
    }
  };

  const loadPlaylists = async () => {
    const { data, error } = await listPlaylistsNumbered();
    if (data) {
      playlists = data;
    } else {
      playlists = [];
    }
    if (error) {
      notify(`${error.error}`, "error");
    }
  };

  const saveSchedule = async () => {
    const { error } = await newSchedule({ body: scheduleToAdd });
    if (error) {
      notify(`${error.error}`, "error");
    }
    await loadSchedules();
  };

  const getPlaylistName = (id: number) => {
    const p = playlists.find((p) => p.id == id);
    return p ? p.name : "Unknown";
  };

  const editSchedule = (s: Schedule) => {
    scheduleToAdd = { ...s };
  };

  const clearSchedule = () => {
    scheduleToAdd = { ...emptySchedule };
  };

  const removeSchedule = async (s: Schedule) => {
    const { error } = await delSchedule({ path: { schedule: s.name } });
    if (error) {
      notify(`${error.error}`, "error");
    }
    await loadSchedules();
  };
</script>

<svelte:head>
  <title>LEDPlayr: Schedules</title>
</svelte:head>

<div class="p-5">
  <h1 class="text-2xl">Manage Schedules</h1>

  <div class="divider"></div>

  <fieldset class="fieldset max-w-xl gap-2">
    <legend class="fieldset-legend text-xl">Create / Update Schedule</legend>

    <label class="input input-bordered w-full max-w-xl">
      <span class="label">Name:</span>
      <input type="text" bind:value={scheduleToAdd.name} placeholder="Name" />
    </label>

    <label class="label">
      Enabled?
      <input type="checkbox" bind:checked={scheduleToAdd.enabled} class="toggle" />
    </label>

    <label class="input input-bordered w-full max-w-xl">
      <span class="label">Start Date</span>
      <input type="date" bind:value={scheduleToAdd.start_date} />
    </label>

    <label class="input input-bordered w-full max-w-xl">
      <span class="label">End Date</span>
      <input type="date" bind:value={scheduleToAdd.end_date} />
    </label>

    <label class="input input-bordered w-full max-w-xl">
      <span class="label">Start Time</span>
      <input type="time" bind:value={scheduleToAdd.start_time} />
    </label>

    <label class="input input-bordered w-full max-w-xl">
      <span class="label">End Time</span>
      <input type="time" bind:value={scheduleToAdd.end_time} />
    </label>

    <div class="join w-full max-w-xl">
      <label class="select select-bordered w-full">
        <span class="label">Select a playlist</span>

        <select bind:value={scheduleToAdd.playlist_id} class="join-item flex-grow">
          {#each playlists as p (p.id)}
            <option value={p.id}>{p.name}</option>
          {/each}
        </select>
      </label>

      <button class="btn join-item" onclick={loadPlaylists}>
        <PhArrowsClockwise />
      </button>
    </div>

    <label class="label">
      Monday
      <input type="checkbox" bind:checked={scheduleToAdd.monday} class="toggle" />
    </label>

    <label class="label">
      Tuesday
      <input type="checkbox" bind:checked={scheduleToAdd.tuesday} class="toggle" />
    </label>

    <label class="label">
      Wednesday
      <input type="checkbox" bind:checked={scheduleToAdd.wednesday} class="toggle" />
    </label>

    <label class="label">
      Thursday
      <input type="checkbox" bind:checked={scheduleToAdd.thursday} class="toggle" />
    </label>

    <label class="label">
      Friday
      <input type="checkbox" bind:checked={scheduleToAdd.friday} class="toggle" />
    </label>

    <label class="label">
      Saturday
      <input type="checkbox" bind:checked={scheduleToAdd.saturday} class="toggle" />
    </label>

    <label class="label">
      Sunday
      <input type="checkbox" bind:checked={scheduleToAdd.sunday} class="toggle" />
    </label>

    <div class="my-3 grid w-full max-w-xl grid-cols-2 gap-4">
      <button onclick={saveSchedule} class="btn btn-primary">
        <PhFloppyDiskDuotone /> Save Schedule
      </button>
      <button onclick={clearSchedule} class="btn btn-neutral">
        <PhBackspaceDuotone /> Clear Schedule
      </button>
    </div>
  </fieldset>

  <div class="divider"></div>

  <h2 class="text-xl">Edit Schedules</h2>

  <button class="btn join-item my-4" onclick={loadSchedules}>
    <PhArrowsClockwise /> Refresh Schedules
  </button>

  {#if schedules.length > 0}
    <table class="table">
      <thead class="hidden sm:table-header-group">
        <tr>
          <th>Name</th>
          <th>Enabled</th>
          <th>Playlist</th>
          <th>Start Date</th>
          <th>End Date</th>
          <th>Start Time</th>
          <th>End Time</th>
          <th>Days</th>
          <th></th>
        </tr>
      </thead>
      <tbody class="grid grid-cols-1 gap-2 sm:table-row-group">
        {#each schedules as row (row.name)}
          <tr class="hover card mb-4 flex flex-col border sm:table-row sm:border-none">
            <td class="flex flex-row sm:table-cell">
              <span class="flex-grow font-semibold sm:hidden">Name:</span>
              {row.name}
            </td>
            <td class="flex flex-row sm:table-cell">
              <span class="flex-grow font-semibold sm:hidden">Enabled:</span>
              {row.enabled}
            </td>
            <td class="flex flex-row sm:table-cell">
              <span class="flex-grow font-semibold sm:hidden">Playlist:</span>
              {getPlaylistName(row.playlist_id)}
            </td>
            <td class="flex flex-row sm:table-cell">
              <span class="flex-grow font-semibold sm:hidden">Start Date:</span>
              {row.start_date}
            </td>
            <td class="flex flex-row sm:table-cell">
              <span class="flex-grow font-semibold sm:hidden">End Date:</span>
              {row.end_date}
            </td>
            <td class="flex flex-row sm:table-cell">
              <span class="flex-grow font-semibold sm:hidden">Start Time:</span>
              {row.start_time}
            </td>
            <td class="flex flex-row sm:table-cell">
              <span class="flex-grow font-semibold sm:hidden">End Time:</span>
              {row.end_time}
            </td>
            <td class="flex flex-row sm:table-cell">
              <span class="flex-grow font-semibold sm:hidden">Days:</span>
              {row.monday ? "Mo " : ""}
              {row.tuesday ? "Tu " : ""}
              {row.wednesday ? "We " : ""}
              {row.thursday ? "Th " : ""}
              {row.friday ? "Fr " : ""}
              {row.saturday ? "Sa " : ""}
              {row.sunday ? "Su " : ""}
            </td>
            <td class="grid grid-cols-2 gap-2 sm:table-cell sm:w-36">
              <button
                onclick={() => {
                  editSchedule(row);
                }}
                class="btn btn-primary sm:mx-2 sm:h-8 sm:min-h-8 sm:w-8 sm:rounded-full sm:p-0 sm:text-sm">
                <PhNotePencilDuotone />
              </button>
              <Delete
                showText={false}
                callback={async () => {
                  await removeSchedule(row);
                }}
                class="sm:mx-2 sm:h-8 sm:min-h-8 sm:w-8 sm:rounded-full sm:p-0 sm:text-sm" />
            </td>
          </tr>
        {/each}
      </tbody>
    </table>
  {:else}
    <div><span class="italic">No schedules. Trying creating one</span></div>
  {/if}
</div>
