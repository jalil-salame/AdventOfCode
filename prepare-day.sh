#!/bin/sh

fail() {
	echo "$1" >&2
	exit 1
}

today="$(date +'%-d')"
this_year="$(date +'%Y')"
day="${1:-$today}"
day_zp="$(printf '%02d' "$day")"
year="${2:-$this_year}"
lang="${3:-rust}"

day_dir="$year/day/$day_zp"

mkdir -pv "$day_dir"

case "$lang" in
'rust')
	prev="$PWD"
	cd "$day_dir" || fail "failed to cd into '$day_dir'"
	cargo generate --init --name "aoc-$year-$day_zp" --path="$prev"/2022/templates/aoc-template/
	cd "$prev" || fail "Couldn't cd back to the root"
	;;
*)
	fail "No template for languange '$lang'"
	;;
esac

input="$day_dir/input"

if [ -f "session" ]; then
	session="$(cat "session")"
else
	fail "Couldn't get input: Session cookie doesn't exist"
fi

curl -b "$session" -o "$input" "https://adventofcode.com/$year/day/$day/input" || fail "Couldn't download input for AoC $day $year"
