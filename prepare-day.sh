#!/bin/sh

today="$(date +'%-d')"
this_year="$(date +'%Y')"
day="${1:-$today}"
day_zp="$(printf '%02d' "$day")"
year="${2:-$this_year}"
lang="${3:-rust}"

day_dir="$year/day/$day_zp"

mkdir -pv "$day_dir"
if [ -f "$year/session" ]; then
	session="$(cat "$year/session")"
else
	echo "Session cookie doesn't exist"
	exit 1
fi

input="$day_dir/input"

if [ -f "$input" ]; then
	echo "$input already exists"
	exit 0
else
	curl -b "$session" -o "$input" "https://adventofcode.com/$year/day/$day/input"
fi

case "$lang" in
'rust')
	cd "$day_dir" || exit 1
	cargo init --name "aoc-$year-$day_zp"
	cp ../../../templates/main.rs src/main.rs
	;;
*)
	echo "No template for languange '$lang'"
	exit 1
	;;
esac
