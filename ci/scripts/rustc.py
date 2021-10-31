#!/usr/bin/env python3

import glob
import json
import subprocess

from os.path import basename, splitext

subprocess.run(["./x.py", "doc", "compiler", "--stage", "1"])

crates = []
jsons = glob.glob("build/x86_64-unknown-linux-gnu/compiler-doc/*.json")
for json_path in jsons:
    json_name = basename(json_path)
    crates.append(splitext(json_name)[0])
    subprocess.run([
        "mv",
        f"build/x86_64-unknown-linux-gnu/compiler-doc/{json_name}",
        "../roogle-index/crate"
    ])

with open("rustc.json", mode='w') as f:
    json.dump(crates, f)
