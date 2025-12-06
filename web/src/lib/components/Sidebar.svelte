<script lang="ts">
  import { asset, resolve } from "$app/paths";
  import Stats from "./Stats.svelte";
  import Theme from "./Theme.svelte";
</script>

<div class="flex max-h-full min-h-full flex-col">
  <div class="bg-base-200 text-base-content w-80 p-4 text-center">
    <a href={resolve("/")}>
      <img src={asset("/logo.png")} alt="LEDplayr Logo" class="m-auto w-32 md:w-64" />
    </a>
  </div>
  <ul class="menu bg-base-200 text-base-content w-80 flex-grow flex-nowrap overflow-auto p-4">
    <li>
      <h2 class="menu-title">Scheduler</h2>
      <ul>
        <li><a href={resolve("/sequences")}>Manage Sequences</a></li>
        <li><a href={resolve("/playlists")}>Manage Playlists</a></li>
        <li><a href={resolve("/schedules")}>Manage Schedules</a></li>
      </ul>
    </li>
    <li>
      <h2 class="menu-title">Display</h2>
      <ul>
        <li><a href={resolve("/outputs")}>Manage Outputs</a></li>
        <li><a href={resolve("/models")}>View Models</a></li>
        <li><a href={resolve("/display")}>Test Display</a></li>
      </ul>
    </li>
    <li>
      <h2 class="menu-title">Inputs</h2>
      <ul>
        <li><a href={resolve("/buttons")}>View Buttons</a></li>
      </ul>
    </li>
    <li>
      <h2 class="menu-title">Help</h2>
      <ul>
        <li><a href={resolve("/about")}>About</a></li>
      </ul>
    </li>
    <li>
      <h2 class="menu-title">Developer</h2>
      <ul>
        <li><a href={resolve("/logs")}>View Server Logs</a></li>
        <li><a href={resolve("/config")}>System Configuration</a></li>
        <!-- eslint-disable-next-line svelte/no-navigation-without-resolve -->
        <li><a rel="external" target="_blank" href="/rapidoc">OpenAPI</a></li>
      </ul>
    </li>
  </ul>

  <div class="bg-base-300 text-center">
    <Theme />
  </div>

  <footer class="footer bg-base-300 px-10 py-2 md:py-5">
    <Stats />
  </footer>
</div>
