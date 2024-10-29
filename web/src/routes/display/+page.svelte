<script lang="ts">
  import type { Color, Model, Sequence } from "$lib/client";

  import PhPause from "virtual:icons/ph/pause";
  import PhPlay from "virtual:icons/ph/play";
  import PhTestTube from "virtual:icons/ph/test-tube";

  import { Canvas } from "@threlte/core";
  import { onMount } from "svelte";
  import { getModels, runTest, startScheduler, stopScheduler } from "$lib/client";
  import TestModel from "$lib/components/test/TestModel.svelte";
  import VirtualDisplay from "$lib/components/VirtualDisplay.svelte";
  import { patterns, playerStatus } from "$lib/stores";
  import { notify, updateStatus } from "$lib/utils";

  const gray: Color = { r: 25, g: 25, b: 25 };
  let models: Record<string, [Model, Sequence | undefined]> = $state({});
  let step = $state(100);

  onMount(async () => {
    const { data, error } = await getModels();
    if (data) {
      models = Object.fromEntries(
        data
          .sort((a, b) => a.StartChannel - b.StartChannel)
          .map((m) => {
            return [m.Name, [m, undefined]];
          }),
      );
    }
    if (error) {
      notify(`${error.error}`, "error");
    }
  });

  let colors = $derived.by(() => {
    return Object.values(models)
      .map(([m, s]) => {
        const count = m.ChannelCount / 3;

        if (s) {
          if ("solid" in s) {
            return Array(count).fill(s.solid);
          } else if ("chase" in s) {
            let data = Array(count).fill(gray);
            let width = Math.min(s.chase.width, count);
            for (let i = 0; i < width; i++) {
              data[i] = s.chase.color;
            }
            return data;
          } else if ("pattern" in s) {
            const p = $patterns[`${s.pattern}@${count}`];
            if (p) {
              return p;
            }
          } else if ("moving_pattern" in s) {
            const p = $patterns[`${s.moving_pattern}@${count}`];
            if (p) {
              return p;
            }
          }
        }

        return Array(count).fill(gray);
      })
      .flat();
  });

  const startTest = async () => {
    const d = Object.fromEntries(
      Object.values(models)
        .map(([m, s]) => [m.Name, s])
        .filter(([_m, s]) => s !== undefined),
    );

    const { error } = await runTest({ body: { step_ms: step, tests: d } });
    if (error) {
      notify(`${error}`, "error");
    }
    await updateStatus();
  };

  const stop = async () => {
    const { error } = await stopScheduler();
    if (error) {
      notify(`${error}`, "error");
    }
    await updateStatus();
  };

  const start = async () => {
    const { error } = await startScheduler();
    if (error) {
      notify(`${error}`, "error");
    }
    await updateStatus();
  };
</script>

<svelte:head>
  <title>LEDPlayr: Test Display</title>
</svelte:head>

<div class="flex flex-grow flex-col p-5">
  <h1 class="text-2xl">Test Display</h1>

  <div class="divider"></div>

  <div class="grid grid-cols-1 gap-4 md:grid-cols-3">
    <div>
      <h2 class="text-lg">Test Control</h2>

      <p class="w-full text-center">Status: {$playerStatus}</p>

      <div class="m-4 flex flex-row gap-4">
        <button class="btn" onclick={stop} disabled={$playerStatus == "Stopped"}>
          <PhPause />Stop
        </button>
        <button
          class="btn"
          onclick={startTest}
          disabled={$playerStatus == "Started" || $playerStatus == "Testing"}>
          <PhTestTube />Start Test
        </button>
        <button
          class="btn"
          onclick={start}
          disabled={$playerStatus == "Started" || $playerStatus == "Testing"}>
          <PhPlay />Start Scheduler
        </button>
      </div>

      <h2 class="mb-4 text-lg">Configure Models</h2>

      <label class="label cursor-pointer">
        Step (ms)
        <input type="number" min="10" bind:value={step} />
      </label>

      <div class="p-4">
        <div class="mt-4 flex flex-col gap-2">
          {#each Object.keys(models) as m}
            <TestModel model={models[m][0]} bind:sequence={models[m][1]} />
          {/each}
        </div>
      </div>
    </div>

    <div class="mt-4 h-[32rem] rounded-xl border bg-base-200 md:col-span-2">
      <Canvas>
        <VirtualDisplay {colors} />
      </Canvas>
    </div>
  </div>
</div>
