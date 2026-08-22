-- 移除 oss_obj 表中 is_completed 字段的默认值
ALTER TABLE oss_obj ALTER COLUMN is_completed DROP DEFAULT;
