// This file is auto-generated by @hey-api/openapi-ts

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

export type NumberedPlaylist = {
  id: number;
  name: string;
};

export type PlayerState = "Start" | "Stop";

export type Playlist = {
  desc: string;
  empty: boolean;
  leadIn: Array<PlaylistEntry>;
  leadOut: Array<PlaylistEntry>;
  loopCount: number;
  mainPlaylist: Array<PlaylistEntry>;
  name?: string | null;
  playlistInfo?: PlaylistInfo | null;
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
  status: PlayerState;
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

export type GetLogsResponse = Array<string>;

export type GetLogsError = Status;

export type GetModelsResponse = Array<Model>;

export type GetModelsError = Status;

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
