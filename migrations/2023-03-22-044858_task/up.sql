CREATE TABLE task (
    id INTEGER NOT NULL,
    title TEXT NOT NULL,
    done BOOLEAN NOT NULL DEFAULT FALSE,
    PRIMARY KEY (id)
);