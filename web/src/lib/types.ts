export type NotificationLevel = "success" | "info" | "warning" | "error";
export interface Notification {
  level: NotificationLevel;
  message: string;
  timeout: number;
}

export type PlayerStatus = "Unknown" | "Started" | "Stopped" | "Testing";
