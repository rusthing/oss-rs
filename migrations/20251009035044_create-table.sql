/*==============================================================*/
/* DBMS name:      PostgreSQL 9.x                               */
/* Created on:     2025/10/9 11:48:59                           */
/*==============================================================*/


/*==============================================================*/
/* Table: oss_bucket                                            */
/*==============================================================*/
create table oss_bucket (
                            _id                  INT8                 not null,
                            name                 VARCHAR(50)          not null,
                            _creator_id          INT8                 not null,
                            _create_timestamp    INT8                 not null,
                            _updator_id          INT8                 not null,
                            _update_timestamp    INT8                 not null,
                            constraint PK_OSS_BUCKET primary key (_id),
                            constraint AK_NAME_OSS_BUCK unique (name)
);

comment on table oss_bucket is
'桶';

comment on column oss_bucket._id is
'ID';

comment on column oss_bucket.name is
'名称';

comment on column oss_bucket._creator_id is
'创建人的用户ID';

comment on column oss_bucket._create_timestamp is
'建立时间戳';

comment on column oss_bucket._updator_id is
'修改人的用户ID';

comment on column oss_bucket._update_timestamp is
'修改时间戳';

/*==============================================================*/
/* Index: oss_bucket_PK                                         */
/*==============================================================*/
create unique index oss_bucket_PK on oss_bucket (
                                                 _id
    );

/*==============================================================*/
/* Table: oss_obj                                               */
/*==============================================================*/
create table oss_obj (
                         _id                  INT8                 not null,
                         name                 VARCHAR(100)         not null,
                         is_completed         BOOL                 not null default true,
                         path                 VARCHAR(100)         not null,
                         size                 INT8                 not null,
                         hash                 VARCHAR(64)          not null,
                         url                  VARCHAR(200)         not null,
                         _creator_id          INT8                 not null,
                         _create_timestamp    INT8                 not null,
                         _updator_id          INT8                 not null,
                         _update_timestamp    INT8                 not null,
                         constraint PK_OSS_OBJ primary key (_id)
);

comment on table oss_obj is
'对象';

comment on column oss_obj._id is
'ID';

comment on column oss_obj.name is
'名称(上传时的文件原名，带后缀名)';

comment on column oss_obj.is_completed is
'是否完成';

comment on column oss_obj.path is
'路径
存储文件的路径';

comment on column oss_obj.size is
'大小';

comment on column oss_obj.hash is
'Hash';

comment on column oss_obj.url is
'URL';

comment on column oss_obj._creator_id is
'创建人的用户ID';

comment on column oss_obj._create_timestamp is
'建立时间戳';

comment on column oss_obj._updator_id is
'修改人的用户ID';

comment on column oss_obj._update_timestamp is
'修改时间戳';

/*==============================================================*/
/* Index: oss_obj_PK                                            */
/*==============================================================*/
create unique index oss_obj_PK on oss_obj (
                                           _id
    );

/*==============================================================*/
/* Table: oss_obj_ref                                           */
/*==============================================================*/
create table oss_obj_ref (
                             _id                  INT8                 not null,
                             obj_id               INT8                 not null,
                             bucket_id            INT8                 not null,
                             name                 VARCHAR(100)         not null,
                             ext                  VARCHAR(10)          not null,
                             _creator_id          INT8                 not null,
                             _create_timestamp    INT8                 not null,
                             _updator_id          INT8                 not null,
                             _update_timestamp    INT8                 not null,
                             constraint PK_OSS_OBJ_REF primary key (_id)
);

comment on table oss_obj_ref is
'对象引用';

comment on column oss_obj_ref._id is
'ID';

comment on column oss_obj_ref.obj_id is
'对象ID';

comment on column oss_obj_ref.bucket_id is
'桶ID';

comment on column oss_obj_ref.name is
'名称(上传时的文件原名，带后缀名)';

comment on column oss_obj_ref.ext is
'扩展名';

comment on column oss_obj_ref._creator_id is
'创建人的用户ID';

comment on column oss_obj_ref._create_timestamp is
'建立时间戳';

comment on column oss_obj_ref._updator_id is
'修改人的用户ID';

comment on column oss_obj_ref._update_timestamp is
'修改时间戳';

/*==============================================================*/
/* Index: oss_obj_ref_PK                                        */
/*==============================================================*/
create unique index oss_obj_ref_PK on oss_obj_ref (
                                                   _id
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
    add constraint FK_OSS_OBJ__RELATIONS_OSS_OBJ foreign key (obj_id)
        references oss_obj (_id)
        on delete restrict on update restrict;

alter table oss_obj_ref
    add constraint FK_OSS_OBJ__RELATIONS_OSS_BUCK foreign key (bucket_id)
        references oss_bucket (_id)
        on delete restrict on update restrict;

