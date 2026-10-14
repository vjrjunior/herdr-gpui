#!/usr/bin/env bash
set -euo pipefail

usage() {
    cat <<'EOF'
Usage: cache.sh [--trim]

Reports how much disk this checkout's Cargo build cache takes and how much
is free, flagging a disk too full for the gates. Other checkouts' caches are
listed but never touched, and neither are ~/.cargo or the installed app.

  --trim   Remove the incremental caches, then run `cargo clean` when the
           build directory is still over budget or the disk is short of room

Environment:
  CARGO_TARGET_DIR          Build directory (default: target)
  ORBITA_TARGET_BUDGET_GB   Largest build directory --trim keeps (default: 60)
  ORBITA_MIN_FREE_GB        Free space a sync needs (default: 40)
EOF
}

trim=0
for arg in "$@"; do
    case "$arg" in
        --trim) trim=1 ;;
        -h|--help) usage; exit 0 ;;
        *) echo "Unknown option: $arg" >&2; usage >&2; exit 2 ;;
    esac
done

cd "$(git rev-parse --show-toplevel)"
target="${CARGO_TARGET_DIR:-target}"
budget=$(( ${ORBITA_TARGET_BUDGET_GB:-60} * 1048576 ))
min_free=$(( ${ORBITA_MIN_FREE_GB:-40} * 1048576 ))

size_kb() {
    if [ -e "$1" ]; then
        { du -sk "$1" 2>/dev/null || true; } | awk 'NR == 1 { print $1 } END { if (NR == 0) print 0 }'
    else
        echo 0
    fi
}

free_kb() {
    df -Pk . | awk 'NR == 2 { print $4 }'
}

human() {
    awk -v k="$1" 'BEGIN {
        if (k >= 1048576) printf "%.1fG", k / 1048576
        else if (k >= 1024) printf "%.0fM", k / 1024
        else printf "%dK", k
    }'
}

report() {
    local size free parts=() part
    size=$(size_kb "$target")
    for part in debug/incremental debug/deps release; do
        if [ -e "$target/$part" ]; then
            parts+=("$part $(human "$(size_kb "$target/$part")")")
        fi
    done
    printf '%-18s %s' "$target/" "$(human "$size")"
    if [ "${#parts[@]}" -gt 0 ]; then
        local joined
        joined=$(printf '%s, ' "${parts[@]}")
        printf ' (%s)' "${joined%, }"
    fi
    if [ "$size" -gt "$budget" ]; then
        printf '  OVER the %s budget' "$(human "$budget")"
    fi
    echo
    free=$(free_kb)
    printf '%-18s %s' "free disk" "$(human "$free")"
    if [ "$free" -lt "$min_free" ]; then
        printf '  LOW: under %s; trim before the rebase' "$(human "$min_free")"
    fi
    echo
    local here other
    here=$(pwd -P)
    git worktree list --porcelain | sed -n 's/^worktree //p' | while IFS= read -r other; do
        if [ "$(cd "$other" 2>/dev/null && pwd -P)" = "$here" ] || [ ! -d "$other/target" ]; then
            continue
        fi
        printf '%-18s %s  (another checkout; not touched)\n' "$other/target" "$(human "$(size_kb "$other/target")")"
    done
}

if [ "$trim" = 0 ]; then
    report
    exit 0
fi

before=$(size_kb "$target")
rm -rf "$target/debug/incremental" "$target/release/incremental"
if [ "$(size_kb "$target")" -gt "$budget" ] || [ "$(free_kb)" -lt "$min_free" ]; then
    cargo clean --quiet
    cleaned="cargo clean"
else
    cleaned="incremental caches only"
fi
after=$(size_kb "$target")
echo "trimmed $target/ ($cleaned): $(human "$before") -> $(human "$after"), freed $(human $((before - after)))"
report
