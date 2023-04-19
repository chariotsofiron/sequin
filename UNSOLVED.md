## 52 56 48 64 32

first differences: +4, -8, +16, -32
a(n) = -2A(n-1)



# Cultural

## 3 5 7 11 13

The primes: a(n) = prime(n+1)

## 1 8 27 64 125 216

The cubes: 1^3, 2^3, 3^3, 4^3, 5^3, 6^3

## 23 21 24 19 26 15 28 11 30 7 36
primes alternating
-2 +3 -5 +7 -11 +13 -17 +19 -23 +29 -31


## 3, 4, 6, 8, 12, 14

primes + 1

# Prev term with n expression

## 1 10 37 82 145 226

a(n) = a(n-1) + (2n-1)*9
82 = a(3) = a(2) + 5*9
prev + odds * 9

## -2 1 6 13 22 33

a(n) = a(n-1) + (2n+1)
prev + odds

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

one one, two ones, one two one one, one one two two one one, etc.
## 1 2 2 4 8 32

a(n) = a(n-1) * a(n-2)