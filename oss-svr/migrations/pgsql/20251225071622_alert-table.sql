-- 迁移脚本：将 url 字段从 oss_obj 表迁移到 oss_obj_ref 表
BEGIN;

-- 1. 从 oss_obj 表中删除 url 字段（先保存数据）
-- 创建临时表保存数据
CREATE TEMP TABLE temp_url_data AS
SELECT _id, url
FROM oss_obj
WHERE url IS NOT NULL;

-- 2. 从 oss_obj 表中删除 url 字段
ALTER TABLE oss_obj DROP CONSTRAINT IF EXISTS AK_URL_OSS_OBJ;
ALTER TABLE oss_obj DROP COLUMN IF EXISTS url;

-- 3. 在 oss_obj_ref 表中添加 url 字段
ALTER TABLE oss_obj_ref ADD COLUMN IF NOT EXISTS url VARCHAR(200) NOT NULL DEFAULT '';

-- 4. 迁移数据：将临时表中的 url 数据更新到 oss_obj_ref 表
-- 假设 oss_obj_ref 表中的 obj_id 与 oss_obj 表中的 _id 关联
UPDATE oss_obj_ref
SET url = temp_url_data.url
    FROM temp_url_data
WHERE oss_obj_ref.obj_id = temp_url_data._id;

-- 5. 删除默认值约束（如果之前添加了）
ALTER TABLE oss_obj_ref ALTER COLUMN url DROP DEFAULT;

-- 6. 在 oss_obj_ref 表中添加唯一约束
ALTER TABLE oss_obj_ref ADD CONSTRAINT AK_URL_OSS_OBJ_ UNIQUE (url);

-- 7. 更新表注释
COMMENT ON COLUMN oss_obj_ref.url IS 'URL';

COMMIT;
