-- ============================================================
-- 模型变更：creator_id -> creater_id, updator_id -> updater_id
-- 统一审计字段命名规范，并规范列注释：
--   creator_id -> creater_id  注释 '创建者ID'
--   updator_id -> updater_id  注释 '更新者ID'
--   update_ms                 注释 '更新时间戳'
-- 对应 model/create-table-pgsql.sql
-- ============================================================

-- 1. oss_bucket 表
ALTER TABLE oss_bucket RENAME COLUMN creator_id TO creater_id;
ALTER TABLE oss_bucket RENAME COLUMN updator_id TO updater_id;

COMMENT ON COLUMN oss_bucket.creater_id IS '创建者ID';
COMMENT ON COLUMN oss_bucket.updater_id IS '更新者ID';
COMMENT ON COLUMN oss_bucket.update_ms IS '更新时间戳';

-- 2. oss_obj 表
ALTER TABLE oss_obj RENAME COLUMN creator_id TO creater_id;
ALTER TABLE oss_obj RENAME COLUMN updator_id TO updater_id;

COMMENT ON COLUMN oss_obj.creater_id IS '创建者ID';
COMMENT ON COLUMN oss_obj.updater_id IS '更新者ID';
COMMENT ON COLUMN oss_obj.update_ms IS '更新时间戳';

-- 3. oss_obj_ref 表
ALTER TABLE oss_obj_ref RENAME COLUMN creator_id TO creater_id;
ALTER TABLE oss_obj_ref RENAME COLUMN updator_id TO updater_id;

COMMENT ON COLUMN oss_obj_ref.creater_id IS '创建者ID';
COMMENT ON COLUMN oss_obj_ref.updater_id IS '更新者ID';
COMMENT ON COLUMN oss_obj_ref.update_ms IS '更新时间戳';
