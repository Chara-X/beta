-- database: :memory:
UPDATE /* OR REPLACE / OR IGNORE */ "table" AS "alias" /* INDEXED BY "index" / NOT INDEXED */
SET
    ("column", "...") = ("expr", "...")
WHERE
    "expr" RETURNING "expr" AS "alias",
    "*",
    "...";
