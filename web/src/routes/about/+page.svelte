<script>
  import PhGithubLogoBold from "~icons/ph/github-logo-bold";

  import { asset } from "$app/paths";
  import { sysInfo } from "$lib/stores";
</script>

<svelte:head>
  <title>LEDPlayr: About</title>
</svelte:head>

<div class="hero min-h-screen">
  <div class="hero-content text-center">
    <div class="max-w-md">
      <img class="m-auto my-4" src={asset("/logo.png")} alt="LEDplayr Logo" />
      <h1 class="text-5xl font-bold">LEDPlayr</h1>
      <div class="py-6">
        <p>{$sysInfo?.LocalGitVersion ?? "Version Unknown"}</p>
        <p>{$sysInfo?.OSRelease ?? "OS Unknown"}</p>
        <p>{$sysInfo?.Kernel ?? "Kernel Unknown"}</p>
      </div>
      <a
        class="btn btn-primary"
        rel="external"
        target="_blank"
        href="https://github.com/LEDPlayr/ledplayr">
        <PhGithubLogoBold /> View on Github
      </a>
    </div>
  </div>
</div>
