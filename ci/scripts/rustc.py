#!/usr/bin/env python3

import glob
import json
import subprocess

from os.path import basename, splitext

subprocess.run(["./x.py", "doc", "compiler", "--stage", "1"])

crates = []
jsons = glob.glob("build/x86_64-unknown-linux-gnu/compiler-doc/*.json")
for json in jsons:
    json = basename(json)
    crates.append(splitext(json)[0])
    subprocess.run([
        "mv",
        f"build/x86_64-unknown-linux-gnu/compiler-doc/{json}",
        "../roogle-index/crate"
    ])

with open("rustc.json", mode='w') as f:
    json.dump(crates, f)
