<script lang="ts">
  import { darkMode } from "$lib/stores";
</script>

<div class="border-b border-base-content">
  <ul class="menu menu-horizontal px-1">
    <li>
      <details>
        <summary>Theme</summary>
        <ul class="rounded-t-none bg-base-100 p-2">
          <li>
            <input
              type="radio"
              name="theme-dropdown"
              bind:group={$darkMode}
              class="btn btn-ghost btn-sm btn-block justify-start"
              aria-label="System"
              value={null} />
          </li>
          <li>
            <input
              type="radio"
              name="theme-dropdown"
              bind:group={$darkMode}
              class="btn btn-ghost btn-sm btn-block justify-start"
              aria-label="Light"
              value={false} />
          </li>
          <li>
            <input
              type="radio"
              name="theme-dropdown"
              bind:group={$darkMode}
              class="btn btn-ghost btn-sm btn-block justify-start"
              aria-label="Dark"
              value={true} />
          </li>
        </ul>
      </details>
    </li>
  </ul>
</div>
