/*==============================================================*/
/* DBMS name:      PostgreSQL 9.x                               */
/* Created on:     2026/8/25 16:00:18                           */
/*==============================================================*/


/*==============================================================*/
/* Table: oss_bucket                                            */
/*==============================================================*/
create table oss_bucket (
   id                   INT8                 not null,
   name                 VARCHAR(50)          not null,
   remark               VARCHAR(50)          null,
   creator_id           INT8                 not null,
   create_ts            INT8                 not null,
   updator_id           INT8                 not null,
   update_ts            INT8                 not null,
   constraint PK_OSS_BUCKET primary key (id),
   constraint AK_NAME_OSS_BUCKET unique (name)
);

comment on table oss_bucket is
'桶';

comment on column oss_bucket.id is
'ID';

comment on column oss_bucket.name is
'名称';

comment on column oss_bucket.remark is
'备注';

comment on column oss_bucket.creator_id is
'创建人的用户ID';

comment on column oss_bucket.create_ts is
'建立时间戳';

comment on column oss_bucket.updator_id is
'修改人的用户ID';

comment on column oss_bucket.update_ts is
'修改时间戳';

/*==============================================================*/
/* Index: oss_bucket_PK                                         */
/*==============================================================*/
create unique index oss_bucket_PK on oss_bucket (
id
);

/*==============================================================*/
/* Table: oss_obj                                               */
/*==============================================================*/
create table oss_obj (
   id                   INT8                 not null,
   is_completed         BOOL                 not null,
   path                 VARCHAR(255)         not null,
   size                 INT8                 null,
   hash                 VARCHAR(64)          null,
   creator_id           INT8                 not null,
   create_ts            INT8                 not null,
   updator_id           INT8                 not null,
   update_ts            INT8                 not null,
   constraint PK_OSS_OBJ primary key (id),
   constraint AK_PATH_OSS_OBJ unique (path),
   constraint AK_SIZE_AND_HASH_OSS_OBJ unique (size, hash)
);

comment on table oss_obj is
'对象';

comment on column oss_obj.id is
'ID';

comment on column oss_obj.is_completed is
'是否完成';

comment on column oss_obj.path is
'路径
存储文件的路径';

comment on column oss_obj.size is
'大小';

comment on column oss_obj.hash is
'Hash';

comment on column oss_obj.creator_id is
'创建人的用户ID';

comment on column oss_obj.create_ts is
'建立时间戳';

comment on column oss_obj.updator_id is
'修改人的用户ID';

comment on column oss_obj.update_ts is
'修改时间戳';

/*==============================================================*/
/* Index: oss_obj_PK                                            */
/*==============================================================*/
create unique index oss_obj_PK on oss_obj (
id
);

/*==============================================================*/
/* Table: oss_obj_ref                                           */
/*==============================================================*/
create table oss_obj_ref (
   id                   INT8                 not null,
   obj_id               INT8                 not null,
   bucket_id            INT8                 not null,
   name                 VARCHAR(100)         not null,
   ext                  VARCHAR(10)          null,
   download_url         VARCHAR(200)         not null,
   preview_url          VARCHAR(200)         null,
   creator_id           INT8                 not null,
   create_ts            INT8                 not null,
   updator_id           INT8                 not null,
   update_ts            INT8                 not null,
   constraint PK_OSS_OBJ_REF primary key (id),
   constraint AK_URL_OSS_OBJ_REF unique (download_url)
);

comment on table oss_obj_ref is
'对象引用';

comment on column oss_obj_ref.id is
'ID';

comment on column oss_obj_ref.obj_id is
'对象ID';

comment on column oss_obj_ref.bucket_id is
'桶ID';

comment on column oss_obj_ref.name is
'名称(上传时的文件原名，带后缀名)';

comment on column oss_obj_ref.ext is
'文件扩展名';

comment on column oss_obj_ref.download_url is
'下载URL';

comment on column oss_obj_ref.preview_url is
'预览URL';

comment on column oss_obj_ref.creator_id is
'创建人的用户ID';

comment on column oss_obj_ref.create_ts is
'建立时间戳';

comment on column oss_obj_ref.updator_id is
'修改人的用户ID';

comment on column oss_obj_ref.update_ts is
'修改时间戳';

/*==============================================================*/
/* Index: oss_obj_ref_PK                                        */
/*==============================================================*/
create unique index oss_obj_ref_PK on oss_obj_ref (
id
);

/*==============================================================*/
/* Index: Relationship_1_FK                                     */
/*==============================================================*/
create  index Relationship_1_FK on oss_obj_ref (
obj_id
);

/*==============================================================*/
/* Index: Relationship_2_FK                                     */
/*==============================================================*/
create  index Relationship_2_FK on oss_obj_ref (
bucket_id
);

alter table oss_obj_ref
   add constraint fk_obj_id__from__oss_obj foreign key (obj_id)
      references oss_obj (id)
      on delete restrict on update restrict;

alter table oss_obj_ref
   add constraint fk_bucket_id__from__oss_bucket foreign key (bucket_id)
      references oss_bucket (id)
      on delete restrict on update restrict;

