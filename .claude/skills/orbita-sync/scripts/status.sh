#!/usr/bin/env bash
set -euo pipefail

usage() {
    cat <<'EOF'
Usage: status.sh [--no-fetch]

Read-only picture of the fork against upstream: what upstream added, which
files would conflict, which fork commits would stop a rebase, and where the
local clone stands. Changes nothing but the remote-tracking refs it fetches.

  --no-fetch   Use the remote-tracking refs already present

Environment:
  FORK_BRANCH   Branch carrying the fork's changes (default: vjr)
EOF
}

fetch=1
for arg in "$@"; do
    case "$arg" in
        --no-fetch) fetch=0 ;;
        -h|--help) usage; exit 0 ;;
        *) echo "Unknown option: $arg" >&2; usage >&2; exit 2 ;;
    esac
done

branch="${FORK_BRANCH:-vjr}"
cd "$(git rev-parse --show-toplevel)"

if [ "$fetch" = 1 ]; then
    git fetch upstream --prune --quiet
    git fetch origin --quiet
fi

base=$(git merge-base "$branch" upstream/main)
behind=$(git rev-list --count "$branch"..upstream/main)
changes=$(git rev-list --count --no-merges "$branch"..upstream/main)
ahead=$(git rev-list --count upstream/main.."$branch")

echo "== State"
echo "upstream/main  $(git log -1 --format='%h %cd %s' --date=short upstream/main)"
echo "fork base      $(git log -1 --format='%h %cd %s' --date=short "$base")"
echo "$branch is $behind behind upstream/main ($changes without merges) and $ahead ahead"
echo "current branch $(git symbolic-ref --quiet --short HEAD || echo detached)"
dirty=$(git status --short --untracked-files=no)
if [ -n "$dirty" ]; then
    echo "working tree   DIRTY (scripts/fork/update.sh will refuse to run):"
    printf '%s\n' "$dirty" | sed 's/^/     /'
else
    echo "working tree   clean (untracked files are not checked)"
fi
for ref in main "$branch"; do
    if git rev-parse --quiet --verify "origin/$ref" >/dev/null; then
        git rev-list --left-right --count "origin/$ref...$ref" |
            awk -v ref="origin/$ref" '{ printf "%-14s %s commits only on origin, %s only local\n", ref, $1, $2 }'
    fi
done

if [ "$behind" = 0 ]; then
    echo
    echo "UP TO DATE: upstream/main has nothing the fork lacks."
    exit 0
fi

echo
echo "== Upstream changes ($changes, newest first): sha date files +added -removed | subject"
git log --no-merges --format='@%h %ad|%s' --date=short --shortstat "$branch"..upstream/main |
    awk '
        function flush() { if (line != "") print line (big ? " [large]" : "") " | " subject }
        /^@/ { flush(); split(substr($0, 2), parts, "|"); line = parts[1]; subject = substr($0, length(parts[1]) + 3); big = 0; next }
        /file/ { files = $1; added = 0; removed = 0
                 for (i = 1; i <= NF; i++) { if ($i ~ /insertion/) added = $(i-1); if ($i ~ /deletion/) removed = $(i-1) }
                 big = (added + removed > 10000)
                 line = line " " files "f +" added " -" removed }
        END { flush() }
    '

echo
echo "== By type"
git log --no-merges --format='%s' "$branch"..upstream/main |
    awk '
        { type = "other"
          if (match($0, /^[a-z]+(\([^)]*\))?!?:/)) { type = $0; sub(/[(!:].*/, "", type) }
          count[type]++ }
        END { for (type in count) printf "%4d %s\n", count[type], type }
    ' | sort -rn

echo
echo "== Total (commits marked [large] are mostly moved code)"
git diff --shortstat "$base" upstream/main

echo
echo "== Upstream commits that touch the rules, gates, or build"
git log --no-merges --format='%h %s' "$branch"..upstream/main -- \
    AGENTS.md CLAUDE.md justfile scripts .github/workflows rust-toolchain.toml Cargo.toml | sed 's/^$/none/'

echo
echo "== Config keys and defaults upstream changed (config-gpui.example.toml)"
example=$(git diff --unified=0 "$base" upstream/main -- crates/herdr-gpui/config-gpui.example.toml |
    grep -E '^[+-][^+-]' || true)
if [ -z "$example" ]; then
    echo "none"
else
    printf '%s\n' "$example" | head -80
    lines=$(printf '%s\n' "$example" | wc -l | tr -d ' ')
    if [ "$lines" -gt 80 ]; then
        echo "... $((lines - 80)) more lines"
    fi
fi

echo
echo "== Simulated merge of upstream/main into $branch: conflict hunks per file"
merged=$(git merge-tree --write-tree --name-only --no-messages "$branch" upstream/main || true)
tree=$(printf '%s\n' "$merged" | head -1)
conflicts=$(printf '%s\n' "$merged" | tail -n +2)
if [ -z "$conflicts" ]; then
    echo "none"
else
    printf '%s\n' "$conflicts" | while IFS= read -r file; do
        hunks=$(git show "$tree:$file" 2>/dev/null | grep -c '^<<<<<<<' || true)
        printf '%3s  %s\n' "$hunks" "$file"
    done
fi

if [ -n "$conflicts" ]; then
    echo
    echo "== Commits behind each conflicting file"
    printf '%s\n' "$conflicts" | while IFS= read -r file; do
        echo "$file"
        git log --no-merges --format='     upstream %h %s' "$branch"..upstream/main -- "$file"
        git log --format='     fork     %h %s' "$base..$branch" -- "$file"
    done
fi

echo
echo "== Changed on both sides but merging cleanly (where breaks hide)"
both=$(comm -12 <(git diff --name-only "$base" upstream/main | sort) <(git diff --name-only "$base" "$branch" | sort))
clean=$(comm -23 <(printf '%s\n' "$both") <(printf '%s\n' "$conflicts" | sort))
if [ -z "$clean" ]; then
    echo "none"
else
    printf '%s\n' "$clean"
fi

echo
echo "== Rebase preview (approximate): fork commits that would stop, oldest first"
current=upstream/main
stops=0
while IFS= read -r commit; do
    status=0
    result=$(git merge-tree --write-tree --name-only --no-messages --merge-base="$commit^" "$current" "$commit") || status=$?
    if [ "$status" -gt 1 ]; then
        echo "preview stopped at $(git log -1 --format='%h %s' "$commit"): git merge-tree failed"
        break
    fi
    if [ "$status" = 1 ]; then
        stops=$((stops + 1))
        git log -1 --format='%h %s' "$commit"
        printf '%s\n' "$result" | tail -n +2 | sed 's/^/     /'
    fi
    current=$(printf '%s\n' "$result" | head -1)
done < <(git rev-list --reverse "$base..$branch")
echo "$stops of $ahead fork commits stop. Later commits replay onto unresolved earlier ones, so treat the count as a floor."

echo
echo "== Fork commits ($ahead, newest first)"
git log --format='%h %s' "$base..$branch"
