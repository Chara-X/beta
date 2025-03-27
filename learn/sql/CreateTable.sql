-- database: :memory:
CREATE TEMP TABLE IF NOT EXISTS
    "table"
    /* AS "select"; */
    (
        "column" "type" NOT NULL DEFAULT ("expr") AS ("expr") STORED,
        PRIMARY KEY ("column", "...") /* ON CONFLICT REPLACE / ON CONFLICT IGNORE */,
        FOREIGN KEY ("column", "...") REFERENCES "table" ("column", "...") ON UPDATE "action" ON DELETE "action",
        UNIQUE ("column", "...") /* ON CONFLICT REPLACE / ON CONFLICT IGNORE */,
        CHECK ("expr"),
        "..."
    ) STRICT,
    WITHOUT ROWID;
