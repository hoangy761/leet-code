# Remove Element

LeetCode 27. `remove_element` takes `nums` and `val`, deletes every `val` in place, and returns how many elements are not `val`. The first `len_nums` slots hold those elements. Order may change. Slots after the return value are ignored.

## What this solution does

Two pointers. `left_index` walks from the left and stops on `val`. `right_index` walks from the last index and skips `val`s that are already at the end. A keeper at `right_index` is swapped into `left_index`, so the `val` moves to the discarded tail.

`len_nums` starts as `nums.len()` and is the value returned. It drops by 1 for every `val` the inner loop skips, and again after each swap.

## Walkthrough

1. Empty `nums`: return 0.
2. `right_index = len_nums - 1`, `left_index = 0`.
3. One element (`left_index == right_index`): return 0 if it equals `val`, else 1.
4. Outer loop: `left_index <= right_index`.
5. `nums[left_index] != val`: leave it, then `left_index += 1`.
6. `nums[left_index] == val`: inner loop while `right_index >= left_index` and `nums[right_index] == val`. Each step does `right_index -= 1` and `len_nums -= 1`. If `right_index == 0` inside that loop, index 0 is still `val` and the rest of the range was too, so return `len_nums - 1`.
7. Inner loop ends with `right_index < left_index`: the tail from `left_index` was all `val`. Return `len_nums`.
8. Otherwise swap `nums[left_index]` with `nums[right_index]`, then `len_nums -= 1`. The `val` now sits at the old `right_index`, outside the kept length.
9. The `right_index == 0` check after the swap does not run. A swap requires `nums[right_index]` to be a keeper, while `left_index == 0` in that same moment requires `nums[0] == val`. The all-`val` case already returned in step 6.
10. `right_index -= 1`, then `left_index += 1`, and the outer loop repeats.
11. Pointers cross: return `len_nums`.

## Example

`nums = [3, 2, 2, 3]`, `val = 3` (the first sample in the comment under `main`).

| left_index | right_index | len_nums | nums | what happened |
| --- | --- | --- | --- | --- |
| 0 | 3 | 4 | `[3, 2, 2, 3]` | start |
| 0 | 2 | 3 | `[3, 2, 2, 3]` | inner loop drops index 3 (`3`) |
| 0 | 2 | 2 | `[2, 2, 3, 3]` | swap index 0 with index 2 |
| 1 | 1 | 2 | `[2, 2, 3, 3]` | `right_index -= 1`, `left_index += 1` |
| 2 | 1 | 2 | `[2, 2, 3, 3]` | `nums[1]` is `2`, so `left_index` moves past `right_index` |

Return `2`. The kept prefix is `[2, 2]`.

`main` calls `[4, 5]` with `val = 5` and prints `1`. Index 0 is `4`, then the inner loop drops index 1 and `right_index < left_index`, so it returns through step 7. The array stays `[4, 5]`.

## Complexity

- Time: O(n). `left_index` only moves right and `right_index` only moves left, each at most `n` steps. The inner loop does not rescan.
- Space: O(1). A few index variables, swaps happen in `nums`.

## Edge cases in this code

- Empty input returns 0 before either index is used.
- A single element returns 0 or 1 and never enters the loops.
- `right_index == 0` inside the inner loop covers an array that is all `val`, including `[val]` only through the one-element branch above.
- `right_index < left_index` covers a suffix of `val` after some keepers are already in place, as in `main`.

## Cleaner approach

One write index is enough. Scan `i` from left to right. When `nums[i] != val`, copy it to `nums[k]` and increment `k`. Return `k`. Same O(n) time and O(1) extra memory, without the nested loop or the early returns.

```rust
let mut k = 0;
for i in 0..nums.len() {
    if nums[i] != val {
        nums[k] = nums[i];
        k += 1;
    }
}
k as i32
```
