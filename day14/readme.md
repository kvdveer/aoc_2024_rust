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

Part 2 is quite slow, as I allocate a grid each time.