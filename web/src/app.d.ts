/* Fix for svelte-5 until
 * https://github.com/unplugin/unplugin-icons/blob/main/types/svelte.d.ts
 * is updated in
 * https://github.com/unplugin/unplugin-icons/pull/381
 */
declare module "virtual:icons/*" {
  import type { Component } from "svelte";
  import type { SvelteHTMLElements } from "svelte/elements";

  const component: Component<SvelteHTMLElements["svg"]>;

  export default component;
}

declare module "~icons/*" {
  import type { Component } from "svelte";
  import type { SvelteHTMLElements } from "svelte/elements";

  const component: Component<SvelteHTMLElements["svg"]>;

  export default component;
}

export {};
