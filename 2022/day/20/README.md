# Advent of Code Template

This is a rust template that you can use with `cargo-generate` to quickly create
a Advent of Code solution in Rust

## Usage

1. Create a cargo workspace:

   ```toml
   # file: AdventOfCode/2022/Cargo.toml
   [workspace]
   members = [ "day/*" ]

   [workspace.dependencies]
   color-eyre = "0.6.2"
   ```

2. Create a subdirectory under that workspace (ie. I use `day/01`)
3. Move into the created directory
4. Run `cargo generate --init --name 'aoc-2022-01' gh:salameme/aoc-template`
