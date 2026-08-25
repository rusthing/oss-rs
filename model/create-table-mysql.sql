/*==============================================================*/
/* DBMS name:      MySQL 5.0                                    */
/* Created on:     2026/8/25 15:58:48                           */
/*==============================================================*/


/*==============================================================*/
/* Table: oss_bucket                                            */
/*==============================================================*/
create table oss_bucket
(
   id                   bigint not null  comment 'ID',
   name                 varchar(50) not null  comment '名称',
   remark               varchar(50)  comment '备注',
   creator_id           bigint not null  comment '创建人的用户ID',
   create_ts            bigint not null  comment '建立时间戳',
   updator_id           bigint not null  comment '修改人的用户ID',
   update_ts            bigint not null  comment '修改时间戳',
   primary key (id),
   key AK_NAME (name)
);

alter table oss_bucket comment '桶';

/*==============================================================*/
/* Table: oss_obj                                               */
/*==============================================================*/
create table oss_obj
(
   id                   bigint not null  comment 'ID',
   is_completed         bool not null  comment '是否完成',
   path                 varchar(255) not null  comment '路径
             存储文件的路径',
   size                 bigint  comment '大小',
   hash                 varchar(64)  comment 'Hash',
   creator_id           bigint not null  comment '创建人的用户ID',
   create_ts            bigint not null  comment '建立时间戳',
   updator_id           bigint not null  comment '修改人的用户ID',
   update_ts            bigint not null  comment '修改时间戳',
   primary key (id),
   key AK_PATH (path),
   key AK_SIZE_AND_HASH (size, hash)
);

alter table oss_obj comment '对象';

/*==============================================================*/
/* Table: oss_obj_ref                                           */
/*==============================================================*/
create table oss_obj_ref
(
   id                   bigint not null  comment 'ID',
   obj_id               bigint not null  comment '对象ID',
   bucket_id            bigint not null  comment '桶ID',
   name                 varchar(100) not null  comment '名称(上传时的文件原名，带后缀名)',
   ext                  varchar(10)  comment '文件扩展名',
   download_url         varchar(200) not null  comment '下载URL',
   preview_url          varchar(200)  comment '预览URL',
   creator_id           bigint not null  comment '创建人的用户ID',
   create_ts            bigint not null  comment '建立时间戳',
   updator_id           bigint not null  comment '修改人的用户ID',
   update_ts            bigint not null  comment '修改时间戳',
   primary key (id),
   key AK_URL (download_url)
);

alter table oss_obj_ref comment '对象引用';

alter table oss_obj_ref add constraint fk_obj_id__from__oss_obj foreign key (obj_id)
      references oss_obj (id) on delete restrict on update restrict;

alter table oss_obj_ref add constraint fk_bucket_id__from__oss_bucket foreign key (bucket_id)
      references oss_bucket (id) on delete restrict on update restrict;

