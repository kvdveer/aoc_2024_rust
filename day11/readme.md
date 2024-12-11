# Advent of Code day 11

## Part 1

I chose to perform the actual expansion, even though the answer doesn't actually use it. This way, it was easier to verify that the code was correct. Performance was reasonable, despite the number list expanding to 200K elements.


## Part 2

Here, the actual expansion would not have worked: the final expansion would have consumed several terabytes of memory. Fortunately, we can reduce the problem to a counting problem: 0 expands to 1 element in 1 or 2 expansions, it would expand into 2 elements after 3 expansions (20 and 24). We don't need to store the expanded list, we just need to count the number of elements that would be generated.

Even with this insight, performance was not good enough. Fortunately, many of these expansion are repeated. I just slapped a `#[memoize]` on the function, and the performance was sub-microsecond.

## Performance

| Part | Best | Mean |  Worst |
| ---- | ---- | ---- | ------ |
| parse | 303.06 ns | 317.22 ns | 332.93 ns |
| part1 | 20.948 ms | 22.052 ms | 23.218 ms |
| part1 (optimized) | 636.95 ns | 676.94 ns | 716.51 ns |
| part2 | 670.41 ns | 743.18 ns | 828.40 ns |
| complete | 1.4012 µs | 1.4466 µs | 1.5007 µs |

Part 1 dominates these test results, but fortunately we can just transplant the part 2 soution to part 1 and get a near-100% speedup.
