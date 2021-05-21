-- Your SQL goes here
CREATE TYPE logging_level AS ENUM ('debug', 'info', 'warning', 'error');

CREATE TABLE logs (
    id          SERIAL PRIMARY KEY,
    "message"   TEXT NOT NULL,
    "level"     logging_level NOT NULL,
    "datetime"  TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP    
);