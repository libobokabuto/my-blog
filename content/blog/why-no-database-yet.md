---
title: 为什么第一版不急着上数据库
summary: 内容站的第一性问题通常不是数据模型多复杂，而是内容如何组织、页面如何呈现、写作和发布路径是否顺畅。
date: 2026-06-18
series: building-content-site
reading_minutes: 4
tags:
  - Rust
  - Architecture
  - Product
related:
  - docs-and-code-in-one-repo
---

第一版博客的关键，不是“我能不能很快接一套数据库”，而是“我有没有先把真正要长期维护的链路跑通”。

对现在这个项目来说，最重要的是：

- 正式的 Leptos SSR 项目骨架
- Markdown 内容读取与渲染
- 首页、博客列表、博客详情和关于页
- 后续能继续扩 notes 和 projects 的稳定结构

数据库当然以后可能会需要，但那应该发生在内容组织已经跑顺、页面结构已经稳定之后，而不是作为第一阶段的心理安全感来源。

如果一开始就把复杂度压上来，项目会更容易变成“技术栈堆叠练习”，而不是一个真正会持续更新的网站。
