-- 迁移脚本：将列名去下划线前缀，统一命名规范
-- _id             -> id
-- _creator_id     -> creator_id
-- _create_timestamp -> create_ts
-- _updator_id     -> updator_id
-- _update_timestamp -> update_ts

-- ============================================================
-- 1. 删除 oss_obj_ref 表的外键约束（重命名引用列前需要先删除）
-- ============================================================
ALTER TABLE oss_obj_ref DROP CONSTRAINT IF EXISTS fk_obj_id__from__oss_obj;
ALTER TABLE oss_obj_ref DROP CONSTRAINT IF EXISTS fk_bucket_id__from__oss_bucket;

-- ============================================================
-- 2. 重命名 oss_bucket 表的列
-- ============================================================
ALTER TABLE oss_bucket RENAME COLUMN _id TO id;
ALTER TABLE oss_bucket RENAME COLUMN _creator_id TO creator_id;
ALTER TABLE oss_bucket RENAME COLUMN _create_timestamp TO create_ts;
ALTER TABLE oss_bucket RENAME COLUMN _updator_id TO updator_id;
ALTER TABLE oss_bucket RENAME COLUMN _update_timestamp TO update_ts;

-- 更新 oss_bucket 表的列注释
COMMENT ON COLUMN oss_bucket.id IS 'ID';
COMMENT ON COLUMN oss_bucket.creator_id IS '创建人的用户ID';
COMMENT ON COLUMN oss_bucket.create_ts IS '建立时间戳';
COMMENT ON COLUMN oss_bucket.updator_id IS '修改人的用户ID';
COMMENT ON COLUMN oss_bucket.update_ts IS '修改时间戳';

-- ============================================================
-- 3. 重命名 oss_obj 表的列
-- ============================================================
ALTER TABLE oss_obj RENAME COLUMN _id TO id;
ALTER TABLE oss_obj RENAME COLUMN _creator_id TO creator_id;
ALTER TABLE oss_obj RENAME COLUMN _create_timestamp TO create_ts;
ALTER TABLE oss_obj RENAME COLUMN _updator_id TO updator_id;
ALTER TABLE oss_obj RENAME COLUMN _update_timestamp TO update_ts;

-- 更新 oss_obj 表的列注释
COMMENT ON COLUMN oss_obj.id IS 'ID';
COMMENT ON COLUMN oss_obj.creator_id IS '创建人的用户ID';
COMMENT ON COLUMN oss_obj.create_ts IS '建立时间戳';
COMMENT ON COLUMN oss_obj.updator_id IS '修改人的用户ID';
COMMENT ON COLUMN oss_obj.update_ts IS '修改时间戳';

-- ============================================================
-- 4. 重命名 oss_obj_ref 表的列
-- ============================================================
ALTER TABLE oss_obj_ref RENAME COLUMN _id TO id;
ALTER TABLE oss_obj_ref RENAME COLUMN _creator_id TO creator_id;
ALTER TABLE oss_obj_ref RENAME COLUMN _create_timestamp TO create_ts;
ALTER TABLE oss_obj_ref RENAME COLUMN _updator_id TO updator_id;
ALTER TABLE oss_obj_ref RENAME COLUMN _update_timestamp TO update_ts;

-- 更新 oss_obj_ref 表的列注释
COMMENT ON COLUMN oss_obj_ref.id IS 'ID';
COMMENT ON COLUMN oss_obj_ref.creator_id IS '创建人的用户ID';
COMMENT ON COLUMN oss_obj_ref.create_ts IS '建立时间戳';
COMMENT ON COLUMN oss_obj_ref.updator_id IS '修改人的用户ID';
COMMENT ON COLUMN oss_obj_ref.update_ts IS '修改时间戳';

-- ============================================================
-- 5. 重建外键约束（引用列名已更新为 id）
-- ============================================================
ALTER TABLE oss_obj_ref
    ADD CONSTRAINT fk_obj_id__from__oss_obj
        FOREIGN KEY (obj_id) REFERENCES oss_obj (id)
            ON DELETE RESTRICT ON UPDATE RESTRICT;

ALTER TABLE oss_obj_ref
    ADD CONSTRAINT fk_bucket_id__from__oss_bucket
        FOREIGN KEY (bucket_id) REFERENCES oss_bucket (id)
            ON DELETE RESTRICT ON UPDATE RESTRICT;