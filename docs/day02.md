# Day 2: TITLE
## Part 1
TODO

## Problem solving approach part 1
My first instinct was to take the input number and split it in half if possible, i.e let's say we have the number `9027`, splitting it in half would give us `90` for the left side and `27` for the right side, then we could compare the raw byte/ASCII value, for each side and if they are equal we have found an invalid id and can do `+=` with a variable defined somewhere to add the current number to what we have so far. then we increment our number i.e `9027` by 1, we get `9028` and start the comparison progress again. While this could work i'm not happy about this approach because there are a lot of unnecessary comparisons, let's for example say our range has 10'000 numbers this would mean we have O(n) comparisons with n being the amount of numbers in our range, here it'd be O(10'000).

This is especially inefficient because if the current number has an odd length, such as for example the number `10721` we're unable to split it into even halfs. We could either split it with left side being `10` and right side `721` or left side with `107` and right side with `21` either way we don't have matching sides and we would be doing useless comparisons in this entire odd range, so we might as well skip all numbers with an odd length.

So finally the idea is the following, we receive an input number and then check if it has an even length for example a number in the range 4000 - 9999 can contain invalid ids and has an even length whereas a number in the range 10000 - 99999 does not have any invalid ids, thus we can make an if block like `if (num_digits % 2) == 0` where a number could contain an invalid id and in odd ranges we have the else block where we handle the skipping to the beginning of where the next invalid id could be.

let's start with a number range that could contain a valid id, for example 4810-100000, when we take the number 4800 and split it into half we receive a left side and a ride side, the left side being 48 and the right side being 10. Looking at this format we see theres 3 scenarios that can happen if the left side remains constaint. That is the left side is larger than the right side, i.e 48 > 10, the left side is equal to the right side i.e if we had 4848 then left = 48 and right = 48 thus 48 == 48 and finally the left side is smaller than th eright side i.e if we had 4870 then left = 48 and right = 70. These 3 cases are the if branches we have to handle.

In the first case where left > right, we can subtract the value of right from left, i.e left - right = 48 - 10 = 38. This result represents the absolute difference between left and right and when we add this to our whole number we see that left == right and thus we have found an invalid id, i.e 4810 + 38 = 4848. in the code this happens in

```
if left_half > right_half {
    current_num += left_half - right_half;
}
```

the 2nd case is where left == right, because here we have an invalid id, we can add it to the total sum, this part is simple. however the more difficult part is advancing the whole number now, if we have 4848, we know the next invalid id according to the rules will be 4949. the primitive way would be increment the whole number by 1 and then do another check and repeat this, but this would mean that in this case we'd be doing 4949 - 4848 = 101 comparisons to get to 4949 and 100 of those would be useless. There is a better way, we know that any natural number $N$ can be expressed in a general base $b$ using the following notation:

$$N = \sum_{i=0}^{n} d_i \cdot b^i \quad \text{where } 0 \leq d_i < b \text{ and } b \in \mathbb{Z}_{\geq 2}$$

Variables:
- $N$: The total value of the number.
- $b$: The base (radix) of the number system.
- $n$: The index of the highest order digit (where the total number of digits is $n+1$).
- $i$: The positional index, starting at $0$ for the least significant digit
- $d_i$: The specific digit at position $i$.

Constraints:
- $0 \leq d_i < b$: This range ensures that every number has a unique representation. It requires that any value equal to or greater than the base must carry over to the next position $b^{i+1}$. For example, in base $10$, the value ten must be written as $1 \cdot 10^1 + 0 \cdot 10^0$ ($10$) rather than $0 \cdot 10^1 + 10 \cdot 10^0$, which would be the case if $d_i$ were allowed to equal $b$.
- $b \in \mathbb{Z}_{\geq 2}$: The base must be an integer greater than or equal to $2$. A base of $0$ or $1$ fails to provide the necessary non-zero digits ($d_i$) to represent positive quantities through powers of the base. For example, if $b=1$, then $1^i$ is always $1$, and the only valid digit $d_i < 1$ is $0$, making it impossible to represent any number other than zero.

To illustrate this, representing the number $4848$ in base $10$ requires an index of $n=3$, resulting in the expansion $(4 \cdot 10^3) + (8 \cdot 10^2) + (4 \cdot 10^1) + (8 \cdot 10^0)$. This sum evaluates to $4000 + 800 + 40 + 8 = 4848$, demonstrating how each digit $d_i$ is mapped to its specific power of the base to reconstruct the total value $N$.

We can exploit this structure to skip directly from $4848$ to the next invalid id of $4949$, without checking every value in between. Consider only the left half of the number, $48$, which corresponds to $4800$ once the right half is zeroed out. Incrementing this left half by one means adding $100$ to the whole number, since $100$ is the base raised to half the digit length with $n = 4$ digits, we take $n/2 = 4/2 = 2$, giving $10^2 = 100$. Adding this to our current number gives $4848 + 100 = 4948$. A further $+1$ then forces the right half to match the now incremented left half, producing $4949$. In a single step, we've jumped over every valid id in between and landed on the next invalid one. The entire logic is implemented as

```
else if left_half == right_half {
    total_sum += current_num;
    current_num += half_base + 1;
}
```

The final case occurs when left < right, for example $4870$, where the left half is $48$ and the right half is $70$. Since left < right, no invalid id exists with the left half held at $48$. The next invalid id must occur once the left half itself increments to $49$, which we compute directly as $\text{left\_half} + 1 = 48 + 1 = 49$ and store for later use.

To reach this next range, the right half must increment until it reaches $b^{n/2}$, our half_base of $100$. This is the same carry threshold established earlier, because each digit $d_i$ is bounded by $0 \leq d_i < b$, a half consisting of $n/2$ digits rolls over to $0$ and increments its neighbor precisely when it reaches $b^{n/2}$. The distance remaining before this carry is the right's deficit from the base, $\text{right\_deficit} = \text{half\_base} - \text{right\_half} = 100 - 70 = 30$. Adding this deficit to our current number forces the carry, $4870 + 30 = 4900$, where the left half has now become $49$, matching the value we stored above. The only step left is to place this new left half into the right half position, which for a base of $100$ is done by simple addition $4900 + 49 = 4949$, this yields our next invalid id. In code this happens as follows

```
else if left_half < right_half {
    let next_left_half = left_half + 1;
    let right_deficit = half_base - right_half;
    current_num += right_deficit + next_left_half;
}
```

We see that in both the cases where left < right and left > right the number gets set to an invalid id and only in the equal branch does it get added to the sum and incremented. This works well and keeps getting repeated until our whole number reaches an odd length such as for example 10000. This part gets handled by the else block from `if (num_digits % 2) == 0`. We know that our previous number had a digit length of 4, and 10000 has a digit length of 5, the next range of invalid ids will start at 100000. 100000 is made up of $1 * 10^5 + 0 * 10^4 + 0 * 10^3 + 0 * 10^2 + 0 * 10^1 + 0 * 10^0$ and our current number has a length of 5, we see that matches the power of the beginning range of our invalid ids. thus we can simply assign $10^{digit_length} = 10^5 = 100000$ to our current number, like below. 

```
else {
    let next_pow10 = 10_i64.pow(num_digits);
    current_num = next_pow10;
```

And this is how we've solved part 1.

## Part 2

## Problem solving approach part 2