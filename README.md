# Sequin

Sequin is a program for identifying sequences of rational numbers. It tries to find the simplest rule that fits the observed numbers.


## Examples

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

## Types of sequences

- Binomial: `S(n) = a*S(n-1) + b`
- Alternator: `S(n) = if n|2 {a*S(n-1) + b} else {c*S(n-1) + d}`
- Fibonacci: `S(n) = a*S(n-2) + b*S(n-1)`
- Factor
- Difference
- Zipped
- OEIS: query database for known patterns

