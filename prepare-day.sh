#!/bin/sh

today="$(date +'%-d')"
this_year="$(date +'%Y')"
day="${1:-$today}"
day_zp="$(printf '%02d' "$day")"
year="${2:-$this_year}"

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
