-- 将 oss_obj 表的 path 字段长度从 VARCHAR(100) 扩展到 VARCHAR(255)
ALTER TABLE oss_obj
    ALTER COLUMN path TYPE VARCHAR(255);