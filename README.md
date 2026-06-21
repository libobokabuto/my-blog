# my-blog

一个以内容为核心、使用全栈 Rust 正式实现的个人网站项目。

这个仓库既是我的博客站点，也是我用 `Leptos`、`Leptos SSR`、`Axum` 和 `Markdown` 持续推进的 Rust 实战项目。它更偏向“可长期维护的内容站”，而不是一开始就做成带后台、带数据库、带复杂交互的大型系统。

## 项目定位

- 用 Rust 贯通页面渲染、服务端输出和内容组织
- 先把博客主链路做扎实，再扩展更多内容类型
- 保留设计参考稿，让产品规划、视觉参考和正式代码并行演进

## 技术栈

- `Rust`
- `Leptos`
- `Leptos SSR`
- `Axum`
- `Markdown`
- `pulldown-cmark`
- `CSS`

## 当前已实现

- 正式的 `Leptos SSR` 项目骨架
- 首页 `/`
- 博客列表页 `/blog`
- 博客详情页 `/blog/:slug`
- 笔记页 `/notes`
- 项目页 `/projects`
- 关于页 `/about`
- 基于本地 `Markdown` 的博客内容读取与渲染
- 基于本地 `Markdown` 的笔记与项目内容读取
- 博客详情页上一篇 / 下一篇导航
- `rss.xml`
- `sitemap.xml`

## 当前暂不实现

- 数据库
- 评论系统
- 站内搜索
- 后台管理

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
├─ deploy/               # 部署相关配置
├─ Cargo.toml            # Rust workspace 配置
└─ README.md
```

## 内容组织

- `content/blog`：正式博客文章
- `content/notes`：后续笔记内容预留
- `content/projects`：后续项目展示内容预留

当前 `blog`、`notes` 和 `projects` 都已经接进正式页面系统，其中 `blog` 是最完整的主内容链路，`notes` 与 `projects` 是第一版正式补齐的轻量内容页。

## 设计参考

`static/demo-v1.html`、`static/css/demo-v1.css`、`static/js/demo-v1.js` 作为第一版视觉与信息结构参考稿继续保留。

它们现在的职责是“设计参考”，不是正式运行时代码。

## 本地运行

先确认本机已经具备：

- Rust 工具链：`stable-x86_64-pc-windows-msvc`
- `cargo-leptos`
- Visual Studio C++ Build Tools
- `wasm32-unknown-unknown` target

开发模式运行：

```powershell
cargo leptos watch
```

默认访问地址：

```text
http://127.0.0.1:3000
```

生产构建：

```powershell
cargo leptos build --release
```

## 适合从哪里开始看

如果你是来学习这个项目，可以按这个顺序阅读：

1. `README.md`
2. `PRDS/第一版/产品需求.md`
3. `PRDS/第一版/开发计划.md`
4. `app/src/lib.rs`
5. `app/src/content.rs`
6. `content/blog/*.md`

## 版本与迭代记录

项目迭代记录不放在总 `README` 里，统一放在：

- `PRDS/第一版/VERSION_NOTES.md`

如果你想看当前完成范围、第一版待补项和阶段性变化，直接看那份文档会更清楚。
