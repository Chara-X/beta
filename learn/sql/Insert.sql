-- database: :memory:
INSERT /* OR REPLACE / OR IGNORE */ INTO "table" AS "alias" ("column", "...")
/* "select" */
DEFAULT
VALUES
    RETURNING "expr" AS "alias",
    "*",
    "...";
