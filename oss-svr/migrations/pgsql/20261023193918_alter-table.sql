-- ============================================================
-- 模型变更：oss_obj_ref 的唯一约束改按列名命名
--   AK_URL_OSS_OBJ_REF -> AK_DOWNLOAD_URL_OSS_OBJ_REF
-- 约束建在 download_url 上，名字要与框架的唯一键回查规则
-- ak_{列名_and_连接}_{表名} 一致，否则重复引用只能报「获取 unique 字段列表错误」，
-- 接口退化到 500。
-- 对应 model/create-table-pgsql.sql
-- 按新名字建过表的库（或约束本就不存在）会跳过，可重复执行。
-- ============================================================

-- 1. 重命名唯一约束（旧名来自 20260316085205：列改叫 download_url，约束名还没跟上）
DO $$
BEGIN
    IF EXISTS (
        SELECT 1 FROM pg_constraint WHERE conname = 'ak_url_oss_obj_ref'
    ) THEN
        ALTER TABLE oss_obj_ref
            RENAME CONSTRAINT ak_url_oss_obj_ref TO ak_download_url_oss_obj_ref;
    END IF;
END $$;
