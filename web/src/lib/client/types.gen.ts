// This file is auto-generated by @hey-api/openapi-ts

export type BinaryFile = Blob | File;

export type Button = {
  battery: number;
  error: string;
  id: number;
  input: boolean;
  last: number;
  now: number;
  status: string;
};

export type ChannelOutput = {
  channelCount: number;
  enabled: boolean;
  startChannel: number;
  timeout: number;
  type: string;
  universes: Array<Universe>;
};

export type Channels = {
  channelOutputs: Array<ChannelOutput>;
};

export type Chase = {
  color: Color;
  width: number;
};

export type Color = {
  b: number;
  g: number;
  r: number;
};

export type DiskUtilization = {
  Media: FreeTotal;
  Root: FreeTotal;
};

export type FileUpload = {
  /**
   * File or files to upload
   */
  myfile: Blob | File;
};

export type FreeTotal = {
  Free: number;
  Total: number;
};

export type Mesh = {
  id: number;
  name: string;
  pos_x: number;
  pos_y: number;
  pos_z: number;
  rot_x: number;
  rot_y: number;
  rot_z: number;
  scale_x: number;
  scale_y: number;
  scale_z: number;
};

export type Model = {
  ChannelCount: number;
  ChannelCountPerNode: number;
  Name: string;
  Orientation: string;
  StartChannel: number;
  StartCorner: string;
  StrandsPerString: number;
  StringCount: number;
  Type: string;
  xLights: boolean;
};

export type Models = {
  models: Array<Model>;
};

export type NewButton = {
  battery: number;
  error: string;
  input: boolean;
  last: number;
  now: number;
  status: string;
};

export type NewMesh = {
  name: string;
  pos_x: number;
  pos_y: number;
  pos_z: number;
  rot_x: number;
  rot_y: number;
  rot_z: number;
  scale_x: number;
  scale_y: number;
  scale_z: number;
};

export type NewScene = {
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
  name: string;
};

export type NumberedPlaylist = {
  id: number;
  name: string;
};

export type Pattern =
  | "spectral"
  | "blues"
  | "greens"
  | "greys"
  | "oranges"
  | "purples"
  | "reds"
  | "turbo"
  | "viridis"
  | "inferno"
  | "magma"
  | "plasma"
  | "cividis"
  | "warm"
  | "cool"
  | "cube_helix"
  | "sinebow"
  | "rainbow";

export const Pattern = {
  SPECTRAL: "spectral",
  BLUES: "blues",
  GREENS: "greens",
  GREYS: "greys",
  ORANGES: "oranges",
  PURPLES: "purples",
  REDS: "reds",
  TURBO: "turbo",
  VIRIDIS: "viridis",
  INFERNO: "inferno",
  MAGMA: "magma",
  PLASMA: "plasma",
  CIVIDIS: "cividis",
  WARM: "warm",
  COOL: "cool",
  CUBE_HELIX: "cube_helix",
  SINEBOW: "sinebow",
  RAINBOW: "rainbow",
} as const;

export type PlayerState =
  | "start"
  | {
      testing: TestSpec;
    }
  | "stop";

export type PlayerStatus = "start" | "testing" | "stop";

export const PlayerStatus = {
  START: "start",
  TESTING: "testing",
  STOP: "stop",
} as const;

export type Playlist = {
  desc: string;
  empty: boolean;
  leadIn: Array<PlaylistEntry>;
  leadOut: Array<PlaylistEntry>;
  loopCount: number;
  mainPlaylist: Array<PlaylistEntry>;
  name?: string | null;
  playlistInfo?: null | PlaylistInfo;
  random: boolean;
  repeat: boolean;
  version: number;
};

export type PlaylistEntry = {
  duration?: number | null;
  enabled: boolean;
  playOnce: boolean;
  sequenceName: string;
  type: string;
};

export type PlaylistInfo = {
  total_duration: number;
  total_items: number;
};

export type Scene = {
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
  id: number;
  name: string;
};

export type Schedule = {
  enabled: boolean;
  end_date: string;
  end_time: string;
  friday: boolean;
  monday: boolean;
  name: string;
  playlist_id: number;
  saturday: boolean;
  start_date: string;
  start_time: string;
  sunday: boolean;
  thursday: boolean;
  tuesday: boolean;
  wednesday: boolean;
};

export type SchedulerStatus = {
  status: PlayerStatus;
};

export type Sequence =
  | {
      solid: Color;
    }
  | {
      chase: Chase;
    }
  | {
      pattern: Pattern;
    }
  | {
      moving_pattern: Pattern;
    }
  | {
      custom_pattern: Array<Color>;
    }
  | {
      custom_moving_pattern: Array<Color>;
    };

export type SequenceMeta = {
  /**
   * Number of channels
   */
  ChannelCount: number;
  /**
   * ID of the sequence (Likely the creation timestamp)
   */
  ID: string;
  /**
   * Name of the sequence
   */
  Name: string;
  /**
   * Number of framess
   */
  NumFrames: number;
  /**
   * Step time in milliseconds
   */
  StepTime: number;
  /**
   * Any additional variables
   */
  variableHeaders: {
    [key: string]: string;
  };
};

export type Status = {
  /**
   * What went wrong
   */
  error?: string | null;
  /**
   * Status
   */
  status: string;
};

export type SystemInfo = {
  /**
   * The CVS branch we're running on
   */
  Branch: string;
  /**
   * The description of this system
   */
  HostDescription: string;
  /**
   * Hostname of the system
   */
  HostName: string;
  /**
   * The IP addresses for this host
   */
  IPs: Array<string>;
  /**
   * The kernel version
   */
  Kernel: string;
  /**
   * The version for CVS
   */
  LocalGitVersion: string;
  /**
   * The logo of this system
   */
  Logo: string;
  /**
   * The mode of this system
   */
  Mode: string;
  /**
   * The OS Release
   */
  OSRelease: string;
  /**
   * The OS Version
   */
  OSVersion: string;
  /**
   * The platform we're running on
   */
  Platform: string;
  /**
   * The latest upstream CVS version
   */
  RemoteGitVersion: string;
  /**
   * Any additional contex to the platform
   */
  SubPlatform: string;
  /**
   * The location of updates
   */
  UpgradeSource: string;
  /**
   * The current system utilization
   */
  Utilization: SystemUtilization;
  /**
   * The platform we're running on
   */
  Variant: string;
  /**
   * The version of "FPPF" we're runnnig
   */
  Version: string;
  /**
   * The background color to use in the UI
   */
  backgroundColor: string;
  /**
   * Is multisync supported
   */
  multisync: boolean;
  /**
   * The type of system
   */
  typeId: number;
  /**
   * The persistent UUID for this system
   */
  uuid: string;
};

export type SystemUtilization = {
  CPU: number;
  Disk: DiskUtilization;
  Memory: number;
  Uptime: string;
};

export type TestSpec = {
  step_ms: number;
  tests: {
    [key: string]: Sequence;
  };
};

export type Universe = {
  active: boolean;
  address: string;
  channelCount: number;
  deDuplicate: boolean;
  description: string;
  id: number;
  monitor: boolean;
  priority: number;
  startChannel: number;
  type: number;
};

export type NewButtonData = {
  body: NewButton;
};

export type NewButtonResponse = Status;

export type NewButtonError = Status;

export type GetButtonData = {
  path: {
    /**
     * The ID of the button
     */
    button: number;
  };
};

export type GetButtonResponse = Button;

export type GetButtonError = Status;

export type UpdateButtonData = {
  body: NewButton;
  path: {
    /**
     * The ID of the button
     */
    button: number;
  };
};

export type UpdateButtonResponse = Status;

export type UpdateButtonError = Status;

export type DelButtonData = {
  path: {
    /**
     * The ID of the button
     */
    button: number;
  };
};

export type DelButtonResponse = Status;

export type DelButtonError = Status;

export type ListButtonsResponse = Array<Button>;

export type ListButtonsError = Status;

export type GetOutputsResponse = Channels;

export type GetOutputsError = Status;

export type UploadOutputsData = {
  body: Channels;
};

export type UploadOutputsResponse = Status;

export type UploadOutputsError = Status;

export type GetDisplayResponse = string;

export type GetDisplayError = Status;

export type UploadDisplayData = {
  body: Blob | File;
};

export type UploadDisplayResponse = Status;

export type UploadDisplayError = Status;

export type GetLogData = {
  path: {
    /**
     * The name of the log to display
     */
    name: string;
  };
};

export type GetLogResponse = string;

export type GetLogError = Status;

export type ListLogsResponse = Array<string>;

export type ListLogsError = Status;

export type NewMeshData = {
  body: NewMesh;
};

export type NewMeshResponse = Status;

export type NewMeshError = Status;

export type DownloadMeshData = {
  path: {
    /**
     * The name of the mesh
     */
    mesh: string;
  };
};

export type DownloadMeshResponse = Blob | File;

export type DownloadMeshError = Status;

export type UpdateMeshData = {
  body: NewMesh;
  path: {
    /**
     * The name of the mesh
     */
    mesh: string;
  };
};

export type UpdateMeshResponse = Status;

export type UpdateMeshError = Status;

export type DelMeshData = {
  path: {
    /**
     * The name of the mesh
     */
    mesh: string;
  };
};

export type DelMeshResponse = Status;

export type DelMeshError = Status;

export type ListMeshesResponse = Array<Mesh>;

export type ListMeshesError = Status;

export type ListModelsResponse = Array<Model>;

export type ListModelsError = Status;

export type UploadModelsData = {
  body: Models;
};

export type UploadModelsResponse = Status;

export type UploadModelsError = Status;

export type NewPlaylistData = {
  body: Playlist;
};

export type NewPlaylistResponse = Status;

export type NewPlaylistError = Status;

export type GetPlaylistData = {
  path: {
    /**
     * The name of the playlist
     */
    playlist: string;
  };
};

export type GetPlaylistResponse = Playlist;

export type GetPlaylistError = Status;

export type UpdatePlaylistData = {
  body: Playlist;
  path: {
    /**
     * The name of the playlist
     */
    playlist: string;
  };
};

export type UpdatePlaylistResponse = Status;

export type UpdatePlaylistError = Status;

export type DelPlaylistData = {
  path: {
    /**
     * The name of the playlist
     */
    playlist: string;
  };
};

export type DelPlaylistResponse = Status;

export type DelPlaylistError = Status;

export type ListPlaylistsResponse = Array<string>;

export type ListPlaylistsError = Status;

export type ListPlaylistsNumberedResponse = Array<NumberedPlaylist>;

export type ListPlaylistsNumberedError = Status;

export type NewSceneData = {
  body: NewScene;
};

export type NewSceneResponse = Status;

export type NewSceneError = Status;

export type DelSceneData = {
  path: {
    /**
     * The name of the scene
     */
    scene: string;
  };
};

export type DelSceneResponse = Status;

export type DelSceneError = Status;

export type ListScenesResponse = Array<Scene>;

export type ListScenesError = Status;

export type GetSceneData = {
  path: {
    /**
     * The name of the scene
     */
    scene: string;
  };
};

export type GetSceneResponse = Scene;

export type GetSceneError = Status;

export type UpdateSceneData = {
  body: NewScene;
  path: {
    /**
     * The name of the scene
     */
    scene: string;
  };
};

export type UpdateSceneResponse = Status;

export type UpdateSceneError = Status;

export type NewScheduleData = {
  body: Schedule;
};

export type NewScheduleResponse = Status;

export type NewScheduleError = Status;

export type GetScheduleData = {
  path: {
    /**
     * The name of the schedule
     */
    schedule: string;
  };
};

export type GetScheduleResponse = Schedule;

export type GetScheduleError = Status;

export type UpdateScheduleData = {
  body: Schedule;
  path: {
    /**
     * The name of the schedule
     */
    schedule: string;
  };
};

export type UpdateScheduleResponse = Status;

export type UpdateScheduleError = Status;

export type DelScheduleData = {
  path: {
    /**
     * The name of the schedule
     */
    schedule: string;
  };
};

export type DelScheduleResponse = Status;

export type DelScheduleError = Status;

export type GetSchedulerStatusResponse = SchedulerStatus;

export type GetSchedulerStatusError = Status;

export type StartSchedulerResponse = Status;

export type StartSchedulerError = Status;

export type StopSchedulerResponse = Status;

export type StopSchedulerError = Status;

export type ListSchedulesResponse = Array<Schedule>;

export type ListSchedulesError = Status;

export type GetSequenceData = {
  path: {
    /**
     * The sequence to download
     */
    filename: string;
  };
};

export type GetSequenceResponse = Blob | File;

export type GetSequenceError = Status;

export type DelSequenceData = {
  path: {
    /**
     * The sequence to download
     */
    filename: string;
  };
};

export type DelSequenceResponse = Status;

export type DelSequenceError = Status;

export type GetSequenceMetaData = {
  path: {
    /**
     * The sequence to download
     */
    filename: string;
  };
};

export type GetSequenceMetaResponse = SequenceMeta;

export type GetSequenceMetaError = Status;

export type ListSequencesResponse = Array<string>;

export type ListSequencesError = Status;

export type SystemInfoResponse = SystemInfo;

export type SystemInfoError = unknown;

export type RunTestData = {
  body: TestSpec;
};

export type RunTestResponse = Status;

export type RunTestError = Status;

export type GetTestSequenceData = {
  body: Sequence;
  query: {
    /**
     * Lenght of the LED chain
     */
    length: number;
  };
};

export type GetTestSequenceResponse = Array<Color>;

export type GetTestSequenceError = Status;

export type FileUploadData = {
  body: FileUpload;
};

export type FileUploadResponse = Status;

export type FileUploadError = Status;

export type FppCommandData = {
  query: {
    /**
     * The FFP command
     */
    command: string;
    /**
     * The file to move
     */
    file: string;
  };
};

export type FppCommandResponse = Status;

export type FppCommandError = Status;
