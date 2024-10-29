<script lang="ts">
  import { darkMode } from "$lib/stores";
</script>

<div class="border-b border-base-content">
  <div class="dropdown">
    <div role="button" tabindex="0" class="btn btn-ghost m-1">
      <div>Theme</div>
    </div>

    <ul class="menu dropdown-content z-[1] w-52 rounded-box bg-base-100 p-2 shadow">
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
  </div>
</div>
