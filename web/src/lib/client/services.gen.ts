import type { Options } from "@hey-api/client-fetch";
import type {
  DelMeshData,
  DelMeshError,
  DelMeshResponse,
  DelPlaylistData,
  DelPlaylistError,
  DelPlaylistResponse,
  DelSceneData,
  DelSceneError,
  DelSceneResponse,
  DelScheduleData,
  DelScheduleError,
  DelScheduleResponse,
  DelSequenceData,
  DelSequenceError,
  DelSequenceResponse,
  DownloadMeshData,
  DownloadMeshError,
  DownloadMeshResponse,
  FileUploadData,
  FileUploadError,
  FileUploadResponse,
  FppCommandData,
  FppCommandError,
  FppCommandResponse,
  GetDisplayError,
  GetDisplayResponse,
  GetLogData,
  GetLogError,
  GetLogResponse,
  GetOutputsError,
  GetOutputsResponse,
  GetPlaylistData,
  GetPlaylistError,
  GetPlaylistResponse,
  GetSceneData,
  GetSceneError,
  GetSceneResponse,
  GetScheduleData,
  GetScheduleError,
  GetScheduleResponse,
  GetSchedulerStatusError,
  GetSchedulerStatusResponse,
  GetSequenceData,
  GetSequenceError,
  GetSequenceMetaData,
  GetSequenceMetaError,
  GetSequenceMetaResponse,
  GetSequenceResponse,
  GetTestSequenceData,
  GetTestSequenceError,
  GetTestSequenceResponse,
  ListLogsError,
  ListLogsResponse,
  ListMeshesError,
  ListMeshesResponse,
  ListModelsError,
  ListModelsResponse,
  ListPlaylistsError,
  ListPlaylistsNumberedError,
  ListPlaylistsNumberedResponse,
  ListPlaylistsResponse,
  ListScenesError,
  ListScenesResponse,
  ListSchedulesError,
  ListSchedulesResponse,
  ListSequencesError,
  ListSequencesResponse,
  NewMeshData,
  NewMeshError,
  NewMeshResponse,
  NewPlaylistData,
  NewPlaylistError,
  NewPlaylistResponse,
  NewSceneData,
  NewSceneError,
  NewSceneResponse,
  NewScheduleData,
  NewScheduleError,
  NewScheduleResponse,
  RunTestData,
  RunTestError,
  RunTestResponse,
  StartSchedulerError,
  StartSchedulerResponse,
  StopSchedulerError,
  StopSchedulerResponse,
  SystemInfoError,
  SystemInfoResponse,
  UpdateMeshData,
  UpdateMeshError,
  UpdateMeshResponse,
  UpdatePlaylistData,
  UpdatePlaylistError,
  UpdatePlaylistResponse,
  UpdateSceneData,
  UpdateSceneError,
  UpdateSceneResponse,
  UpdateScheduleData,
  UpdateScheduleError,
  UpdateScheduleResponse,
  UploadDisplayData,
  UploadDisplayError,
  UploadDisplayResponse,
  UploadModelsData,
  UploadModelsError,
  UploadModelsResponse,
  UploadOutputsData,
  UploadOutputsError,
  UploadOutputsResponse,
} from "./types.gen";

import { createClient, createConfig, formDataBodySerializer } from "@hey-api/client-fetch";

// This file is auto-generated by @hey-api/openapi-ts

export const client = createClient(createConfig());

/**
 * Retrieve outputs.json
 * Download the outputs in JSON format
 */
export const getOutputs = <ThrowOnError extends boolean = false>(
  options?: Options<unknown, ThrowOnError>,
) => {
  return (options?.client ?? client).get<GetOutputsResponse, GetOutputsError, ThrowOnError>({
    ...options,
    url: "/api/channel/output/universeOutputs",
  });
};

/**
 * Upload outputs.json
 * Upload the outputs in JSON format
 */
export const uploadOutputs = <ThrowOnError extends boolean = false>(
  options: Options<UploadOutputsData, ThrowOnError>,
) => {
  return (options?.client ?? client).post<
    UploadOutputsResponse,
    UploadOutputsError,
    ThrowOnError
  >({
    ...options,
    url: "/api/channel/output/universeOutputs",
  });
};

/**
 * Retrieve VirtualDisplayMap
 * Download the VirtualDisplayMap
 */
export const getDisplay = <ThrowOnError extends boolean = false>(
  options?: Options<unknown, ThrowOnError>,
) => {
  return (options?.client ?? client).get<GetDisplayResponse, GetDisplayError, ThrowOnError>({
    ...options,
    url: "/api/configfile/virtualdisplaymap",
  });
};

/**
 * Upload VirtualDisplayMap
 * Upload the VirtualDisplayMap
 */
export const uploadDisplay = <ThrowOnError extends boolean = false>(
  options: Options<UploadDisplayData, ThrowOnError>,
) => {
  return (options?.client ?? client).post<
    UploadDisplayResponse,
    UploadDisplayError,
    ThrowOnError
  >({
    ...options,
    url: "/api/configfile/virtualdisplaymap",
  });
};

/**
 * Get a specific log
 */
export const getLog = <ThrowOnError extends boolean = false>(
  options: Options<GetLogData, ThrowOnError>,
) => {
  return (options?.client ?? client).get<GetLogResponse, GetLogError, ThrowOnError>({
    ...options,
    url: "/api/log/{name}",
  });
};

/**
 * Get log filenames
 */
export const listLogs = <ThrowOnError extends boolean = false>(
  options?: Options<unknown, ThrowOnError>,
) => {
  return (options?.client ?? client).get<ListLogsResponse, ListLogsError, ThrowOnError>({
    ...options,
    url: "/api/logs",
  });
};

/**
 * New mesh
 * Create a new mesh
 */
export const newMesh = <ThrowOnError extends boolean = false>(
  options: Options<NewMeshData, ThrowOnError>,
) => {
  return (options?.client ?? client).post<NewMeshResponse, NewMeshError, ThrowOnError>({
    ...options,
    url: "/api/mesh",
  });
};

/**
 * Get a 3D mesh
 * Download a 3D mesh for the virtual display
 */
export const downloadMesh = <ThrowOnError extends boolean = false>(
  options: Options<DownloadMeshData, ThrowOnError>,
) => {
  return (options?.client ?? client).get<DownloadMeshResponse, DownloadMeshError, ThrowOnError>(
    {
      ...options,
      url: "/api/mesh/{mesh}",
    },
  );
};

/**
 * Update a mesh
 * Create or update the given mesh
 */
export const updateMesh = <ThrowOnError extends boolean = false>(
  options: Options<UpdateMeshData, ThrowOnError>,
) => {
  return (options?.client ?? client).put<UpdateMeshResponse, UpdateMeshError, ThrowOnError>({
    ...options,
    url: "/api/mesh/{mesh}",
  });
};

/**
 * Delete a mesh
 * Delete the given mesh
 */
export const delMesh = <ThrowOnError extends boolean = false>(
  options: Options<DelMeshData, ThrowOnError>,
) => {
  return (options?.client ?? client).delete<DelMeshResponse, DelMeshError, ThrowOnError>({
    ...options,
    url: "/api/mesh/{mesh}",
  });
};

/**
 * List all meshes
 * List all 3D meshes
 */
export const listMeshes = <ThrowOnError extends boolean = false>(
  options?: Options<unknown, ThrowOnError>,
) => {
  return (options?.client ?? client).get<ListMeshesResponse, ListMeshesError, ThrowOnError>({
    ...options,
    url: "/api/meshes",
  });
};

/**
 * Retrieve models.json
 * Download the models in JSON format
 */
export const listModels = <ThrowOnError extends boolean = false>(
  options?: Options<unknown, ThrowOnError>,
) => {
  return (options?.client ?? client).get<ListModelsResponse, ListModelsError, ThrowOnError>({
    ...options,
    url: "/api/models",
  });
};

/**
 * Upload models.json
 * Upload the models in JSON format
 */
export const uploadModels = <ThrowOnError extends boolean = false>(
  options: Options<UploadModelsData, ThrowOnError>,
) => {
  return (options?.client ?? client).post<
    UploadModelsResponse,
    UploadModelsError,
    ThrowOnError
  >({
    ...options,
    url: "/api/models",
  });
};

/**
 * New playlist
 * Create a new playlist
 */
export const newPlaylist = <ThrowOnError extends boolean = false>(
  options: Options<NewPlaylistData, ThrowOnError>,
) => {
  return (options?.client ?? client).post<NewPlaylistResponse, NewPlaylistError, ThrowOnError>({
    ...options,
    url: "/api/playlist",
  });
};

/**
 * Get a playlist
 * Read back a playlist
 */
export const getPlaylist = <ThrowOnError extends boolean = false>(
  options: Options<GetPlaylistData, ThrowOnError>,
) => {
  return (options?.client ?? client).get<GetPlaylistResponse, GetPlaylistError, ThrowOnError>({
    ...options,
    url: "/api/playlist/{playlist}",
  });
};

/**
 * Update a playlist
 * Create or update the given playlist
 */
export const updatePlaylist = <ThrowOnError extends boolean = false>(
  options: Options<UpdatePlaylistData, ThrowOnError>,
) => {
  return (options?.client ?? client).put<
    UpdatePlaylistResponse,
    UpdatePlaylistError,
    ThrowOnError
  >({
    ...options,
    url: "/api/playlist/{playlist}",
  });
};

/**
 * Delete a playlist
 * Delete the given playlist
 */
export const delPlaylist = <ThrowOnError extends boolean = false>(
  options: Options<DelPlaylistData, ThrowOnError>,
) => {
  return (options?.client ?? client).delete<
    DelPlaylistResponse,
    DelPlaylistError,
    ThrowOnError
  >({
    ...options,
    url: "/api/playlist/{playlist}",
  });
};

/**
 * List playlists
 * List the name of all playlists
 */
export const listPlaylists = <ThrowOnError extends boolean = false>(
  options?: Options<unknown, ThrowOnError>,
) => {
  return (options?.client ?? client).get<
    ListPlaylistsResponse,
    ListPlaylistsError,
    ThrowOnError
  >({
    ...options,
    url: "/api/playlists",
  });
};

/**
 * List playlists with ID
 * List the playlists with their ID
 */
export const listPlaylistsNumbered = <ThrowOnError extends boolean = false>(
  options?: Options<unknown, ThrowOnError>,
) => {
  return (options?.client ?? client).get<
    ListPlaylistsNumberedResponse,
    ListPlaylistsNumberedError,
    ThrowOnError
  >({
    ...options,
    url: "/api/playlists/numbered",
  });
};

/**
 * New scene
 * Create a new scene
 */
export const newScene = <ThrowOnError extends boolean = false>(
  options: Options<NewSceneData, ThrowOnError>,
) => {
  return (options?.client ?? client).post<NewSceneResponse, NewSceneError, ThrowOnError>({
    ...options,
    url: "/api/scene",
  });
};

/**
 * Delete a scene
 * Delete the given scene
 */
export const delScene = <ThrowOnError extends boolean = false>(
  options: Options<DelSceneData, ThrowOnError>,
) => {
  return (options?.client ?? client).delete<DelSceneResponse, DelSceneError, ThrowOnError>({
    ...options,
    url: "/api/scene/{scene}",
  });
};

/**
 * List scenes
 * List all scenes
 */
export const listScenes = <ThrowOnError extends boolean = false>(
  options?: Options<unknown, ThrowOnError>,
) => {
  return (options?.client ?? client).get<ListScenesResponse, ListScenesError, ThrowOnError>({
    ...options,
    url: "/api/scenes",
  });
};

/**
 * Get a scene
 * Read a single scene
 */
export const getScene = <ThrowOnError extends boolean = false>(
  options: Options<GetSceneData, ThrowOnError>,
) => {
  return (options?.client ?? client).get<GetSceneResponse, GetSceneError, ThrowOnError>({
    ...options,
    url: "/api/scenes/{scene}",
  });
};

/**
 * Update a scene
 * Create or update the given scene
 */
export const updateScene = <ThrowOnError extends boolean = false>(
  options: Options<UpdateSceneData, ThrowOnError>,
) => {
  return (options?.client ?? client).put<UpdateSceneResponse, UpdateSceneError, ThrowOnError>({
    ...options,
    url: "/api/scenes/{scene}",
  });
};

/**
 * New schedule
 * Create a new schedule
 */
export const newSchedule = <ThrowOnError extends boolean = false>(
  options: Options<NewScheduleData, ThrowOnError>,
) => {
  return (options?.client ?? client).post<NewScheduleResponse, NewScheduleError, ThrowOnError>({
    ...options,
    url: "/api/schedule",
  });
};

/**
 * Get a schedule
 * Read back a schedule
 */
export const getSchedule = <ThrowOnError extends boolean = false>(
  options: Options<GetScheduleData, ThrowOnError>,
) => {
  return (options?.client ?? client).get<GetScheduleResponse, GetScheduleError, ThrowOnError>({
    ...options,
    url: "/api/schedule/{schedule}",
  });
};

/**
 * Update a schedule
 * Create or update the given schedule
 */
export const updateSchedule = <ThrowOnError extends boolean = false>(
  options: Options<UpdateScheduleData, ThrowOnError>,
) => {
  return (options?.client ?? client).put<
    UpdateScheduleResponse,
    UpdateScheduleError,
    ThrowOnError
  >({
    ...options,
    url: "/api/schedule/{schedule}",
  });
};

/**
 * Delete a schedule
 * Delete the given schedule
 */
export const delSchedule = <ThrowOnError extends boolean = false>(
  options: Options<DelScheduleData, ThrowOnError>,
) => {
  return (options?.client ?? client).delete<
    DelScheduleResponse,
    DelScheduleError,
    ThrowOnError
  >({
    ...options,
    url: "/api/schedule/{schedule}",
  });
};

/**
 * Get the scheduler status
 */
export const getSchedulerStatus = <ThrowOnError extends boolean = false>(
  options?: Options<unknown, ThrowOnError>,
) => {
  return (options?.client ?? client).get<
    GetSchedulerStatusResponse,
    GetSchedulerStatusError,
    ThrowOnError
  >({
    ...options,
    url: "/api/scheduler",
  });
};

/**
 * Start the scheduler
 */
export const startScheduler = <ThrowOnError extends boolean = false>(
  options?: Options<unknown, ThrowOnError>,
) => {
  return (options?.client ?? client).get<
    StartSchedulerResponse,
    StartSchedulerError,
    ThrowOnError
  >({
    ...options,
    url: "/api/scheduler/start",
  });
};

/**
 * Stop the scheduler
 */
export const stopScheduler = <ThrowOnError extends boolean = false>(
  options?: Options<unknown, ThrowOnError>,
) => {
  return (options?.client ?? client).get<
    StopSchedulerResponse,
    StopSchedulerError,
    ThrowOnError
  >({
    ...options,
    url: "/api/scheduler/stop",
  });
};

/**
 * List schedules
 * List the name of all schedules
 */
export const listSchedules = <ThrowOnError extends boolean = false>(
  options?: Options<unknown, ThrowOnError>,
) => {
  return (options?.client ?? client).get<
    ListSchedulesResponse,
    ListSchedulesError,
    ThrowOnError
  >({
    ...options,
    url: "/api/schedules",
  });
};

/**
 * Get a sequence
 * Download a sequence file
 */
export const getSequence = <ThrowOnError extends boolean = false>(
  options: Options<GetSequenceData, ThrowOnError>,
) => {
  return (options?.client ?? client).get<GetSequenceResponse, GetSequenceError, ThrowOnError>({
    ...options,
    url: "/api/sequence/{filename}",
  });
};

/**
 * Delete a sequence
 * Remove a sequence file
 */
export const delSequence = <ThrowOnError extends boolean = false>(
  options: Options<DelSequenceData, ThrowOnError>,
) => {
  return (options?.client ?? client).delete<
    DelSequenceResponse,
    DelSequenceError,
    ThrowOnError
  >({
    ...options,
    url: "/api/sequence/{filename}",
  });
};

/**
 * Get a sequence's metadata
 * Get the metadata belonging to a sequence
 */
export const getSequenceMeta = <ThrowOnError extends boolean = false>(
  options: Options<GetSequenceMetaData, ThrowOnError>,
) => {
  return (options?.client ?? client).get<
    GetSequenceMetaResponse,
    GetSequenceMetaError,
    ThrowOnError
  >({
    ...options,
    url: "/api/sequence/{filename}/meta",
  });
};

/**
 * List all sequences
 * List all sequence files
 */
export const listSequences = <ThrowOnError extends boolean = false>(
  options?: Options<unknown, ThrowOnError>,
) => {
  return (options?.client ?? client).get<
    ListSequencesResponse,
    ListSequencesError,
    ThrowOnError
  >({
    ...options,
    url: "/api/sequences",
  });
};

/**
 * Get system info.
 * Get the high-level system information. This endpoint is used
 * to simulate FPP and make us discoverable by other software such
 * as xLights. Some values are hard-coded to ensure compatibility.
 */
export const systemInfo = <ThrowOnError extends boolean = false>(
  options?: Options<unknown, ThrowOnError>,
) => {
  return (options?.client ?? client).get<SystemInfoResponse, SystemInfoError, ThrowOnError>({
    ...options,
    url: "/api/system/info",
  });
};

/**
 * Run LED test patterns
 */
export const runTest = <ThrowOnError extends boolean = false>(
  options: Options<RunTestData, ThrowOnError>,
) => {
  return (options?.client ?? client).post<RunTestResponse, RunTestError, ThrowOnError>({
    ...options,
    url: "/api/test/run",
  });
};

/**
 * Get the pattern of LED colors for the given test
 */
export const getTestSequence = <ThrowOnError extends boolean = false>(
  options: Options<GetTestSequenceData, ThrowOnError>,
) => {
  return (options?.client ?? client).post<
    GetTestSequenceResponse,
    GetTestSequenceError,
    ThrowOnError
  >({
    ...options,
    url: "/api/test/sequence",
  });
};

/**
 * Upload a file
 * Accepts fseq sequences or media files such as
 * images and videos. The uploaded file is automatically
 * sorted into the relevant upload directory so a call to
 * `moveFile` isn't required and will be ignore.
 */
export const fileUpload = <ThrowOnError extends boolean = false>(
  options: Options<FileUploadData, ThrowOnError>,
) => {
  return (options?.client ?? client).post<FileUploadResponse, FileUploadError, ThrowOnError>({
    ...options,
    ...formDataBodySerializer,
    headers: {
      "Content-Type": null,
      ...options?.headers,
    },
    url: "/api/upload",
  });
};

/**
 * Run an FPP Command
 * This method isn't really implemented. The only command you
 * can issue is `moveFile` and all that really does is check
 * whether a file exists or not - it doesn't move it because
 * that's handled at upload time.
 */
export const fppCommand = <ThrowOnError extends boolean = false>(
  options: Options<FppCommandData, ThrowOnError>,
) => {
  return (options?.client ?? client).get<FppCommandResponse, FppCommandError, ThrowOnError>({
    ...options,
    url: "/fppxml.php",
  });
};
