<script lang="ts">
  import { base } from "$app/paths";
  import { Pattern } from "$lib/client";

  type Props = {
    pattern: Pattern;
    onChange?: (p: Pattern) => void;
  };

  let { pattern = $bindable(), onChange = undefined }: Props = $props();
  let open = $state(false);

  const possiblePatterns = Object.values(Pattern);

  const choosePattern = (p: Pattern) => {
    pattern = p;

    if (onChange) {
      onChange(p);
    }

    open = false;
  };
</script>

<details class="dropdown" bind:open>
  <summary class="btn m-1">
    <div class="flex flex-col gap-2">
      <span>{pattern}</span>
      <img alt="Pattern for {pattern}" src="{base}/gradients/{pattern}.png" />
    </div>
  </summary>

  <ul
    class="menu dropdown-content z-[5] max-h-96 w-64 flex-row overflow-y-auto rounded-box bg-base-100 p-2 shadow">
    {#each possiblePatterns as p}
      <li class="flex-shrink">
        <button
          class:active={pattern == p}
          class="flex flex-col"
          onclick={() => {
            choosePattern(p);
          }}>
          {p}
          <img alt="Pattern for {p}" src="{base}/gradients/{p}.png" />
        </button>
      </li>
    {/each}
  </ul>
</details>
