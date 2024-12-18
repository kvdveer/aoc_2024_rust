# Advent of Code day 18

## Part 1

This sounds like a _very_ simple pathfinding problem. Treating it as such, yields a correct solution, so apparently it is that simple.

## Part 2

The goal now switches to find a point at which there is no path left. The easiest eway to solve that is with a binary search, although I'm sure there's a pathfinding-only solution. I'll leave that for a later optimization.

## Performance

| name     | best | average | worst |
| ---      | ---:| ---:| ---:|
| parse    | 190.41 µs | 200.84 µs | 211.58 µs |
| part1    | 596.60 µs | 609.62 µs | 623.88 µs |
| part2    | 30.467 ms | 30.953 ms | 31.443 ms |
| complete | 24.206 ms | 24.923 ms | 25.667 ms |

Part 2 takes rather long, and it really doesn't need to, as we're still finding for the best route for every step of the binary search. We only need to know if a route is available, not what the best route is. A switch to depth-first search does indeed give a 30% performance gain.

| name     | best | average | worst |
| ---      | ---:| ---:| ---:|
| part2    | 15.846 ms | 15.952 ms | 16.062 ms |
| complete | 17.655 ms | 17.718 ms | 17.782 ms |

As part of the breadth-first search, we need to check if a coordinate is blocked. This was done by checking if the coordinate is in a list. Using a hash-set is faster, but it requires us to build set hash-set for every step of the binary search. This can be avoided by building a hashmap with all blocked coordinates, and the time at which they become blocked. This drops a whopping 95% off our puzzle time, yielding a 20x speedup.

| name     | best | average | worst |
| ---      | ---:| ---:| ---:|
| part2    | 955.70 µs | 966.26 µs | 976.05 µs |
| complete | 1.6323 ms | 1.6467 ms | 1.6635 ms |
