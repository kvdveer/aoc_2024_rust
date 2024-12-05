# Advent of Code day 5

This looks like a graph problem to me.

The example rules are at least internally consistent, although not entirely defined (there's no rule to order 13 and 53)

The actual puzzle rules do contain contradicting rules, so we need to filter them on a per-sequence basis should we ever apply them.

## Part 1

I got sucked into trying to process the rules, in an effort to make the result faster. This took up a lot of time, ant was clearly a case of premature optimization. In the end I only built a lookup table.

My implementation is now O(log(N)*M^2), where N is the number of rules, M is the sequence length.

## Part 2

I was again tempted into premature optimization, but this time I managed to stop earlier. I just re-used the code from part 1 to check if a sequence is valid.

For the invalid sequences do the following:

* Filter out the relevant rules O(N *M)
* Expand implied rules O(M^2)
* Sort the token according to the implied rules O(N*log(M))

The rule expansion is the most expensive part.

## Performance

| part | best | average | worst |
|---|---|---|---|
|parse | 46.034 µs | 46.347 µs | 46.700 µs |
|part1 | 406.05 µs | 407.95 µs | 409.88 µs |
|part2 | 11.503 ms | 11.563 ms | 11.623 ms |
|complete | 12.404 ms | 12.557 ms | 12.724 ms |