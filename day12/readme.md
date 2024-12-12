# Advent of Code day 12

## Part 1

Finding adjacent area's is kind of like a flood fill algorithm, so I implemented that. Initially I also allowed for diagonal connections, but that was incorrect and resulted in a lot of head scratching (the diagonal case is not in the example input).

## Part 2

I have to admit I cheated for this one. I had figured out that I had to count corners, as each side connects exactly two of them. I looked at reddit to see someone suggesting checking 3 positions around each corner. I then derived which permutations should be counted as a corner.

## Performance

| Part | Best | Average | Worst |
|---|---:|---:|---:|
| parse/puzzle      | 193.12 µs | 195.46 µs | 198.16 µs |
| part1/puzzle      | 2.5149 ms | 2.5295 ms | 2.5455 ms |
| part2/puzzle      | 3.5748 ms | 3.5901 ms | 3.6063 ms |
| complete/puzzle   | 6.5357 ms | 6.6003 ms | 6.6677 ms |

Several miliseconds is maybe longer that I would like this solution to take, but I'm done with this puzzle, so I'll accept it.
