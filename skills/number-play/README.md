# number-play

Hand it any numbers. It finds what is true about them, and tells you which parts it made up.

number-play is an agent skill for Claude Code and Codex, plus one small Rust tool with no dependencies:

- **`numplay`** does the arithmetic. It gives exact factors and primality for any 64-bit number, 3n+1 routes and where they join, the calendar parts of a date, and line, word and character counts for a text. It reads arguments and one file or standard input, and prints results.

## Why

Numbers are fun to read into. An address, an age, a date or a like count will always turn something up, because there are many ways to look and a hit only needs one of them. The skill keeps the fun and adds the sorting:

- **Structure** is true of the number in any base. 127 is prime however you write it.
- **Convention** is true only as written. 11/17 reads as a prime day on this calendar, in base ten, with the month first.
- **Made** is what turned up because someone chose the operation after seeing the target.

Every finding gets one of those tags, and every answer ends with a count of how many readings were tried and how many landed.

## What it does

| You ask | The skill |
|---|---|
| "Play with these numbers" | Runs `numplay facts` and `numplay collatz`, reports each finding with its tag, and ends with the count |
| "Is today a prime day?" | Runs `numplay day` and says which way of writing the date makes it true |
| "Write a post that counts itself" | Drafts it, checks it with `numplay count`, and shows it only when every claim matches |
| "That's just base ten" | Sorts the claims: structure survives a change of base, and convention does not |

The house rules are in [SKILL.md](SKILL.md): give every finding, keep the count, make no forecasts, let the tool do the arithmetic, and flag a number's ugly public meanings or a derivable birth date before a post goes out.

## A post that counts itself

```text
Count this post.
5 lines.
17 words.
89 characters, total.
Every count on every line is prime.
```

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

Go ahead, count.

## Install

Copy this folder into your skills folder:

- **Claude Code:** `~/.claude/skills/number-play`
- **Codex:** `~/.codex/skills/number-play`. `agents/openai.yaml` holds Codex's display text.

Then build the tool once. This needs Windows and a Rust toolchain:

```powershell
pwsh -File ~/.claude/skills/number-play/build.ps1
```

`build.ps1` builds `numplay`, runs its tests, and copies `numplay.exe` into `bin\`. No executable is committed to this repository.

## Using the tool directly

```powershell
bin\numplay.exe --help
bin\numplay.exe facts 127 496
bin\numplay.exe collatz 6 27 97
bin\numplay.exe day 2026-11-17 --to 2027-01-01
bin\numplay.exe count references\example-post.txt
```

`facts 127` prints:

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
```

Invalid requests exit 64 and print no partial result. Every command and its output is in [references/cli.md](references/cli.md).

## Limits

- Whole numbers from 1 to 18,446,744,073,709,551,615.
- A prime's position among the primes is given up to ten million.
- "Highly composite" is checked up to one million.
- `collatz` takes twelve numbers at a time.
- `count` reads UTF-8. A character is one Unicode scalar value, so some emoji count as more than one, and a platform's own counter may differ.

## Platform

Built and tested on Windows 11. `numplay` is portable Rust, but builds on other platforms are untested.

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT), at your option.
