CREATE TABLE buttons(
    id INTEGER NOT NULL PRIMARY KEY,

    status TEXT NOT NULL DEFAULT "",
    error TEXT NOT NULL DEFAULT "",
    battery REAL NOT NULL DEFAULT 0,
    input BOOLEAN NOT NULL DEFAULT 0,
    last INTEGER NOT NULL DEFAULT 0,
    now INTEGER NOT NULL DEFAULT 0,

    action TEXT CHECK(action IN ('no_action', 'schedule', 'playlist', 'sequence', 'stop')) NOT NULL,
    action_target TEXT NOT NULL
);
