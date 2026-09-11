-- ============================================================
-- 模型变更：oss_obj.completed 增加默认值 false
-- 对应 model/create-table-pgsql.sql（Created on 2026/9/11 16:38:21）
-- ============================================================

ALTER TABLE oss_obj ALTER COLUMN completed SET DEFAULT FALSE;