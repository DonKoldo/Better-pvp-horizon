#!/usr/bin/env python3
"""Packages this bundle as better_pvp-<version>.hnmod (same layout as horizon-mods' native.py).
It uses private fields of the game, so it is pinned to one Horizon commit (no `hooks` key)."""
import json, zipfile, tomllib, hashlib
from pathlib import Path
root = Path(__file__).parent
bundle = root / "mixins" / "better_pvp"
m = tomllib.loads((bundle / "horizon-mixin.toml").read_text())
c = json.loads((root / "compatibility.json").read_text())
assert m["compatibility"]["horizon_revision"] == c["horizon_revision"]
manifest = {"schema": 1, "id": m["mod"]["id"], "version": m["mod"]["version"], "game": c["game"],
            "horizon": c["horizon_revision"], "mixins": c["horizon_mixins_revision"]}
files = {"horizon-native.json": (json.dumps(manifest, indent=2, sort_keys=True) + "\n").encode()}
for p in sorted(bundle.iterdir()):
    if p.suffix in (".rs", ".toml"):
        files[f"mixins/{manifest['id']}/{p.name}"] = p.read_bytes()
out = root / f"{manifest['id']}-{manifest['version']}.hnmod"
with zipfile.ZipFile(out, "w", zipfile.ZIP_DEFLATED) as z:
    for name, data in sorted(files.items()):
        i = zipfile.ZipInfo(name, (2026, 1, 1, 0, 0, 0)); i.compress_type = zipfile.ZIP_DEFLATED
        i.external_attr = 0o100644 << 16
        z.writestr(i, data)
print(out, hashlib.sha256(out.read_bytes()).hexdigest())
