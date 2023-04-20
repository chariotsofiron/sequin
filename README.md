# Sequin

A library for identifying and generating sequences of numbers. The goal is to be able to solve 80% of the problems one might encounter in an IQ test.

It tries to find the simplest rule that fits the observed sequence.

## Strategy

Sequence should have at least 4 terms

1. Linear differences
2. Binomial recursive: a(n) = c * a(n-1) + d
3. Fibonacci: a(n) = c * a(n-1) + d * a(n-2)
4. Cultural: primes, squares, pi digits, etc.

## Meta strategies

Strategies that combine multiple strategies.

- Zipped
- One mapped


# solved by oeis

## 3, 4, 6, 8, 12, 14

primes + 1

## 1 11 21 1211 111221

one one, two ones, one two one one, one one two two one one, etc.

## 3 5 7 11 13

The primes: a(n) = prime(n+1)