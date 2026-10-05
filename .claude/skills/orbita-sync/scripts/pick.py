#!/usr/bin/env python3
import re
import sys

USAGE = """usage: pick.py FILE            list the conflict blocks with sizes and first lines
       pick.py FILE o,t,b,r,...  resolve them, one letter per block in order:
                                 o = ours (upstream), t = theirs (the fork commit),
                                 b = ours then theirs, r = theirs then ours
"""

PATTERN = re.compile(r"<<<<<<< [^\n]*\n(.*?)=======\n(.*?)>>>>>>> [^\n]*\n", re.S)


def main():
    if len(sys.argv) not in (2, 3) or sys.argv[1] in ("-h", "--help"):
        sys.stderr.write(USAGE)
        return 2
    path = sys.argv[1]
    text = open(path).read()
    blocks = list(PATTERN.finditer(text))
    if len(sys.argv) == 2:
        for index, block in enumerate(blocks):
            line = text.count("\n", 0, block.start()) + 1
            ours, theirs = block.group(1), block.group(2)
            print(f"[{index}] line {line}: ours {ours.count(chr(10))} lines, theirs {theirs.count(chr(10))} lines")
            for name, side in (("ours", ours), ("theirs", theirs)):
                for row in side.split("\n")[:3]:
                    print(f"      {name}: {row[:110]}")
        return 0
    choices = sys.argv[2].split(",")
    if len(choices) != len(blocks):
        sys.stderr.write(f"{path}: {len(blocks)} blocks, {len(choices)} choices\n")
        return 1
    out = []
    position = 0
    for choice, block in zip(choices, blocks):
        ours, theirs = block.group(1), block.group(2)
        sides = {"o": ours, "t": theirs, "b": ours + theirs, "r": theirs + ours}
        if choice not in sides:
            sys.stderr.write(f"unknown choice {choice!r}\n")
            return 2
        out.append(text[position : block.start()])
        out.append(sides[choice])
        position = block.end()
    out.append(text[position:])
    open(path, "w").write("".join(out))
    print(f"resolved {len(blocks)} blocks in {path}")
    return 0


sys.exit(main())
