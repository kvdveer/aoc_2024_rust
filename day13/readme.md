# Advent of Code day 13

## Part 1

Eventhough I was aware that there's a arithametic solution to this problem, I
chose to implement an interative solution, as it's easier to debug.

## Part 2

Of course, part 2 makes the numbers bigger, so the iterative solution is no longer feasible.
I spent a lot of time trying to solve for one axis (diophantine linear equation), but I couldn't get it to work. Then I discovered a major misdirect in the question formuation: it asks for the cheapest solution, suggesting there is more than one solution. In reality this is a simple system of linear equations, and there is only one solution. After solving the equations, and making sure the divisions yield integer results, I found the solution.

## Performance

Without iteration, each solution requires only a few processor instructions. Most of the time is spent parsing the input.

| name | best | average | worst |
| --- | ---:| ---:| ---:|
| parse puzzle      | 38.279 µs | 38.648 µs | 39.023 µs |
| part1 puzzle      | 1.8992 µs | 1.9375 µs | 1.9736 µs |
| part2 puzzle      | 1.9207 µs | 1.9373 µs | 1.9545 µs |
| complete puzzle   | 42.821 µs | 43.176 µs | 43.611 µs |
