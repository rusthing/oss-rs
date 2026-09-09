-- ============================================================
-- 为 oss_bucket 和 oss_obj_ref 表新增 enabled 字段
-- ============================================================

ALTER TABLE oss_bucket ADD COLUMN enabled BOOL NOT NULL DEFAULT TRUE;
COMMENT ON COLUMN oss_bucket.enabled IS '启用';

ALTER TABLE oss_obj_ref ADD COLUMN enabled BOOL NOT NULL DEFAULT TRUE;
COMMENT ON COLUMN oss_obj_ref.enabled IS '启用';