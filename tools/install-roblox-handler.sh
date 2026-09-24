#!/usr/bin/env bash
set -euo pipefail

ROBLOX=${1:-./roblox}
ROBLOX=$(readlink -f -- "$ROBLOX")

if [[ ! -x "$ROBLOX" ]]; then
  echo "roblox executable not found or not executable: $ROBLOX" >&2
  exit 1
fi

DATA_HOME=${XDG_DATA_HOME:-"$HOME/.local/share"}
APP_DIR="$DATA_HOME/applications"
DESKTOP="$APP_DIR/roblox.desktop"
mkdir -p "$APP_DIR"

# Desktop Entry Exec fields use their own quoting rules.
escape_exec() {
  local s=$1
  s=${s//\\/\\\\}
  s=${s//"/\\"}
  s=${s// /\\ }
  s=${s//$'\t'/\\t}
  printf "%s" "$s"
}
EXEC_PATH=$(escape_exec "$ROBLOX")

cat > "$DESKTOP" <<EOF
[Desktop Entry]
Type=Application
Name=Roblox
Comment=Launch Roblox through Cordial
Exec=$EXEC_PATH %u
Terminal=false
NoDisplay=true
MimeType=x-scheme-handler/roblox;x-scheme-handler/roblox-player;
EOF

xdg-mime default roblox.desktop x-scheme-handler/roblox
xdg-mime default roblox.desktop x-scheme-handler/roblox-player
update-desktop-database "$APP_DIR" >/dev/null 2>&1 || true

echo "Registered roblox:// and roblox-player: with $ROBLOX"
