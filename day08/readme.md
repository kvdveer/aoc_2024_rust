# Advent of code day 8

## Part 1

Yet another grid problem, yet another appearance of 'aoc_grid'. We have to project pairs of points on another grid, and count how many grid cells are visited at least once. The grid is finite, so some projections will be outside the grid.

My solution was to build a list of antenna's for each frequency, do a cartesian product of all antennas. For each pair, we find the difference, then subtract the difference from one, and add it to the other. Getting the sign for this arrithmatic is a bit tricky, but as there's only two possibilities, I just cheated an chose the one that got the right answer. 😇

## Part 2

The second part seemed strangely easier: we just have to do these projections unbounded.

There's a bit of a challenge figuring out where to start and where to stop projecting. Two strategies come to mind:

* Start at point A an add projections until we're outside the grid, then repeat for point B.
* Draw a line between two valid projections ouside the grid, and then project all points on that line.

I chose the second one. because of how I calculated the points outside of the grid, this yields quite a few invalid points, so in hindsight, the first strategy might have been better.

## Performance

| part | best | mean | worst |
| --- | ---:| ---:| ---:|
| parse | 28.353 µs | 28.478 µs | 28.607 µs |
| part1 | 90.132 µs | 91.031 µs | 91.896 µs |
| part2 | 106.42 µs | 108.03 µs | 109.73 µs |
| complete | 243.22 µs | 244.51 µs | 245.99 µs |

As I'm far sub-millisecond, I'm happy with this. There might be a tiny bit of optimization left in the code, but I'm not going to bother with that.
