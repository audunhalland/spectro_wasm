#!/usr/bin/env python

import glob, os, subprocess, shutil

subprocess.check_call(['cargo', 'build', '--target=wasm32-unknown-emscripten'])

dbg_pattern = 'target/wasm32-unknown-emscripten/debug/deps/*-????????????????.%s'

for ext in ['js', 'wasm', 'wast']:
    shutil.copy(sorted(glob.iglob(dbg_pattern % ext),
                       key=os.path.getctime)[-1],
                'www/spectro2.%s' % ext)

subprocess.check_call(['http-server', 'www'])
