# Readings by tag

A reading is one question asked of a number or a set of numbers. Each one you try counts toward the tally, whether or not it lands.

## Structure

These come from `numplay facts` and `numplay collatz`. They hold in any base and on any calendar.

- Whether the number is prime, and its position among the primes.
- Its prime factors.
- Its divisors: how many there are, and whether the smaller ones sum to less than the number (deficient), more (abundant) or exactly the number (perfect).
- The kinds on the tool's list: perfect, highly composite, Mersenne prime, twin prime, power of two, square, cube, triangular, Fibonacci, factorial.
- Its 3n+1 route: the steps, the peak, and where two routes join.

Two notes worth saying out loud:

- Joining routes are common. The routes of 6, 27 and 97 all share the road from 10.
- A prime's position can be prime too. 127 is the 31st prime, 31 is the 11th, 11 is the 5th, 5 is the 3rd, 3 is the 2nd and 2 is the 1st. Run `facts` on the position to follow a chain.

## Convention

Name the convention each time. Where a reading needs arithmetic the tool does not print, work it out and then run `facts` on the result.

- **Base ten:** digit sums, palindromes, repeated digits, digits read in pairs.
- **Calendar:** the month, the day, the two orders of joining them, the year, the day of the year, the ISO week, and the days to another date. `numplay day` prints all of these.
- **Clock:** a time read as one number, or as minutes past midnight.
- **Units:** the same length gives different numbers in inches and in centimetres.
- **Letters:** a letter read as its place in the alphabet.
- **Joining:** two numbers written side by side and read as one.

## Made

Name the operation and who chose it.

- Adding, subtracting, multiplying or dividing two of the person's numbers to reach a third.
- Writing two results side by side to reach a year or a date.
- Rearranging letters or digits.
- Reporting only the readings that landed. This is the quiet one, and it is why the count line exists.

## Counting readings

- One reading is one question: "is it prime", "what do the digits sum to", "what if I multiply these two".
- Count the ones that missed.
- A reading lands when it turns up something the person would call notable. Say what you treated as notable.
- End with a line like: "Tried 10 readings, 6 landed: 2 structure, 3 convention, 1 made."
