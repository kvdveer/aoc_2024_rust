# Advent of Code day 16

## Part 1

It was about time we got some pathfinding question. I delegated most of the work to the excelent `pathfinding` crate.
The only non-standard thing was computing the price for rotation, but the `pathfinding` crate makes that easy as well.

## Part 2

Finding the shortest path is standard, but finding all shortest paths usually is not.
I implemented this by calculating a fee for visiting a node again, and then retrying pathfinding until no more new nodes are found.

## Performance

| name | best | average | worst |
| --- | ---:| ---:| ---:|
| parse |      213.95 µs | 215.24 µs | 216.72 µs |
| part1 |      5.0180 ms | 5.0349 ms | 5.0525 ms |
| part2 |      16.445 ms | 17.016 ms | 17.617 ms |
| complete |   22.019 ms | 22.306 ms | 22.604 ms |

After reviewing the code, I noticed that `pathfinding` has the `astar_bag` method, which solves this exact problem. This gave me a satifying ~50% performance gain, as well as a significant reduction in code complexity.

| name | best | average | worst |
| --- | ---:| ---:| ---:|
| part2      | 8.5179 ms | 8.5817 ms | 8.6574 ms |
| complete   | 11.851 ms | 11.894 ms | 11.942 ms |

It occurred to me that part 1 could be further optimized using the Dijkstra algorithm, as only a single path is needed. This gave a 40% performance gain. Unfortunately, the second part can't use Dijkstra as it needs to find all shortest paths, which Dijkstra optimizes away.

| name | best | average | worst |
| --- | ---:| ---:| ---:|
| part1 |     2.9723 ms | 2.9853 ms | 2.9999 ms |
| complete |  11.135 ms | 11.321 ms | 11.530 ms |
