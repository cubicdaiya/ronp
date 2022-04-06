# gonp

ronp is a diff algorithm implementation in Rust.

# Algorithm

The algorithm `ronp` uses is based on "An O(NP) Sequence Comparison Algorithm" by described by Sun Wu, Udi Manber and Gene Myers.
An O(NP) Sequence Comparison Algorithm(following, Wu's O(NP) Algorithm) is the efficient algorithm for comparing two sequences.

## Computational complexity

The computational complexity of Wu's O(NP) Algorithm is averagely O(N+PD), in the worst case, is O(NP).

# Examples

## strdiff

```
$ make strdiff ARGS="abc abd"
...
A:abc
B:abd
editdistance: 2
```
