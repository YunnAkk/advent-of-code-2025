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

## Problem solving approach part 2