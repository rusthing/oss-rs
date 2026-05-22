# oss-rs-svr

这是 oss-rs 项目的服务端组件，基于 Rust 构建的高性能对象存储服务。

## 项目概述

oss-svr 是一个功能完整的对象存储服务系统，提供类似于阿里云 OSS、AWS S3 等云存储服务的核心功能。该服务使用 Axum 作为 Web 框架，SeaORM 作为数据库 ORM，支持文件上传、下载、预览、存储桶管理等完整功能。

## 核心功能

### 文件管理
- **文件上传**：支持单文件上传、分片上传，可配置上传缓冲区大小和文件大小限制
- **文件下载**：支持普通下载和 Range 请求（断点续传）
- **文件预览**：支持在线预览图片、文档等文件类型
- **流媒体播放**：支持视频文件的流式传输和分段加载
- **文件哈希校验**：使用 SHA256 确保数据完整性

### 存储桶管理
- 创建、修改、删除存储桶（Bucket）
- 存储桶列表查询
- 存储桶元数据管理

### 对象管理
- 对象的增删改查操作
- 对象元数据管理
- 对象引用（Object Reference）管理，支持关联业务数据

### API 与安全
- RESTful API 设计，符合行业最佳实践
- Swagger UI 自动生成的 API 文档
- CORS 跨域资源共享支持
- IP 白名单/黑名单机制
- 本地访问控制（local-only URIs）
- URI 访问权限控制（forbidden URIs）
- HTTPS/TLS 加密传输支持

### 高级特性
- 基于 IDWorker 的分布式唯一 ID 生成
- 数据库迁移自动化（支持 MySQL 和 PostgreSQL）
- 配置文件热重载（无需重启服务）
- 优雅停机与信号处理（SIGTERM/SIGKILL）
- 结构化日志记录与追踪
- 健康检查端点支持

## 技术栈

### 核心框架
- **Rust 2024 Edition**：现代系统编程语言，保证内存安全和高性能
- **Axum**：Tokio 生态的 Ergonomic Web 框架
- **Tokio**：异步运行时，提供高并发处理能力
- **SeaORM**：异步 ORM 框架，支持 MySQL 和 PostgreSQL

### 数据处理
- **SQLx**：编译期 SQL 验证的异步 SQL 工具包
- **Serde**：序列化/反序列化框架
- **Validator**：数据验证框架
- **Bytesize**：人性化的字节大小解析

### API 文档
- **Utoipa**：OpenAPI 3.0 文档生成
- **Utoipa-Swagger-UI**：交互式 API 文档界面

### 工具库
- **Clap**：命令行参数解析
- **Chrono**：日期时间处理
- **Regex**：正则表达式
- **Multer**：Multipart 表单数据解析
- **Futures-util**：异步工具集

### 内部依赖
- **robotech**：内部微服务框架（提供 Web、DB、宏等功能）
- **wheel-rs**：内部工具库
- **idworker**：分布式 ID 生成器

## 项目架构

```
src/
├── app/              # 应用配置模块
│   ├── mod.rs
│   └── app_config.rs
├── dao/              # 数据访问层（Data Access Object）
│   ├── mod.rs
│   ├── oss_bucket_dao.rs
│   ├── oss_obj_dao.rs
│   └── oss_obj_ref_dao.rs
├── dto/              # 数据传输对象（Data Transfer Object）
│   ├── mod.rs
│   ├── oss_bucket_dto.rs
│   ├── oss_obj_dto.rs
│   └── oss_obj_ref_dto.rs
├── model/            # 数据库模型（SeaORM Entities）
│   ├── mod.rs
│   ├── oss_bucket.rs
│   ├── oss_obj.rs
│   └── oss_obj_ref.rs
├── svc/              # 业务逻辑层（Service）
│   ├── mod.rs
│   ├── oss_bucket_svc.rs    # 存储桶服务
│   ├── oss_file_svc.rs      # 文件服务（核心业务逻辑）
│   ├── oss_obj_svc.rs       # 对象服务
│   └── oss_obj_ref_svc.rs   # 对象引用服务
├── vo/               # 视图对象（View Object，API 响应格式）
│   ├── mod.rs
│   ├── oss_bucket_vo.rs
│   ├── oss_obj_vo.rs
│   └── oss_obj_ref_vo.rs
├── web/              # Web 层
│   ├── api_doc/      # API 文档配置
│   ├── ctrl/         # 控制器（Controllers）
│   │   ├── oss_bucket_ctrl.rs
│   │   ├── oss_file_ctrl.rs     # 文件上传下载控制器
│   │   ├── oss_obj_ctrl.rs
│   │   └── oss_obj_ref_ctrl.rs
│   ├── router/       # 路由配置
│   │   ├── oss_bucket_router.rs
│   │   ├── oss_file_router.rs
│   │   ├── oss_obj_router.rs
│   │   └── oss_obj_ref_router.rs
│   └── mod.rs
├── lib.rs            # 库入口
└── main.rs           # 程序入口（启动流程、信号处理）
```

### 分层架构说明

1. **Web 层（web/）**：处理 HTTP 请求路由、参数解析、响应格式化
2. **控制器层（ctrl/）**：接收请求、参数验证、调用服务层、返回响应
3. **服务层（svc/）**：核心业务逻辑实现、事务管理、业务流程编排
4. **数据访问层（dao/）**：数据库 CRUD 操作封装
5. **模型层（model/）**：数据库表结构映射（SeaORM Entities）

### 数据流转

```
HTTP Request → Router → Controller → Service → DAO → Database
                    ↓
                 DTO/VO转换
                    ↓
HTTP Response ← Serializer ← VO
```

## API 接口概览

### 存储桶接口（/oss/bucket）
- `POST /oss/bucket` - 创建存储桶
- `PUT /oss/bucket` - 更新存储桶信息
- `DELETE /oss/bucket/:id` - 删除存储桶
- `GET /oss/bucket/:id` - 获取存储桶详情
- `GET /oss/bucket/list` - 列出所有存储桶（仅本地访问）

### 文件接口（/oss/file）
- `POST /oss/file/upload` - 上传文件（支持 multipart/form-data）
- `GET /oss/file/download/:obj_ref_id` - 下载文件（支持 Range 请求）
- `GET /oss/file/preview/:obj_ref_id` - 预览文件
- `DELETE /oss/file/:obj_ref_id` - 删除文件

### 对象接口（/oss/obj）
- `POST /oss/obj` - 创建对象记录
- `PUT /oss/obj` - 更新对象信息
- `DELETE /oss/obj/:id` - 删除对象
- `GET /oss/obj/:id` - 获取对象详情
- `GET /oss/obj/list` - 列出对象（仅本地访问）

### 对象引用接口（/oss/obj-ref）
- `POST /oss/obj-ref` - 创建对象引用
- `PUT /oss/obj-ref` - 更新对象引用
- `DELETE /oss/obj-ref/:id` - 删除对象引用
- `GET /oss/obj-ref/:id` - 获取对象引用详情
- `GET /oss/obj-ref/list` - 列出对象引用（仅本地访问）

### 系统接口
- `GET /health` - 健康检查端点
- `GET /swagger-ui/` - Swagger API 文档界面
- `GET /api-doc/openapi.json` - OpenAPI JSON 规范

> **注意**：标记为"仅本地访问"的接口只能通过 localhost 或配置的 IP 白名单访问。

## 配置说明

### 配置文件位置

默认配置文件：`oss-svr.toml`（可通过 `-c` 参数指定其他路径）

### 配置项详解

#### OSS 配置段 `[oss]`

```toml
[oss]
file-dir-format = "%Y/%m/%d"           # 文件存储目录格式（strftime 格式）
upload-file-limit-size = "100MiB"      # 单个文件上传大小限制
upload-buffer-size = "2MiB"            # 上传缓冲区大小
download-buffer-size = "2MiB"          # 下载缓冲区大小
```

**文件目录格式示例**：
- `%Y/%m/%d` → `2026/05/22`
- `%Y-%m` → `2026-05`
- `%Y/%W` → `2026/21`（年/周）

#### 数据库配置段 `[db]`

```toml
[db]
# PostgreSQL 连接字符串
url = "postgres://ossrs:ossrs@pgsql:5432/ossrs"

# MySQL 连接字符串（二选一）
# url = "mysql://ossrs:ossrs@mysql:3306/ossrs"
```

**支持的数据库**：
- PostgreSQL 12+
- MySQL 8.0+

#### Web 服务器配置段 `[web-server]`

```toml
[web-server]
log-enabled = true                     # 是否启用请求日志
bind = "127.0.0.1"                     # 绑定地址（支持字符串或数组）
port = 9840                            # 监听端口
cors.enabled = true                    # 启用 CORS
reuse-port = false                     # 是否重用端口（SO_REUSEPORT）

# 禁止访问的 URI 列表（格式：METHOD:path）
forbidden-urns = [
    "GET:/swagger-ui/"                 # 生产环境建议禁用 Swagger
]

# 仅限本地访问的 URI 列表
local-only-urns = [
    "GET:/oss/bucket/list",
    "GET:/oss/obj/list",
    "GET:/oss/obj-ref/list"
]

# IP 白名单（CIDR 格式）
ip-white-list = [
    "127.0.0.1/32",
    "192.168.31.0/24"
]

# IP 黑名单
ip-black-list = [
    "192.168.31.102/32"
]

# HTTPS 配置
[web-server.https]
enabled = false                        # 是否启用 HTTPS
cert = "certs/cert.pem"                # SSL 证书路径
key = "certs/key.pem"                  # SSL 私钥路径
```

**多地址绑定示例**：
```toml
bind = ["127.0.0.1", "192.168.1.100"]
# 或
listen = ["0.0.0.0:9840", "[::]:9840"]  # IPv4 + IPv6
```

#### ID Worker 配置段 `[id-worker]`

```toml
[id-worker]
epoch = 1758108749211                  # 纪元时间戳（毫秒）
machine-id = 1                         # 机器 ID（0-1023）
node-id = 1                            # 节点 ID（0-4095）
```

ID Worker 用于生成分布式唯一 ID（雪花算法），确保在多节点部署时 ID 不冲突。

### 环境变量

项目支持通过环境变量覆盖配置：

- `RUST_LOG`：日志级别（trace/debug/info/warn/error）
- `OSS_CONFIG_FILE`：默认配置文件路径
- `DATABASE_URL`：数据库连接字符串（优先级高于配置文件）

## 安装与部署

### 前置要求

- **Rust**：1.75+（2024 Edition）
- **数据库**：PostgreSQL 12+ 或 MySQL 8.0+
- **操作系统**：Linux/macOS/Windows

### 开发环境搭建

1. **克隆仓库**
   ```bash
   git clone https://github.com/rusthing/oss-rs.git
   cd oss-rs
   ```

2. **初始化数据库**
   
   创建数据库和用户：
   ```sql
   -- PostgreSQL
   CREATE USER ossrs WITH PASSWORD 'ossrs';
   CREATE DATABASE ossrs OWNER ossrs;
   
   -- MySQL
   CREATE USER 'ossrs'@'%' IDENTIFIED BY 'ossrs';
   CREATE DATABASE ossrs CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;
   GRANT ALL PRIVILEGES ON ossrs.* TO 'ossrs'@'%';
   ```
   
   数据库迁移会在服务启动时自动执行（通过 `db_migrate!` 宏）。

3. **配置服务**
   
   复制并编辑配置文件：
   ```bash
   cp oss-svr.toml.example oss-svr.toml
   vim oss-svr.toml
   ```

4. **编译运行**
   ```bash
   # 开发模式
   cargo run
   
   # 指定配置文件和端口
   cargo run -- -c ./custom-config.toml -p 8080
   
   # 发布模式（优化编译）
   cargo build --release
   ./target/release/oss-svr
   ```

### Docker 部署

项目根目录包含 `Dockerfile`，可以构建容器镜像：

```bash
# 构建镜像
docker build -t oss-svr:latest .

# 运行容器
docker run -d \
  --name oss-svr \
  -p 9840:9840 \
  -v $(pwd)/oss-svr.toml:/app/oss-svr.toml \
  -v uploads:/app/uploads \
  oss-svr:latest
```

或使用 Docker Compose（包含数据库）：

```yaml
version: '3.8'
services:
  oss-svr:
    build: .
    ports:
      - "9840:9840"
    volumes:
      - ./oss-svr.toml:/app/oss-svr.toml
      - uploads:/app/uploads
    depends_on:
      - postgres
  
  postgres:
    image: postgres:15-alpine
    environment:
      POSTGRES_USER: ossrs
      POSTGRES_PASSWORD: ossrs
      POSTGRES_DB: ossrs
    volumes:
      - pgdata:/var/lib/postgresql/data

volumes:
  uploads:
  pgdata:
```

### 生产环境部署

1. **交叉编译**（以 Linux ARM64 为例）
   ```bash
   rustup target add aarch64-unknown-linux-gnu
   cargo build --release --target aarch64-unknown-linux-gnu
   ```

2. **systemd 服务配置**
   
   创建 `/etc/systemd/system/oss-svr.service`：
   ```ini
   [Unit]
   Description=OSS Rust Service
   After=network.target postgresql.service
   
   [Service]
   Type=simple
   User=ossrs
   WorkingDirectory=/opt/oss-svr
   ExecStart=/opt/oss-svr/oss-svr -c /etc/oss-svr/config.toml
   Restart=on-failure
   RestartSec=5s
   
   [Install]
   WantedBy=multi-user.target
   ```

   启动服务：
   ```bash
   systemctl daemon-reload
   systemctl enable oss-svr
   systemctl start oss-svr
   ```

## 命令行参数

```
oss-svr v1.1.7 - 对象存储服务

AUTHOR: zbz

USAGE: oss-svr [OPTIONS]

OPTIONS:
  -c, --config-file <CONFIG_FILE>  配置文件的路径
  -p, --port <PORT>                Web服务器的端口号
  -s, --signal <SIGNAL>            信号指令 [default: start]
                                   可选值：start, restart, stop/s, kill/k
  -h, --help                       显示帮助信息
  -V, --version                    显示版本信息
```

### 信号管理

服务支持优雅的信号处理：

- **start**（默认）：检查是否已有实例运行，然后启动新实例
- **restart**：发送 SIGTERM 停止旧进程，然后启动新进程（零停机重启）
- **stop** / **s**：发送 SIGTERM 优雅停机
- **kill** / **k**：发送 SIGKILL 强制终止

示例：
```bash
# 优雅重启服务
oss-svr --signal restart

# 停止服务
oss-svr --signal stop
```

## 开发指南

### 代码规范

- 遵循 Rust 官方代码风格指南
- 使用 `cargo fmt` 格式化代码
- 使用 `cargo clippy` 检查代码质量
- 公共 API 需要编写文档注释

### 添加新功能

1. **定义数据模型**（`src/model/`）
   - 使用 SeaORM CLI 生成 Entity
   - 手动编写或迁移脚本

2. **创建 DTO/VO**（`src/dto/`, `src/vo/`）
   - DTO：服务层间数据传输
   - VO：API 响应格式化

3. **实现 Service**（`src/svc/`）
   - 编写业务逻辑
   - 处理事务和错误

4. **添加 Controller**（`src/web/ctrl/`）
   - 参数解析和验证
   - 调用 Service 层
   - 返回响应

5. **配置路由**（`src/web/router/`）
   - 注册路由规则
   - 设置中间件

6. **更新 API 文档**
   - 在 Controller 添加 Utoipa 注解
   - 测试 Swagger UI

### 测试

```bash
# 运行单元测试
cargo test

# 运行集成测试
cargo test --test '*'

# 生成测试覆盖率报告
cargo install cargo-tarpaulin
cargo tarpaulin --out Html
```

### 调试技巧

1. **启用详细日志**
   ```bash
   RUST_LOG=debug cargo run
   ```

2. **数据库查询日志**
   在配置文件中启用 SeaORM 的 `debug-print` feature（已启用）

3. **API 测试**
   使用 Swagger UI 或 curl：
   ```bash
   # 上传文件
   curl -X POST http://localhost:9840/oss/file/upload \
     -F "file=@test.pdf" \
     -F "bucket_id=1"
   
   # 下载文件
   curl -O http://localhost:9840/oss/file/download/123456
   
   # Range 请求（断点续传）
   curl -H "Range: bytes=0-1023" http://localhost:9840/oss/file/download/123456
   ```

## 性能优化建议

1. **数据库优化**
   - 为常用查询字段添加索引
   - 使用连接池（SeaORM 内置）
   - 定期清理过期数据

2. **文件存储**
   - 使用 SSD 存储上传文件
   - 考虑使用对象存储后端（如 MinIO、Ceph）
   - 实施文件去重策略

3. **网络优化**
   - 启用 Gzip/Brotli 压缩
   - 使用 CDN 加速静态资源
   - 配置合理的缓冲区大小

4. **并发处理**
   - Tokio 运行时自动管理线程池
   - 调整 `tokio` worker 线程数
   - 监控异步任务执行情况

## 监控与运维

### 日志管理

服务使用 `tracing` 框架进行结构化日志记录：

- 日志文件位置：根据 `log.toml` 配置
- 日志轮转：支持按大小和时间轮转
- 日志级别：trace < debug < info < warn < error

### 健康检查

```bash
curl http://localhost:9840/health
```

返回示例：
```json
{
  "status": "healthy",
  "timestamp": "2026-05-22T10:30:00Z",
  "database": "connected",
  "uptime_seconds": 3600
}
```

### 指标收集

未来版本计划集成 Prometheus 指标：
- HTTP 请求延迟分布
- 上传/下载吞吐量
- 数据库连接池状态
- 文件系统使用情况

## 常见问题

### Q: 如何更改文件存储路径？

A: 当前版本文件存储在 `uploads/` 目录（相对于工作目录）。如需自定义，修改 `oss_file_svc.rs` 中的路径逻辑。

### Q: 支持哪些文件格式？

A: 理论上支持任意文件格式。预览功能支持常见图片（JPG/PNG/GIF）、PDF、文本文件等。视频流媒体支持 MP4/WebM 格式。

### Q: 如何实现文件访问权限控制？

A: 当前版本通过 IP 白名单/黑名单和 URI 访问控制提供基础安全。细粒度的用户级权限控制可在后续版本中通过 JWT Token 或 OAuth2 实现。

### Q: 数据库迁移失败怎么办？

A: 检查：
1. 数据库连接配置是否正确
2. 数据库用户权限是否足够
3. 查看日志中的具体错误信息
4. 手动执行迁移脚本（位于 `migrations/` 目录）

### Q: 如何处理大文件上传？

A: 服务支持分片上传和流式处理：
1. 调整 `upload-file-limit-size` 配置
2. 增加 `upload-buffer-size` 提升性能
3. 前端使用分片上传库（如 Resumable.js）
4. 考虑使用专门的上传服务或 CDN

## 版本历史

- **v1.1.7**（当前版本）
  - 支持 MySQL 和 PostgreSQL 双数据库
  - 完善 Range 请求支持
  - 优化文件上传下载性能
  - 添加配置文件热重载

- **v1.0.x**
  - 初始版本发布
  - 基础 CRUD 功能
  - Swagger API 文档

## 许可证

本项目采用 MIT 许可证，详情请见 [LICENSE](../LICENSE) 文件。

## 贡献指南

欢迎提交 Issue 和 Pull Request！

1. Fork 本仓库
2. 创建特性分支（`git checkout -b feature/AmazingFeature`）
3. 提交更改（`git commit -m 'Add some AmazingFeature'`）
4. 推送到分支（`git push origin feature/AmazingFeature`）
5. 开启 Pull Request

## 联系方式

- 项目主页：https://github.com/rusthing/oss-rs
- 问题反馈：https://github.com/rusthing/oss-rs/issues
- 作者：zbz