# Advent of Code day 1

The first day nice warm-up, as usual. It mostly tests your ability to follow instructions, as the solution is spelled out in the problem statement.

## Part 1

The solutions basically boils down to building two lists, sorting them, and then iterating over the two lists to find the aboslute difference.
I had chosen to parse the input as u64, meaning that I could not represent negative numbers, which gave a bit more code when calculating the absolute difference.

## Part 2

This calls for some pre-processing of the second list, to efficiently find the counts. I chose to use a `HashMap` for this, as it is the most efficient way to count the occurences of each number in the list. The rest of the solution is just a matter of iterating over the first list, and checking if the complement is in the second list.


## Reflection 

I chose to parse the input as a list as pairs, but that did not turn out to be helpful at all. In hindsight, I should've parsed to lists of numbers.

Solve times:
```
day01/parse/puzzle      time:   [44.850 µs 45.102 µs 45.376 µs]                                
day01/part1/puzzle      time:   [13.215 µs 13.285 µs 13.369 µs]                                
day01/part2/puzzle      time:   [31.260 µs 31.467 µs 31.683 µs]                                
```

## Optimization 1: Use i32 instead of u64

Instead of parsing the input as u64, I parse the input as i32. None of the numbers exceed the range of i32, so this is a safe optimization.
My reasoning is that handling 32-bit integers is faster than handling 64-bit integers, as it is less data. This yielded a few percentage points of improvement.

```
day01/parse/puzzle      time:   [43.241 µs 43.389 µs 43.558 µs]                                
                        change: [-3.8018% -3.1089% -2.4377%] (p = 0.00 < 0.05)
day01/part1/puzzle      time:   [12.504 µs 12.566 µs 12.628 µs]                                
                        change: [-7.5372% -7.0116% -6.4823%] (p = 0.00 < 0.05)
day01/part2/puzzle      time:   [28.189 µs 28.459 µs 28.741 µs]                                
                        change: [-10.080% -9.4786% -8.8217%] (p = 0.00 < 0.05)
```