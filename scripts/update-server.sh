#!/usr/bin/env bash
# ============================================================
# my-blog 一键更新脚本（服务器端）
#
# 用法：
#   首次：先把脚本拉到服务器上
#       cd /srv/my-blog && git pull
#   之后每次更新，在服务器上运行这一条命令即可：
#       cd /srv/my-blog && bash scripts/update-server.sh
#
# 脚本做的事：
#   1. git pull --ff-only 拉取最新代码（无更新则直接退出，不浪费编译时间）
#   2. 编译（编译期间旧版服务继续运行，不停服）
#   3. 编译成功后才重启 systemd 服务（仅中断 1~2 秒）
#   4. 确认服务已恢复
#
# 可配置项见下方"可配置项"，默认值对应仓库 deploy/ 下的示例文件。
# ============================================================
set -euo pipefail

# ---------- 可配置项（按你的服务器实际情况修改） ----------
PROJECT_DIR="${MY_BLOG_DIR:-/srv/my-blog}"      # 项目根目录（服务器上的实际路径）
SERVICE_NAME="${MY_BLOG_SERVICE:-my-blog}"      # systemd 服务名
BUILD_LOG="/tmp/my-blog-build.log"              # 编译日志路径
# 编译命令自动选择：服务器装了 cargo-leptos 就用它（产物完整，含 wasm/static），
# 否则退回部署文档 v1.0 里的 cargo build 方式。
if command -v cargo-leptos >/dev/null 2>&1; then
  BUILD_CMD=(cargo leptos build --release)
else
  BUILD_CMD=(cargo build -p server --bin server --release)
fi
# -----------------------------------------------------------

cd "$PROJECT_DIR"

log() { echo "==> [$(date '+%F %T')] $*"; }
die() { echo "!! $*" >&2; exit 1; }

log "开始更新：$PROJECT_DIR"

# 1) 拉取最新代码
OLD_COMMIT=$(git rev-parse --short HEAD)
log "git pull 拉取最新代码"
if ! git pull --ff-only; then
  die "git pull 失败。常见原因：服务器上有未提交改动（先运行 git status 查看/处理），或网络/认证问题。"
fi
NEW_COMMIT=$(git rev-parse --short HEAD)

if [ "$OLD_COMMIT" = "$NEW_COMMIT" ]; then
  log "代码没有更新（仍为 $NEW_COMMIT），跳过编译与重启"
  exit 0
fi
log "更新内容：$OLD_COMMIT -> $NEW_COMMIT"

# 2) 编译（最耗时；编译期间旧版服务不停止）
log "开始编译：${BUILD_CMD[*]}"
START=$(date +%s)
if ! "${BUILD_CMD[@]}" 2>&1 | tee "$BUILD_LOG"; then
  die "编译失败，服务未重启（旧版本继续运行）。日志：$BUILD_LOG"
fi
END=$(date +%s)
log "编译完成，耗时 $((END - START)) 秒"

# 3) 重启服务（只中断 1~2 秒）
log "重启服务 $SERVICE_NAME"
sudo systemctl restart "$SERVICE_NAME"

# 4) 确认服务恢复
sleep 2
if systemctl is-active --quiet "$SERVICE_NAME"; then
  log "服务已恢复 ✅  版本 $NEW_COMMIT 更新完成"
else
  die "服务未正常运行！排查：journalctl -u $SERVICE_NAME -n 50"
fi
