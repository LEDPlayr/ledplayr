// This file is auto-generated by @hey-api/openapi-ts

export const ChannelOutputSchema = {
  type: "object",
  required: ["type", "startChannel", "enabled", "timeout", "channelCount", "universes"],
  properties: {
    channelCount: {
      type: "integer",
      format: "int32",
    },
    enabled: {
      type: "boolean",
    },
    startChannel: {
      type: "integer",
      format: "int32",
      minimum: 0,
    },
    timeout: {
      type: "integer",
      format: "int32",
      minimum: 0,
    },
    type: {
      type: "string",
    },
    universes: {
      type: "array",
      items: {
        $ref: "#/components/schemas/Universe",
      },
    },
  },
} as const;

export const ChannelsSchema = {
  type: "object",
  required: ["channelOutputs"],
  properties: {
    channelOutputs: {
      type: "array",
      items: {
        $ref: "#/components/schemas/ChannelOutput",
      },
    },
  },
} as const;

export const DiskUtilizationSchema = {
  type: "object",
  required: ["Media", "Root"],
  properties: {
    Media: {
      $ref: "#/components/schemas/FreeTotal",
    },
    Root: {
      $ref: "#/components/schemas/FreeTotal",
    },
  },
} as const;

export const FileUploadSchema = {
  type: "object",
  required: ["myfile"],
  properties: {
    myfile: {
      type: "string",
      format: "binary",
      description: "File or files to upload",
    },
  },
} as const;

export const FreeTotalSchema = {
  type: "object",
  required: ["Free", "Total"],
  properties: {
    Free: {
      type: "integer",
      format: "int64",
      minimum: 0,
    },
    Total: {
      type: "integer",
      format: "int64",
      minimum: 0,
    },
  },
} as const;

export const ModelSchema = {
  type: "object",
  required: [
    "Name",
    "xLights",
    "ChannelCount",
    "Orientation",
    "StartChannel",
    "StringCount",
    "ChannelCountPerNode",
    "StrandsPerString",
    "StartCorner",
    "Type",
  ],
  properties: {
    ChannelCount: {
      type: "integer",
      format: "int32",
      example: 6,
      minimum: 0,
    },
    ChannelCountPerNode: {
      type: "integer",
      format: "int32",
      example: 3,
      minimum: 0,
    },
    Name: {
      type: "string",
      example: "Single_line",
    },
    Orientation: {
      type: "string",
      example: "horizontal",
    },
    StartChannel: {
      type: "integer",
      format: "int32",
      example: 1,
      minimum: 0,
    },
    StartCorner: {
      type: "string",
      example: "BL",
    },
    StrandsPerString: {
      type: "integer",
      format: "int32",
      example: 1,
      minimum: 0,
    },
    StringCount: {
      type: "integer",
      format: "int32",
      example: 1,
      minimum: 0,
    },
    Type: {
      type: "string",
      example: "Channel",
    },
    xLights: {
      type: "boolean",
      example: true,
    },
  },
} as const;

export const ModelsSchema = {
  type: "object",
  required: ["models"],
  properties: {
    models: {
      type: "array",
      items: {
        $ref: "#/components/schemas/Model",
      },
    },
  },
} as const;

export const NumberedPlaylistSchema = {
  type: "object",
  required: ["name", "id"],
  properties: {
    id: {
      type: "integer",
      format: "int32",
      example: 1,
    },
    name: {
      type: "string",
      example: "name",
    },
  },
} as const;

export const PlayerStateSchema = {
  type: "string",
  enum: ["Start", "Stop"],
} as const;

export const PlaylistSchema = {
  type: "object",
  required: [
    "version",
    "repeat",
    "loopCount",
    "empty",
    "desc",
    "random",
    "leadIn",
    "mainPlaylist",
    "leadOut",
  ],
  properties: {
    desc: {
      type: "string",
      example: "",
    },
    empty: {
      type: "boolean",
      example: true,
    },
    leadIn: {
      type: "array",
      items: {
        $ref: "#/components/schemas/PlaylistEntry",
      },
    },
    leadOut: {
      type: "array",
      items: {
        $ref: "#/components/schemas/PlaylistEntry",
      },
    },
    loopCount: {
      type: "integer",
      format: "int32",
      example: 0,
    },
    mainPlaylist: {
      type: "array",
      items: {
        $ref: "#/components/schemas/PlaylistEntry",
      },
    },
    name: {
      type: "string",
      example: "sample",
      nullable: true,
    },
    playlistInfo: {
      allOf: [
        {
          $ref: "#/components/schemas/PlaylistInfo",
        },
      ],
      nullable: true,
    },
    random: {
      type: "boolean",
      example: false,
    },
    repeat: {
      type: "boolean",
      example: false,
    },
    version: {
      type: "integer",
      format: "int32",
      example: 3,
      minimum: 0,
    },
  },
} as const;

export const PlaylistEntrySchema = {
  type: "object",
  required: ["sequenceName", "playOnce", "enabled", "type"],
  properties: {
    duration: {
      type: "number",
      format: "float",
      example: 30,
      nullable: true,
    },
    enabled: {
      type: "boolean",
      example: true,
    },
    playOnce: {
      type: "boolean",
      example: false,
    },
    sequenceName: {
      type: "string",
      example: "sequence.fseq",
    },
    type: {
      type: "string",
      example: "sequence",
    },
  },
} as const;

export const PlaylistInfoSchema = {
  type: "object",
  required: ["total_duration", "total_items"],
  properties: {
    total_duration: {
      type: "number",
      format: "float",
      example: 0,
    },
    total_items: {
      type: "integer",
      format: "int32",
      example: 0,
    },
  },
} as const;

export const ScheduleSchema = {
  type: "object",
  required: [
    "name",
    "playlist_id",
    "enabled",
    "start_date",
    "end_date",
    "start_time",
    "end_time",
    "monday",
    "tuesday",
    "wednesday",
    "thursday",
    "friday",
    "saturday",
    "sunday",
  ],
  properties: {
    enabled: {
      type: "boolean",
      example: true,
    },
    end_date: {
      type: "string",
      format: "date",
      example: "1970-01-01",
    },
    end_time: {
      type: "string",
      format: "time",
      example: "00:00",
    },
    friday: {
      type: "boolean",
      example: true,
    },
    monday: {
      type: "boolean",
      example: true,
    },
    name: {
      type: "string",
      example: "Schedule",
    },
    playlist_id: {
      type: "integer",
      format: "int32",
      example: 1,
    },
    saturday: {
      type: "boolean",
      example: true,
    },
    start_date: {
      type: "string",
      format: "date",
      example: "1920-01-01",
    },
    start_time: {
      type: "string",
      format: "time",
      example: "00:00",
    },
    sunday: {
      type: "boolean",
      example: true,
    },
    thursday: {
      type: "boolean",
      example: true,
    },
    tuesday: {
      type: "boolean",
      example: true,
    },
    wednesday: {
      type: "boolean",
      example: true,
    },
  },
} as const;

export const SchedulerStatusSchema = {
  type: "object",
  required: ["status"],
  properties: {
    status: {
      $ref: "#/components/schemas/PlayerState",
    },
  },
} as const;

export const SequenceMetaSchema = {
  type: "object",
  required: ["Name", "ID", "StepTime", "NumFrames", "ChannelCount", "variableHeaders"],
  properties: {
    ChannelCount: {
      type: "integer",
      format: "int32",
      description: "Number of channels",
      example: 10,
      minimum: 0,
    },
    ID: {
      type: "string",
      description: "ID of the sequence (Likely the creation timestamp)",
      example: "12345",
    },
    Name: {
      type: "string",
      description: "Name of the sequence",
      example: "sequence.fseq",
    },
    NumFrames: {
      type: "integer",
      format: "int32",
      description: "Number of framess",
      example: 100,
      minimum: 0,
    },
    StepTime: {
      type: "integer",
      format: "int32",
      description: "Step time in milliseconds",
      example: 50,
      minimum: 0,
    },
    variableHeaders: {
      type: "object",
      description: "Any additional variables",
      additionalProperties: {
        type: "string",
      },
      example: {
        sp: "xLights",
      },
    },
  },
} as const;

export const StatusSchema = {
  type: "object",
  required: ["status"],
  properties: {
    error: {
      type: "string",
      description: "What went wrong",
      example: "Could not open file",
      nullable: true,
    },
    status: {
      type: "string",
      description: "Status",
      example: "error",
    },
  },
} as const;

export const SystemInfoSchema = {
  type: "object",
  required: [
    "HostName",
    "HostDescription",
    "Platform",
    "Variant",
    "SubPlatform",
    "backgroundColor",
    "Mode",
    "Logo",
    "Version",
    "Branch",
    "multisync",
    "OSVersion",
    "OSRelease",
    "uuid",
    "Utilization",
    "Kernel",
    "LocalGitVersion",
    "RemoteGitVersion",
    "UpgradeSource",
    "IPs",
    "typeId",
  ],
  properties: {
    Branch: {
      type: "string",
      description: "The CVS branch we're running on",
      example: "main",
    },
    HostDescription: {
      type: "string",
      description: "The description of this system",
      example: "Rust based FPP alternative",
    },
    HostName: {
      type: "string",
      description: "Hostname of the system",
      example: "localhost",
    },
    IPs: {
      type: "array",
      items: {
        type: "string",
      },
      description: "The IP addresses for this host",
      example: ["127.0.0.1"],
    },
    Kernel: {
      type: "string",
      description: "The kernel version",
      example: "6.4.4",
    },
    LocalGitVersion: {
      type: "string",
      description: "The version for CVS",
      example: "1.0.0",
    },
    Logo: {
      type: "string",
      description: "The logo of this system",
      example: "debian.png",
    },
    Mode: {
      type: "string",
      description: "The mode of this system",
      example: "player",
    },
    OSRelease: {
      type: "string",
      description: "The OS Release",
      example: "",
    },
    OSVersion: {
      type: "string",
      description: "The OS Version",
      example: "Stretch",
    },
    Platform: {
      type: "string",
      description: "The platform we're running on",
      example: "Linux",
    },
    RemoteGitVersion: {
      type: "string",
      description: "The latest upstream CVS version",
      example: "1.0.0",
    },
    SubPlatform: {
      type: "string",
      description: "Any additional contex to the platform",
      example: "",
    },
    UpgradeSource: {
      type: "string",
      description: "The location of updates",
      example: "git",
    },
    Utilization: {
      $ref: "#/components/schemas/SystemUtilization",
    },
    Variant: {
      type: "string",
      description: "The platform we're running on",
      example: "Debian",
    },
    Version: {
      type: "string",
      description: `The version of "FPPF" we're runnnig`,
      example: "6.0",
    },
    backgroundColor: {
      type: "string",
      description: "The background color to use in the UI",
      example: "#c01015",
    },
    multisync: {
      type: "boolean",
      description: "Is multisync supported",
      example: false,
    },
    typeId: {
      type: "integer",
      format: "int32",
      description: "The type of system",
      example: 1,
      minimum: 0,
    },
    uuid: {
      type: "string",
      description: "The persistent UUID for this system",
      example: "82ae0c57-9a54-4911-9dc2-a1d2e512da7b",
    },
  },
} as const;

export const SystemUtilizationSchema = {
  type: "object",
  required: ["CPU", "Memory", "Uptime", "Disk"],
  properties: {
    CPU: {
      type: "number",
      format: "float",
    },
    Disk: {
      $ref: "#/components/schemas/DiskUtilization",
    },
    Memory: {
      type: "number",
      format: "float",
    },
    Uptime: {
      type: "string",
    },
  },
} as const;

export const UniverseSchema = {
  type: "object",
  required: [
    "description",
    "active",
    "address",
    "startChannel",
    "channelCount",
    "id",
    "deDuplicate",
    "priority",
    "monitor",
    "type",
  ],
  properties: {
    active: {
      type: "boolean",
    },
    address: {
      type: "string",
      format: "ipv4",
    },
    channelCount: {
      type: "integer",
      format: "int32",
      minimum: 0,
    },
    deDuplicate: {
      type: "boolean",
    },
    description: {
      type: "string",
    },
    id: {
      type: "integer",
      format: "int32",
      minimum: 0,
    },
    monitor: {
      type: "boolean",
    },
    priority: {
      type: "integer",
      format: "int32",
      minimum: 0,
    },
    startChannel: {
      type: "integer",
      format: "int32",
      minimum: 0,
    },
    type: {
      type: "integer",
      format: "int32",
      minimum: 0,
    },
  },
} as const;
