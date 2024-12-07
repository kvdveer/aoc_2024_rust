# Advent of Code day 7

## Part 1

This is where the big numbers come in, although nothing beyond U64.
We the naive approach would be trying all permuations of the available operators. The lists aren't all that long, so this is feasible. This calls for recursion.

In the recursive function, we can check some simple rules to prune the search tree:

* `A * B = C` can't be true if `B % A != 0`
* `A + B = C` can't be true if `B < A`

## Part 2

This just adds another operator. The concatenation operator calls for string operations, but that would slow things down a lot. Furtunately we can do the operation entirely in the integer domain. There's also a very simple rule to prune the search tree:

* `A || B == C` can't be true if C doesn't end with B, which can be checked with a simple modulo operation with the right divisor.

## Performance

I don't think my solution needs further optimization, as we're well in the sub-millisecond range. That's good enough for me.

| part |   best |   median |      mean |
|-----:|-------:|---------:|----------:|
| parse | 204.08 µs | 213.68 µs | 222.20 µs |
| part1 | 95.044 µs | 96.021 µs | 96.989 µs |
| part2 | 275.75 µs | 278.17 µs | 280.95 µs |
| complete | 619.94 µs | 623.80 µs | 627.53 µs |
