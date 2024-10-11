import type { NotificationLevel } from "./types";

import { notifications } from "./stores";

export function notify(message: string, level: NotificationLevel = "info", timeout = 3000) {
  notifications.set({ level, message, timeout });
}
