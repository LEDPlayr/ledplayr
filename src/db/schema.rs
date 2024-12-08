// @generated automatically by Diesel CLI.

diesel::table! {
    buttons (id) {
        id -> Integer,
        status -> Text,
        error -> Text,
        battery -> Float,
        input -> Bool,
        last -> Integer,
        now -> Integer,
    }
}

diesel::table! {
    meshes (id) {
        id -> Integer,
        name -> Text,
        scale_x -> Float,
        scale_y -> Float,
        scale_z -> Float,
        pos_x -> Float,
        pos_y -> Float,
        pos_z -> Float,
        rot_x -> Float,
        rot_y -> Float,
        rot_z -> Float,
    }
}

diesel::table! {
    playlists (id) {
        id -> Integer,
        name -> Text,
        description -> Text,
        repeat -> Bool,
        loop_count -> Integer,
    }
}

diesel::table! {
    playlists_sequences (playlist_id, sequence_id, sort_by) {
        playlist_id -> Integer,
        sequence_id -> Integer,
        sort_by -> Integer,
        enabled -> Bool,
        play_once -> Bool,
    }
}

diesel::table! {
    scenes (id) {
        id -> Integer,
        name -> Text,
        cam_pos_x -> Float,
        cam_pos_y -> Float,
        cam_pos_z -> Float,
        cam_rot_x -> Float,
        cam_rot_y -> Float,
        cam_rot_z -> Float,
        cam_zoom -> Float,
        ctrl_x -> Float,
        ctrl_y -> Float,
        ctrl_z -> Float,
    }
}

diesel::table! {
    schedules (id) {
        id -> Integer,
        name -> Text,
        playlist_id -> Integer,
        enabled -> Bool,
        start_date -> Integer,
        end_date -> Integer,
        start_time -> BigInt,
        end_time -> BigInt,
        monday -> Bool,
        tuesday -> Bool,
        wednesday -> Bool,
        thursday -> Bool,
        friday -> Bool,
        saturday -> Bool,
        sunday -> Bool,
    }
}

diesel::table! {
    sequences (id) {
        id -> Integer,
        name -> Text,
        timestamp -> Text,
        step_time -> Integer,
        frames -> Integer,
        channels -> Integer,
    }
}

diesel::table! {
    variables (id) {
        id -> Integer,
        sequence_id -> Integer,
        name -> Text,
        value -> Text,
    }
}

diesel::joinable!(playlists_sequences -> playlists (playlist_id));
diesel::joinable!(playlists_sequences -> sequences (sequence_id));
diesel::joinable!(schedules -> playlists (playlist_id));
diesel::joinable!(variables -> sequences (sequence_id));

diesel::allow_tables_to_appear_in_same_query!(
    buttons,
    meshes,
    playlists,
    playlists_sequences,
    scenes,
    schedules,
    sequences,
    variables,
);
