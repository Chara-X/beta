-- database: :memory:
DELETE FROM "table" AS "alias" /* INDEXED BY "index" / NOT INDEXED */
WHERE
    "expr" RETURNING "expr" AS "alias",
    "*",
    "...";
