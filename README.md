# my-blog

一个以内容为核心、使用全栈 Rust 构建的个人博客项目，当前已经收束到第五版 `v1.0`，目标是稳定、可运行、可部署。

这个仓库既是博客站点本身，也是我用 `Leptos SSR`、`Axum`、`MySQL`、`Redis` 和 `Markdown` 持续推进的 Rust 实战项目。`v1.0` 的重点不是继续扩张功能，而是把已经进入代码结构的后端能力真正跑通，整理成适合本地维护和服务器部署的成品版本。

## v1.0 已完成什么

`v1.0` 当前已经具备这些核心能力：

- 公开站点：`/`、`/me`、`/blog`、`/notes`、`/projects`、`/tags`、`/archive`、`/about`、`/search`
- 后台入口：`/admin`、`/admin/content`、`/admin/search`、`/admin/stats`、`/admin/tasks`、`/admin/sync`
- 内容来源：继续使用仓库内 `Markdown` 作为正式内容源
- 搜索链路：支持真实搜索索引重建，并将索引文档与重建记录写入 MySQL
- 统计链路：支持最小可用的统计快照写入与后台展示
- 任务链路：支持搜索重建、同步等服务端任务记录写入与展示
- 同步链路：支持最小可用的同步源登记与同步运行记录写入
- 运行态存储：Redis 用于记录搜索索引状态与轻量运行信息
- 站点输出：支持 `rss.xml` 和 `sitemap.xml`
- 部署资料：已补齐 `.env.example`、部署说明、Nginx 示例配置、`systemd` 示例配置

## v1.0 刻意不做什么

以下内容明确被留在 `v1.0` 之外：

- 完整用户系统
- 多角色权限与复杂 RBAC
- 评论社区
- 开放平台 API
- 复杂工作流编排
- 独立 worker / 队列系统
- 大规模视觉重设计

## 技术栈

- `Rust`
- `Leptos`
- `Leptos SSR`
- `Axum`
- `MySQL`
- `Redis`
- `Markdown`
- `pulldown-cmark`
- `CSS`

## 内容放在哪里

站点内容继续按照三类目录维护：

- `content/blog/*.md`：博客文章
- `content/notes/*.md`：学习笔记
- `content/projects/*.md`：项目记录

`v1.0` 继续坚持“Markdown 写内容，数据库存索引和运行记录”的方案，不把内容编辑切到数据库。

## 目录结构

```text
my-blog/
├─ app/                  # 共享 UI、路由、页面组件、内容装配逻辑
├─ client/               # 浏览器端 hydrate 入口
├─ server/               # Axum + Leptos SSR 服务端入口
├─ style/                # 全站样式入口
├─ content/              # Markdown 内容源
├─ static/               # 静态资源与视觉参考稿
├─ PRDS/                 # 产品需求、开发计划、版本记录
├─ LEARNING/             # Rust 学习记录
├─ scripts/              # 辅助脚本
├─ deploy/               # 部署说明与示例配置
├─ Cargo.toml            # Rust workspace 配置
└─ README.md
```

## 本地运行

先准备好这些基础环境：

- Rust 工具链：`stable-x86_64-pc-windows-msvc`
- Visual Studio C++ Build Tools
- MySQL
- Redis

然后设置环境变量：

```powershell
$env:SITE_URL='http://127.0.0.1:3000'
$env:BLOG_DATABASE_URL='mysql://root:your-password@127.0.0.1:3306/my-blog'
$env:BLOG_REDIS_URL='redis://127.0.0.1:6379/'
$env:RUST_LOG='server=info,tower_http=info'
```

启动开发服务：

```powershell
cargo run -p server --bin server
```

构建发布版本：

```powershell
cargo build -p server --bin server --release
```

## 部署相关

部署资料已经整理到 `deploy/`：

- `deploy/v1.0-部署说明.md`
- `deploy/nginx.my-blog.conf.example`
- `deploy/my-blog.service.example`
- `.env.example`

如果你准备把它放到服务器上，优先看 `deploy/v1.0-部署说明.md`。

## 文档入口

如果你想快速理解当前版本，建议按这个顺序阅读：

1. `README.md`
2. `PRDS/第五版/产品需求.md`
3. `PRDS/第五版/开发计划.md`
4. `PRDS/第五版/VERSION_NOTES.md`
5. `PRDS/第五版/v1.0-后续演进清单.md`
6. `deploy/v1.0-部署说明.md`

## 后续演进

`v1.0` 之后准备继续做的事情，已经单独整理到：

- `PRDS/第五版/v1.0-后续演进清单.md`

它把后续功能、内容建设、运维增强，以及明确不属于 `v1.0` 的事项拆开列清楚，方便继续推进时不打乱当前可部署版本。
