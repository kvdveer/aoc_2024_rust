# Advent of Code day 12

## Part 1

Finding adjacent area's is kind of like a flood fill algorithm, so I implemented that. Initially I also allowed for diagonal connections, but that was incorrect and resulted in a lot of head scratching (the diagonal case is not in the example input).

## Part 2

I have to admit I cheated for this one. I had figured out that I had to count corners, as each side connects exactly two of them. I looked at reddit to see someone suggesting checking 3 positions around each corner. I then derived which permutations should be counted as a corner.

## Performance

| Part | Best | Average | Worst |
|---|---:|---:|---:|
| parse/puzzle      | 193.12 µs | 195.46 µs | 198.16 µs |
| part1/puzzle      | 2.5149 ms | 2.5295 ms | 2.5455 ms |
| part2/puzzle      | 3.5748 ms | 3.5901 ms | 3.6063 ms |
| complete/puzzle   | 6.5357 ms | 6.6003 ms | 6.6677 ms |

Several miliseconds is maybe longer that I would like this solution to take.

A quick inspection with linux perf showed me that quite a bit of time is spend in BuildHasher::hash_one method. This probably means that we can gain some performance by using a fixed array instead. That does indead shave over 40% of the time of the complete puzzle.

| Part | Best | Average | Worst |
|---|---:|---:|---:|
| parse/puzzle      | 165.62 µs | 168.81 µs | 172.47 µs |
| part1/puzzle      | 1.2270 ms | 1.2477 ms | 1.2703 ms |
| part2/puzzle      | 2.3279 ms | 2.3609 ms | 2.4042 ms |
| complete/puzzle   | 3.4513 ms | 3.4796 ms | 3.5081 ms |

Next on the `perf` hitlist is vector allocation. This is probably because I use VecDeque. However, I don't need FIFO behaviour, so using a Vec as a stack should be faster. This indeed shaves off another 20% off the time.

| Part | Best | Average | Worst |
|---|---:|---:|---:|
| parse      | 150.98 µs | 151.99 µs | 153.03 µs |
| part1      | 854.73 µs | 864.57 µs | 875.18 µs |
| part2      | 1.7837 ms | 1.7989 ms | 1.8153 ms |
| complete   | 2.8755 ms | 2.8919 ms | 2.9124 ms |

     Running benches/criterion.rs (/home/koert/src/ondergetekende/aoc_2024_rust/target/release/deps/criterion-1cc24d09e47a7aa9)
Gnuplot not found, using plotters backend
                        change: [-3.8083% -2.1808% -0.5963%] (p = 0.01 < 0.05)
                        Change within noise threshold.
Found 3 outliers among 100 measurements (3.00%)
  3 (3.00%) high mild
                        change: [-19.009% -17.551% -16.045%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 6 outliers among 100 measurements (6.00%)
  4 (4.00%) high mild
  2 (2.00%) high severe
                        change: [-19.559% -17.735% -15.936%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild
                        change: [-20.582% -19.576% -18.570%] (p = 0.00 < 0.05)
                        Performance has improved.