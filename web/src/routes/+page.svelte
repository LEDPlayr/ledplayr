<script lang="ts">
  import { asset } from "$app/paths";
</script>

<svelte:head>
  <title>LEDPlayr: Home</title>
</svelte:head>

<div class="hero min-h-screen">
  <div class="hero-content text-center">
    <div class="max-w-md">
      <h1 class="text-2xl">Welcome to LEDPlayr</h1>
      <img class="m-auto my-4" src={asset("/logo.png")} alt="LEDplayr Logo" />
      <p>Use the menu on the left to navigate</p>
    </div>
  </div>
</div>
