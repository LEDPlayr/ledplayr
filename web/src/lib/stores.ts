import type { Color, SystemInfo } from "./client";
import type { Notification, PlayerStatus } from "./types";

import { writable } from "svelte/store";

export const sysInfo = writable<SystemInfo | undefined>(undefined);
export const playerStatus = writable<PlayerStatus>("unknown");
export const notifications = writable<Notification>(undefined);

export const darkMode = writable<boolean | null>(
  JSON.parse(localStorage.getItem("darkMode") || "null"),
);

export const patterns = writable<Record<string, Color[]>>({});
