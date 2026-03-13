-- PostgreSQL 脚本：直接重命名唯一约束

-- 1. 重命名 oss_bucket 表的约束
ALTER TABLE oss_bucket
    RENAME CONSTRAINT AK_NAME_OSS_BUCK TO AK_NAME_OSS_BUCKET;

-- 2. 重命名 oss_obj_ref 表的约束
ALTER TABLE oss_obj_ref
    RENAME CONSTRAINT AK_URL_OSS_OBJ_ TO AK_URL_OSS_OBJ_REF;
