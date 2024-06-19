# Caboose Number Search
This is a simple program written in rust to look for Caboose numbers (better known as Euler's lucky numbers).
These are numbers c such that for all n < c, n^2 - n + c is prime.

My simple implementation uses a pre-computed list of prime numbers up to 100,000,000, and then checks all numbers from 1 to c to see if they are caboose numbers.

Any improvements or suggestions are welcome.


# How to use

If you don't have rust installed, there are many sources online to help you do that.
Once you do have it, simply clone the repository, cd into it, and run the following command in the terminal:

```bash
cargo run
```
This will start printing out the process of finding the caboose numbers, showing the progress even if it's not finding any.
The program will automatically halt if it finds a caboose number bigger than 41.


# How it works

## Narrowing down the search field
Instead of checking all possible values of c, we can remove the huge majority of candidates by using the following facts (true for any c > 2):
1. in order for n^2 - n + c to be prime for n=1, c must be prime.
2. in order for n^2 - n + c to be prime for n=2, c must be the lower part of a twin prime pair. 

These two properties alone reduce our search field massively. 
We can extend this further using any n we want, but I'll stop here for this version.


## Checking the primality of n^2 - n + c
In order to check if a number is prime, when it's smaller than the last prime number we have on file, it's as simple as performing a binary search. 
I've implemented some logic to narrow down the field of search for these, like not looking at any primes smaller than c itself, but there is probably more work to be done there. 