# Sequin

A library for identifying sequences of numbers. It tries to find the simplest rule that fits the observed sequence.


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

$ sequin --info 2, 5, 6, 20, 18, 80, 54, 320
Zipped([Binom(2, 3, 0), Binom(5, 4, 0)])
162
```