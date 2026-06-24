---
title: "my-blog"
summary: "一个以内容为核心、用 Leptos SSR、MySQL、Redis 和 Markdown 落地的个人博客项目。"
status: "v1.0 可部署"
background: "希望把这个仓库从可浏览的个人内容档案推进成可长期维护、可真实部署的个人内容系统。"
role: "产品设计、内容建模、前后端实现"
timeline:
  - "第一版：完成站点骨架、博客主链路与正式视觉基调"
  - "第二版：补齐 notes / projects 详情、标签归档与搜索"
  - "第三版：推进个人主页、内容组织、关联、搜索与治理"
  - "第四版：建立内容后台、搜索索引、统计、任务与同步的后端骨架"
  - "第五版：完成 MySQL / Redis 真实链路并收束为 v1.0 可部署版本"
outcomes:
  - "已建立 Leptos SSR + Markdown 的正式实现路径"
  - "已形成首页、个人主页、标签总览、归档与搜索入口"
  - "已跑通搜索重建、统计快照、任务记录、同步记录的真实写入闭环"
retrospective:
  - "先做内容驱动结构，再逐步接入后端链路，更适合个人项目稳步演进"
  - "文档先行能显著减少返工和范围漂移"
stack:
  - Rust
  - Leptos
  - Leptos SSR
  - Axum
  - Markdown
repo_url:
live_url:
---

当前版本优先把博客、笔记、项目展示、后台治理与部署链路做稳，后续再按需要继续补功能和内容。
