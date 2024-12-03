# Advent of Code day 3

## Part 1

We need to find something that looks lik a function call in a string of text.
To me, this feels like a regular expression problem, so that's how I solved it.

Regex engines are rather generic, so a more specialized piece of code would probably be faster.

## Part 2

This still feels like a regex problem to me, but now with a very simple state machine added to it.

## Reflection

While this was a fun problem to solve, I feel it wasn't very challenging.

## Optimization

Baseline:

| test | min | mean | max |
| --- | --- | --- | --- |
| parse| 10.694 µs | 10.789 µs | 10.898 µs |
| part 1| 194.32 µs | 197.43 µs | 201.29 µs |
| part 2| 269.87 µs | 271.28 µs | 272.89 µs |

### Use regex on bytes instead of strings

My reasoning here is that the regex engine has to handle all kinds of UTF-8 edge-cases that just aren't present in the puzzle input (which is in ASCII).
Rust doesn't have a direct way of parsing numbers from bytes, so I had to implement a function doing just that (which doesn't actually check that there's numbers in the input).

| test | min | mean | max |
| --- | --- | --- | --- |
| parse | 10.944 µs (-11%) | 11.071 µs (-9.7%) | 11.203 µs (-7.47%) |
| part 1 | 192.24 µs (-14.7%) | 193.37 µs (-13.7%) | 194.66 µs (-12.6%) |
| part 2 | 283.45 µs (-.8%) | 287.62 µs (0%) | 291.83 µs (+1.1%) |

The speed gans aren't all that impressive, and in my measurement, the parse method got faster. That's odd, as I hadn't touched that method at all. I think the difference is within the margin of error, so I'm going to call this a failed optimization.
