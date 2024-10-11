<script lang="ts">
  import PhQuestionMark from "virtual:icons/ph/question-mark";
  import PhTrash from "virtual:icons/ph/trash";

  interface Props {
    callback?: () => Promise<void>;
    showText?: boolean;
    class?: string;
    "aria-label"?: string;
  }
  const {
    callback = async () => {},
    showText = true,
    class: cls = "",
    "aria-label": aria_label = "Delete item",
  }: Props = $props();

  let click = $state(0);
  let text = $derived.by(() => {
    if (click == 1) {
      return "Are you sure?";
    }
    return "Delete?";
  });
  let disabled = $state(false);

  const onclick = async () => {
    click += 1;

    if (click > 1) {
      disabled = true;
      await callback();
    }
  };

  const reset = () => {
    click = 0;
    disabled = false;
  };
</script>

<button
  class="btn btn-error {cls}"
  aria-label={aria_label}
  {disabled}
  {onclick}
  onfocusout={reset}
  onmouseout={reset}
  onblur={reset}>
  {#if click == 1}
    <PhQuestionMark />
  {:else}
    <PhTrash />
  {/if}
  {#if showText}
    {text}
  {/if}
</button>
