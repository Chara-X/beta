-- database: :memory:
SELECT DISTINCT
    *
FROM
    "table" as "alias" /* INDEXED BY "index" / NOT INDEXED */
    /* LEFT RIGHT FULL */
    JOIN "table" as "alias" ON "expr"
    /* LEFT RIGHT FULL */
    JOIN "..."
WHERE
    "expr"
GROUP BY
    "expr",
    "..."
HAVING
    "expr"
    /* UNION "select" / INTERSECT "select" / EXCEPT "select" */
ORDER BY
    "expr" DESC NULLS LAST,
    "..."
LIMIT
    "expr", "expr";

VALUES
    ("expr", "..."),
    ("...");
