# axum-starter

## 目录结构（分层）

```
src/
  config/               # 配置加载
  core/                 # 核心能力（日志、错误、抽取器、JWT、中间件等）
  infra/                # 基础设施（数据库、服务启动、链路追踪）
  entity/               # 数据库实体
  modules/              # 业务模块（handler/service/repo）
    auth/
    user/
    router.rs           # 路由聚合
```

1. clone the repository

```bash
git clone https://github.com/1IANZ/axum-starter.git

```

2. install sea-orm-cli (optional)

```bash
cargo install sea-orm-cli
```

3. create `.env` file

```bash
DATABASE_URL=postgres://user:password@localhost:5432/db_name
```

4. use `sea-orm-cli` to generate entity

```bash
   sea-orm-cli generate entity -s demo --with-serde both --model-extra-attributes 'serde(rename_all ="camelCase")' --date-time-crate chrono  -o ./src/entity
```

5. run the server

```bash
   cargo run
```
