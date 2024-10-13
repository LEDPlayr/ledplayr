<script lang="ts">
  import type { Model } from "$lib/client";

  import { Canvas } from "@threlte/core";
  import { onMount } from "svelte";
  import { getModels } from "$lib/client";
  import VirtualDisplay from "$lib/components/VirtualDisplay.svelte";
  import { notify } from "$lib/utils";

  type ModelView = Model & { visible: boolean };
  let models: Record<string, ModelView> = $state({});
  let pointColor = $state("#ff0000");

  onMount(async () => {
    const { data, error } = await getModels();
    if (data) {
      models = Object.fromEntries(
        data
          .sort((a, b) => a.StartChannel - b.StartChannel)
          .map((m) => {
            return [
              m.Name,
              {
                visible: true,
                ...m,
              },
            ];
          }),
      );
    }
    if (error) {
      notify(`${error.error}`, "error");
    }
  });

  const checkAll = () => {
    for (const m in models) {
      models[m].visible = true;
    }
  };

  const uncheckAll = () => {
    for (const m in models) {
      models[m].visible = false;
    }
  };

  const visible = $derived.by(() => {
    return Object.values(models)
      .filter((m) => m.visible)
      .map((m) => [m.StartChannel - 1, m.StartChannel + m.ChannelCount - 1]);
  });
</script>

<svelte:head>
  <title>LEDPlayr: Virtual Display</title>
</svelte:head>

<div class="flex flex-grow flex-col p-5">
  <h1 class="text-2xl">Virtual 3D Display</h1>

  <div class="divider"></div>

  <div class="grid grid-cols-1 gap-4 md:grid-cols-3">
    <div>
      <h2 class="mb-4 text-lg">Show/Hide models</h2>

      <label class="label cursor-pointer">
        Point Color
        <input type="color" bind:value={pointColor} />
      </label>

      <button onclick={checkAll} class="btn btn-ghost btn-sm">Select All</button>
      <button onclick={uncheckAll} class="btn btn-ghost btn-sm">Unselect All</button>

      <div class="max-h-[32rem] overflow-y-scroll p-4">
        <ul class="mt-4 flex flex-col gap-2">
          {#each Object.keys(models) as m}
            <li class="rounded-lg border border-base-300 p-2">
              <div class="form-control">
                <label class="label cursor-pointer">
                  <span class="label-text">{models[m].Name}</span>
                  <input type="checkbox" bind:checked={models[m].visible} class="checkbox" />
                </label>
              </div>
            </li>
          {/each}
        </ul>
      </div>
    </div>

    <div class="mt-4 min-h-[32rem] rounded-xl border bg-base-200 md:col-span-2">
      <Canvas>
        <VirtualDisplay {visible} color={pointColor} />
      </Canvas>
    </div>
  </div>
</div>
