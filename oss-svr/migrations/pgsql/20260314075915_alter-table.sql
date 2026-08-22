-- 修改 oss_obj 表的 size 和 hash 字段为 NULL
ALTER TABLE oss_obj
    ALTER COLUMN size DROP NOT NULL,
ALTER COLUMN hash DROP NOT NULL;