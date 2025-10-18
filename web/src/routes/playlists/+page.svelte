<script lang="ts">
  import type { Playlist } from "$lib/client";

  import PhArrowFatDown from "~icons/ph/arrow-fat-down";
  import PhArrowFatUp from "~icons/ph/arrow-fat-up";
  import PhArrowsClockwise from "~icons/ph/arrows-clockwise";
  import PhFloppyDiskDuotone from "~icons/ph/floppy-disk-duotone";

  import { onMount } from "svelte";

  import {
    delPlaylist,
    getPlaylist,
    listPlaylists,
    listSequences,
    newPlaylist,
    updatePlaylist,
  } from "$lib/client";
  import Delete from "$lib/components/Delete.svelte";
  import { notify } from "$lib/utils";

  let playlistToAdd: Playlist = $state({
    name: "",
    desc: "",
    empty: true,
    leadIn: [],
    leadOut: [],
    loopCount: 1,
    mainPlaylist: [],
    random: false,
    repeat: true,
    version: 3,
  });
  let selectedPlaylistName: string | undefined = $state();
  let selectedPlaylist: Playlist | undefined = $state();
  let playlists: string[] = $state([]);
  let sequences: string[] = $state([]);
  let selectedSequenceName: string | undefined = $state();

  onMount(async () => {
    await loadPlaylists();
  });

  const loadPlaylists = async () => {
    const { data, error } = await listPlaylists();
    if (data) {
      playlists = data;
    } else {
      playlists = [];
    }
    if (error) {
      notify(`${error.error}`, "error");
    }
  };

  const loadSequences = async () => {
    const { data, error } = await listSequences();
    if (data) {
      sequences = data;
    } else {
      sequences = [];
    }
    if (error) {
      notify(`${error.error}`, "error");
    }
  };

  const addPlaylist = async () => {
    const { data, error } = await newPlaylist({
      body: playlistToAdd,
    });
    if (data) {
      playlistToAdd.name = "";
      playlistToAdd.desc = "";
      playlistToAdd.repeat = true;
      playlistToAdd.loopCount = 1;
      await loadPlaylists();
    }
    if (error) {
      notify(`${error.error}`, "error");
    }
  };

  const loadPlaylist = async (playlist: string) => {
    const { data, error } = await getPlaylist({ path: { playlist } });
    if (data) {
      selectedPlaylist = data;
      await loadSequences();
    }
    if (error) {
      notify(`${error.error}`, "error");
    }
  };

  const onPlaylistChange = async (
    event: Event & { currentTarget: EventTarget & HTMLSelectElement },
  ) => {
    const playlist = event.currentTarget.value;
    await loadPlaylist(playlist);
  };

  const deletePlaylist = async () => {
    if (!selectedPlaylistName) return;

    const { data, error } = await delPlaylist({ path: { playlist: selectedPlaylistName } });
    if (data) {
      selectedPlaylistName = "";
      selectedPlaylist = undefined;
      await loadPlaylists();
    }
    if (error) {
      notify(`${error.error}`, "error");
    }
  };

  const addSequenceToPlaylist = async () => {
    if (!selectedSequenceName || !selectedPlaylist) return;

    selectedPlaylist.mainPlaylist.push({
      sequenceName: selectedSequenceName,
      enabled: true,
      playOnce: true,
      type: "sequence",
    });

    await savePlaylist();
  };

  const moveSequenceUp = async (idx: number) => {
    if (!selectedPlaylist || idx - 1 < 0) return;
    const removed = selectedPlaylist.mainPlaylist.splice(idx, 1);
    selectedPlaylist.mainPlaylist.splice(idx - 1, 0, ...removed);
    await savePlaylist();
  };

  const moveSequenceDown = async (idx: number) => {
    if (!selectedPlaylist || idx >= selectedPlaylist.mainPlaylist.length) return;
    const removed = selectedPlaylist.mainPlaylist.splice(idx, 1);
    selectedPlaylist.mainPlaylist.splice(idx + 1, 0, ...removed);
    await savePlaylist();
  };

  const removeSequence = async (idx: number) => {
    if (!selectedPlaylist) return;
    selectedPlaylist.mainPlaylist.splice(idx, 1);
    await savePlaylist();
  };

  const savePlaylist = async () => {
    if (!selectedPlaylist || !selectedPlaylist.name) return;

    const { error } = await updatePlaylist({
      path: { playlist: selectedPlaylist.name },
      body: selectedPlaylist,
    });

    if (error) {
      notify(`${error.error}`, "error");
    }
    await loadPlaylist(selectedPlaylist.name);
  };
</script>

<svelte:head>
  <title>LEDPlayr: Playlists</title>
</svelte:head>

<div class="p-5">
  <h1 class="text-2xl">Manage Playlists</h1>

  <div class="divider"></div>

  <fieldset class="fieldset max-w-xl gap-2">
    <legend class="fieldset-legend text-xl">Create a New Playlist</legend>

    <label class="input input-bordered w-full max-w-xl">
      <span class="label">Name:</span>
      <input type="text" bind:value={playlistToAdd.name} placeholder="Name" />
    </label>

    <label class="input input-bordered w-full max-w-xl">
      <span class="label">Description:</span>
      <input type="text" bind:value={playlistToAdd.desc} placeholder="Description" />
    </label>

    <label class="label">
      Repeat forever?
      <input type="checkbox" bind:checked={playlistToAdd.repeat} class="toggle" />
    </label>

    <label class="input input-bordered w-full max-w-xl">
      <span class="label">Loop Count:</span>
      <input
        type="number"
        disabled={playlistToAdd.repeat}
        bind:value={playlistToAdd.loopCount}
        min={1} />
    </label>
  </fieldset>

  <button onclick={addPlaylist} class="btn btn-neutral my-3 w-full max-w-xl">
    Create Playlist
  </button>

  <div class="divider"></div>

  <h2 class="text-xl">Edit Playlists</h2>

  <div class="grid-cols-2 gap-4 lg:grid">
    <div class="join w-full max-w-xl">
      <label class="select select-bordered w-full">
        <span class="label">Playlist</span>

        <select
          onchange={onPlaylistChange}
          bind:value={selectedPlaylistName}
          class="join-item flex-grow">
          {#each playlists as p (p)}
            <option>{p}</option>
          {/each}
        </select>
      </label>

      <button class="btn join-item" onclick={loadPlaylists}>
        <PhArrowsClockwise />
      </button>
    </div>

    {#if selectedPlaylist}
      <div>
        <h3 class="mt-4 text-lg lg:mt-0">Playlist Info.</h3>

        <table class="table">
          <tbody>
            <tr>
              <th>Name</th>
              <td>
                <input
                  type="text"
                  class="input input-sm input-bordered w-full"
                  placeholder="Name"
                  bind:value={selectedPlaylist.name} />
              </td>
            </tr>
            <tr>
              <th>Description</th>
              <td>
                <input
                  type="text"
                  class="input input-sm input-bordered w-full"
                  placeholder="Description"
                  bind:value={selectedPlaylist.desc} />
              </td>
            </tr>
            <tr>
              <th>Repeat</th>
              <td>
                <select
                  bind:value={selectedPlaylist.repeat}
                  class="select select-bordered select-sm w-full">
                  <option value={true}>True</option>
                  <option value={false}>False</option>
                </select>
              </td>
            </tr>
            <tr>
              <th>Loop Count</th>
              <td>
                <input
                  type="number"
                  class="input input-sm input-bordered w-full"
                  min={1}
                  bind:value={selectedPlaylist.loopCount} />
              </td>
            </tr>
            <tr>
              <th>Total Sequences</th><td>{selectedPlaylist.playlistInfo?.total_items}</td>
            </tr>
            <tr>
              <th>Total Duration (s)</th>
              <td>{selectedPlaylist.playlistInfo?.total_duration}</td>
            </tr>
          </tbody>
        </table>

        <div class="grid grid-cols-2 gap-4">
          <Delete callback={deletePlaylist} />

          <button onclick={savePlaylist} class="btn btn-primary">
            <PhFloppyDiskDuotone />
            Save Playlist
          </button>
        </div>
      </div>

      <fieldset class="fieldset w-full max-w-xl">
        <legend class="fieldset-legend text-lg">Add a sequence</legend>

        <div class="join w-full max-w-xl">
          <label class="select select-bordered w-full">
            <span class="label">Sequence</span>

            <select bind:value={selectedSequenceName} class="join-item flex-grow">
              {#each sequences as s (s)}
                <option>{s}</option>
              {/each}
            </select>
          </label>

          <button class="btn join-item" onclick={loadSequences}>
            <PhArrowsClockwise />
          </button>
        </div>

        <button
          onclick={addSequenceToPlaylist}
          disabled={!selectedSequenceName}
          class="btn btn-primary mt-4">
          Add to playlist
        </button>
      </fieldset>

      <div>
        <h3 class="mt-4 text-lg lg:mt-0">List of seqeunces</h3>

        {#if selectedPlaylist.mainPlaylist.length > 0}
          <ul class="mt-4 flex flex-col gap-4">
            {#each selectedPlaylist.mainPlaylist as s, i (i)}
              <li class="border-base-300 rounded-lg border p-4">
                <div class="flex flex-col gap-2 lg:flex-row">
                  <span class="flex-grow leading-8 {s.enabled ? '' : 'text-error'}">
                    {s.sequenceName} ({s.duration}s)
                  </span>

                  <select
                    bind:value={s.enabled}
                    onchange={savePlaylist}
                    class="select select-bordered select-sm flex-grow"
                    aria-label="Determines whether this sequences is enabled">
                    <option value={true}>Enable</option>
                    <option value={false}>Disable</option>
                  </select>

                  <select
                    bind:value={s.playOnce}
                    onchange={savePlaylist}
                    class="select select-bordered select-sm flex-grow"
                    aria-label="Determines whether this sequences play once, or repeats">
                    <option value={true}>Play Once</option>
                    <option value={false}>Repeat</option>
                  </select>

                  <div class="flex flex-row gap-2">
                    <button
                      disabled={i == 0}
                      onclick={async () => {
                        await moveSequenceUp(i);
                      }}
                      class="btn btn-circle btn-ghost btn-sm"
                      aria-label="Move this sequence up in the playlist order">
                      <PhArrowFatUp />
                    </button>

                    <button
                      disabled={i == selectedPlaylist.mainPlaylist.length - 1}
                      onclick={async () => {
                        await moveSequenceDown(i);
                      }}
                      class="btn btn-circle btn-ghost btn-sm"
                      aria-label="Move this sequence down in the playlist order">
                      <PhArrowFatDown />
                    </button>

                    <Delete
                      showText={false}
                      callback={async () => {
                        await removeSequence(i);
                      }}
                      class="btn-circle btn-sm"
                      aria-label="Remove this sequence from the playlist" />
                  </div>
                </div>
              </li>
            {/each}
          </ul>
        {:else}
          <span class="italic">Playlist is currently empty</span>
        {/if}
      </div>
    {/if}
  </div>
</div>
