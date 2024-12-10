# Advent of Code day 10

## Part 1

We need to find the number of distinct summits reachable from each trail-head.
Although probably not the fastest way to do this, I decided just to walk every valid path from
each trail-head and count the number of summits reached. Along the way I forgot to deduplicate the trailheads reached, causing me to find way too many summits. I fixed this by deduplicating the trail-heads reached before counting them.

## Part 2

We need to find the number of distinct paths from a trail-head to the summit. That was exactly the bug I had in part 1, so I just removed the deduplication code.

## Performance

| Name              |    Best  |   Average |  Worst    |
|-------------------|----------|-----------|-----------|
| parse      | 56.312 µs | 56.658 µs | 57.080 µs |
| part1      | 427.13 µs | 433.71 µs | 440.45 µs |
| part2      | 45.425 µs | 46.104 µs | 46.900 µs |
| complete   | 654.13 µs | 668.28 µs | 682.46 µs |

Part 1 does a lot of  allocations in the loop, which is not good for performance. Instead of deduplicating after the trail-heads have been found, I could have dedupicated paths. This requires a boolean grid, which is just a one-time allocation. If I have the time, I'll revisit this.
