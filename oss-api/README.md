# oss-api

OSS 项目的共享类型定义库，为 OSS 服务端和 API 客户端提供统一的 DTO、VO 和 MO 类型。

## 模块说明

### DTO（Data Transfer Object）

数据传输对象，用于 API 请求/响应的序列化与反序列化。

- `OssBucketDto`：存储桶 DTO
- `OssObjDto`：对象 DTO
- `OssObjRefDto`：对象引用 DTO

### VO（View Object）

视图对象，用于前端展示的数据结构。

- `OssBucketVo`：存储桶 VO
- `OssObjVo`：对象 VO
- `OssObjRefVo`：对象引用 VO

### MO（Model Object）

数据库实体模型，由 SeaORM 代码生成器自动生成。仅在启用 `server` feature 时可用。

- `oss_bucket`：存储桶表实体
- `oss_obj`：对象表实体
- `oss_obj_ref`：对象引用表实体

## Features

| Feature  | 说明                                | 依赖                                       |
| -------- | ----------------------------------- | ------------------------------------------ |
| `server` | 启用服务端功能，包含 ORM 和数据校验 | `sea-orm`, `o2o`, `validator`, `robotech/macros` |

## 使用示例

### 作为 API 客户端依赖（默认）

```toml
[dependencies]
oss-api = { version = "1.6" }
```

### 作为服务端依赖

```toml
[dependencies]
oss-api = { version = "1.6", features = ["server"] }
```

## 许可证

MIT
