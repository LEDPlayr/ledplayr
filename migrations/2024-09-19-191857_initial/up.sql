CREATE TABLE sequences (
    id INTEGER NOT NULL PRIMARY KEY,

    name TEXT NOT NULL UNIQUE,
    timestamp TEXT NOT NULL,
    step_time INTEGER NOT NULL,
    frames INTEGER NOT NULL,
    channels INTEGER NOT NULL
);

CREATE TABLE playlists(
    id INTEGER NOT NULL PRIMARY KEY,

    name TEXT NOT NULL UNIQUE,
    description TEXT NOT NULL DEFAULT "",
    repeat BOOLEAN NOT NULL DEFAULT 0,
    loop_count INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE schedules(
    id INTEGER NOT NULL PRIMARY KEY,

    name TEXT NOT NULL UNIQUE,
    playlist_id INTEGER NOT NULL,
    enabled BOOLEAN NOT NULL DEFAULT 0,

    start_date INTEGER NOT NULL,
    end_date INTEGER NOT NULL,
    start_time BIGINT NOT NULL,
    end_time BIGINT NOT NULL,

    monday BOOLEAN NOT NULL DEFAULT 0,
    tuesday BOOLEAN NOT NULL DEFAULT 0,
    wednesday BOOLEAN NOT NULL DEFAULT 0,
    thursday BOOLEAN NOT NULL DEFAULT 0,
    friday BOOLEAN NOT NULL DEFAULT 0,
    saturday BOOLEAN NOT NULL DEFAULT 0,
    sunday BOOLEAN NOT NULL DEFAULT 0,

    FOREIGN KEY (playlist_id) REFERENCES playlists(id)
);

CREATE TABLE playlists_sequences(
    playlist_id INTEGER REFERENCES playlists(id) NOT NULL,
    sequence_id INTEGER REFERENCES sequences(id) NOT NULL,

    sort_by INTEGER NOT NULL,

    enabled BOOLEAN NOT NULL DEFAULT 0,
    play_once BOOLEAN NOT NULL DEFAULT 0,

    PRIMARY KEY(playlist_id, sequence_id, sort_by)
);

CREATE TABLE variables(
    id INTEGER NOT NULL PRIMARY KEY,

    sequence_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    value TEXT NOT NULL,

    FOREIGN KEY (sequence_id) REFERENCES sequences(id)
);

