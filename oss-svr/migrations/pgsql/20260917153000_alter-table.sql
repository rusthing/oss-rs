-- ============================================================
-- 模型变更：create_ts -> create_ms, update_ts -> update_ms
-- 统一时间戳字段命名规范：ts 后缀改为 ms 后缀，体现毫秒级精度
-- 对应 model/create-table-pgsql.sql
-- ============================================================

-- 1. oss_bucket 表
ALTER TABLE oss_bucket RENAME COLUMN create_ts TO create_ms;
ALTER TABLE oss_bucket RENAME COLUMN update_ts TO update_ms;

COMMENT ON COLUMN oss_bucket.create_ms IS '创建时间戳';
COMMENT ON COLUMN oss_bucket.update_ms IS '修改时间戳';

-- 2. oss_obj 表
ALTER TABLE oss_obj RENAME COLUMN create_ts TO create_ms;
ALTER TABLE oss_obj RENAME COLUMN update_ts TO update_ms;

COMMENT ON COLUMN oss_obj.create_ms IS '创建时间戳';
COMMENT ON COLUMN oss_obj.update_ms IS '修改时间戳';

-- 3. oss_obj_ref 表
ALTER TABLE oss_obj_ref RENAME COLUMN create_ts TO create_ms;
ALTER TABLE oss_obj_ref RENAME COLUMN update_ts TO update_ms;

COMMENT ON COLUMN oss_obj_ref.create_ms IS '创建时间戳';
COMMENT ON COLUMN oss_obj_ref.update_ms IS '修改时间戳';