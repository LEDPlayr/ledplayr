<script lang="ts">
  import { asset } from "$app/paths";
  import { Pattern } from "$lib/client";

  type Props = {
    pattern: Pattern;
    onChange?: (p: Pattern) => void;
  };

  const uid = $props.id();
  let { pattern = $bindable(), onChange = undefined }: Props = $props();
  let popover: HTMLElement | undefined = $state();

  const possiblePatterns = Object.values(Pattern);

  const choosePattern = (p: Pattern) => {
    pattern = p;

    if (onChange) {
      onChange(p);
    }

    popover?.hidePopover();
  };
</script>

<button popovertarget="popover-{uid}" style="anchor-name:--anchor-{uid}">
  <div class="flex flex-col gap-2">
    <span>{pattern}</span>
    <img alt="Pattern for {pattern}" src={asset(`/gradients/${pattern}.png`)} />
  </div>
</button>

<ul
  class="menu dropdown rounded-box bg-base-100 max-h-96 w-64 flex-row overflow-y-auto p-2 shadow"
  popover
  bind:this={popover}
  id="popover-{uid}"
  style="position-anchor:--anchor-{uid}">
  {#each possiblePatterns as p (p)}
    <li class="flex-shrink">
      <button
        class:active={pattern == p}
        class="flex flex-col"
        onclick={() => {
          choosePattern(p);
        }}>
        {p}
        <img alt="Pattern for {p}" src={asset(`/gradients/${p}.png`)} />
      </button>
    </li>
  {/each}
</ul>
