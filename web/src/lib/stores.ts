import type { SystemInfo } from "./client";
import type { Notification } from "./types";

import { writable } from "svelte/store";

export const sysInfo = writable<SystemInfo | undefined>(undefined);
export const notifications = writable<Notification>(undefined);

export const darkMode = writable<boolean | null>(
  JSON.parse(localStorage.getItem("darkMode") || "null"),
);
