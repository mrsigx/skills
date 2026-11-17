//! Exact whole-number arithmetic for number play.
//!
//! The crate has no dependencies. It reads its arguments and, for `count`, one text file or
//! standard input, and it writes only its result. Every routine works on whole numbers from 1 to
//! `u64::MAX`.

use std::collections::{HashMap, HashSet};
use std::fmt::Write as _;
use std::io::Read as _;

pub const USAGE: &str = concat!(
    "numplay ",
    env!("CARGO_PKG_VERSION"),
    " - exact whole-number arithmetic for number play\n",
    "\n",
    "USAGE:\n",
    "  numplay facts <n>...\n",
    "      What is true of each number: factors, kinds, and the length of its 3n+1 route.\n",
    "  numplay collatz <n>...\n",
    "      Each number's full 3n+1 route, and where the routes join (at most 12 numbers).\n",
    "  numplay day <YYYY-MM-DD>... [--to <YYYY-MM-DD>]\n",
    "      The calendar parts of each date and whether each is prime.\n",
    "  numplay count <file>\n",
    "      Lines, words and characters of a UTF-8 text. Use - to read standard input.\n",
    "  numplay --help | --version\n",
    "\n",
    "Numbers are whole, written in digits, from 1 to 18446744073709551615.\n",
    "Invalid requests exit 64 and print no partial result.\n",
);

/// A prime's position among the primes is reported up to this value.
const PRIME_INDEX_LIMIT: u64 = 10_000_000;
/// "Highly composite" is checked exactly up to this value and not reported above it.
const HIGHLY_COMPOSITE_LIMIT: u64 = 1_000_000;
const COLLATZ_MAX_NUMBERS: usize = 12;
const COLLATZ_MAX_STEPS: u64 = 1_000_000;
/// These bases make the Miller-Rabin test exact for every 64-bit number.
const WITNESSES: [u64; 12] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];
const WEEKDAYS: [&str; 7] = [
    "Monday",
    "Tuesday",
    "Wednesday",
    "Thursday",
    "Friday",
    "Saturday",
    "Sunday",
];

/// Run one command and return what it prints. An `Err` means nothing should be printed to stdout.
pub fn execute(args: &[String]) -> Result<String, String> {
    let Some((command, rest)) = args.split_first() else {
        return Err("no command given".to_string());
    };
    match command.as_str() {
        "--help" | "-h" | "help" => Ok(USAGE.to_string()),
        "--version" => Ok(format!("numplay {}\n", env!("CARGO_PKG_VERSION"))),
        "facts" => facts(rest),
        "collatz" => collatz(rest),
        "day" => day(rest),
        "count" => count(rest),
        other => Err(format!("unknown command '{other}'")),
    }
}

// ---------------------------------------------------------------------------------------------
// Parsing
// ---------------------------------------------------------------------------------------------

fn parse_number(text: &str) -> Result<u64, String> {
    if text.is_empty() || !text.bytes().all(|b| b.is_ascii_digit()) {
        return Err(format!("'{text}' is not a whole number written in digits"));
    }
    let n: u64 = text
        .parse()
        .map_err(|_| format!("'{text}' is larger than {}", u64::MAX))?;
    if n == 0 {
        return Err("0 is out of range; numbers start at 1".to_string());
    }
    Ok(n)
}

fn parse_numbers(args: &[String], command: &str) -> Result<Vec<u64>, String> {
    if args.is_empty() {
        return Err(format!("{command} needs at least one number"));
    }
    args.iter().map(|arg| parse_number(arg)).collect()
}

// ---------------------------------------------------------------------------------------------
// Primes and factors
// ---------------------------------------------------------------------------------------------

fn mul_mod(a: u64, b: u64, modulus: u64) -> u64 {
    (a as u128 * b as u128 % modulus as u128) as u64
}

fn pow_mod(base: u64, mut exponent: u64, modulus: u64) -> u64 {
    let mut result = 1 % modulus;
    let mut base = base % modulus;
    while exponent > 0 {
        if exponent & 1 == 1 {
            result = mul_mod(result, base, modulus);
        }
        base = mul_mod(base, base, modulus);
        exponent >>= 1;
    }
    result
}

/// Exact primality for any 64-bit number (deterministic Miller-Rabin).
pub fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    for p in WITNESSES {
        if n % p == 0 {
            return n == p;
        }
    }
    let mut odd = n - 1;
    let mut twos = 0;
    while odd % 2 == 0 {
        odd /= 2;
        twos += 1;
    }
    'witness: for a in WITNESSES {
        let mut x = pow_mod(a, odd, n);
        if x == 1 || x == n - 1 {
            continue;
        }
        for _ in 1..twos {
            x = mul_mod(x, x, n);
            if x == n - 1 {
                continue 'witness;
            }
        }
        return false;
    }
    true
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

/// Whole square root, exact for values below 2^100.
fn isqrt(n: u128) -> u128 {
    let mut root = (n as f64).sqrt() as u128;
    while root * root > n {
        root -= 1;
    }
    while (root + 1) * (root + 1) <= n {
        root += 1;
    }
    root
}

fn icbrt(n: u64) -> u64 {
    let n = n as u128;
    let mut root = (n as f64).cbrt() as u128;
    while root * root * root > n {
        root -= 1;
    }
    while (root + 1) * (root + 1) * (root + 1) <= n {
        root += 1;
    }
    root as u64
}

/// Find one proper factor of an odd composite that is not a perfect square.
fn pollard_rho(n: u64) -> u64 {
    let mut constant = 1u64;
    loop {
        let step = |x: u64| ((x as u128 * x as u128 + constant as u128) % n as u128) as u64;
        let (mut slow, mut fast, mut factor) = (2u64, 2u64, 1u64);
        while factor == 1 {
            slow = step(slow);
            fast = step(step(fast));
            factor = gcd(slow.abs_diff(fast), n);
        }
        if factor != n {
            return factor;
        }
        constant += 1;
    }
}

fn split(n: u64, factors: &mut Vec<u64>) {
    if n == 1 {
        return;
    }
    if is_prime(n) {
        factors.push(n);
        return;
    }
    let root = isqrt(n as u128) as u64;
    if root * root == n {
        split(root, factors);
        split(root, factors);
        return;
    }
    let factor = pollard_rho(n);
    split(factor, factors);
    split(n / factor, factors);
}

/// Prime factors in ascending order, repeated as often as they divide `n`. Empty for 0 and 1.
pub fn factorize(n: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    let mut rest = n;
    if rest < 2 {
        return factors;
    }
    while rest % 2 == 0 {
        factors.push(2);
        rest /= 2;
    }
    let mut p = 3u64;
    while p <= 5_000 && p * p <= rest {
        while rest % p == 0 {
            factors.push(p);
            rest /= p;
        }
        p += 2;
    }
    split(rest, &mut factors);
    factors.sort_unstable();
    factors
}

fn group(factors: &[u64]) -> Vec<(u64, u32)> {
    let mut groups: Vec<(u64, u32)> = Vec::new();
    for &p in factors {
        match groups.last_mut() {
            Some((prime, exponent)) if *prime == p => *exponent += 1,
            _ => groups.push((p, 1)),
        }
    }
    groups
}

fn format_groups(groups: &[(u64, u32)]) -> String {
    groups
        .iter()
        .map(|&(p, e)| if e == 1 { p.to_string() } else { format!("{p}^{e}") })
        .collect::<Vec<_>>()
        .join(" x ")
}

fn divisor_count(groups: &[(u64, u32)]) -> u64 {
    groups.iter().map(|&(_, e)| e as u64 + 1).product()
}

fn divisor_sum(groups: &[(u64, u32)]) -> u128 {
    groups
        .iter()
        .map(|&(p, e)| {
            let mut term = 1u128;
            let mut power = 1u128;
            for _ in 0..e {
                power *= p as u128;
                term += power;
            }
            term
        })
        .product()
}

/// "7 (prime)", "12 (2^2 x 3)", and plain words for 0 and 1.
fn describe(n: u64) -> String {
    match n {
        0 => "0".to_string(),
        1 => "1 (neither prime nor composite)".to_string(),
        _ if is_prime(n) => format!("{n} (prime)"),
        _ => format!("{n} ({})", format_groups(&group(&factorize(n)))),
    }
}

fn ordinal(n: u64) -> String {
    let suffix = match (n % 10, n % 100) {
        (_, 11..=13) => "th",
        (1, _) => "st",
        (2, _) => "nd",
        (3, _) => "rd",
        _ => "th",
    };
    format!("{n}{suffix}")
}

/// The position of each prime input among the primes, for inputs up to `PRIME_INDEX_LIMIT`.
fn prime_indices(numbers: &[u64], primes: &[bool]) -> HashMap<u64, u64> {
    let wanted: HashSet<u64> = numbers
        .iter()
        .zip(primes)
        .filter(|(n, prime)| **prime && **n <= PRIME_INDEX_LIMIT)
        .map(|(n, _)| *n)
        .collect();
    let mut found = HashMap::new();
    let Some(&top) = wanted.iter().max() else {
        return found;
    };
    let top = top as usize;
    let mut composite = vec![false; top + 1];
    let mut seen = 0u64;
    for i in 2..=top {
        if composite[i] {
            continue;
        }
        seen += 1;
        if wanted.contains(&(i as u64)) {
            found.insert(i as u64, seen);
        }
        if i <= top / i {
            let mut multiple = i * i;
            while multiple <= top {
                composite[multiple] = true;
                multiple += i;
            }
        }
    }
    found
}

/// The inputs up to `HIGHLY_COMPOSITE_LIMIT` that have more divisors than every smaller number.
fn highly_composite(numbers: &[u64]) -> HashSet<u64> {
    let mut set = HashSet::new();
    let Some(top) = numbers
        .iter()
        .copied()
        .filter(|&n| n <= HIGHLY_COMPOSITE_LIMIT)
        .max()
    else {
        return set;
    };
    let top = top as usize;
    let mut divisors = vec![0u32; top + 1];
    for i in 1..=top {
        let mut multiple = i;
        while multiple <= top {
            divisors[multiple] += 1;
            multiple += i;
        }
    }
    let mut best = 0u32;
    for n in 1..=top {
        if divisors[n] > best {
            best = divisors[n];
            set.insert(n as u64);
        }
    }
    set
}

fn kinds(n: u64, prime: bool, proper_sum: u128, highly: bool) -> Vec<String> {
    let mut found = Vec::new();
    if proper_sum == n as u128 {
        found.push("perfect number".to_string());
    }
    if highly {
        found.push("highly composite number".to_string());
    }
    if prime {
        if let Some(next) = n.checked_add(1) {
            if next.is_power_of_two() {
                found.push(format!("Mersenne prime (2^{} - 1)", next.trailing_zeros()));
            }
        }
        let mut twins = Vec::new();
        if n > 2 && is_prime(n - 2) {
            twins.push((n - 2).to_string());
        }
        if let Some(above) = n.checked_add(2) {
            if is_prime(above) {
                twins.push(above.to_string());
            }
        }
        if !twins.is_empty() {
            found.push(format!("twin prime with {}", twins.join(" and ")));
        }
    }
    if n.is_power_of_two() {
        found.push(format!("power of two (2^{})", n.trailing_zeros()));
    }
    let root = isqrt(n as u128) as u64;
    if root * root == n {
        found.push(format!("square ({root}^2)"));
    }
    let cube_root = icbrt(n);
    if (cube_root as u128).pow(3) == n as u128 {
        found.push(format!("cube ({cube_root}^3)"));
    }
    let eight_n_plus_one = 8 * n as u128 + 1;
    let triangle_root = isqrt(eight_n_plus_one);
    if triangle_root * triangle_root == eight_n_plus_one {
        found.push(format!("triangular number (T{})", (triangle_root - 1) / 2));
    }

    let mut fibonacci = Vec::new();
    let (mut current, mut next, mut index) = (1u64, 1u64, 1u32);
    while current <= n {
        if current == n {
            fibonacci.push(format!("F{index}"));
        }
        let Some(sum) = current.checked_add(next) else {
            if next == n && next != current {
                fibonacci.push(format!("F{}", index + 1));
            }
            break;
        };
        current = next;
        next = sum;
        index += 1;
    }
    if !fibonacci.is_empty() {
        found.push(format!("Fibonacci number ({})", fibonacci.join(" and ")));
    }

    let mut factorials = Vec::new();
    let (mut value, mut k) = (1u64, 0u64);
    loop {
        if value == n {
            factorials.push(format!("{k}!"));
        }
        k += 1;
        match value.checked_mul(k) {
            Some(product) if product <= n => value = product,
            _ => break,
        }
    }
    if !factorials.is_empty() {
        found.push(format!("factorial ({})", factorials.join(" and ")));
    }
    found
}

// ---------------------------------------------------------------------------------------------
// facts
// ---------------------------------------------------------------------------------------------

fn facts(args: &[String]) -> Result<String, String> {
    let numbers = parse_numbers(args, "facts")?;
    let primes: Vec<bool> = numbers.iter().map(|&n| is_prime(n)).collect();
    let indices = prime_indices(&numbers, &primes);
    let highly = highly_composite(&numbers);
    let mut out = String::new();
    for (position, (&n, &prime)) in numbers.iter().zip(&primes).enumerate() {
        if position > 0 {
            out.push('\n');
        }
        let groups = group(&factorize(n));
        let proper_sum = divisor_sum(&groups) - n as u128;
        let (steps, peak) = collatz_walk(n, |_| {})?;
        let _ = writeln!(out, "n: {n}");
        out.push_str("structure (true in any base):\n");
        match (prime, indices.get(&n)) {
            (true, Some(index)) => {
                let _ = writeln!(out, "  prime: yes, the {} prime", ordinal(*index));
            }
            (true, None) => out.push_str("  prime: yes\n"),
            (false, _) => out.push_str("  prime: no\n"),
        }
        if groups.is_empty() {
            out.push_str("  factors: none (1 has no prime factors)\n");
        } else {
            let _ = writeln!(out, "  factors: {}", format_groups(&groups));
        }
        let weight = if proper_sum == n as u128 {
            "perfect"
        } else if proper_sum > n as u128 {
            "abundant"
        } else {
            "deficient"
        };
        let _ = writeln!(
            out,
            "  divisors: {}; the proper ones sum to {proper_sum} ({weight})",
            divisor_count(&groups)
        );
        let found = kinds(n, prime, proper_sum, highly.contains(&n));
        if found.is_empty() {
            out.push_str("  kinds: none on this tool's list\n");
        } else {
            let _ = writeln!(out, "  kinds: {}", found.join(", "));
        }
        let _ = writeln!(out, "  3n+1: {steps} steps to 1, peak {peak}");
        out.push_str("base ten (true only as written):\n");
        let digits = n.to_string();
        let digit_sum: u32 = digits.bytes().map(|b| (b - b'0') as u32).sum();
        let palindrome = digits.bytes().eq(digits.bytes().rev());
        let _ = writeln!(out, "  digits: {}, digit sum {digit_sum}", digits.len());
        let _ = writeln!(
            out,
            "  palindrome: {}",
            if palindrome { "yes" } else { "no" }
        );
    }
    Ok(out)
}

// ---------------------------------------------------------------------------------------------
// collatz
// ---------------------------------------------------------------------------------------------

/// Walk the 3n+1 route from `n` down to 1, handing every value to `visit`. Returns the number of
/// steps and the highest value reached.
fn collatz_walk(n: u64, mut visit: impl FnMut(u128)) -> Result<(u64, u128), String> {
    let mut value = n as u128;
    let mut steps = 0u64;
    let mut peak = value;
    visit(value);
    while value != 1 {
        value = if value % 2 == 0 {
            value / 2
        } else {
            if value > (u128::MAX - 1) / 3 {
                return Err(format!(
                    "the 3n+1 route of {n} climbs past what this tool can hold"
                ));
            }
            3 * value + 1
        };
        steps += 1;
        if steps > COLLATZ_MAX_STEPS {
            return Err(format!(
                "the 3n+1 route of {n} is longer than {COLLATZ_MAX_STEPS} steps"
            ));
        }
        peak = peak.max(value);
        visit(value);
    }
    Ok((steps, peak))
}

fn collatz(args: &[String]) -> Result<String, String> {
    let numbers = parse_numbers(args, "collatz")?;
    if numbers.len() > COLLATZ_MAX_NUMBERS {
        return Err(format!(
            "collatz takes at most {COLLATZ_MAX_NUMBERS} numbers at a time"
        ));
    }
    let mut routes: Vec<Vec<u128>> = Vec::new();
    let mut out = String::new();
    for (position, &n) in numbers.iter().enumerate() {
        let mut route = Vec::new();
        let (steps, peak) = collatz_walk(n, |value| route.push(value))?;
        if position > 0 {
            out.push('\n');
        }
        let _ = writeln!(out, "n: {n}");
        let _ = writeln!(out, "steps: {steps}");
        let _ = writeln!(out, "peak: {peak}");
        let listed: Vec<String> = route.iter().map(|value| value.to_string()).collect();
        let _ = writeln!(out, "route: {}", listed.join(" "));
        routes.push(route);
    }
    if numbers.len() > 1 {
        // A route never repeats a value before it reaches 1, so each value has one position.
        let positions: Vec<HashMap<u128, usize>> = routes
            .iter()
            .map(|route| route.iter().enumerate().map(|(i, &v)| (v, i)).collect())
            .collect();
        out.push_str("\njoins:\n");
        for a in 0..numbers.len() {
            for b in a + 1..numbers.len() {
                let (steps_a, value) = routes[a]
                    .iter()
                    .enumerate()
                    .find(|(_, value)| positions[b].contains_key(*value))
                    .map(|(i, &value)| (i, value))
                    .expect("every route ends at 1");
                let steps_b = positions[b][&value];
                let _ = writeln!(
                    out,
                    "  {} and {} join at {value} (after {steps_a} and {steps_b} steps)",
                    numbers[a], numbers[b]
                );
            }
        }
        if numbers.len() > 2 {
            let shared = routes[0]
                .iter()
                .find(|value| positions[1..].iter().all(|p| p.contains_key(*value)))
                .expect("every route ends at 1");
            let _ = writeln!(
                out,
                "  all {} routes share the road from {shared}",
                numbers.len()
            );
        }
    }
    Ok(out)
}

// ---------------------------------------------------------------------------------------------
// day
// ---------------------------------------------------------------------------------------------

#[derive(Clone, Copy)]
struct Date {
    year: i64,
    month: u32,
    day: u32,
}

impl Date {
    fn text(self) -> String {
        format!("{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }
}

fn is_leap(year: i64) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

fn days_in_month(year: i64, month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        _ if is_leap(year) => 29,
        _ => 28,
    }
}

fn parse_date(text: &str) -> Result<Date, String> {
    let malformed = || format!("'{text}' is not a date written YYYY-MM-DD");
    let bytes = text.as_bytes();
    if bytes.len() != 10 || !text.is_ascii() || bytes[4] != b'-' || bytes[7] != b'-' {
        return Err(malformed());
    }
    let field = |part: &str| -> Result<u32, String> {
        if !part.bytes().all(|b| b.is_ascii_digit()) {
            return Err(malformed());
        }
        part.parse().map_err(|_| malformed())
    };
    let year = field(&text[0..4])? as i64;
    let month = field(&text[5..7])?;
    let day = field(&text[8..10])?;
    if year < 1 || !(1..=12).contains(&month) || day < 1 || day > days_in_month(year, month) {
        return Err(format!("'{text}' is not a real calendar date"));
    }
    Ok(Date { year, month, day })
}

/// Days since 1970-01-01 on the Gregorian calendar.
fn day_number(date: Date) -> i64 {
    let year = if date.month <= 2 {
        date.year - 1
    } else {
        date.year
    };
    let era = year.div_euclid(400);
    let year_of_era = year.rem_euclid(400);
    let month_from_march = (date.month as i64 + 9) % 12;
    let day_of_year = (153 * month_from_march + 2) / 5 + date.day as i64 - 1;
    let day_of_era = year_of_era * 365 + year_of_era / 4 - year_of_era / 100 + day_of_year;
    era * 146_097 + day_of_era - 719_468
}

/// Monday is 1 and Sunday is 7.
fn iso_weekday(date: Date) -> u32 {
    ((day_number(date) + 3).rem_euclid(7) + 1) as u32
}

fn january_first(year: i64) -> Date {
    Date {
        year,
        month: 1,
        day: 1,
    }
}

fn day_of_year(date: Date) -> u32 {
    (day_number(date) - day_number(january_first(date.year)) + 1) as u32
}

fn iso_weeks_in_year(year: i64) -> u32 {
    let first = iso_weekday(january_first(year));
    if first == 4 || (first == 3 && is_leap(year)) {
        53
    } else {
        52
    }
}

/// The ISO 8601 week-numbering year and week.
fn iso_week(date: Date) -> (i64, u32) {
    let week = (day_of_year(date) + 10 - iso_weekday(date)) / 7;
    if week == 0 {
        (date.year - 1, iso_weeks_in_year(date.year - 1))
    } else if week > iso_weeks_in_year(date.year) {
        (date.year + 1, 1)
    } else {
        (date.year, week)
    }
}

fn joined(first: String, second: String) -> u64 {
    format!("{first}{second}")
        .parse()
        .expect("two short digit strings fit in 64 bits")
}

fn day(args: &[String]) -> Result<String, String> {
    let mut dates = Vec::new();
    let mut to: Option<Date> = None;
    let mut remaining = args.iter();
    while let Some(arg) = remaining.next() {
        if arg == "--to" {
            if to.is_some() {
                return Err("--to was given twice".to_string());
            }
            let value = remaining
                .next()
                .ok_or_else(|| "--to needs a date".to_string())?;
            to = Some(parse_date(value)?);
        } else {
            dates.push(parse_date(arg)?);
        }
    }
    if dates.is_empty() {
        return Err("day needs at least one date".to_string());
    }
    let mut out = String::new();
    for (position, &date) in dates.iter().enumerate() {
        if position > 0 {
            out.push('\n');
        }
        let (month, day) = (date.month as u64, date.day as u64);
        let (iso_year, week) = iso_week(date);
        let _ = writeln!(out, "date: {}", date.text());
        let _ = writeln!(
            out,
            "weekday: {}",
            WEEKDAYS[iso_weekday(date) as usize - 1]
        );
        out.push_str("calendar parts (true only on this calendar, in base ten):\n");
        let _ = writeln!(out, "  month: {}", describe(month));
        let _ = writeln!(out, "  day: {}", describe(day));
        let _ = writeln!(
            out,
            "  month then day: {}",
            describe(joined(month.to_string(), day.to_string()))
        );
        if day < 10 {
            let _ = writeln!(
                out,
                "  month then day, two-digit day: {}",
                describe(joined(month.to_string(), format!("{day:02}")))
            );
        }
        let _ = writeln!(
            out,
            "  day then month: {}",
            describe(joined(day.to_string(), month.to_string()))
        );
        if month < 10 {
            let _ = writeln!(
                out,
                "  day then month, two-digit month: {}",
                describe(joined(day.to_string(), format!("{month:02}")))
            );
        }
        let _ = writeln!(out, "  year: {}", describe(date.year as u64));
        let _ = writeln!(out, "  day of year: {}", describe(day_of_year(date) as u64));
        let _ = writeln!(
            out,
            "  ISO week: {}, in ISO year {iso_year}",
            describe(week as u64)
        );
        if let Some(target) = to {
            let apart = day_number(target) - day_number(date);
            let direction = if apart >= 0 { "to" } else { "since" };
            let _ = writeln!(
                out,
                "  days {direction} {}: {}",
                target.text(),
                describe(apart.unsigned_abs())
            );
        }
    }
    Ok(out)
}

// ---------------------------------------------------------------------------------------------
// count
// ---------------------------------------------------------------------------------------------

/// What a text holds once a leading byte-order mark and trailing line breaks are set aside.
#[derive(Debug, PartialEq, Eq)]
pub struct TextCount {
    /// Lines that hold at least one word.
    pub lines: u64,
    /// Lines that hold no word.
    pub blank_lines: u64,
    pub words: u64,
    /// One entry per line that holds a word, in order.
    pub words_per_line: Vec<u64>,
    /// Unicode scalar values, spaces included, line breaks not included.
    pub characters: u64,
    pub line_breaks: u64,
    pub lines_ending_in_space: u64,
}

/// Count a text the way a reader would check a post that describes itself. Words are runs of
/// characters between whitespace, so "11," is one word.
pub fn count_text(text: &str) -> Result<TextCount, String> {
    let text = text.strip_prefix('\u{feff}').unwrap_or(text);
    let text = text.replace("\r\n", "\n").replace('\r', "\n");
    let text = text.trim_end_matches('\n');
    if text.trim().is_empty() {
        return Err("the text has no words".to_string());
    }
    let mut count = TextCount {
        lines: 0,
        blank_lines: 0,
        words: 0,
        words_per_line: Vec::new(),
        characters: 0,
        line_breaks: 0,
        lines_ending_in_space: 0,
    };
    let mut all_lines = 0u64;
    for line in text.split('\n') {
        all_lines += 1;
        count.characters += line.chars().count() as u64;
        if line != line.trim_end() {
            count.lines_ending_in_space += 1;
        }
        let words = line.split_whitespace().count() as u64;
        if words == 0 {
            count.blank_lines += 1;
        } else {
            count.lines += 1;
            count.words += words;
            count.words_per_line.push(words);
        }
    }
    count.line_breaks = all_lines - 1;
    Ok(count)
}

fn render_count(count: &TextCount) -> String {
    let mut out = String::new();
    let per_line: Vec<String> = count.words_per_line.iter().map(u64::to_string).collect();
    let prime_lines = count
        .words_per_line
        .iter()
        .filter(|&&words| is_prime(words))
        .count();
    let _ = writeln!(out, "lines: {}", describe(count.lines));
    let _ = writeln!(out, "blank lines: {}", count.blank_lines);
    let _ = writeln!(out, "words: {}", describe(count.words));
    let _ = writeln!(out, "words per line: {}", per_line.join(" "));
    let _ = writeln!(
        out,
        "lines with a prime word count: {prime_lines} of {}",
        count.lines
    );
    let _ = writeln!(
        out,
        "characters: {}, line breaks not counted",
        describe(count.characters)
    );
    let _ = writeln!(
        out,
        "characters with line breaks: {}",
        describe(count.characters + count.line_breaks)
    );
    if count.lines_ending_in_space > 0 {
        let _ = writeln!(
            out,
            "note: {} line(s) end with a space, and those spaces are counted",
            count.lines_ending_in_space
        );
    }
    out
}

fn count(args: &[String]) -> Result<String, String> {
    let [source] = args else {
        return Err("count needs exactly one file path, or - for standard input".to_string());
    };
    let bytes = if source == "-" {
        let mut buffer = Vec::new();
        std::io::stdin()
            .read_to_end(&mut buffer)
            .map_err(|error| format!("cannot read standard input: {error}"))?;
        buffer
    } else {
        std::fs::read(source).map_err(|error| format!("cannot read '{source}': {error}"))?
    };
    let text = String::from_utf8(bytes)
        .map_err(|_| "the text is not valid UTF-8; save it as UTF-8 and try again".to_string())?;
    Ok(render_count(&count_text(&text)?))
}
