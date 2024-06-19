# Caboose Number Search
This is a simple program written in rust to look for Caboose numbers (better known as Euler's lucky numbers).
These are numbers c such that for all n < c, n^2 - n + c is prime.

My simple implementation uses a pre-computed list of prime numbers up to 100,000,000, and then checks all numbers from 1 to c to see if they are caboose numbers.

Any improvements or suggestions are welcome.