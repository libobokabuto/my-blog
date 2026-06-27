# my-blog

一个以内容优先为原则构建的个人网站项目，包含首页、博客、笔记、项目、标签、归档、搜索与个人主页。

这个仓库既是站点本身，也是我用 `Rust`、`Leptos SSR`、`Axum` 与 `Markdown` 持续推进的全栈实践项目。它不是单纯的仓库导航页，也不是只展示技术栈的个人主页，而是一个真正面向公开浏览的内容产品。

## 当前画面

### 首页

![my-blog 首页浅色版](static/images/projects/my-blog/home-light.jpeg)

### 项目页

![my-blog 项目列表浅色版](static/images/projects/my-blog/projects-light.jpeg)

## 当前版本

当前版本已经具备这些对外可见的核心页面：

- 首页：内容入口与浏览分流
- 主页：个人状态、热力图与常用入口
- 博客：正式文章列表与详情
- 笔记：按技术板块整理的过程记录
- 项目：长期作品的列表与详情展示
- 标签 / 归档 / 搜索：继续查找内容的辅助入口

站点内容继续以仓库内的 `Markdown` 为正式来源，后台链路则已经接入搜索索引、统计快照、任务记录与部署所需的基础设施。

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

站点内容继续按三类目录维护：

- `content/blog/*.md`：博客文章
- `content/notes/*.md`：学习笔记
- `content/projects/*.md`：项目介绍

当前版本继续坚持“Markdown 写内容，数据库存索引和运行记录”的方案，不把内容编辑切到数据库里。

## 目录结构

```text
my-blog/
├─ app/                  # 共享 UI、路由、页面组件、内容装配逻辑
├─ client/               # 浏览器端 hydrate 入口
├─ server/               # Axum + Leptos SSR 服务端入口
├─ style/                # 全站样式入口
├─ content/              # Markdown 内容源
├─ static/               # 静态资源与项目截图
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

推荐在仓库根目录放一个不提交到 Git 的 `.env.local`，内容可参考 `.env.example`：

```powershell
SITE_URL=http://127.0.0.1:3000
BLOG_DATABASE_URL=mysql://root:your-password@127.0.0.1:3306/my-blog
BLOG_REDIS_URL=redis://127.0.0.1:6379/
RUST_LOG=server=info,tower_http=info
```

然后直接启动：

```powershell
.\scripts\start-local.ps1
```

脚本会自动做这些事：

- 设置 PowerShell UTF-8 输出编码
- 读取 `.env.local` 或 `.env`
- 自动停止旧的 `target\debug\server.exe`
- 执行 `cargo run -p server --bin server`

如果暂时不想放 `.env.local`，也可以临时传参启动：

```powershell
.\scripts\start-local.ps1 -DatabaseUrl 'mysql://root:你的密码@127.0.0.1:3306/my-blog' -RedisUrl 'redis://127.0.0.1:6379/' -SiteUrl 'http://127.0.0.1:3000'
```

只检查环境，不真正启动服务：

```powershell
.\scripts\start-local.ps1 -CheckOnly
```

停止本地服务：

```powershell
.\scripts\stop-local.ps1
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

如果准备把它放到服务器上，优先看 `deploy/v1.0-部署说明.md`。

## 文档入口

如果想快速理解当前版本，建议按这个顺序阅读：

1. `README.md`
2. `PRDS/第五版/产品需求.md`
3. `PRDS/第五版/开发计划.md`
4. `PRDS/第五版/VERSION_NOTES.md`
5. `PRDS/第五版/v1.0-后续演进清单.md`
6. `deploy/v1.0-部署说明.md`
