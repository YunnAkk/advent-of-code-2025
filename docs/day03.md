# Day 3: TITLE
## Part 1
TODO

## Problem Solving Approach: Part 1
Each bank is a string of digits $d_0, d_1, \dots, d_{n-1}$. Because the battery contributing the tens digit must physically precede the battery contributing the ones digit, any valid combination is constrained to indices $i < j$, where $i$ and $j$ represent indices over the string of digits and the joltage it produces is $v(i,j) = 10 \cdot d_i + d_j$. This ordering constraint is what makes the problem a subsequence selection rather than a simple max/second-max query.

A brute force solution would check every valid pair $(i, j)$ with $i < j$. For a bank of $n$ digits, the count of such pairs is $(n-1) + (n-2) + \dots + 1$, since the digit at position $0$ can pair with any of the $n-1$ digits after it, the digit at position $1$ with any of the remaining $n-2$, and so on down to the second-to-last digit, which pairs only with the last. This sum has the closed form $\binom{n}{2} = \frac{n(n-1)}{2}$ (the standard combinatorial count of unordered pairs drawn from $n$ elements) which expands to $\frac{1}{2}n^2 - \frac{1}{2}n$. As $n$ grows, the $n^2$ term dominates, so the brute force cost scales as $O(n^2)$. For the joltage ratings given here $n$ is small, but the structure of the problem admits an $O(n)$ single-pass solution, which is what the `find_largest_two_digits` function implements.

### The Place Value Dominance Lemma
The key observation is that the tens digit dominates the ones digit. To prove that a larger tens digit $d_i$ is always superior to a smaller tens digit $d_i'$, we compare the minimum possible value of $d_i$ against the maximum possible value of $d_i'$.

Let $d_i$ and $d_i'$ be two candidate tens digits where $d_i > d_i'$. Since these are integers, we know that $d_i \geq d_i' + 1$. The smallest value $d_i$ can produce is when the ones digit is $0$

$$V_{min} = 10 \cdot d_i + 0 = 10 \cdot d_i \geq 10 \cdot (d_i' + 1)$$

The largest value $d_i'$ can produce is when the ones digit is $9$

$$V_{max} = 10 \cdot d_i' + 9$$

Since $10 \cdot (d_i' + 1) > 10 \cdot d_i' + 9$, the minimum of the larger digit is strictly greater than the maximum of the smaller digit

$$10 \cdot d_i + d_j \geq 10 \cdot (d_i' + 1) > 10 \cdot d_i' + 9 \geq 10 \cdot d_i' + d_j'$$

Thus increasing the tens digit by even a single unit outweighs any possible change in the ones digit. This means the two decisions are not symmetric, we should never trade a larger tens digit for a larger ones digit. The optimization therefore decomposes into a strict priority order, first maximize $d_i$ and only among indices achieving that maximum do we then maximize $d_j$ subject to $j > i$.

This also resolves ties correctly. Consider $d = [9, 9, 1]$. Choosing $i = 0$ (the first $9$) leaves the suffix $\{9, 1\}$ available for $j$, yielding $99$. Choosing $i = 1$ (the second $9$) leaves only $\{1\}$, yielding $91$. Hence among duplicate maxima, the leftmost occurrence is strictly preferable, since it maximizes the size of the candidate suffix from which $j$ can be drawn. This is why the update condition in the implementation is a strict `current > left` rather than `>=`: a tie never displaces an earlier tens-digit candidate, preserving the largest possible remaining suffix for the ones digit.

### From Lemma to Linear Scan
The lemma justifies maintaining two running quantities while scanning left to right:

- `left`: the maximum digit seen so far that still has at least one digit after it (a candidate tens digit).
- `right`: the maximum digit seen strictly after the current `left`'s position (the best available ones digit for that tens digit).

Whenever a strictly larger tens candidate is found, it necessarily produces a better result than the current best regardless of what `right` currently holds, so `right` is safely reset to just the very next digit and rebuilt from there:

```rust
for (&current, &next) in digits_vector
    .iter()
    .zip(digits_vector.iter().skip(1))
    .skip(1)
{
    if current > left {
        left = current;
        right = next;
    } else if next > right {
        right = next;
    }
}
```

Note the asymmetry in what is compared, `current` (the digit at the present index) is tested against `left` to decide whether the tens digit should advance, while `next` (the following digit) is tested against `right` to decide whether a better ones digit has appeared. This ordering is what guarantees `right` is only ever populated from an index strictly greater than `left`'s index. $i < j$ is maintained implicitly by the iteration structure rather than by an explicit index comparison.

A subtler consequence of the outer `.skip(1)` on the zipped iterator is that `current` ranges only over $d_1, \dots, d_{n-2}$, the last digit $d_{n-1}$ is never offered as a candidate for `left`. This is correct by construction: the last digit has no successor to pair with as a ones digit, so it must never be promoted to the tens position. The base case handles the symmetric edge at the front of the array by initializing `left = digits_vector[0]` and `right = digits_vector[1]` before the loop begins, since $d_0$ is a valid tens digit candidate that the skipped first pair of the iterator would otherwise miss.

## Part 2
TODO

## Problem Solving Approach: Part 2

[Go to Day 3 Code](../src/days/day03.rs)  
[Go to Day 4](day04.md)