-- 将 oss_obj_ref 表的 url 字段改名为 download_url，并新增 preview_url 字段

-- 1. 将 url 字段改名为 download_url
ALTER TABLE oss_obj_ref
    RENAME COLUMN url TO download_url;

-- 2. 新增 preview_url 字段（可空）
ALTER TABLE oss_obj_ref
    ADD COLUMN preview_url VARCHAR(200) NULL;

-- 3. 修改 ext 字段为可空
ALTER TABLE oss_obj_ref
    ALTER COLUMN ext DROP NOT NULL;

-- 4. 更新唯一约束（字段名从 url 改为 download_url）
ALTER TABLE oss_obj_ref
DROP CONSTRAINT IF EXISTS ak_url_oss_obj_ref;

ALTER TABLE oss_obj_ref
    ADD CONSTRAINT ak_url_oss_obj_ref UNIQUE (download_url);

-- 5. 更新字段注释
COMMENT ON COLUMN oss_obj_ref.download_url IS '下载 URL';
COMMENT ON COLUMN oss_obj_ref.preview_url IS '预览 URL';
