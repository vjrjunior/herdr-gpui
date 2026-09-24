#!/usr/bin/env bash
set -euo pipefail

usage() {
    cat <<'EOF'
Usage: scripts/fork/update.sh [--no-sync] [--skip-ci] [--push]

Syncs main with upstream, rebases the fork branch onto it, verifies,
builds the macOS bundle, and installs it as a separate app.

  --no-sync   Build and install the current branch without fetching or rebasing
  --skip-ci   Skip `just ci` before building
  --push      Push main and the fork branch to origin after a successful build

Environment:
  FORK_BRANCH     Branch carrying the fork's changes (default: vjr)
  FORK_APP_NAME   Installed app name (default: Orbita)
  FORK_BUNDLE_ID  Installed bundle identifier (default: dev.vjrjunior.orbita)
  FORK_APPS_DIR   Install directory (default: /Applications)
EOF
}

sync=1
ci=1
push=0
for arg in "$@"; do
    case "$arg" in
        --no-sync) sync=0 ;;
        --skip-ci) ci=0 ;;
        --push) push=1 ;;
        -h|--help) usage; exit 0 ;;
        *) echo "Unknown option: $arg" >&2; usage >&2; exit 2 ;;
    esac
done

branch="${FORK_BRANCH:-vjr}"
app_name="${FORK_APP_NAME:-Orbita}"
bundle_id="${FORK_BUNDLE_ID:-dev.vjrjunior.orbita}"
apps_dir="${FORK_APPS_DIR:-/Applications}"

test "$(uname -s)" = Darwin || { echo "macOS only" >&2; exit 1; }
cd "$(git rev-parse --show-toplevel)"

if [ -n "$(git status --porcelain --untracked-files=no)" ]; then
    echo "Working tree has uncommitted changes; commit or stash them first." >&2
    exit 1
fi

if [ "$sync" = 1 ]; then
    git fetch upstream
    git fetch origin
    git switch main
    git merge --ff-only upstream/main
    git switch "$branch"
    if ! git rebase main; then
        echo >&2
        echo "Rebase stopped on a conflict. Resolve it, run 'git rebase --continue'," >&2
        echo "then rerun this script with --no-sync." >&2
        exit 1
    fi
else
    git switch "$branch"
fi

if [ "$ci" = 1 ]; then
    just ci
fi

HERDR_APP_NAME="$app_name" just bundle

built=target/release/Herdr.app
plist="$built/Contents/Info.plist"
plutil -replace CFBundleIdentifier -string "$bundle_id" "$plist"
plutil -replace CFBundleName -string "$app_name" "$plist"
plutil -replace CFBundleDisplayName -string "$app_name" "$plist"
plutil -lint "$plist"
codesign --force --deep --sign - "$built"

target="$apps_dir/$app_name.app"
if pgrep -f "$target/Contents/MacOS/Herdr" >/dev/null; then
    echo "Note: $app_name is running; restart it to use the new build."
fi
rm -rf "$target"
ditto "$built" "$target"
echo "Installed $target ($(git rev-parse --short HEAD) on $branch)"

if [ "$push" = 1 ]; then
    git push origin main
    git push --force-with-lease origin "$branch"
fi
