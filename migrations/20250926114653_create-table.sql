/*==============================================================*/
/* DBMS name:      PostgreSQL 9.x                               */
/* Created on:     2025/9/25 12:02:26                           */
/*==============================================================*/


/*==============================================================*/
/* Table: oss_obj                                               */
/*==============================================================*/
create table oss_obj (
                         id                   INT8                 not null,
                         name                 VARCHAR(100)         not null,
                         bucket               VARCHAR(50)          not null,
                         is_completed         BOOL                 not null default true,
                         ext                  VARCHAR(10)          null,
                         path                 VARCHAR(100)         null,
                         size                 INT8                 null,
                         hash                 VARCHAR(64)          null,
                         url                  VARCHAR(200)         null,
                         creator_id           INT8                 null,
                         create_timestamp     INT8                 null,
                         updator_id           INT8                 null,
                         update_timestamp     INT8                 null,
                         constraint PK_OSS_OBJ primary key (id)
);

comment on table oss_obj is
'对象';

comment on column oss_obj.id is
'ID';

comment on column oss_obj.name is
'名称(上传时的文件原名，带后缀名)';

comment on column oss_obj.bucket is
'分组';

comment on column oss_obj.is_completed is
'是否完成';

comment on column oss_obj.ext is
'扩展名';

comment on column oss_obj.path is
'路径
存储文件的路径';

comment on column oss_obj.size is
'大小';

comment on column oss_obj.hash is
'Hash';

comment on column oss_obj.url is
'URL';

comment on column oss_obj.creator_id is
'创建人的用户ID';

comment on column oss_obj.create_timestamp is
'建立时间戳';

comment on column oss_obj.updator_id is
'修改人的用户ID';

comment on column oss_obj.update_timestamp is
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
                             name                 VARCHAR(100)         not null,
                             bucket               VARCHAR(50)          not null,
                             creator_id           INT8                 null,
                             create_timestamp     INT8                 null,
                             updator_id           INT8                 null,
                             update_timestamp     INT8                 null,
                             constraint PK_OSS_OBJ_REF primary key (id)
);

comment on table oss_obj_ref is
'对象引用';

comment on column oss_obj_ref.id is
'ID';

comment on column oss_obj_ref.obj_id is
'对象ID';

comment on column oss_obj_ref.name is
'名称(上传时的文件原名，带后缀名)';

comment on column oss_obj_ref.bucket is
'分组';

comment on column oss_obj_ref.creator_id is
'创建人的用户ID';

comment on column oss_obj_ref.create_timestamp is
'建立时间戳';

comment on column oss_obj_ref.updator_id is
'修改人的用户ID';

comment on column oss_obj_ref.update_timestamp is
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

alter table oss_obj_ref
    add constraint FK_OSS_OBJ__RELATIONS_OSS_OBJ foreign key (obj_id)
        references oss_obj (id)
        on delete restrict on update restrict;

