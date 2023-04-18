# Sequin

A library for identifying and generating sequences of numbers. The goal is to be able to solve 80% of the problems one might encounter in an IQ test.

It tries to find the simplest rule that fits the observed sequence.

## Strategy

1. Linear differences
2. Binomial recursive: a(n) = c * a(n-1) + d
3. Fibonacci: a(n) = c * a(n-1) + d * a(n-2)
4. Cultural: primes, squares, pi digits, etc.

## Meta strategies

Strategies that combine multiple strategies.

- Zipped
- One mapped


