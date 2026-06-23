# 内容模型与治理 PRD

## 功能定位

这个模块用于提升第三版内容字段密度，并建立发布前校验能力。

## 当前状态

状态：已在第一批正式实现中完成

## 第三版目标

- 增强 `blog / notes / projects` front matter 表达能力
- 支持系列、阅读时长、阶段、来源、时间线等字段
- 建立发布前校验流程

## 当前实现范围

本轮优先实现：

1. 扩展三类内容的 front matter 字段
2. 新增系列页 `/series/:slug`
3. 增加内容校验脚本，检查字段、slug、坏链接

## 本轮补充约束

- 不引入数据库
- 不引入后台编辑器
- 校验以本地内容仓库为中心

## 当前实现结果

- 已完成三类内容 front matter 字段增强
- 已完成系列页 `/series/:slug`
- 已完成内容校验二进制入口 `cargo run -p server --bin validate_content`
- 当前校验逻辑已覆盖 front matter、slug、坏链接和关键缺失项
