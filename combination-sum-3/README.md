# Combination sum III

Given `k` and `n`, find every combination of `k` numbers in the range `1..=9` whose sum is `n`.

Examples:
```
k = 2, n = 8

solution is [[1, 7], [2, 6], [3, 5]]
```

```
k = 3, n = 9

solution is [[1, 2, 6], [1, 3, 5], [2, 3, 4]]
```

Here i used a recursive algorithm with backtracking.

Runtime 0ms, 100% fastest (no idea about runtime complexity).
memory usage not great but not bad, should be space complexity `O(k)` since it recurses max k+1 steps.