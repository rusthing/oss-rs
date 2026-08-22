-- 检查并更新外键约束名称（如果仍使用旧名称）
-- 1. 删除旧的外键约束（如果存在）
ALTER TABLE oss_obj_ref DROP CONSTRAINT IF EXISTS FK_OSS_OBJ__RELATIONS_OSS_OBJ;
ALTER TABLE oss_obj_ref DROP CONSTRAINT IF EXISTS FK_OSS_OBJ__RELATIONS_OSS_BUCK;

-- 2. 添加新的外键约束（使用新名称）
ALTER TABLE oss_obj_ref
    ADD CONSTRAINT fk_obj_id__from__oss_obj
        FOREIGN KEY (obj_id) REFERENCES oss_obj (_id)
            ON DELETE RESTRICT ON UPDATE RESTRICT;
ALTER TABLE oss_obj_ref
    ADD CONSTRAINT fk_bucket_id__from__oss_bucket
        FOREIGN KEY (bucket_id) REFERENCES oss_bucket (_id)
            ON DELETE RESTRICT ON UPDATE RESTRICT;
