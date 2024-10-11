<script lang="ts">
  import { afterNavigate } from "$app/navigation";
  import Navbar from "$lib/components/Navbar.svelte";
  import Notifications from "$lib/components/Notifications.svelte";
  import Sidebar from "$lib/components/Sidebar.svelte";

  import "../app.scss";

  let { children } = $props();
  let drawer = $state(false);

  afterNavigate(() => {
    drawer = false;
  });
</script>

<div class="drawer lg:drawer-open">
  <input id="drawer" type="checkbox" bind:checked={drawer} class="drawer-toggle" />

  <div class="drawer-content flex flex-col">
    <Navbar />

    {@render children()}
  </div>

  <div class="drawer-side">
    <label for="drawer" aria-label="close sidebar" class="drawer-overlay"></label>

    <Sidebar />
  </div>
</div>

<Notifications />
