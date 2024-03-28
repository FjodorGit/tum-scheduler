-- This file should undo anything in `up.sql`
DELETE FROM organization
WHERE NOT EXISTS (
        SELECT
            1
        FROM
            lecture
        WHERE
            lecture.organization = organization.id);

