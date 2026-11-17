---
name: number-play
description: Play with any numbers a person hands over (an address, an age, a date, a like count) and say what is true of them, with every finding tagged as structure, convention or made, and a count of how many readings were tried. Also checks whether a date is a "prime day" and writes short posts that state their own line, word and character counts, verified before they are shown. Use for "is this number special", "play with my numbers", "is today a prime day", number coincidences and signs, base-ten arguments, or a clever self-counting post.
---

# Number Play

Find everything worth saying about the numbers a person hands you. Then sort it: what belongs to the number, what belongs to the way we write numbers and dates, and what you made by choosing where to look. The finding is the fun. The sorting is what makes it worth trusting.

Tag every finding, in every list and answer:

- **Structure:** true of the number in any base, calendar or unit. Factors, primality, perfect and Mersenne numbers, 3n+1 routes. Seven pebbles cannot be laid out in equal rows, whatever you call seven.
- **Convention:** true only as written or scheduled. Base-ten digits, dates, clock times, units, letters read as numbers. It is real, and it belongs to the notation.
- **Made:** found by picking an operation after seeing what it could hit, such as multiplying two of their numbers to reach a third they already care about. Say who picked the operation. Usually that is you.

When more than one tag fits, use made first, then convention. A prime you reached by choosing which two numbers to join is made, even though primality is structure.

## House rules

1. **Give every finding.** Report each hit the first time, including the ones you think are chance, with its tag and your read beside it. Do not hold one back to manage how the person will take it.
2. **Keep the count.** One reading is one question asked of the numbers, and each line the tool prints about a number counts as one. Count the ones that missed. End the whole answer with one line: "Tried 10 readings, 6 landed: 2 structure, 3 convention, 1 made." A reading lands when it turns up something the person would call notable, so say what you treated as notable. A handful of small numbers checked many ways will always hit something, and the count is how the person can tell.
3. **Numbers do not forecast.** A pattern in a date, an ID or a like count shows how numbers work. It says nothing about what will happen. Asked for lucky numbers, pick for fun and say the odds are the same as any other pick.
4. **The tool does the arithmetic.** Every factorization, route, date part and count in an answer comes from `numplay`, with the command shown. The tool does not add, subtract or multiply, so do those by hand, say so, and run `numplay facts` on the result. If the tool is not built, build it with `build.ps1`. If you cannot, say so and label all of your arithmetic as unchecked.
5. **Flag before it goes public.** If a number also carries an ugly public meaning, say so before it lands in a post. If a post would let a reader work out a birth date or a home address, say that too. Then the person decides.

## Play with numbers

Take the numbers as given. Ask what one is only when the answer changes the reading, such as a like count that is still moving against an address that is fixed.

1. Run `numplay facts` on all of them in one call, and `numplay collatz` on the set when there are two or more.
2. Lead with structure. Shared 3n+1 roads are real and common: the routes of 6, 27 and 97 all share the road from 10. Say so when routes join at a small number.
3. Add conventions next, each one named as a convention.
4. Add anything made last, with the operation and who chose it.
5. Report every reading that landed, and stop looking after about thirty readings unless the person asks for more.
6. Close with one number worth trying next, then the count line.

For readings beyond what the tool prints, see [readings by tag](references/readings.md).

## Check a day

Run `numplay day <YYYY-MM-DD>`. Add `--to <date>` for the days until a birthday or a deadline.

A date is a prime day when its month and day, joined into one number, make a prime. The tool prints up to four ways of joining them: month first or day first, each with and without a leading zero. Say which ones hold. They often disagree, and that is part of the answer.

A "prime day" is a convention, and a good one. The year being prime is structure; that this year carries that number is convention.

## Write a post that counts itself

A post that states its own number of lines, words or characters invites readers to check. It has to survive the check.

1. Draft it, then save the exact text to a file.
2. Run `numplay count <file>` and compare every claim in the post with the output.
3. Change the wording or the numbers and run it again. Changing a number changes the character count, so expect a few rounds.
4. Show the post only when every claim matches the tool's count of the final text. Show the output with it.

State only the counts the person asked for. They need not be prime unless the post says they are. Save the file as UTF-8, and use the character count without line breaks unless the person's platform counts them.

Say how the tool counts, since platforms differ. A word is anything between whitespace. A character is one Unicode scalar value, spaces included, and line breaks are not characters. An emoji with a skin tone, or a flag, counts as more than one. If the post names a time or a place, getting it posted then and there is the person's job.

## Settle an argument

When someone says a pattern is "just base ten", or that numbers are invented, sort the claims by tag before answering. Structure survives a change of base or calendar. Convention does not, and conceding it costs nothing. Give them the part that is theirs.

## The tool

Use `bin/numplay.exe` on Windows; build it once with `build.ps1` (see the README), since no executable is committed. Both paths are relative to this skill's folder. Portable Rust source is in `numplay/`. The [CLI reference](references/cli.md) has every command, sample output and the limits. The executable reads its arguments and one text file or standard input, and writes stdout and stderr only. Invalid requests exit 64 and print no partial result.

The tool does not announce a limit, so check for these yourself and say when one applies: whole numbers from 1 to 18,446,744,073,709,551,615; a prime's position is given up to ten million; "highly composite" is checked up to one million; `collatz` takes twelve numbers at a time.

## Deliver

Report the findings grouped by tag with the commands that produced them, your read on each, any flags, and the count line. Keep it short enough to enjoy.
