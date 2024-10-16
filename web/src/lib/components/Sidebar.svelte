<script lang="ts">
  import { base } from "$app/paths";
  import { route } from "$lib/ROUTES";
  import Stats from "./Stats.svelte";
  import Theme from "./Theme.svelte";
</script>

<div class="flex max-h-full min-h-full flex-col">
  <div class="w-80 bg-base-200 p-4 text-center text-base-content">
    <a href="/">
      <img src="{base}/logo.png" alt="LEDplayr Logo" class="m-auto w-32 md:w-64" />
    </a>
  </div>
  <ul class="menu w-80 flex-grow flex-nowrap overflow-auto bg-base-200 p-4 text-base-content">
    <li>
      <h2 class="menu-title">Scheduler</h2>
      <ul>
        <li><a href={route("/sequences")}>Manage Sequences</a></li>
        <li><a href={route("/playlists")}>Manage Playlists</a></li>
        <li><a href={route("/schedules")}>Manage Schedules</a></li>
      </ul>
    </li>
    <li>
      <h2 class="menu-title">Display</h2>
      <ul>
        <li><a href={route("/outputs")}>Manage Outputs</a></li>
        <li><a href={route("/display")}>Virtual 3D Display</a></li>
      </ul>
    </li>
    <li>
      <h2 class="menu-title">Help</h2>
      <ul>
        <li><a href={route("/about")}>About</a></li>
      </ul>
    </li>
    <li>
      <h2 class="menu-title">Developer</h2>
      <ul>
        <li><a href={route("/logs")}>View Server Logs</a></li>
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
