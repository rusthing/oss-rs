-- ============================================================
-- 模型变更：oss_obj.is_completed 改名为 completed
-- 对应 model/create-table-pgsql.sql（Created on 2026/9/11 14:22:56）
-- 由提交 171ffc1（* is_completed -> completed）生成
-- ============================================================

-- 1. oss_obj.is_completed -> completed（无默认值、无独立索引，可安全改名）
ALTER TABLE oss_obj RENAME COLUMN is_completed TO completed;

-- 2. 时间戳字段注释统一：建立时间戳 -> 创建时间戳
COMMENT ON COLUMN oss_bucket.create_ts IS '创建时间戳';
COMMENT ON COLUMN oss_obj.create_ts IS '创建时间戳';
COMMENT ON COLUMN oss_obj_ref.create_ts IS '创建时间戳';

-- 说明：oss_obj.completed 的注释「是否完成」文本未变，RENAME 后注释自动跟随，
-- 无需额外更新。
