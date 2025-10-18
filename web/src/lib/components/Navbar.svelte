<script>
  import { resolve } from "$app/paths";
</script>

<div class="navbar bg-neutral text-neutral-content block lg:hidden">
  <div class="flex-none">
    <label for="drawer" class="btn btn-square btn-ghost lg:hidden">
      <svg
        xmlns="http://www.w3.org/2000/svg"
        fill="none"
        viewBox="0 0 24 24"
        class="inline-block h-5 w-5 stroke-current">
        <path
          stroke-linecap="round"
          stroke-linejoin="round"
          stroke-width="2"
          d="M4 6h16M4 12h16M4 18h16"></path>
      </svg>
    </label>
  </div>

  <div class="flex-1"></div>

  <div class="flex-none">
    <a class="btn btn-ghost text-xl" href={resolve("/")}>LEDPlayr</a>
  </div>
</div>
