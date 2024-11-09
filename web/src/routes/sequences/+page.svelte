<script lang="ts">
  import type { Component } from "svelte";
  import type { SequenceMeta } from "$lib/client";

  import PhArrowsClockwise from "virtual:icons/ph/arrows-clockwise";
  import PhCheck from "virtual:icons/ph/check";
  import PhClockCountdown from "virtual:icons/ph/clock-countdown";
  import PhUploadSimple from "virtual:icons/ph/upload-simple";
  import PhWarning from "virtual:icons/ph/warning";

  import prettyBytes from "pretty-bytes";
  import { onMount } from "svelte";

  import {
    delSequence,
    fileUpload,
    getSequenceMeta,
    listSequences,
  } from "$lib/client/services.gen";
  import Delete from "$lib/components/Delete.svelte";

  interface Upload {
    icon: Component;
    file: File;
    status: string;
  }

  let sequences: string[] = $state([]);
  let uploads: Upload[] = $state([]);
  let selectedSequenceName: string | undefined = $state();
  let selectedSequence: SequenceMeta | undefined = $state();

  onMount(async () => {
    await loadSequences();
  });

  const loadSequences = async () => {
    const { data } = await listSequences();
    if (data) {
      sequences = data;
    } else {
      sequences = [];
    }
  };

  const uploadsChanged = (event: Event & { currentTarget: EventTarget & HTMLInputElement }) => {
    const fileInput = event.currentTarget;

    if (!fileInput.files) return;

    uploads = [];
    for (const f of fileInput.files) {
      uploads.push({ icon: PhClockCountdown, file: f, status: "Awaiting upload..." });
    }
  };

  const uploadFiles = async (event: Event) => {
    event.preventDefault();

    for (let u of uploads) {
      u.icon = PhUploadSimple;
      u.status = "Uploading...";
      const { data, error } = await fileUpload({
        body: {
          myfile: u.file,
        },
      });
      if (data) {
        u.icon = PhCheck;
        u.status = "Successful";
      }
      if (error) {
        u.icon = PhWarning;
        u.status = error.error || "Unknown error";
      }
    }

    await loadSequences();
  };

  const selectSequence = async (
    el: Event & { currentTarget: EventTarget & HTMLSelectElement },
  ) => {
    const filename = el.currentTarget.value;
    const { data } = await getSequenceMeta({ path: { filename } });
    if (data) {
      selectedSequence = data;
    }
  };

  const deleteSequence = async () => {
    if (!selectedSequenceName) return;
    const { data } = await delSequence({ path: { filename: selectedSequenceName } });
    if (data) {
      selectedSequenceName = "";
      selectedSequence = undefined;
      await loadSequences();
    }
  };
</script>

<svelte:head>
  <title>LEDPlayr: Sequences</title>
</svelte:head>

<div class="p-5">
  <h1 class="text-2xl">Manage Sequences</h1>

  <div class="divider"></div>

  <h2 class="text-xl">Upload a New Sequence</h2>

  <form onsubmit={uploadFiles}>
    <div class="grid-cols-2 gap-4 lg:grid">
      <div>
        <label class="form-control w-full max-w-xl">
          <div class="label">
            <span class="label-text">Pick a file</span>
          </div>
          <input
            onchange={uploadsChanged}
            type="file"
            accept=".fseq"
            multiple
            class="file-input file-input-bordered w-full max-w-xl" />
        </label>
      </div>

      <div>
        {#if uploads.length > 0}
          <h3 class="mt-4 text-lg lg:mt-0">To Upload</h3>
          <ul class="menu">
            {#each uploads as upload}
              <li>
                <span>
                  <upload.icon />
                  {upload.file.name} - {prettyBytes(upload.file.size)} - {upload.status}
                </span>
              </li>
            {/each}
          </ul>

          <label class="form-control w-full max-w-xl py-4">
            <button type="submit" class="btn btn-ghost">
              Upload <PhUploadSimple />
            </button>
          </label>
        {/if}
      </div>
    </div>
  </form>

  <div class="divider"></div>

  <h2 class="text-xl">Edit Sequences</h2>

  <div class="grid-cols-2 gap-4 lg:grid">
    <label class="form-control w-full max-w-xl">
      <div class="label">
        <span class="label-text">Select a sequence to edit</span>
      </div>

      <div class="join w-full">
        <select
          onchange={selectSequence}
          bind:value={selectedSequenceName}
          class="join-item select select-bordered flex-grow">
          {#each sequences as seq}
            <option>{seq}</option>
          {/each}
        </select>

        <button class="btn join-item" onclick={loadSequences}>
          <PhArrowsClockwise />
        </button>
      </div>
    </label>
    {#if selectedSequence}
      <div>
        <h3 class="mt-4 text-lg lg:mt-0">Sequence Info.</h3>

        <table class="table">
          <tbody>
            <tr><th>Name</th><td>{selectedSequence.Name}</td></tr>
            <tr><th>Channel Count</th><td>{selectedSequence.ChannelCount}</td></tr>
            <tr><th>Number of Frames</th><td>{selectedSequence.NumFrames}</td></tr>
            <tr><th>Step Time (ms)</th><td>{selectedSequence.StepTime}</td></tr>
            {#each Object.entries(selectedSequence.variableHeaders) as [k, v]}
              <tr><th>Variable {k}</th><td>{v}</td></tr>
            {/each}
          </tbody>
        </table>

        <Delete callback={deleteSequence} />
      </div>
    {/if}
  </div>
</div>
