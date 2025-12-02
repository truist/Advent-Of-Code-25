#!/usr/bin/env bash
set -e
set -u
set -o pipefail

if [[ $# -ne 1 ]]; then
	echo "args: <two-digit-day, e.g. '01'>" >&2
	exit 1
fi

day="$1"

NEWMAIN="./src/main.rs"
TESTTXT="./test.txt"
INPUTTXT="./input.txt"

cargo new --name "aoc-$day" "$day"

cd "$day"

touch "$TESTTXT"
touch "$INPUTTXT"
cargo add clap --features derive

cp ../template/template.rs "$NEWMAIN"
sed -i ""  "s/NEWDAY/$day/g" "$NEWMAIN"

vim "$NEWMAIN" "$TESTTXT" "$INPUTTXT"
