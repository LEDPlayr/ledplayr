export type NotificationLevel = "success" | "info" | "warning" | "error";
export interface Notification {
  level: NotificationLevel;
  message: string;
  timeout: number;
}

export type PlayerStatus = "Unknown" | "Started" | "Stopped" | "Testing";

export type CamPos = {
  cam_pos_x: number;
  cam_pos_y: number;
  cam_pos_z: number;
  cam_rot_x: number;
  cam_rot_y: number;
  cam_rot_z: number;
  cam_zoom: number;
  ctrl_x: number;
  ctrl_y: number;
  ctrl_z: number;
};
