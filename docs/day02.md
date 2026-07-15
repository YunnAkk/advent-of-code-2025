# Day 2: TITLE
## Part 1
TODO

## Problem Solving Approach: Part 1
My initial approach was to split the input number in half, given a number such as $9027$, this yields $90$ for the left half and $27$ for the right half and then compare the raw byte/ASCII value of the two halves. If they are identical, the number is an invalid ID, and we accumulate it into a running total. We then increment to $9028$ and repeat. While this could work, it's inefficient. It performs a comparison for every number in the range. For a range containing $10\,000$ numbers, this results in $O(n)$ comparisons where $n$ is the size of the range, here that would correspond to $O(10\,000)$.

This inefficiency is particularly apparent for numbers with an odd digit count. Consider the number $10721$. Splitting it yields either a left half of $10$ and a right half of $721$ or a left half of $107$ and a right half of $21$. In neither case can the halves match, so no number with an odd digit count can ever be an invalid ID. Every comparison within such a range is wasted work, and we should skip these ranges entirely.

This observation leads to the core idea. Given an input number, we first check whether its digit count is even. For example, numbers in the range $4000$ – $9999$ have four digits and can contain invalid IDs, whereas numbers in the range $10000$ – $99999$ have five digits and cannot. This motivates a simple conditional, when `(num_digits % 2) == 0` is `true`, the number lies in a range that may contain invalid IDs and we proceed with the splitting logic. in the `else` branch, we skip ahead to the next power of ten where invalid IDs can resume.

Let us now examine the even length case in detail. Consider the range $4810$ – $100000$. Taking the number $4810$ and splitting it into halves gives a left half of $48$ and a right half of $10$. Holding the left half fixed three scenarios can arise: the left half is greater than the right (e.g. $4810$ where $48 > 10$), the two halves are equal (e.g. $4848$, where both halves are $48$), or the left half is less than the right half (e.g. $4870$, where $48 < 70$). Each scenario corresponds to a distinct branch in the algorithm.

### Case 1: Left $>$ Right
Let our current number be $4810$, we compute the difference $\text{left} - \text{right} = 48 - 10 = 38$. Adding this to the current number yields $4810 + 38 = 4848$, at which point the two halves are equal and we have located an invalid ID. Implementation:

```rust
if left_half > right_half {
    current_num += left_half - right_half;
}
```

### Case 2: Left $=$ Right

The current number is an invalid ID, so we add it to the running total. The interesting part is advancing efficiently. Given $4848$, we know the next invalid ID is $4949$. A naive increment by one and check loop would require $4949 - 4848 = 101$ iterations, $100$ of which are wasted.

We can do better by exploiting positional notation. Any natural number $N$ can be expressed in base $b$ as:

$$N = \sum_{i=0}^{n} d_i \cdot b^i \quad \text{where } 0 \leq d_i < b \text{ and } b \in \mathbb{Z}_{\geq 2}$$

Variables:
- $N$: The total value of the number.
- $b$: The base (radix) of the number system.
- $n$: The index of the highest order digit (where the total number of digits is $n+1$).
- $i$: The positional index, starting at $0$ for the least significant digit.
- $d_i$: The specific digit at position $i$.

Constraints:
- $0 \leq d_i < b$: This range ensures that every number has a unique representation. It requires that any value equal to or greater than the base must carry over to the next position $b^{i+1}$. For example, in base $10$, the value of ten must be written as $1 \cdot 10^1 + 0 \cdot 10^0$ ($10$) rather than $0 \cdot 10^1 + 10 \cdot 10^0$, which would be the case if $d_i$ were allowed to equal $b$.
- $b \in \mathbb{Z}_{\geq 2}$: The base must be an integer greater than or equal to $2$. A base of $0$ or $1$ fails to provide the necessary non zero digits ($d_i$) to represent positive quantities through powers of the base. If $b=1$, then $1^i = 1$ for all $i$, and the only valid digit satisfying $d_i < 1$ is $0$, making it impossible to represent any number other than zero.

To illustrate, representing $4848$ in base $10$ with index $n = 3$ gives the expansion $(4 \cdot 10^3) + (8 \cdot 10^2) + (4 \cdot 10^1) + (8 \cdot 10^0) = 4000 + 800 + 40 + 8 = 4848$, demonstrating how each digit $d_i$ maps to its corresponding power of the base to reconstruct the total value $N$.

This structure is exploited to skip directly from $4848$ to the next invalid id of $4949$, without checking every value in between. Consider only the left half of the number, $48$, which corresponds to $4800$ when the right half is zeroed out. Incrementing the left half by one amounts to adding $10^{n/2} \quad \text{where } n = \text{length of the full number}$, to the full number. With $n = 4$ digits, this is $10^{4/2} = 10^2 = 100$. Adding $100$ to $4848$ yields $4948$. A further addition of $1$ forces the right half to match the incremented left half, producing $4949$. In a single arithmetic step, we bypass every valid ID in between. Implementation:

```rust
else if left_half == right_half {
    total_sum += current_num;
    current_num += half_base + 1;
}
```

### Case 3: Left $<$ Right
Consider $4870$, where the left half is $48$ and the right half is $70$. Since $48 < 70$, no invalid ID exists with a left half of $48$. The right half has already surpassed the point where equality was possible. The next invalid ID must therefore have a left half of $49$, computed as $`\text{left\_half} + 1 = 48 + 1 = 49`$.

To reach this next range, the right half must increment until it reaches $b^{n/2}$, the half-base of $100$. This is precisely the carry threshold established by the positional notation constraint. A half consisting of $n/2$ digits rolls over to $0$ and increments its neighbor when it reaches $b^{n/2}$. The remaining distance before this carry occurs is the right half's deficit from the base $`\text{right\_deficit} = \text{half\_base} - \text{right\_half} = 100 - 70 = 30`$. Adding this deficit to the current number triggers the carry $4870 + 30 = 4900$, where the left half is now $49$. The final step is to place the new left half into the right half position, which is a simple addition $4900 + 49 = 4949$. This yields the next invalid ID. Implementation:

```rust
else if left_half < right_half {
    let next_left_half = left_half + 1;
    let right_deficit = half_base - right_half;
    current_num += right_deficit + next_left_half;
}
```

### Handling Odd-Length Ranges
Observe that in both the $\text{left} > \text{right}$ and $\text{left} < \text{right}$ cases, the number is adjusted to land on an invalid ID. Only in the equality branch is it added to the sum and then advanced, this is done to avoid summing the same ID twice. This loop continues until the number reaches an odd digit count. For example, $10000$ which has five digits. Since no odd length number can be an invalid ID, we skip directly to the next even length range. The number $10000$ has a digit length of $5$, the next range of invalid IDs begins at $100000$, which is $10^5$. Expressed in positional notation, $100000 = 1 \cdot 10^5 + 0 \cdot 10^4 + 0 \cdot 10^3 + 0 \cdot 10^2 + 0 \cdot 10^1 + 0 \cdot 10^0$ and the exponent of the leading term matches the current digit length. More generally, if the current digit length is $k$ (odd), we assign $10^k$ to the current number, jumping to the start of the next even-length range. Implementation:

```rust
else {
    let next_pow10 = 10_i64.pow(num_digits);
    current_num = next_pow10;
}
```

This completes the solution for Part 1.

## Part 2
TODO

## Problem solving approach part 2
While the bilateral partitioning used in Part 1 effectively identifies IDs that repeat exactly twice, it lacks the flexibility to generalize to sequences with arbitrary repetition frequencies. To identify invalid IDs defined by repeating sequences, we leverage the mathematical property of periodicity. A periodic ID of total length $L$ is formed by a pattern (prefix) of length $P$ that repeats $k$ times. This relationship is defined by

$$L = P \cdot k$$

### Periodic Analysis and Constraints
The problem specifies that a pattern must repeat at least twice, establishing the constraint $k \ge 2$. From

$$P = \frac{L}{k}$$

and the inverse proportionality between $P$ and $k$, the maximum possible pattern length $P_{\text{max}}$ for any length $L$ is reached when $k$ is at its minimum

$$P_{\max} = \frac{L}{k_{\min}} = \frac{L}{2}$$

A further constraint is that $P$ must divide $L$ exactly without remainder. The candidate pattern lengths for a given $L$ are therefore the divisors of $L$ in the range $1 \le P \le L/2$.

### The Repunit Multiplier and Geometric Series
Any number in positional notation expands as a sum of its digits weighted by powers of the base. For a periodic number, this expansion factors neatly. Consider $424242$ with pattern $42$ and $P = 2$:

$$424242 = 42 \cdot 10^4 + 42 \cdot 10^2 + 42 \cdot 10^0 = 42 \cdot (10^4 + 10^2 + 1) = 42 \cdot 10101$$

The factor $10101$ is a repunit multiplier ($M$). This value acts as a scaling constant that distributes the $P$ digit pattern across the full $L$ digit span at regular intervals. In base 10 arithmetic, multiplying the pattern by $10^{i \cdot P}$ shifts the pattern exactly $i \cdot P$ positions to the left. By summing $k$ such terms where the exponents are $\{0, P, 2P, \dots, (k-1)P\}$, each instance of the pattern is placed sequentially so that the end of one instance is perfectly adjacent to the start of the next. 

Because each term in the multiplier $M$ is a power of $10^P$, $M$ is a geometric series with a common ratio $r = 10^P$ and a total of $k$ terms

$$M = \sum_{i=0}^{k-1} (10^P)^i = 1 + 10^P + 10^{2P} + \dots + 10^{(k-1)P}$$

The closed form of a geometric series $S = a + ar + ar^2 + \cdots + ar^{n-1}$ is derived by multiplying $S$ with $r$

$$rS = r(a + ar + ar^2 + \cdots + ar^{n-1})$$

$$= ar + ar^2 + ar^3 + \cdots + ar^{n}$$

Then subtracting one equation from the other

$$S - rS = a + ar + ar^2 + \cdots + ar^{n-1} - (ar + ar^2 + ar^3 + \cdots + ar^{n})$$

$$S - rS = a - ar^{n}$$

$$S(1 - r) = a(1 - r^{n})$$

$$S = a \frac{1 - r^{n}}{1 - r} = a\frac{r^{n} - 1}{r - 1}$$

where the last equality multiplies numerator and denominator by $-1$. Substituting the starting value $a = 1$, the common ratio $r = 10^P$, and the number of terms $n = k = L/P$

$$M = \frac{(10^P)^{L/P} - 1}{10^P - 1} = \frac{10^L - 1}{10^P - 1}$$

The last step applies the exponent rule $(10^P)^{L/P} = 10^{P \cdot (L/P)} = 10^L$. The simplification $P \cdot (L/P) = L$ follows directly from the definition $k = L/P$ combined with the constraint $P \cdot k = L$.

Example: For $424242$ with $P = 2$, $L = 6$

$$M = \frac{10^6 - 1}{10^2 - 1} = \frac{999999}{99} = 10101 \qquad 42 \cdot 10101 = 424242$$

### Code Implementation
The implementation loops over total lengths $L$ from start_len to end_len and for each $L$, over pattern lengths $P$ from $1$ to $L/2$

```rust
let start_len = get_number_length(start);
let end_len = get_number_length(end);
for l in start_len..=end_len {
    let r_n = 10_i64.pow(l);          // 10^L
    let numerator = r_n - 1;          // 10^L - 1
    // ...
    for p in 1..=(l / 2) {
        if l % p != 0 { continue; }   // P must divide L
        let block = 10_i64.pow(p);    // 10^P
        let repunit_multiplier = numerator / (block - 1);  // M = (10^L - 1) / (10^P - 1)
        // ...
    }
}
```

Note that unlike Part 1, odd length numbers are not skipped. An odd length ID can be invalid when its pattern length is also odd (For example the number $111$ with $P = 1$, $k = 3$, or $824824824$ with $P = 3$, $k = 3$).


### Skip Ahead Optimization
As in Part 1, iterating through every possible ID to check for periodicity is computationally expensive. Instead, we utilize the monotonicity of the function $f(\text{pattern}) = \text{pattern} \cdot M$ to solve for the valid pattern range within a given interval $[start, end]$.

For the lower bound we require the smallest integer $p$ such that $p \cdot M \geq start$, i.e. $\lceil start / M \rceil$. Since Rust's integer division truncates toward zero, we synthesize ceiling division via the identity

$$\left\lceil \frac{a}{b} \right\rceil = \left\lfloor \frac{a + b - 1}{b} \right\rfloor$$

yielding the expression 

$$\frac{(start + M - 1)}{M}$$

Adding $M - 1$ to the numerator ensures that any non zero remainder pushes the quotient up to the subsequent integer, effectively synthesizing the ceiling function through integer truncation, while integral multiples remain unaffected.

For the upper bound we require the largest integer $p$ such that $p \cdot M \leq end$, calculated as $\lfloor end/M \rfloor$. Rust's integer division already floors for non negative operands, which is exactly $\lfloor end/M \rfloor$.

The asymmetry, ceiling on the lower bound and floor on the upper bound reflects the inequality directions. We seek "at least start" and "at most end".

Finally, we clamp the division results against the physical constraints of a $P$ digit range. A $P$ digit pattern cannot have a leading zero, a pattern such as 012 would produce 012012, which as an integer equals 12012 (a 5 digit number rather than 6), thereby violating the fixed length assumption that the ID has length $L$. The valid pattern range is therefore

$$[10^{P-1}, 10^P - 1]$$

The lower bound is clamped upward against $`10^{P-1}`$ (the smallest valid $P$ digit number), while the upper bound is clamped downward against $`10^P - 1`$ (the largest valid $P$ digit number). This prevents overflow into $(P+1)$ digit patterns belonging to a different $P$ iteration.

```rust
let pattern_min = 10_i64.pow(p - 1); // 10^(P-1): smallest P-digit number (no leading zero)
let pattern_max = block - 1;         // 10^P - 1: largest P-digit number

let valid_min = std::cmp::max(pattern_min,
                (start + repunit_multiplier - 1) / repunit_multiplier);
let valid_max = std::cmp::min(pattern_max,
                end / repunit_multiplier);

if valid_min > valid_max {
    continue;
}
```

If `valid_min > valid_max`, then no $P$ digit pattern yields a product in $[start, end]$ for this $(L, P)$ pair and the inner loop body is skipped entirely.

### Representation Redundancy and Pattern Primitivity
Next we have to handle a critical challenge in this implementation which arises from the potential for representation redundancy. As established, a periodic ID of length $L$ can be decomposed into a pattern of length $P$ and a repetition factor $k$, such that $L = P \cdot k$. However, this decomposition is not necessarily unique. For instance, the ID $111111$ can be validly described by several $(P, k)$ pairs:

| Pattern | $P$ (length) | $k$ (repetitions) |
|---------|----------|--------------|
| $1$     | $1$      | $6$          |
| $11$    | $2$      | $3$          |
| $111$   | $3$      | $2$          |

If the algorithm iterates through all possible pattern lengths and repetition factors without a uniqueness constraint, a single value would be aggregated into the total sum multiple times leading to an incorrect result. To ensure each periodic number is counted exactly once, we must enforce the condition that $P$ is a primitive pattern. A pattern is defined as primitive if it cannot be further decomposed into a smaller repeating sub unit. In the example above, only $P=1$ is primitive. $11$ and $111$ are periodic repetitions of the fundamental unit $1$.

#### The is_primitive_pattern Verification
Before adding a value to the summation, the function `is_primitive_pattern` is invoked to validate the primitivity of the current pattern. The logic proceeds in two steps:

**1. Candidate Period Identification**: The function iterates through all possible sub periods $T$ where $1 \leq T < P$. A sub period $T$ is only a candidate for periodicity if it is a proper divisor of $P$. If $T$ does not divide $P$ evenly, it is mathematically impossible for the pattern to be a perfect repetition of a block of length $T$.

**2. Periodicity Verification via Modular Indexing**: To determine if a pattern is a repetition of a candidate sub period $T$, we verify if the digit at any index $i$ is identical to the digit at the corresponding position within the first potential block. This is implemented using modular arithmetic

```rust
for sub_period in 1..pattern_len {
    if pattern_len % sub_period != 0 {
        continue;
    }

    let mut is_periodic = true;
    for i in 0..digits.len() {
        if digits[i] != digits[i % sub_period] {
            is_periodic = false;
            break;
        }
    }

    if is_periodic {
        return false;  // a smaller period exists -> not primitive
    }
}

return true;  // no sub-period matched -> pattern is primitive
```

In this loop, `digits[i % sub_period]` maps every index $i$ back to its relative position within the first $T$ digits, representing the aforementioned first potential block. If this condition holds for all $i$, the pattern is proven to be periodic (non primitive), and the function returns `false`.

**Example**: pattern $= 123123$, $P = 6$

When evaluating the pattern $123123$, the function tests the following:

- At $T = 1$: The algorithm compares $\text{digits}[1] = 2$ against $\text{digits}[1 \bmod 1] = \text{digits}[0] = 1$. Since $2 \neq 1$, the pattern is not a repetition of length 1.
- At $T = 2$: The algorithm compares $\text{digits}[2] = 3$ against $\text{digits}[2 \bmod 2] = \text{digits}[0] = 1$. Since $3 \neq 1$, it is not a repetition of length 2.
- At $T = 3$: The algorithm compares every $\text{digits}[i]$ to $\text{digits}[i \bmod 3]$.
    - $i=3: \text{digits}[3] = 1 = \text{digits}[0] = 1$
    - $i=4: \text{digits}[4] = 2 = \text{digits}[1] = 2$
    - $i=5: \text{digits}[5] = 3 = \text{digits}[2] = 3$

All indices satisfy the condition. The pattern is identified as periodic with a period of 3, and is thus rejected.

By filtering for primitivity, the algorithm ensures that $111111$ is only processed when $P = 1$ and $k = 6$. Subsequent attempts to process it via patterns $11$ or $111$ (which would correspond to $P = 2$ and $P = 3$) are caught by `is_primitive_pattern` and excluded, maintaining the integrity of the summation.

And finally, once a primitive pattern is confirmed, the full ID is reconstructed and accumulated

```rust
total_sum += pattern * repunit_multiplier;
```