# Advent of Code day 2

## Parsing

Here, the elves want us to check of some report passes some tests. While the example input looks like a 6x5 grid, the actual input really isn't, as the reports have varying lengths. All numbers are `0<x<100`, and I'll be dealing with differences, so an i8 should be sufficient.

## Part 1

We need to check two conditions:

* The numbers are monotonically increasing or decreasing
* The difference between consecutive is at most 3

While the question isn't all that hard, the mixed condition make a naive approach quite verbose. I think there are two key insight here:

* All conditions refer the differences between pairs of consecutive numbers
* The first difference determine what valid differences are for the rest of the report

## Part 2

For a naive approch, we can just re-use the validity-check from part 1, and then brute-force all possible removals.

## Optimizations

My baseline is

| test | min | time | max |
| --- |--- |--- |--- |
| parse | 131.42 µs | 133.08 µs | 134.89 µs |
| part1 | 2.3743 µs | 2.4305 µs | 2.4931 µs |
| part2 | 105.90 µs | 107.91 µs | 110.12 µs |

I know that for part 2 there's a solution that uses a window-size of 3, and then counting how many of those windows would be valid after a removal. That approach relies on less copying of data. However, so far I could not figure out how to implement the head and the tail correctly.
