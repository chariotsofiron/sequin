# Linear differences

## 9, 73, 241, 561, 1081, 1849, 2913

3rd differences converge

## -3, 3, 27, 69, 129, 207

2nd differences converge

## 2, 0, 1, 3, 4, 2, 3, 5, 6, 4, 5, 7, 8, 6

4 interleaved linear sequences

## 31, 23, 15, 27, 20, 13, 23, 17, 11, 19, 14, 9

3 interleaved linear sequences

## 10, 45, 15, 38, 20, 31

2 interleaved linear sequences (+5, -7)

## 1, 10, 37, 82, 145, 226

2nd differences converge
a(n) = a(n-1) + (2n-1)*9
82 = a(3) = a(2) + 5*9
prev + odds * 9

## -2, 1, 6, 13, 22, 33

2nd differences converge
a(n) = a(n-1) + (2n+1)
prev + odds

## 1, 8, 27, 64, 125, 216

3rd differences converge
The cubes: 1^3, 2^3, 3^3, 4^3, 5^3, 6^3


# Binomial

## 3, 8, 23, 68, 203

A(n) = 3A(n-1) - 1

## 18, 108, 648, 3888

A(n) = 6A(n-1)

## 68 36 20 12 8

A(n) = 1/2 * A(n-1) + 2

## 8, 20, 50, 125

A(n) = 5/2 * A(n-1)

## 2, 5, 6, 20, 18, 80, 54, 320, 162, 1280, 486

{ 3 * a(n-1), 4 * a(n-1) }


# Fibonacci

## 34 -21 13 -8 5 -3

A(n) = A(n-2) + A(n-1)

## 1, 0, 1, -1, 2, -3

A(n) = A(n-2) - A(n-1)

## 13, -21, 34, -55, 89, -144

A(n) = A(n-2) - A(n-1)


# Alternators

## 18, 6, 24, 8, 32

/3 *4

## 3, 12, 24, 33, 66

+9 *2 +9 *2

## 230 460 46 92 9.2

*2, /10, *2, /10



# Once Diffed

## 23, 25, 28, 32, 37, 43

+2, +3, +4, +5, etc.

## 52, 56, 48, 64, 32

first differences: +4, -8, +16, -32
a(n) = -2A(n-1)


## 23 21 24 19 26 15 28 11 30 7 36

primes alternating
-2 +3 -5 +7 -11 +13 -17 +19 -23 +29 -31

## 17, 32, 61, 118, 231

a(n) = 2a(n-1) - n - 1
a(0) = 17
a(1) = 2*17 - 2 = 32


## 8, 16, 24, 36, 48, 64

first differences: 8, 8, 12, 12, 16
alternator: +4, +0


3^2-1, 4^2, 5^2-1, 6^2, 7^2-1
a(n) = {(2n+3)^2-1), (2n)^2}




# Cultural

## 3, 4, 6, 8, 12, 14

primes + 1

## 1 11 21 1211 111221

one one, two ones, one two one one, one one two two one one, etc.

## 1 2 2 4 8 32

a(n) = a(n-1) * a(n-2)

## 3 5 7 11 13

The primes: a(n) = prime(n+1)