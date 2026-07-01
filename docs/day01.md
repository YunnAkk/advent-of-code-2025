# Day 1
## Intro and Part 1
TODO

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="resources/dial_dark.png">
  <source media="(prefers-color-scheme: light)" srcset="resources/dial_light.png">
  <img alt="Dial illustration" src="resources/dial_light.png">
</picture>

## Engineering the First Solution
Looking at the input file, it quickly became apparent that some instructions required turning the dial far more than a single full rotation (e.g. `R895`). This meant I needed to handle the dial's wrapping behavior carefully.

Because we only care about where the dial ends up relative to `0`, all the full rotations are essentially irrelevant for determining the final resting spot. Our dial has 100 positions (0 to 99), so a full rotation is exactly 100 clicks. This is the key to normalizing the dials position after any mathematical operation.

Parsing the direction is straightforward, I used a simple conditional check where `L` corresponds to subtraction and `R` corresponds to addition. We take the current dial position and either add or subtract the requested number of turns. However, standard arithmetic gets tricky when we subtract a number larger than our current position, resulting in a negative number.

To handle this, I wrote a `fn normalize_dial_position(pos: i32)` function. Once we apply the basic addition or subtraction, we take the result and apply a modulo (`%`) by our full rotation value (100). The goal is to filter out the redundant spins and figure out exactly where the dial stopped.

The body of the aforementioned normalize_position is `(pos % FULL_ROTATION + FULL_ROTATION) % FULL_ROTATION`, let's look at the math behind it. Let the position be $P$ (which can be any positive or negative integer) and the full rotation be $R$ (100).

1. $P \pmod R$ (`pos % FULL_ROTATION`), yields a value in the open interval $[-(R-1), R-1] = [-99, 99]$. Because this includes negative numbers, it falls outside our valid dial range of $[0, 99]$.
2. We add $R$ to the result of Step one (`+ FULL_ROTATION`). This maps our negative numbers to their correct positive equivalent (e.g., counting backward from 99). The interval shifts to $[1, 2R-1] = [1, 199]$.
3. While the lower bound is now fixed, the upper bound exceeds our dial's maximum. Which is why I've applied $\pmod R$ one final time. This safely brings any overflow back down, giving us an interval of $[0, 99]$.

## Part 2
TODO

## Adapting the Algorithm
Thanks to the modular state of the codebase, Only one new function is required to handle this updated logic. The core insight is to split each rotation into two components, **full revolutions** and the **remainder**.

Full revolutions: Every complete revolution of 100 clicks crosses 0 exactly once, regardless of the starting position. The count of full revolutions is simply `instruction.turns / FULL_ROTATION`, and I add this directly to the counter variable.

Remainder: After accounting for full revolutions, the remaining clicks (`instruction.turns % FULL_ROTATION`) might cause one additional crossing. To detect this, I compare the dial's position before and after applying the remainder and crucially, before normalization:

```
if prev_dial_pos > LOWER_BOUNDARY && prev_dial_pos < FULL_ROTATION
	&& (curr_dial_pos <= LOWER_BOUNDARY || curr_dial_pos >= FULL_ROTATION)
	{
		count += 1;
	}
```

The first and second condition (`prev_dial_pos > LOWER_BOUNDARY && prev_dial_pos < FULL_ROTATION`) ensures the dial didn't start at 0. This distinction matters because we only want to count moves to 0, not away from it. If the dial is already sitting at 0 and rotates away, that isn't a crossing. The last condition checks whether the unnormalized result has dropped below 0 (left rotation) or exceeded 99 (right rotation), indicating the dial swept through the boundary.

This check can produce at most one additional crossing because, after removing full revolutions, the remainder is strictly less than 100. The dial can wrap around the boundary at most once. For instance, if the dial is at 1 and rotates left by 99 (the maximum possible remainder), the unnormalized position is −98, which normalizes to 2, crossing 0 exactly once. The same logic holds symmetrically for rightward rotations from 99. After performing this check, I normalize the position and proceed to the next instruction. We repeat this for every instruction until we finally find the password.

[Go to Day 2](day02.md)  
[Go to Day 1 code](../src/days/day01.rs)