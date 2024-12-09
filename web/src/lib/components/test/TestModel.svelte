<script lang="ts">
  import type { Model, Pattern, Sequence } from "$lib/client";

  import { tick } from "svelte";

  import { getTestPattern } from "$lib/client";
  import { patterns } from "$lib/stores";
  import { hexToRGB8 } from "$lib/utils";
  import PatternPicker from "./PatternPicker.svelte";

  type Props = {
    readonly model: Model;
    sequence?: Sequence;
  };

  let { model, sequence = $bindable() }: Props = $props();

  let sequence_type: undefined | "solid" | "chase" | "pattern" | "moving_pattern" = $state();
  let pattern: Pattern = $state("spectral");
  let color = $state("#ff0000");
  let width = $state(1);

  let summary = $derived.by(() => {
    switch (sequence_type) {
      case undefined:
        return "disabled";
      case "solid":
        return `solid: ${color}`;
      case "chase":
        return `chase: ${color}@${width}`;
      case "pattern":
        return `pattern: ${pattern}`;
      case "moving_pattern":
        return `moving_pattern: ${pattern}`;
    }
  });

  const loadPattern = async () => {
    if (!sequence) return;
    await tick();

    if ("pattern" in sequence || "moving_pattern" in sequence) {
      const count = model.ChannelCount / 3;

      const { data } = await getTestPattern({
        body: sequence,
        query: { length: count },
      });

      if (data) {
        const k = `${pattern}@${count}`;
        $patterns[k] = data;
      }
    }
  };

  $effect(() => {
    switch (sequence_type) {
      case undefined:
        sequence = undefined;
        break;
      case "solid":
        sequence = { solid: hexToRGB8(color) };
        break;
      case "chase":
        sequence = { chase: { color: hexToRGB8(color), width } };
        break;
      case "pattern":
        sequence = { pattern };
        break;
      case "moving_pattern":
        sequence = { moving_pattern: pattern };
        break;
    }
  });
</script>

<div class="collapse collapse-arrow overflow-visible bg-base-200">
  <input type="checkbox" />

  <div class="collapse-title text-xl font-medium">
    {model.Name} [ {summary} ]
  </div>

  <div class="collapse-content">
    <div class="flex flex-col">
      <label class="label">
        <span class="label-text">Test Mode</span>
        <select
          class="select-outline select"
          bind:value={sequence_type}
          onchange={() => {
            loadPattern();
          }}>
          <option value={undefined}>Disabled</option>
          <option value="solid">Solid</option>
          <option value="chase">Chase</option>
          <option value="pattern">Pattern</option>
          <option value="moving_pattern">Moving Pattern</option>
        </select>
      </label>

      {#if sequence_type == "solid" || sequence_type == "chase"}
        <label class="label cursor-pointer">
          Color
          <input type="color" bind:value={color} />
        </label>
      {/if}

      {#if sequence_type == "chase"}
        <label class="label cursor-pointer">
          Width
          <input type="number" min="1" bind:value={width} />
        </label>
      {/if}

      {#if sequence_type == "pattern" || sequence_type == "moving_pattern"}
        <label class="label">
          <span class="label-text">Pattern</span>
          <PatternPicker bind:pattern onChange={loadPattern} />
        </label>
      {/if}
    </div>
  </div>
</div>
