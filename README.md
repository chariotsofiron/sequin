# Sequin

A library for identifying sequences of rational numbers. It tries to find the simplest rule that fits the observed sequence. Probably not very useful but I've had to solve many of these in online assessments and wondered how much it could be automated.


# Sequences

- Binomial: `S(n) = a*S(n-1) + b`
- Alternator: `S(n) = a*S(n-1) + b if n|2 else c*S(n-1) + d`
- Fibonacci: `S(n) = a*S(n-2) + b*S(n-1) + c`
- Factor
- Difference
- Zipped
- OEIS


# Examples

```shell
$ sequin 3, 8, 23, 68
203

# start at 1/2, multiply by 1/2 each time
$ sequin -n 3 --info 0.5 1/4 1/8
Binom(1/2, 1/2, 0)
1/16
1/32
1/64

$ sequin --info 2, 0, 1, 3, 4, 2, 3, 5, 6, 4, 5, 7
Zipped([Diff(2, Binom(-1, -1, 2)), Diff(0, Binom(3, -1, 2))])
8
```