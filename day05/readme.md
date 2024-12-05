# Advent of Code day 5

This looks like a graph problem to me.

The example rules are at least internally consistent, although not entirely defined (there's no rule to order 13 and 53)

The actual puzzle rules do contain contradicting rules, so we need to filter them on a per-sequence basis should we ever apply them.

## Part 1

I got sucked into trying to process the rules, in an effort to make the result faster. This took up a lot of time, ant was clearly a case of premature optimization. In the end I only built a lookup table.

My implementation is now O(log(N)*M^2), where N is the number of rules, M is the sequence length.

## Part 2

I was again tempted into premature optimization, but this time I managed to stop earlier. I just re-used the code from part 1 to check if a sequence is valid.

My first implemementation did the following:

* Filter out the relevant rules `O(N *M)`
* Expand implied rules `O(M^3)`  _e.g. a|b and b|c implies a|c_
* Sort the token according to the implied rules `O(N*log(M))`

### Optimzation: different algorithm

After some thinking, I figured out a faster way:

* Filter out the relevant rules `O(N * M)`
* Until we have processed all tokens `O(M)`:
  * Find a page number that, is never after some other token `O(N)`,
  * That page number is the next token after sorting
  * Remove that token from the list `O(M)`
  * Remove rules that reference the token

I believe this algorithm is `O(N*M)`.

Criterion confirmed this is substantially faster:

```txt
time:   [2.2857 ms 2.3266 ms 2.3691 ms]                                   
change: [-81.639% -81.118% -80.626%] (p = 0.00 < 0.05)
```

### Optimization: only sort half the sequence

As the question calls for the middle page, we don't need to sort beyond that. Sorting the first half is the hardest part, as it has the most rules. The second half is easier, as it has fewer rules, but even then, this gave me ~8% performance improvement. (on top of some memory mangement fixes)

```txt
time:   [1.9809 ms 1.9985 ms 2.0185 ms]                                
change: [-2.1402% -0.7235% +0.6787%] (p = 0.32 > 0.05)
```

## Performance

| part | best | average | worst |
|---|---|---|---|
|parse | 46.034 µs | 46.347 µs | 46.700 µs |
|part1 | 406.05 µs | 407.95 µs | 409.88 µs |
|part2 | 1.7801 ms | 1.7940 ms | 1.8080 ms  |
|complete |  2.2705 ms | 2.2798 ms | 2.2895 m|
