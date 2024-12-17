# Advent of Code day 17

## Part 1

We need to implement some virtual machine that can run a program that is given to us. The provided example program isn't all that long, so we it's not that hard. This seems mostly a "can you follow complex instructions" assignment.

## Part 2

### Example program

We need to find a specific output (quinne) for a given input. To do that we'll need to analyze the program. The example program ignores 1 bit, and then outputs 3 bits from the A register. We could construct that by concatenating the 4-bit representation of the program.

I din't bother implementing this, because the actual program is different from the example program.

### Actual program

The actual program works differently. It, too is a simple loop that runs until A is depleted. The rest is different.

```text
// Actual program

#0 BST A  /// Store A%3 in B              A carried over from last iteration
#1 BXL 1  /// Toggle last bit             B initialized
#2 CDV B  /// Divide A by B -> C          C initialized
#3 BXL 5  /// Toggle 0b101 bits of B      Magic
#4 ADV 3  /// discard 3 bits from A       A prepared for next iteration
#5 BXC    /// XOR B with C -> B           Magic    
#6 OUT B  /// Output B%8                  B is the result
#7 JNZ 0  ///                             Loop
```

It takes 3 bits from the A register, it does some gnarly math on it, and then outputs 3 bits. The  A register influences the output, but the other registers don't. This means we can brute force the final piece of output by trying 8 values for the A register. There might be multiple matches. For each of the matches, we can brute force the output before that by again trying the 8 possibilities for 3 bits. Per position there could be multiple matches, so this turns into a search tree.

Because of the search tree, I decided to go for a recursive approach.

## Performance

| name     | best | average | worst |
| ---      | ---:| ---:| ---:|
| parse    | 218.82 ns | 220.46 ns | 222.36 ns |
| part1    | 411.15 ns | 416.09 ns | 422.87 ns |
| part2    | 91.200 µs | 91.737 µs | 92.327 µs |
| complete | 93.944 µs | 94.735 µs | 95.584 µs |

This isn't a very iteration-intensive piece of code, so I don't think I could improve performance without hard-coding some puzzle parameters.
