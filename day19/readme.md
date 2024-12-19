# Advent of Code day 19

## Part 1

We need to find some combination of towels that form a certain pattern. While it isn't really formulated a spatial problem, this really is a pathfinding problem, with each towel being a step towards the solution. 

As we're only interrested knowing if there is a path, I uses a simple depth first search to find the path.

## Part 2

The second part is sligtly more complex, as we need to know the number of paths. Fortunately, the `pathfinding` crate has a simple method to count just that.

## Performance

| part | best | mean | worst |
| :- | ---: | ---: | ---: |
|parse      | 238.18 µs | 239.89 µs | 241.87 µs |
|part1      | 24.989 ms | 25.068 ms | 25.149 ms |
|part2      | 25.166 ms | 25.339 ms | 25.518 ms |
|complete   | 50.828 ms | 51.422 ms | 52.064 ms |

I'm not too thrilled about the performance of this solutions, but as it's really a combinatorics problem, I'm not sure how to improve it.
