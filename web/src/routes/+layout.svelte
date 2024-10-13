<script lang="ts">
  import { onMount } from "svelte";
  import { afterNavigate } from "$app/navigation";
  import Navbar from "$lib/components/Navbar.svelte";
  import Notifications from "$lib/components/Notifications.svelte";
  import Sidebar from "$lib/components/Sidebar.svelte";
  import { darkMode } from "$lib/stores";

  import "../app.scss";

  let { children } = $props();
  let drawer = $state(false);
  let prefersDarkMode = $state(false);

  afterNavigate(() => {
    drawer = false;
  });

  onMount(() => {
    // Watch for change of user preferences
    if (window.matchMedia) {
      const wm = window.matchMedia("(prefers-color-scheme: dark)");
      prefersDarkMode = wm.matches;
      wm.onchange = (v) => {
        prefersDarkMode = v.matches;
      };
    }

    // Watch for other tabs changing the local storage
    const updateDarkMode = (ev: StorageEvent) => {
      if (ev.key == "darkMode") {
        $darkMode = JSON.parse(ev.newValue || "null");
      }
    };
    window.addEventListener("storage", updateDarkMode);

    return () => {
      window.removeEventListener("storage", updateDarkMode);
    };
  });

  $effect(() => {
    let theme: string;

    if ($darkMode === null) {
      theme = prefersDarkMode ? "dark" : "light";
    } else {
      theme = $darkMode ? "dark" : "light";
    }

    document.documentElement.setAttribute("data-theme", theme);
    localStorage.setItem("darkMode", JSON.stringify($darkMode));
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
