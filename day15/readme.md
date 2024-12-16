# Advent of Code 15

## Part 1

The rules aren't that hard to implement.

## Part 2

I had to rewrite the movement. Instead of a linear search, I needed a tree, so I switched to a recursive approach to figure out which parts need to be moved, with an early-out in case we encounter a wall.

## Performance

| parse      | 367.03 µs | 373.35 µs | 380.14 µs |
| part1      | 789.56 µs | 804.20 µs | 819.24 µs |
| part2      | 948.52 µs | 967.39 µs | 989.11 µs |
| complete   | 1.9688 ms | 1.9915 ms | 2.0181 ms |
