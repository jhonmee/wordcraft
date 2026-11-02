"""TEST BUILDS ONLY: appends the menu previews' translations to a craft-i18n checkout.

`python apply.py <craft-i18n dir>`. The rows belong in storytold/craft-i18n; until they land there,
the fork's Windows test build adds them so the new menus can be tried in every language.
"""
import os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from rows import L, PLAIN, SHAPES, SHAPES_FIX
root = sys.argv[1]
added = 0
for loc in L:
    have = {}
    for f in ("common.tsv", "wordcraft.tsv"):
        p = os.path.join(root, "locales", loc, f)
        if os.path.exists(p):
            for line in open(p, encoding="utf-8"):
                parts = line.rstrip("\n").split("\t")
                if len(parts) == 3 and not line.startswith("#"):
                    have.setdefault((parts[0], parts[1]), parts[2])
    out = []
    for src, tr in PLAIN.items():
        if ("", src) not in have:
            out.append(("", src, tr[loc]))
    shapes = {src: tr[loc] for src, tr in SHAPES.items()}
    shapes.update({src: v for (l, src), v in SHAPES_FIX.items() if l == loc})
    for src, tr in shapes.items():
        if have.get(("", src)) != tr and ("shapes", src) not in have:
            out.append(("shapes", src, tr))
    if not out:
        continue
    p = os.path.join(root, "locales", loc, "wordcraft.tsv")
    if not os.path.exists(p) and "--all" not in sys.argv:
        # Not a WordCraft interface language yet (no WordCraft catalog): nothing to add.
        continue
    if not os.path.exists(p):
        # A locale with only shared strings so far: start its WordCraft catalog with the header
        # its common.tsv has, naming WordCraft.
        common = open(os.path.join(root, "locales", loc, "common.tsv"), encoding="utf-8").read().split("\n")
        header = []
        for line in common:
            if not line.startswith("#"):
                break
            header.append(line)
        header[0] = header[0].replace("every Crafting App", "WordCraft")
        open(p, "w", encoding="utf-8", newline="").write("\n".join(header) + "\n")
    text = open(p, encoding="utf-8").read()
    if not text.endswith("\n"):
        text += "\n"
    text += "".join(f"{c}\t{s}\t{t}\n" for c, s, t in out)
    open(p, "w", encoding="utf-8", newline="").write(text)
    added += len(out)
print(f"added {added} rows")
