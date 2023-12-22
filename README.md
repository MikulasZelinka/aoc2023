# Advent of Code 2023

## Rust

Mostly in Rust – project structure is based on [How to set up Rust for Advent of Code](https://www.youtube.com/watch?v=fEQv-cqzbPg).

To run a part P (e.g., `2`) of day DD (e.g., `01`):

```bash
cd day-DD && cargo run --bin partP
```

To scaffold a new day:

```bash
./new_day.sh DD
```

## Python

If you see a 🐍 in the commit message, that day is in Python –
likely because it was too tempting to use `numpy` and/or play code ⛳.

- [Day 6](./day-06/main.py)
  - A purposely slow and naive solution that could (should!) be replaced with a single formula. While teaching, the plan was to show how incredibly stupid this is – we'll learn how to use (a `tqdm`) progress bar to see that this will never finish! Unfortunately, it did finish in about two minutes.
- [Day 9](./day-09/main.py)
- [Day 13](./day-13/main.py) (numpy)
- [Day 14](./day-14/main.py) (numpy)

## Godot

Some days are done in Godot / GDScript instead, for ~~research~~ visualisation purposes:

- [Repository](https://github.com/MikulasZelinka/advent-of-godot-2023)
- ["Play" online](https://advent-of-godot-2023.netlify.app/)

Advent of Godot days:

- [Day 10](https://github.com/MikulasZelinka/advent-of-godot-2023/blob/7ab87545216b8aa769f2539f628c524f6a4f842d/scenes/day-10/map.gd)
- [Day 18](https://github.com/MikulasZelinka/advent-of-godot-2023/blob/7ab87545216b8aa769f2539f628c524f6a4f842d/scenes/day-18/day_18.gd)
