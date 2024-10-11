/* Fix for svelte-5 until
 * https://github.com/unplugin/unplugin-icons/blob/main/types/svelte.d.ts
 * is updated
 */
declare global {
  declare module "virtual:icons/*" {
    import { Component } from "svelte";
    import type { SvelteHTMLElements } from "svelte/elements";

    export default Component<SvelteHTMLElements["svg"]>;
  }
}

export {};
