# (ax + b) where x is prev term

## 3, 8, 23, 68, 203

a(n) = 3 * a(n-1) - 1

## 18, 108, 648, 3888

a(n) = 6 * a(n-1)

## 68 36 20 12 8

previous term, divide by 2, add 2

a(n) = 1/2 * a(n-1) + 2

## 8, 20, 50, 125

a(n) = 5/2 * a(n-1)

## 2, 5, 6, 20, 18, 80, 54, 320, 162, 1280, 486

{ 3 * a(n-1), 4 * a(n-1) }

## 52 56 48 64 32

Take first differences: +4, -8, +16, -32
a(n) = -2 * a(n-2)


# Linear differences

## 9, 73, 241, 561, 1081, 1849, 2913

3rd differences converge

## -3, 3, 27, 69, 129, 207

2nd differences converge

## 2, 0, 1, 3, 4, 2, 3, 5, 6, 4, 5, 7, 8, 6

4 interleaved linear sequences

## 31, 23, 15, 27, 20, 13, 23, 17, 11, 19, 14, 9

3 interleaved linear sequences

## 10 45 15 38 20 31

2 interleaved linear sequences (+5, -7)

# Fibonacci

## 34 -21 13 -8 5 -3

a(n) = a(n-2) + a(n-1)

a(2) = a(0) + a(1) = 34 + -21 = 13
a(3) = a(1) + a(2) = -21 + 13 = -8
a(4) = a(2) + a(3) = 13 + -8 = 5

## 1 0 1 -1 2 -3

a(n) = a(n-2) - a(n-1)
a(2) = a(0) - a(1) = 1 - 0 = 1
a(3) = a(1) - a(2) = 0 - 1 = -1
a(4) = a(2) - a(3) = 1 - -1 = 2

## 13, -21, 34, -55, 89, -144

a(n) = a(n-2) - a(n-1)
34 = 13 - -21
-55 = -21 - 34
89 = 34 - -55
-144 = -55 - 89


# Alternators

## 6, 12, 4, 8, 2.66

*2, /3, *2, /3

## 18 6 24 8 32

/3 *4

## 3 12 24 33 66

+9 *2 +9 *2

## 230 460 46 92 9.2

*2, /10, *2, /10


# Cultural

## 3 5 7 11 13

The primes: a(n) = prime(n+1)

## 1 8 27 64 125 216

The cubes: 1^3, 2^3, 3^3, 4^3, 5^3, 6^3


## 3, 4, 6, 8, 12, 14

primes + 1

# Prev term with n expression

## 1 10 37 82 145 226

a(n) = a(n-1) + (2n-1)*9
82 = a(3) = a(2) + 5*9
prev + odds * 9

## -2 1 6 13 22 33

a(n) = a(n-1) + (2n+1)

## 15, 29, 56, 108, 208

x2-1, x2-2, x2-4, x2-8, etc.
a_n = 2 * a(n-1) - 2^n
double previous, minus 2^n

## 11, 11, 22, 66, 264

a(n) = a(n-1) * n
a(2) = a(1) * 2 = 11 * 2 = 22



# just tricky

## 7, 26, 124, 342, 1330

2^3-1, 3^3-1, 5^3-1, 7^3-1, 11^3-1
a_n = prime(n)^3 - 1

- primes, cube, minus 1

## 8 16 24 36 48

3^2-1, 4^2, 5^2-1, 6^2, 7^2-1
a(n) = {(2n+3)^2-1), (2n)^2}



# Not sure how to approach these

## 1 11 21 1211 111221

## 1 2 2 4 8 32

a(n) = a(n-1) * a(n-2)