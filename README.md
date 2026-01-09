# OSS-RS

一个基于 Rust 的对象存储服务系统，提供类似于阿里云 OSS 的功能。该项目使用 Actix-web 作为 Web 框架，SeaORM 作为数据库 ORM，支持文件上传、下载、预览等功能。

## 功能特性

- 文件上传、下载和预览
- 存储桶（Bucket）管理
- 对象引用管理
- RESTful API 接口
- Swagger UI 文档支持
- 支持 Range 请求的断点续传和流媒体播放
- 文件哈希校验确保数据完整性
- 基于 IDWorker 的分布式 ID 生成

## 技术栈

- [Rust](https://www.rust-lang.org/) - 编程语言
- [Actix-web](https://actix.rs/) - Web 框架
- [SeaORM](https://www.sea-ql.org/SeaORM/) - 数据库 ORM
- [PostgreSQL](https://www.postgresql.org/) - 数据库
- [Tokio](https://tokio.rs/) - 异步运行时
- [Serde](https://serde.rs/) - 序列化/反序列化
- [Utoipa](https://github.com/juhaku/utoipa) - API 文档生成

## 项目结构

```
src/
├── api/              # API 接口层
├── base/             # 基础模块
├── cst/              # 常量定义
├── dao/              # 数据访问层
├── model/            # 数据模型
├── ro/               # 返回结果对象
├── config/           # 配置管理
├── svc/              # 业务逻辑层
├── to/               # 传输对象
├── utils/            # 工具类
└── vo/               # 视图对象
```

## 数据模型

### OssBucket（存储桶）
存储桶是对象存储的容器，用于存放对象。

### OssObj（对象）
对象是存储的基本单位，每个对象包含元数据和数据。

### OssObjRef（对象引用）
对象引用用于关联存储桶和对象，并保存额外的元数据。

## API 接口

项目提供完整的 RESTful API 接口，包括：

1. 存储桶管理接口：
   - 创建存储桶
   - 修改存储桶
   - 删除存储桶
   - 获取存储桶信息

2. 对象管理接口：
   - 上传对象
   - 下载对象
   - 预览对象
   - 删除对象

3. 对象引用接口：
   - 创建引用
   - 修改引用
   - 删除引用
   - 获取引用信息

API 文档可通过 Swagger UI 访问：`http://localhost:端口/swagger-ui/`

## 配置文件

项目使用 TOML 格式的配置文件（默认为 `oss-rs.toml`）：

```toml
[oss]
file-dir-format = "%Y/%m/%d"           # 文件存储目录格式
upload-buffer-size = "2MiB"            # 上传缓冲区大小
download-buffer-size = "2MiB"          # 下载缓冲区大小

[db]
url = "postgres://ossrs:ossrs@127.0.0.1/ossrs"  # 数据库连接URL

[id-worker]
epoch = 1758108749211                  # ID生成器纪元时间
machine-id = 1                         # 机器ID
node-id = 1                            # 节点ID
```

## 环境要求

- Rust 2024 edition
- PostgreSQL 数据库
- 系统环境变量配置

## 安装和运行

1. 克隆项目：
   ```bash
   git clone <项目地址>
   cd oss-rs
   ```

2. 配置数据库：
   确保 PostgreSQL 数据库运行正常，并根据配置文件设置正确的连接信息。

3. 运行项目：
   ```bash
   cargo run
   ```

4. 访问服务：
   服务默认运行在 `http://localhost:端口`，具体端口在配置文件中设置。

## 命令行参数

- `-c, --config-file <config-file>`: 指定配置文件路径
- `-p, --port <port>`: 指定 Web 服务器端口

## 许可证

本项目采用 MIT 许可证，详情请见 [LICENSE](LICENSE) 文件。