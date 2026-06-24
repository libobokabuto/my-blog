# 第五版版本记录
## v5-planning

日期：2026-06-24

### 当前状态
- 第五版已正式建立文档边界
- 当前版本定位为第四版骨架的最终收尾版
- 当前重点从“继续扩能力”切换为“跑通闭环、可部署上线、补齐文档”

### 本轮目标
1. 服务端真实运行
2. MySQL 与 Redis 真实链路
3. 搜索重建真实写入
4. 统计、任务、同步最小闭环
5. 部署文档与 `v1.0` 收尾说明

### 本轮明确不做
1. 完整用户系统
2. 多角色权限
3. 评论社区
4. 复杂开放平台
5. 独立 worker 与复杂调度系统

### 建议下一步
1. 先让服务进程正确读取 `BLOG_DATABASE_URL` 与 `BLOG_REDIS_URL`
2. 再跑通 `/admin/search`、`/admin/stats`、`/admin/tasks`、`/admin/sync`
3. 然后回写第五版验证记录和部署说明

## v5-validation

日期：2026-06-24

### 当前状态
- 第五版已完成本地真实链路验证
- 当前版本已进入 `v1.0 可部署` 判定阶段

### 本次完成
1. 清理了被 `x86_64-pc-windows-gnullvm` / `x86_64-pc-windows-msvc` 混用污染的 `target/` 构建产物
2. 重新完成 `cargo check`
3. 成功启动服务端并确认监听 `127.0.0.1:3000`
4. 为服务进程正确注入：
   - `SITE_URL`
   - `BLOG_DATABASE_URL`
   - `BLOG_REDIS_URL`
5. 验证 MySQL `my-blog` 库已可连接
6. 验证 Redis 已可连接
7. 执行了首轮真实搜索重建
8. 执行了首轮真实同步运行
9. 刷新了统计与任务后台，生成正式记录
10. 新增 `.env.example`、部署说明和反向代理 / `systemd` 示例配置

### 当前验证结果
1. `search_documents`：9 条记录
2. `search_rebuild_runs`：1 条成功记录
3. `stats_snapshots`：4 条快照
4. `task_runs`：2 条成功记录
5. `sync_runs`：1 条成功记录
6. `sync_sources`：3 条正式同步源
7. Redis 已写入：
   - `my-blog:search:index:status`
   - `my-blog:search:index:trigger`
   - `my-blog:search:index:message`
   - `my-blog:search:index:document-count`
   - `my-blog:search:index:updated-at`

### 本次明确没做
1. 没有引入完整用户系统
2. 没有引入评论系统
3. 没有引入多角色权限
4. 没有引入队列与独立 worker
5. 没有引入真实第三方 OAuth / 开放平台

### 当前判断
- 第五版已经达到“本地完成真实验证、具备服务器部署条件”的标准
- 当前项目可按第五版部署文档进入 `v1.0` 部署准备阶段

## v1.0-release-docs

日期：2026-06-24

### 当前状态
- 已补充 `v1.0` 后续演进清单
- 已将站点公开文案和 README 统一切换到 `v1.0` 口径
- 已清理站内残留的“第三版 / 第四版 / 骨架 / 占位”类说明性填充文案

### 本次完成
1. 新增 `PRDS/第五版/v1.0-后续演进清单.md`
2. 重写 `README.md`，突出 `v1.0` 已完成能力、部署资料和内容目录规则
3. 更新 `content/projects/my-blog.md`，补齐第四版与第五版时间线
4. 清理首页、`/me`、`/about`、`/notes`、`/archive` 与后台页面的旧版本表述
5. 保留 `v1.0` 范围边界说明，但移除开发阶段的占位化叙述

### 当前判断
- 当前仓库已经更适合以 `v1.0` 可部署版本对外说明
- 后续新增功能与内容建设应优先参考 `PRDS/第五版/v1.0-后续演进清单.md`
