<script lang="ts">
  import type { Color, Mesh, Model, Scene, Sequence } from "$lib/client";

  import PhArrowsIn from "~icons/ph/arrows-in";
  import PhArrowsOut from "~icons/ph/arrows-out";
  import PhFloppyDiskDuotone from "~icons/ph/floppy-disk-duotone";
  import PhFolderOpenDuotone from "~icons/ph/folder-open-duotone";
  import PhPlayDuotone from "~icons/ph/play-duotone";
  import PhStopDutone from "~icons/ph/stop-duotone";
  import PhTestTubeDuotone from "~icons/ph/test-tube-duotone";

  import { Canvas } from "@threlte/core";
  import { AnimationFrames } from "runed";
  import { onMount } from "svelte";
  import * as THREE from "three";

  import {
    listMeshes,
    listModels,
    listScenes,
    runTest,
    startScheduler,
    stop,
    updateScene,
  } from "$lib/client";
  import TestModel from "$lib/components/test/TestModel.svelte";
  import VirtualDisplay from "$lib/components/VirtualDisplay.svelte";
  import { patterns, playerStatus } from "$lib/stores";
  import { isPlaying, notify, rotate, updateStatus } from "$lib/utils";

  const gray: Color = { r: 25, g: 25, b: 25 };

  let models: Record<string, [Model, Sequence | undefined]> = $state({});
  let meshes: Mesh[] = $state([]);

  let scenes: Scene[] = $state([]);
  let selectedScene: Scene | undefined = $state();
  let sceneName = $state("");
  let scene: ReturnType<typeof VirtualDisplay>;
  let light = $state(5);

  let step = $state(40);
  let offset = $state(0);
  let fps = $derived(1000 / step);
  let preview = $state(false);

  let isFs = $state(false);
  let fsWrapper: HTMLDivElement | undefined;

  const animation = new AnimationFrames(
    () => {
      offset++;
    },
    { fpsLimit: () => fps, immediate: false },
  );

  $effect(() => {
    if (preview) {
      animation.start();
    } else {
      animation.stop();
    }
  });

  onMount(async () => {
    registerFullscreen();
    await loadModels();
    await loadScenes();
    await loadMeshes();
  });

  const loadModels = async () => {
    const { data, error } = await listModels();
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
  };

  const loadScenes = async () => {
    const { data, error } = await listScenes();
    if (data) {
      scenes = data;
    }
    if (error) {
      notify(`${error.error}`, "error");
    }
  };

  const loadMeshes = async () => {
    const { data, error } = await listMeshes();
    if (data) {
      meshes = data;
    }
    if (error) {
      notify(`${error.error}`, "error");
    }
  };

  const registerFullscreen = () => {
    const fullscreenChange = () => {
      isFs = !!document.fullscreenElement;
    };

    addEventListener("fullscreenchange", fullscreenChange);
    return () => {
      removeEventListener("fullscreenchange", fullscreenChange);
    };
  };

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
            data = rotate(data, offset);
            return data;
          } else if ("pattern" in s) {
            const p = $patterns[`${s.pattern}@${count}`];
            if (p) {
              return p;
            }
          } else if ("moving_pattern" in s) {
            const p = $patterns[`${s.moving_pattern}@${count}`];
            if (p) {
              return rotate(p, offset);
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

  const stopPlayer = async () => {
    const { error } = await stop();
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

  const saveCamera = async () => {
    if (scene) {
      const cam = scene.getCamera();
      if (cam) {
        const { error } = await updateScene({
          path: { scene: sceneName },
          body: { name: sceneName, ...cam },
        });
        if (error) {
          notify(`${error.error}`, "error");
        } else {
          await loadScenes();
        }
      }
    }
  };

  const restoreCamera = () => {
    if (scene && selectedScene) {
      scene.restoreCamera(selectedScene);
    }
  };

  const toggleFullscreen = async () => {
    if (isFs) {
      document.exitFullscreen();
    } else {
      if (fsWrapper) {
        try {
          await fsWrapper.requestFullscreen();
        } catch (err) {
          notify(`Error attempting to enable fullscreen mode: ${err}`, "error");
        }
      }
    }
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

      <p class="w-full text-center">Status: <span class="capitalize">{$playerStatus}</span></p>

      <div class="m-4 flex flex-row flex-wrap place-content-center gap-4">
        <button class="btn" onclick={stopPlayer} disabled={isPlaying($playerStatus) === false}>
          <PhStopDutone />Stop
        </button>
        <button class="btn" onclick={startTest}>
          <PhTestTubeDuotone />Start Test
        </button>
        <button class="btn" onclick={start}>
          <PhPlayDuotone />Start Scheduler
        </button>
      </div>

      <label class="label">
        Preview Animation?
        <input type="checkbox" bind:checked={preview} class="toggle" />
      </label>

      <h2 class="mb-4 text-lg">Configure Models</h2>

      <label class="label grid cursor-pointer grid-cols-5">
        Step {step}ms
        <input
          type="range"
          min="10"
          max="100"
          bind:value={step}
          step="5"
          class="range col-span-4" />
      </label>

      <div class="p-4">
        <div class="mt-4 flex flex-col gap-2">
          {#each Object.keys(models) as m (m)}
            <TestModel model={models[m][0]} bind:sequence={models[m][1]} />
          {/each}
        </div>
      </div>
    </div>

    <div class="mt-4 flex flex-col gap-4 md:col-span-2">
      <div
        bind:this={fsWrapper}
        class="bg-base-200 h-[32rem]"
        class:rounded-xl={!isFs}
        class:border={!isFs}>
        <Canvas toneMapping={THREE.NoToneMapping}>
          <VirtualDisplay {colors} {light} {meshes} bind:this={scene} />
        </Canvas>

        <button
          class="btn btn-ghost btn-circle relative bottom-11 left-1"
          onclick={toggleFullscreen}>
          {#if isFs}
            <PhArrowsIn />
          {:else}
            <PhArrowsOut />
          {/if}
        </button>
      </div>

      <label class="label grid cursor-pointer grid-cols-5">
        Ambient Light ({light * 10}%)
        <input
          type="range"
          min="0"
          max="10"
          bind:value={light}
          step="0.5"
          class="range col-span-4" />
      </label>

      <div class="grid-cols-2 gap-2 lg:grid">
        <div class="grid gap-2">
          <label class="input input-bordered w-full max-w-xl">
            <span class="label">Name of scene to save</span>

            <input type="text" bind:value={sceneName} />
          </label>

          <button class="btn" disabled={sceneName == ""} onclick={saveCamera}>
            <PhFloppyDiskDuotone />
            Save
          </button>
        </div>

        <div class="grid gap-2">
          <label class="select select-bordered w-full max-w-xl">
            <span class="label">Select a scene</span>

            <select bind:value={selectedScene}>
              {#each scenes as s (s.name)}
                <option value={s}>{s.name}</option>
              {/each}
            </select>
          </label>

          <button class="btn" disabled={!selectedScene} onclick={restoreCamera}>
            <PhFolderOpenDuotone />
            Restore
          </button>
        </div>
      </div>
    </div>
  </div>
</div>
