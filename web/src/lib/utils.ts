import type { PlayerStatus as ClientPlayerStatus, Color } from "./client";
import type { NotificationLevel, PlayerStatus } from "./types";

import { getStatus } from "./client";
import { notifications, playerStatus } from "./stores";

export function notify(message: string, level: NotificationLevel = "info", timeout = 3000) {
  notifications.set({ level, message, timeout });
}

// https://stackoverflow.com/a/5624139/1965026
export function hexToRGBF(hex: string): Color {
  const result = /^#?([a-f\d]{2})([a-f\d]{2})([a-f\d]{2})$/i.exec(hex);
  return result
    ? {
        r: parseInt(result[1], 16) / 255,
        g: parseInt(result[2], 16) / 255,
        b: parseInt(result[3], 16) / 255,
      }
    : { r: 0, g: 0, b: 0 };
}

export function hexToRGB8(hex: string): Color {
  const result = /^#?([a-f\d]{2})([a-f\d]{2})([a-f\d]{2})$/i.exec(hex);
  return result
    ? {
        r: parseInt(result[1], 16),
        g: parseInt(result[2], 16),
        b: parseInt(result[3], 16),
      }
    : { r: 0, g: 0, b: 0 };
}

export function getPlayerStatus(status?: ClientPlayerStatus): PlayerStatus {
  return status || "unknown";
}

export function isPlaying(status: PlayerStatus): boolean | undefined {
  switch (status) {
    case "unknown":
      return undefined;
    case "scheduler":
    case "playlist":
    case "sequence":
    case "testing":
      return true;
  }
  return false;
}

export async function updateStatus() {
  try {
    playerStatus.set(getPlayerStatus((await getStatus()).data));
  } catch (_err) {
    playerStatus.set("unknown");
  }
}

export function rotate<T>(a: Array<T>, n: number): Array<T> {
  const count = -n % a.length;
  return [...a.slice(count, a.length), ...a.slice(0, count)];
}
