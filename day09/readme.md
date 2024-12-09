# Advent of Code day 9

## Part 1

I chose to do an iterative solution for the problem, as this avoid hefty memory allocations. While I had a solution that solved the example quite quickly, with the actual answer I had a wrong outcome. Turns out, I incorrectly handled when there's space remaining at the end of the problem. The example didn't have this (the space after file 5 is fully filled with file 6). The actual test input did have this, though.

After a lot of not so nice words, I found an input (22222) which exhibited the same problem. With a failing case I had something to debug, and I could finally fix the issue.

## Part 2

This time, I could not find a way to do this without a big memory allocation. I think I could've built an actual free-list, but for the initial solve I've chosen to go for a `Vec<bool>`.

## Performance

| Name        | Best | Average | Worst |
|-------------|------|---------|-------|
| parse       | 371.23 µs | 376.24 µs | 381.43 µs |
| part1       | 205.08 µs | 207.57 µs | 210.02 µs |
| part2       | 146.08 ms | 146.81 ms | 147.60 ms |
| complete    | 152.78 ms | 154.19 ms | 155.67 ms |
