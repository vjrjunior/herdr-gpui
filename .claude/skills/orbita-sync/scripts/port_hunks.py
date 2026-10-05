#!/usr/bin/env python3
import re
import subprocess
import sys

USAGE = """usage: port_hunks.py SHA OLD_PATH NEW_PATH... [--dry] [--skip N,N]

Replays the hunks fork commit SHA made to OLD_PATH onto the files upstream
moved that code to. Each hunk's old side is searched in every NEW_PATH, as
written and dedented by four spaces, and replaced by its new side in the
first file where it matches exactly once; context is trimmed until it does.

A hunk that only adds lines matches again after it was applied, so check
with --dry and --skip the hunks git already merged.

  --dry        Report where each hunk would land and change nothing
  --skip N,N   Leave these hunk numbers alone
"""


def run(*args):
    return subprocess.run(args, check=True, capture_output=True, text=True).stdout


def strip(text, dedent):
    if dedent and text.startswith(" " * dedent):
        return text[dedent:]
    return text


def hunks_of(sha, path):
    hunks = []
    current = None
    for line in run("git", "show", "--format=", "-U3", sha, "--", path).split("\n"):
        header = re.match(r"^@@ -(\d+)(?:,\d+)? \+(\d+)(?:,\d+)? @@", line)
        if header:
            current = {"start": int(header.group(1)), "lines": []}
            hunks.append(current)
        elif current is not None and line[:1] in (" ", "+", "-"):
            current["lines"].append(line)
        elif current is not None and line == "":
            current["lines"].append(" ")
    return hunks


def place(lines, targets, texts):
    while len(lines) > 1 and lines[-1] == " " and lines[-2][:1] == " ":
        lines = lines[:-1]
    lead = 0
    while lead < len(lines) and lines[lead][:1] == " ":
        lead += 1
    trail = 0
    while trail < len(lines) - lead and lines[len(lines) - 1 - trail][:1] == " ":
        trail += 1
    for cut in range(0, max(lead, trail) + 1):
        part = lines[min(cut, lead) : len(lines) - min(cut, trail)]
        for path in targets:
            for dedent in (0, 4):
                old = "\n".join(strip(row[1:], dedent) for row in part if row[:1] in (" ", "-"))
                new = "\n".join(strip(row[1:], dedent) for row in part if row[:1] in (" ", "+"))
                if old and texts[path].count(old) == 1:
                    texts[path] = texts[path].replace(old, new)
                    return path, cut, dedent
    return None


def main():
    args = sys.argv[1:]
    dry = False
    skip = set()
    positional = []
    while args:
        arg = args.pop(0)
        if arg in ("-h", "--help"):
            sys.stderr.write(USAGE)
            return 2
        if arg == "--dry":
            dry = True
        elif arg == "--skip":
            skip = {int(value) for value in args.pop(0).split(",")}
        else:
            positional.append(arg)
    if len(positional) < 3:
        sys.stderr.write(USAGE)
        return 2
    sha, old_path, targets = positional[0], positional[1], positional[2:]
    texts = {path: open(path).read() for path in targets}
    failed = 0
    for number, hunk in enumerate(hunks_of(sha, old_path), 1):
        if number in skip:
            print(f"hunk {number} @{hunk['start']}: skipped")
            continue
        placed = place(hunk["lines"], targets, texts)
        if placed:
            print(f"hunk {number} @{hunk['start']}: {placed[0]} (context trimmed {placed[1]}, dedent {placed[2]})")
            continue
        failed += 1
        print(f"hunk {number} @{hunk['start']}: NOT APPLIED")
        for row in hunk["lines"]:
            if row[:1] in ("+", "-"):
                print("    " + row[:150])
    if not dry:
        for path, text in texts.items():
            open(path, "w").write(text)
    return 1 if failed else 0


sys.exit(main())
