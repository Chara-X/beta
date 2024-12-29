-- database: /home/chara-x/daisy/codes/beta/rdcloud/cmd/rdcloud/rdcloud.db
WITH
    x as (
        SELECT
            引入者
        FROM
            troubles
    )
SELECT
    标题
FROM
    troubles
where
    引入者 in x;

SELECT
    *
FROM
    troubles
ORDER BY
    "标识"
LIMIT
    1, 4;
