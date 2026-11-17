# numplay CLI

`numplay` does exact whole-number arithmetic and prints plain text. It reads its arguments and, for `count`, one text file or standard input. It writes stdout and stderr only.

- Numbers are whole, written in digits, from 1 to 18446744073709551615.
- An invalid request exits 64, prints the reason and the usage text to stderr, and prints nothing to stdout. A call with one bad number among good ones prints no partial result.
- `--help` prints the usage text. `--version` prints the version.

## facts

```powershell
bin\numplay.exe facts 127 496
```

```text
n: 127
structure (true in any base):
  prime: yes, the 31st prime
  factors: 127
  divisors: 2; the proper ones sum to 1 (deficient)
  kinds: Mersenne prime (2^7 - 1)
  3n+1: 46 steps to 1, peak 4372
base ten (true only as written):
  digits: 3, digit sum 10
  palindrome: no

n: 496
structure (true in any base):
  prime: no
  factors: 2^4 x 31
  divisors: 10; the proper ones sum to 496 (perfect)
  kinds: perfect number, triangular number (T31)
  3n+1: 110 steps to 1, peak 9232
base ten (true only as written):
  digits: 3, digit sum 19
  palindrome: no
```

- **prime:** exact for every 64-bit number. The position among the primes is given for primes up to 10,000,000 and left out above that.
- **factors:** prime factors in ascending order. `2^4 x 31` means 2 four times and 31 once.
- **divisors:** how many divisors the number has, and the sum of the ones smaller than itself. That sum makes the number deficient, perfect or abundant.
- **kinds:** any of perfect number, highly composite number, Mersenne prime, twin prime, power of two, square, cube, triangular number, Fibonacci number and factorial. "Highly composite" is checked exactly up to 1,000,000 and is not reported above that.
- **3n+1:** halve an even number, triple an odd one and add 1, and repeat until you reach 1. The line gives the number of steps and the highest value on the way.
- **base ten:** the digit count, the digit sum and whether the digits read the same backwards. These belong to base-ten writing, not to the number.

## collatz

```powershell
bin\numplay.exe collatz 6 27 97
```

The output gives each number's steps, peak and full route, then where the routes join. The routes of 27 and 97 are long, so they are cut short here:

```text
n: 6
steps: 8
peak: 16
route: 6 3 10 5 16 8 4 2 1

n: 27
steps: 111
peak: 9232
route: 27 82 41 124 62 31 94 47 ...

n: 97
steps: 118
peak: 9232
route: 97 292 146 73 220 110 55 166 ...

joins:
  6 and 27 join at 10 (after 2 and 105 steps)
  6 and 97 join at 10 (after 2 and 112 steps)
  27 and 97 join at 94 (after 6 and 13 steps)
  all 3 routes share the road from 10
```

- A **join** is the first value on the first number's route that is also on the second's. From there the two routes are the same.
- The last line appears for three or more numbers. It names the first value on the first number's route that every route passes through.
- `collatz` takes at most 12 numbers at a time.

## day

```powershell
bin\numplay.exe day 2026-11-17 --to 2027-01-01
```

```text
date: 2026-11-17
weekday: Tuesday
calendar parts (true only on this calendar, in base ten):
  month: 11 (prime)
  day: 17 (prime)
  month then day: 1117 (prime)
  day then month: 1711 (29 x 59)
  year: 2026 (2 x 1013)
  day of year: 321 (3 x 107)
  ISO week: 47 (prime), in ISO year 2026
  days to 2027-01-01: 45 (3^2 x 5)
```

- Dates are written `YYYY-MM-DD`, from year 0001 to 9999, on the Gregorian calendar. A date that does not exist, such as 2023-02-29, is refused.
- Several dates can be given in one call. `--to` takes one date and is applied to each.
- When the day or the month has one digit, the joined forms are also shown with two digits. For 2026-03-07 that adds `month then day, two-digit day: 307 (prime)` and `day then month, two-digit month: 703 (19 x 37)`.
- When the `--to` date is earlier, the line reads `days since`.
- ISO weeks start on Monday. The first days of January can belong to the last ISO week of the year before, and the last days of December to week 1 of the year after.

## count

```powershell
bin\numplay.exe count references\example-post.txt
```

```text
lines: 5 (prime)
blank lines: 0
words: 17 (prime)
words per line: 3 2 2 3 7
lines with a prime word count: 5 of 5
characters: 89 (prime), line breaks not counted
characters with line breaks: 93 (3 x 31)
```

Use `-` in place of the file to read standard input.

How it counts:

- The text must be UTF-8. A byte-order mark at the start and line breaks at the end are set aside. Windows and Unix line endings count the same.
- A **line** holds at least one word. Lines with no word are reported as blank lines.
- A **word** is a run of characters between whitespace, so `11,` is one word.
- A **character** is one Unicode scalar value. Spaces count and line breaks do not. An accented letter saved as one code point counts once. An emoji with a skin tone, or a flag, counts as more than one.
- If any line ends with a space, a `note:` line says how many do. Those spaces are counted.
- A text with no words is refused.

A platform may count characters differently from this tool. Say which count a post is using.

## Other lines and behaviour

- `kinds: none on this tool's list` means none of the ten kinds applies. The list has powers of two but no other pure powers, so 2187 = 3^7 shows this line; read the power from `factors`.
- 1 prints `factors: none (1 has no prime factors)`, and a part equal to 1 prints `1 (neither prime nor composite)`.
- A number above 1,000,000 is never reported as highly composite, even when it is one.
- Leading zeros are ignored, so `007` is read as 7.
- In `count`, a line holding only spaces is a blank line, its spaces are still counted as characters, and it can trigger the `note:` line.
- In `count`, a lone carriage return is a line break. Other Unicode line separators are not, though they do split words.
- A file that cannot be read exits 64 like any other invalid request.

## Build and test

`build.ps1` does all of this on Windows. By hand, from the repository root:

```powershell
cargo build --release --manifest-path numplay\Cargo.toml --target-dir target
cargo test --release --manifest-path numplay\Cargo.toml --target-dir target
```

The crate has no dependencies, so it builds offline. The tests compare the tool with values computed independently.
