#!/usr/bin/env bash
OUTDIR="tests/fixtures"

[[ ! -d "$OUTDIR" ]] && mkdir -p "$OUTDIR"

echo "Hello World" > $OUTDIR/hello_world.txt
echo "Hello"    "World" > $OUTDIR/hello_world.spaces.txt
echo -n "Hello World" > $OUTDIR/hello_world.n.txt
echo -n "Hello"    "World" > $OUTDIR/hello_world.spaces.n.txt
