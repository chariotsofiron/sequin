## Sequence examples

- https://www.fibonicci.com/numerical-reasoning/number-sequences-test/hard/
- https://www.fibonicci.com/numerical-reasoning/number-sequences-test/hard/explanation/

## Links

Solver using differences
- https://alteredqualia.com/visualization/hn/sequence/
- https://sequencecalculators.com/find-the-sequence-and-next-term-calculator/
- https://www.mathsisfun.com/algebra/sequences-finding-rule.html
- https://pypi.org/project/sequences/


Take first differences and find patterns
Take first ratios and find patterns


Can alternator be replaced by fibonacci?
- 8, 1, 17, 19, 53, 91, 197: Alternator(8, 2, -15, 2, 15) or Fib(8, 1, 2, 1)

It is a superset!


a * 18 + b * 6 + c = 24
a * 6 + b * 24 + c = 8
a * 24 + b * 8 + c = 32
=> a=4/3, b=0, c=0

3, 12, 24, 33, 66
a * 3 + b * 12 + c = 24
a * 12 + b * 24 + c = 33
a * 24 + b * 33 + c = 66
a * 33 + b * 66 + c = 99
=> a=5, b=-3, c=45

66+9 = 75




2,−4,6,−8, 10
a * 2 + b * -4 + c = 6
a * -4 + b * 6 + c = -8
a * 6 + b * -8 + c = 10
=> a=-1, b=-2, c=0

1, 3, 6, 8, 11
a * 1 + b * 3 + c = 6
a * 3 + b * 6 + c = 8
a * 6 + b * 8 + c = 11
=> a=1, b=0, c=5

3, 8, 23, 68, 203
b * 3 + c = 8
b * 8 + c = 23


2, 5, 6, 20, 18, 80, 54, 320, 162
a * 2 + b * 5 + c = 6
a * 5 + b * 6 + c = 20
a * 6 + b * 20 + c = 18


# Solved by oeis

## 3, 4, 6, 8, 12, 14

primes + 1

## 1 11 21 1211 111221

one one, two ones, one two one one, one one two two one one, etc.

## 3 5 7 11 13

The primes: a(n) = prime(n+1)


# Unsolved

## 15, 29, 56, 108, 208

x2-1, x2-2, x2-4, x2-8, etc.
a_n = 2 * a(n-1) - 2^n
double previous, minus 2^n

## 11, 11, 22, 66, 264

a(n) = a(n-1) * n
a(2) = a(1) * 2 = 11 * 2 = 22

## 7, 26, 124, 342, 1330

2^3-1, 3^3-1, 5^3-1, 7^3-1, 11^3-1
a_n = prime(n)^3 - 1

- primes, cube, minus 1

## 89, 76, 65, 52

## 87, 165, 726, 1353

a(n) = a(n-1) + reverse(a(n-1))

## 15, 47, 31, 54, 73

15473 repeated


## 512, 64, 8, 8, 1

a(n) = a(n-2) / a(n-1)

## 1 2 2 4 8 32

a(n) = a(n-1) * a(n-2)

## 23, 21, 24, 19, 26, 15, 28, 11, 30, 7, 36

primes alternating
-2 +3 -5 +7 -11 +13 -17 +19 -23 +29 -31