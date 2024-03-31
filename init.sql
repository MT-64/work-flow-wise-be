drop schema public cascade
create schema public
-- User table insert



INSERT INTO "User" (pk_user_id, username, email, password, department_id, organize_id, first_name, last_name, gender, role, introduction_brief, image, total_credit, is_deleted, deleted_at, created_at, updated_at)
SELECT 
    'user_' || i,
    'username_' || i AS username,
    'email_' || i || '@example.com' AS email,
    '$argon2id$v=19$m=19456,t=2,p=1$AdQTtmDnySmu22KcmIRAWQ$ZtCKydYEQ7MCOZLciL32Ka03oZQDMkxhOrmF9WHd9oA',
    NULL AS department_id,
    NULL AS organize_id,
    'first_name_' || i AS first_name,
    'last_name_' || i AS last_name,
    'Male' AS gender,
    'Employee' AS role,
    'introduction_brief_' || i AS introduction_brief,
    NULL AS image,
    0 AS total_credit,
    FALSE AS is_deleted,
    NULL AS deleted_at,
    NOW() AS created_at,
    NOW() AS updated_at
FROM generate_series(1, 20) AS i;
-- Organize table insert
INSERT INTO "Organize" (pk_organize_id, owner_id, name, address, contact)
SELECT 
    'organize_' || i,
    (SELECT pk_user_id FROM "User" ORDER BY random() LIMIT 1) AS owner_id,
    'organize_name_' || i AS name,
    'address_' || i AS address,
    'contact_' || i AS contact
FROM generate_series(1, 3) AS i;

-- Department table insert
INSERT INTO "Department" (pk_department_id, organize_id, manager_id, name)
SELECT 
    'department_' || i,
    'organize_1' AS organize_id,
    'user_' || i AS manager_id,
    'department_name_' || i AS name
FROM generate_series(1, 5) AS i;

-- Period table insert
INSERT INTO "Period" (pk_period_id, organize_id, name, start_date, end_date)
SELECT 
    'period_' || i as pk_period_id,
    (SELECT pk_organize_id FROM "Organize" ORDER BY random() LIMIT 1) AS organize_id,
    'period_name_' || i AS name,
    NOW() + INTERVAL '30 days' * random() AS start_date,
    NOW() + INTERVAL '60 days' * random() AS end_date
FROM generate_series(1, 5) AS i;

-- Objective table insert
INSERT INTO "Objective" (pk_objective_id, period_id, parent_objective_id, obj_type, supervisor_id, name, description, target, progress, status, created_at, updated_at, deadline)
SELECT 
    'obj_' || i,
    (SELECT pk_period_id FROM "Period" ORDER BY random() LIMIT 1) AS period_id,
    NULL AS parent_objective_id,
    'Other' AS obj_type,
    (SELECT pk_user_id FROM "User" ORDER BY random() LIMIT 1) AS supervisor_id,
    'objective_name_' || i AS name,
    'description_' || i AS description,
    'target_' || i AS target,
    random() AS progress,
    TRUE AS status,
    NOW() AS created_at,
    NOW() AS updated_at,
    NOW() + INTERVAL '30 days' AS deadline
FROM generate_series(1, 100) AS i;


-- Notification table insert
INSERT INTO "Notification" (pk_notification_id, user_id, message, status)
SELECT 
    md5(random()::text || clock_timestamp()::text)::uuid AS pk_notification_id,
    (SELECT pk_user_id FROM "User" ORDER BY random() LIMIT 1) AS user_id,
    'message_' || i AS message,
    FALSE AS status
FROM generate_series(1, 10) AS i;

-- KeyResult table insert
INSERT INTO "KeyResult" (pk_kr_id, objective_id, name, description, target, progress, status, metric, deadline, user_id, created_at, updated_at)
SELECT 
    'kr_' || i,
    (SELECT pk_objective_id FROM "Objective" ORDER BY random() LIMIT 1),
    'name_' || i AS name,
    'description_' || i AS description,
    'target_' || i AS target,
    random() AS progress,
    TRUE AS status,
    'metric_' || i AS metric,
    NOW() + INTERVAL '30 days' AS deadline,
    (SELECT pk_user_id FROM "User" ORDER BY random() LIMIT 1),
    NOW() AS createdAt,
    NOW() AS updatedAt
FROM generate_series(1, 1000) AS i;



-- ObjectiveOnDepartment table insert
INSERT INTO "ObjectiveOnDepartment" (id, obj_id, department_id)
SELECT 
    md5(random()::text || clock_timestamp()::text)::uuid AS id,
    'obj_' || i,
    'department_' || (i % 5) + 1
FROM generate_series(1, 100) AS i;

-- ObjectiveOnUser table insert
INSERT INTO "ObjectiveOnUser" (id, obj_id, user_id)
SELECT 
    md5(random()::text || clock_timestamp()::text)::uuid AS id,
    'obj_' || i,
    'user_' || (i % 20) + 1
FROM generate_series(1, 100) AS i;

-- ObjectiveOnOrg table insert
INSERT INTO "ObjectiveOnOrg" (id, obj_id, org_id)
SELECT 
    md5(random()::text || clock_timestamp()::text)::uuid AS id,
    'obj_' || i,
    'organize_' || (i % 3) + 1
FROM generate_series(1, 100) AS i;

-- Folder table insert
INSERT INTO "Folder" (id, ownerId, parentFolderId, folderName, visibility, createdAt, updatedAt)
SELECT 
    md5(random()::text || clock_timestamp()::text)::uuid AS id,
    (SELECT pk_user_id FROM "User" ORDER BY random() LIMIT 1) AS ownerId,
    NULL AS parentFolderId,
    'folder_name_' || i AS folderName,
    'public' AS visibility,
    NOW() AS createdAt,
    NOW() AS updatedAt
FROM generate_series(1, 10) AS i;

-- File table insert
INSERT INTO "File" (id, ownerId, parentFolderId, filename, extension, visibility, createdAt, updatedAt)
SELECT 
    md5(random()::text || clock_timestamp()::text)::uuid AS id,
    (SELECT pk_user_id FROM "User" ORDER BY random() LIMIT 1) AS ownerId,
    (SELECT id FROM "Folder" ORDER BY random() LIMIT 1) AS parentFolderId,
    'file_name_' || i AS filename,
    'png' AS extension,
    'public' AS visibility,
    NOW() AS createdAt,
    NOW() AS updatedAt
FROM generate_series(1, 10) AS i;

-- FileVersion table insert
INSERT INTO "FileVersion" (id, fileId, versionNumber)
SELECT 
    md5(random()::text || clock_timestamp()::text)::uuid AS id,
    (SELECT id FROM "File" ORDER BY random() LIMIT 1) AS fileId,
    1 AS versionNumber
FROM generate_series(1, 10) AS i;

-- Tag table insert
INSERT INTO "Tag" (id, tagName, ownerId)
SELECT 
    md5(random()::text || clock_timestamp()::text)::uuid AS id,
    'tag_name_' || i AS tagName,
    (SELECT pk_user_id FROM "User" ORDER BY random() LIMIT 1) AS ownerId
FROM generate_series(1, 10) AS i;
