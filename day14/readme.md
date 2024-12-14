# Advent of Code day 14

## Part 1

At first glance, the puzzle seems very easy: just multiply the velocity by the time, and you'll have the new position. This position needs wrapping, but that's just a modulo operation.
After implementing this, the solution was accepted, so apparently it was that easy.


## Part 2

Finding an image of a christmass tree is a challenge, as it doesn't seem like an AOC puzzle. I spent some time figuring out how to approch this, and then I decided I could make some assumptions:

* Ascii art is made of characters, and for them to be recognizable, they need to be adjacent
* The non-christmass trees have rouhly uniform distribution of robots, so they will not be adjacent to eachother
* The image can be found within a reasonably small timeframe.

With these assumptions, for each timestep, I built a matrix of the robots, and counting the number of neigbours. If the number of neighbours is above a certain threshold, I print the matrix and the time. Finding the threshold was found by trial and error, but didn't take too long.

The solution was accepted, and I got a nice ascii art of a christmass tree.

## Performance

| name | best | average | worst |
| --- | ---:| ---:| ---:|
| parse      | 40.229 µs | 41.409 µs | 42.486 µs |
| part1      | 2.8771 µs | 2.8999 µs | 2.9255 µs |
| part2      | 242.19 ms | 248.65 ms | 255.37 ms |
| complete   | 263.99 ms | 273.42 ms | 283.15 ms |

Part 2 is quite slow, as I allocate a grid each time. My initial implementation checked each grid cell. I managed to optimize this by only checking where the robots were. This yielded a satisfying 73% performance boost.

| name | best | average | worst |
| --- | ---:| ---:| ---:|
| parse      | 33.385 µs | 34.575 µs | 35.758 µs |
| part1      | 2.8981 µs | 2.9457 µs | 2.9943 µs |
| part2      | 58.462 ms | 59.516 ms | 60.600 ms |
| complete   | 60.739 ms | 61.470 ms | 62.227 ms |

Still, my tree-detection is quite slow. Another method I could use is entropy analysis. Tallying op the x and y coordinates, should yield a rougly equally distributed set of coordinates. The tree would be the only exception, as it would have a high entropy. Implementing this again yielded a satisfying performance boost of 30%.

| name | best | average | worst |
| --- | ---:| ---:| ---:|
| parse |   46.047 µs |  46.980 µs | 48.050 µs |
| part1 |   2.7849 µs |  2.7999 µs | 2.8158 µs |
| part2 |   36.474 ms |  37.121 ms | 37.806 ms |
| complete |34.602 ms |  34.802 ms | 35.009 ms |
