# Advent of Code day 6

## Part 1

This part is mostly a simple turtle-implementation with collision detection. The turtle  (the turtle) is moved around the map and the cells it visits are marked as visited, until the turtle walks off the map.

After that the visited cells are counted.

Alorithmic complexity is O(n) where n is the number of cells visited.

## Part 2

We now need to see how we can alter the map, to force the turtle into a loop.

I took the brute-force approach, as that would yield the easiest code. I simply tried to add an obstacle in every empty spot, and see if the guard would visit a spot that they had already visited. This approach was not correct: a guard could re-visit a spot while walking in a different direction. That needn't be part of the loop. Instead of a boolean to see if the guard has visited a spot, I stored direction the guard was walking in. In theory, the guard could visit a spot alternatingly in two directions and disguise that they are in a loop that way, but it is extremely unlikely that that would happen for the entire loop.

The algorithmic complexity is O(m^2*n) where n is the number of cells visited, and n the grid size. As the path length correlates to grid size, the complexity is O(n^4).

## Performance

Base line:

| part | best | mean | worst |
|------|------|------|-------|
| parse      | 162.58 µs | 163.55 µs | 164.60  |
| part1      | 31.212 µs | 31.501 µs|  31.831  |
| part2      | 218.74 ms | 219.87 ms | 221.03  |
| complete   | 232.88 ms | 236.87 ms | 241.08 ms   |
