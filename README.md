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