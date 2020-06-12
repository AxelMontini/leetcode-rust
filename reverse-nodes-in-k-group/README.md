`0ms` Rust solution, using constant memory. Never allocates on the heap.

I've yet to begin my university year and i'm not sure how to find an algorithm time complexity, but I would guess it is worst-case `O(2N + k - 1)` (iterates over `N` elements once to reverse them, twice to traverse the nodes and move the `last` reference; then it might have to undo the reverse on the last `k-1` nodes, in case the length is not a multiple of `k`).

So the time complexity is `O(N)` (where `N` is the amount of nodes in the head).