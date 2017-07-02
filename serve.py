#!/usr/bin/env python

import argparse, glob, os, subprocess, shutil

ap = argparse.ArgumentParser()
ap.add_argument('--release', action='store_true')

args = ap.parse_args()

mode = 'release' if args.release else 'debug'
build_cmd = ['cargo', 'build', '--target=wasm32-unknown-emscripten']
if args.release:
    build_cmd.append('--release')

subprocess.check_call(build_cmd)

pattern = 'target/wasm32-unknown-emscripten/%s/deps/*-????????????????.%%s' % mode

for ext in ['js', 'wasm', 'wast']:
    files = sorted(glob.iglob(pattern % ext), key=os.path.getctime)
    if files:
        shutil.copy(files[-1], 'www/spectro2.%s' % ext)

subprocess.check_call(['http-server', 'www'])
