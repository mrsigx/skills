//! Tests for numplay. Expected values were computed independently (Python's integers and
//! `datetime`), not by this crate.

use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};

/// A post that states its own counts. Every claim in it is checked below.
const POST: &str = "Count this post.\n5 lines.\n17 words.\n89 characters, total.\nEvery count on every line is prime.";

const HIGHLY_COMPOSITE: [u64; 38] = [
    1, 2, 4, 6, 12, 24, 36, 48, 60, 120, 180, 240, 360, 720, 840, 1260, 1680, 2520, 5040, 7560,
    10080, 15120, 20160, 25200, 27720, 45360, 50400, 55440, 83160, 110880, 166320, 221760, 277200,
    332640, 498960, 554400, 665280, 720720,
];

fn run(args: &[&str]) -> Result<String, String> {
    let owned: Vec<String> = args.iter().map(|arg| arg.to_string()).collect();
    numplay::execute(&owned)
}

fn ok(args: &[&str]) -> String {
    run(args).unwrap_or_else(|error| panic!("{args:?} failed: {error}"))
}

fn temp_file(name: &str, bytes: &[u8]) -> PathBuf {
    let path = std::env::temp_dir().join(format!("numplay-test-{}-{name}", std::process::id()));
    std::fs::write(&path, bytes).expect("write temp file");
    path
}

fn trial_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    let mut d = 2;
    while d * d <= n {
        if n % d == 0 {
            return false;
        }
        d += 1;
    }
    true
}

#[test]
fn primality_is_exact() {
    for n in 0..=20_000u64 {
        assert_eq!(numplay::is_prime(n), trial_prime(n), "n = {n}");
    }
    for prime in [
        998_244_353u64,
        1_000_000_007,
        2_305_843_009_213_693_951,  // 2^61 - 1
        18_446_744_073_709_551_557, // the largest prime below 2^64
    ] {
        assert!(numplay::is_prime(prime), "{prime} is prime");
    }
    for composite in [
        561u64,                     // Carmichael number
        3_215_031_751,              // passes Miller-Rabin for bases 2, 3, 5 and 7
        3_825_123_056_546_413_051,  // passes for the first nine prime bases
        4_611_686_014_132_420_609,  // (2^31 - 1)^2
        18_446_744_073_709_551_615, // 2^64 - 1
    ] {
        assert!(!numplay::is_prime(composite), "{composite} is composite");
    }
}

#[test]
fn factorization_is_exact() {
    let cases: [(u64, &[u64]); 7] = [
        (1, &[]),
        (600_851_475_143, &[71, 839, 1471, 6857]),
        (2_305_843_009_213_693_951, &[2_305_843_009_213_693_951]),
        (4_611_686_014_132_420_609, &[2_147_483_647, 2_147_483_647]),
        (998_244_359_987_710_471, &[998_244_353, 1_000_000_007]),
        (3_825_123_056_546_413_051, &[149_491, 747_451, 34_233_211]),
        (
            18_446_744_073_709_551_615,
            &[3, 5, 17, 257, 641, 65_537, 6_700_417],
        ),
    ];
    for (n, expected) in cases {
        assert_eq!(numplay::factorize(n), expected, "n = {n}");
    }
    for n in 1..=20_000u64 {
        let factors = numplay::factorize(n);
        assert_eq!(factors.iter().product::<u64>(), n, "product for {n}");
        assert!(factors.iter().all(|&p| trial_prime(p)), "primes for {n}");
        assert!(factors.windows(2).all(|pair| pair[0] <= pair[1]), "order for {n}");
    }
}

#[test]
fn facts_reports_structure_and_base_ten() {
    assert_eq!(
        ok(&["facts", "496"]),
        "n: 496\n\
         structure (true in any base):\n\
         \x20 prime: no\n\
         \x20 factors: 2^4 x 31\n\
         \x20 divisors: 10; the proper ones sum to 496 (perfect)\n\
         \x20 kinds: perfect number, triangular number (T31)\n\
         \x20 3n+1: 110 steps to 1, peak 9232\n\
         base ten (true only as written):\n\
         \x20 digits: 3, digit sum 19\n\
         \x20 palindrome: no\n"
    );
    let mersenne = ok(&["facts", "127"]);
    assert!(mersenne.contains("  prime: yes, the 31st prime\n"));
    assert!(mersenne.contains("  factors: 127\n"));
    assert!(mersenne.contains("  divisors: 2; the proper ones sum to 1 (deficient)\n"));
    assert!(mersenne.contains("  kinds: Mersenne prime (2^7 - 1)\n"));
    assert!(mersenne.contains("  3n+1: 46 steps to 1, peak 4372\n"));

    let twelve = ok(&["facts", "12"]);
    assert!(twelve.contains("  factors: 2^2 x 3\n"));
    assert!(twelve.contains("  divisors: 6; the proper ones sum to 16 (abundant)\n"));
    assert!(twelve.contains("  kinds: highly composite number\n"));

    assert!(ok(&["facts", "144"]).contains("  kinds: square (12^2), Fibonacci number (F12)\n"));
    assert!(ok(&["facts", "120"])
        .contains("  kinds: highly composite number, triangular number (T15), factorial (5!)\n"));
    assert!(ok(&["facts", "8"])
        .contains("  kinds: power of two (2^3), cube (2^3), Fibonacci number (F6)\n"));
    assert!(ok(&["facts", "5"])
        .contains("  kinds: twin prime with 3 and 7, Fibonacci number (F5)\n"));
    assert!(ok(&["facts", "14"]).contains("  kinds: none on this tool's list\n"));
    assert!(ok(&["facts", "121"]).contains("  palindrome: yes\n"));

    let one = ok(&["facts", "1"]);
    assert!(one.contains("  prime: no\n"));
    assert!(one.contains("  factors: none (1 has no prime factors)\n"));
    assert!(one.contains("  divisors: 1; the proper ones sum to 0 (deficient)\n"));
    assert!(one.contains(
        "  kinds: highly composite number, power of two (2^0), square (1^2), cube (1^3), \
         triangular number (T1), Fibonacci number (F1 and F2), factorial (0! and 1!)\n"
    ));
    assert!(one.contains("  3n+1: 0 steps to 1, peak 1\n"));
}

#[test]
fn prime_positions_and_their_limit() {
    for (prime, position) in [
        (2u64, "1st"),
        (3, "2nd"),
        (5, "3rd"),
        (31, "11th"),
        (37, "12th"),
        (41, "13th"),
        (73, "21st"),
        (79, "22nd"),
        (83, "23rd"),
        (7919, "1000th"),
        (9_999_991, "664579th"),
    ] {
        let text = ok(&["facts", &prime.to_string()]);
        assert!(
            text.contains(&format!("  prime: yes, the {position} prime\n")),
            "{prime}: {text}"
        );
    }
    // Above ten million the position is left out, never guessed.
    assert!(ok(&["facts", "10000019"]).contains("  prime: yes\n"));
}

#[test]
fn highly_composite_matches_the_known_list() {
    let mut numbers: Vec<u64> = HIGHLY_COMPOSITE.to_vec();
    numbers.extend([3, 100, 1000, 720_721, 999_999, 1_000_000, 1_081_080]);
    let args: Vec<String> = std::iter::once("facts".to_string())
        .chain(numbers.iter().map(u64::to_string))
        .collect();
    let output = numplay::execute(&args).expect("facts");
    let blocks: Vec<&str> = output.split("\n\n").collect();
    assert_eq!(blocks.len(), numbers.len());
    for (block, n) in blocks.iter().zip(&numbers) {
        assert!(block.starts_with(&format!("n: {n}\n")), "block order for {n}");
        assert_eq!(
            block.contains("highly composite number"),
            HIGHLY_COMPOSITE.contains(n),
            "n = {n}"
        );
    }
}

#[test]
fn large_numbers_stay_exact() {
    let top = ok(&["facts", "18446744073709551615"]);
    assert!(top.contains("  factors: 3 x 5 x 17 x 257 x 641 x 65537 x 6700417\n"));
    assert!(top.contains("  digits: 20, digit sum 87\n"));
    let fibonacci = ok(&["facts", "12200160415121876738"]);
    assert!(fibonacci.contains("Fibonacci number (F93)"));
    let route = ok(&["collatz", "18446744073709551615"]);
    assert!(route.ends_with(" 4 2 1\n"));
}

#[test]
fn collatz_routes_and_joins() {
    assert_eq!(
        ok(&["collatz", "6"]),
        "n: 6\nsteps: 8\npeak: 16\nroute: 6 3 10 5 16 8 4 2 1\n"
    );
    assert_eq!(ok(&["collatz", "1"]), "n: 1\nsteps: 0\npeak: 1\nroute: 1\n");
    assert!(ok(&["collatz", "871"]).starts_with("n: 871\nsteps: 178\npeak: 190996\n"));

    let pair = ok(&["collatz", "27", "97"]);
    assert!(pair.contains("n: 27\nsteps: 111\npeak: 9232\n"));
    assert!(pair.contains("n: 97\nsteps: 118\npeak: 9232\n"));
    assert!(pair.ends_with("\njoins:\n  27 and 97 join at 94 (after 6 and 13 steps)\n"));

    let three = ok(&["collatz", "6", "27", "97"]);
    assert!(three.contains("  6 and 27 join at 10 (after 2 and 105 steps)\n"));
    assert!(three.contains("  6 and 97 join at 10 (after 2 and 112 steps)\n"));
    assert!(three.contains("  27 and 97 join at 94 (after 6 and 13 steps)\n"));
    assert!(three.ends_with("  all 3 routes share the road from 10\n"));

    let thirteen: Vec<&str> = std::iter::once("collatz").chain(["7"; 13]).collect();
    assert!(run(&thirteen).is_err());
}

#[test]
fn day_reports_calendar_parts() {
    assert_eq!(
        ok(&["day", "2026-11-17"]),
        "date: 2026-11-17\n\
         weekday: Tuesday\n\
         calendar parts (true only on this calendar, in base ten):\n\
         \x20 month: 11 (prime)\n\
         \x20 day: 17 (prime)\n\
         \x20 month then day: 1117 (prime)\n\
         \x20 day then month: 1711 (29 x 59)\n\
         \x20 year: 2026 (2 x 1013)\n\
         \x20 day of year: 321 (3 x 107)\n\
         \x20 ISO week: 47 (prime), in ISO year 2026\n"
    );
    let padded = ok(&["day", "2026-03-07"]);
    assert!(padded.contains(
        "  month then day: 37 (prime)\n\
         \x20 month then day, two-digit day: 307 (prime)\n\
         \x20 day then month: 73 (prime)\n\
         \x20 day then month, two-digit month: 703 (19 x 37)\n"
    ));
}

#[test]
fn day_handles_year_boundaries() {
    // date, weekday, day of year, ISO year, ISO week
    let cases = [
        ("2027-01-01", "Friday", 1, 2026, 53),
        ("2024-12-30", "Monday", 365, 2025, 1),
        ("2021-01-03", "Sunday", 3, 2020, 53),
        ("2026-01-01", "Thursday", 1, 2026, 1),
        ("2020-02-29", "Saturday", 60, 2020, 9),
        ("2000-01-01", "Saturday", 1, 1999, 52),
        ("1999-12-31", "Friday", 365, 1999, 52),
        ("2026-03-07", "Saturday", 66, 2026, 10),
    ];
    for (date, weekday, day_of_year, iso_year, iso_week) in cases {
        let text = ok(&["day", date]);
        assert!(text.contains(&format!("weekday: {weekday}\n")), "{date}: {text}");
        assert!(
            text.contains(&format!("  day of year: {day_of_year} (")),
            "{date}: {text}"
        );
        let week_line = text
            .lines()
            .find(|line| line.starts_with("  ISO week: "))
            .expect("week line");
        assert!(
            week_line.starts_with(&format!("  ISO week: {iso_week} (")),
            "{date}: {week_line}"
        );
        assert!(
            week_line.ends_with(&format!(", in ISO year {iso_year}")),
            "{date}: {week_line}"
        );
    }
}

#[test]
fn day_counts_days_between_dates() {
    assert!(ok(&["day", "2026-11-17", "--to", "2027-01-01"])
        .contains("  days to 2027-01-01: 45 (3^2 x 5)\n"));
    assert!(ok(&["day", "2024-03-01", "--to", "2024-02-28"])
        .contains("  days since 2024-02-28: 2 (prime)\n"));
    assert!(ok(&["day", "2026-11-17", "--to", "2026-11-17"]).contains("  days to 2026-11-17: 0\n"));
    let two = ok(&["day", "2026-11-17", "2026-11-18", "--to", "2026-11-20"]);
    assert_eq!(two.matches("date: ").count(), 2);
    assert!(two.contains("  days to 2026-11-20: 3 (prime)\n"));
    assert!(two.contains("  days to 2026-11-20: 2 (prime)\n"));
}

#[test]
fn day_refuses_dates_that_are_not_dates() {
    for bad in [
        vec!["day"],
        vec!["day", "2023-02-29"],
        vec!["day", "2026-13-01"],
        vec!["day", "2026-00-10"],
        vec!["day", "2026-04-31"],
        vec!["day", "2026-1-01"],
        vec!["day", "20261117"],
        vec!["day", "2026-11-17x"],
        vec!["day", "0000-01-01"],
        vec!["day", "2026-11-1\u{663}"],
        vec!["day", "2026-11-17", "--to"],
        vec!["day", "--to", "2026-11-17"],
        vec!["day", "2026-11-17", "--to", "2026-11-18", "--to", "2026-11-19"],
    ] {
        assert!(run(&bad).is_err(), "{bad:?} should be refused");
    }
}

#[test]
fn count_counts_what_a_reader_counts() {
    let expected = numplay::TextCount {
        lines: 5,
        blank_lines: 0,
        words: 17,
        words_per_line: vec![3, 2, 2, 3, 7],
        characters: 89,
        line_breaks: 4,
        lines_ending_in_space: 0,
    };
    assert_eq!(numplay::count_text(POST).unwrap(), expected);
    // The same post, however an editor saved it.
    let windows = POST.replace('\n', "\r\n");
    let old_mac = POST.replace('\n', "\r");
    for variant in [
        windows.clone(),
        old_mac,
        format!("{POST}\n"),
        format!("{windows}\r\n\r\n"),
        format!("\u{feff}{POST}\n"),
    ] {
        assert_eq!(numplay::count_text(&variant).unwrap(), expected);
    }

    let gap = numplay::count_text("a b\n\nc").unwrap();
    assert_eq!((gap.lines, gap.blank_lines, gap.words), (2, 1, 3));
    assert_eq!((gap.characters, gap.line_breaks), (4, 2));

    let trailing = numplay::count_text("a \nb").unwrap();
    assert_eq!((trailing.characters, trailing.lines_ending_in_space), (3, 1));

    // Characters are Unicode scalar values: one for an accented letter, one for a plain emoji.
    let accents = numplay::count_text("n\u{e9} \u{1f642}").unwrap();
    assert_eq!((accents.words, accents.characters), (2, 4));

    for empty in ["", "\n\n", "   \n \t\n", "\u{feff}"] {
        assert!(numplay::count_text(empty).is_err(), "{empty:?}");
    }
}

#[test]
fn the_shipped_example_post_is_true() {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../references/example-post.txt");
    let shipped = std::fs::read_to_string(path).expect("read the example post");
    assert_eq!(shipped.replace("\r\n", "\n").trim_end_matches('\n'), POST);
    let count = numplay::count_text(&shipped).unwrap();
    assert_eq!((count.lines, count.words, count.characters), (5, 17, 89));
    assert!(count.words_per_line.iter().all(|&words| numplay::is_prime(words)));
    for claim in [count.lines, count.words, count.characters] {
        assert!(numplay::is_prime(claim), "{claim} should be prime");
        assert!(POST.contains(&claim.to_string()), "the post should state {claim}");
    }
}

#[test]
fn count_reads_a_file() {
    let path = temp_file("post.txt", format!("\u{feff}{}\r\n", POST.replace('\n', "\r\n")).as_bytes());
    let output = ok(&["count", path.to_str().unwrap()]);
    std::fs::remove_file(&path).ok();
    assert_eq!(
        output,
        "lines: 5 (prime)\n\
         blank lines: 0\n\
         words: 17 (prime)\n\
         words per line: 3 2 2 3 7\n\
         lines with a prime word count: 5 of 5\n\
         characters: 89 (prime), line breaks not counted\n\
         characters with line breaks: 93 (3 x 31)\n"
    );

    let spaced = temp_file("spaced.txt", b"one two \nthree");
    let output = ok(&["count", spaced.to_str().unwrap()]);
    std::fs::remove_file(&spaced).ok();
    assert!(output.ends_with("note: 1 line(s) end with a space, and those spaces are counted\n"));

    let binary = temp_file("binary.bin", &[0xff, 0xfe, 0x41, 0x00]);
    let refused = run(&["count", binary.to_str().unwrap()]);
    std::fs::remove_file(&binary).ok();
    assert!(refused.is_err());
    assert!(run(&["count", "no-such-file-for-numplay.txt"]).is_err());
}

#[test]
fn bad_requests_are_refused() {
    for bad in [
        vec![],
        vec!["bogus"],
        vec!["facts"],
        vec!["facts", "0"],
        vec!["facts", "-5"],
        vec!["facts", "+5"],
        vec!["facts", "12a"],
        vec!["facts", "1.5"],
        vec!["facts", ""],
        vec!["facts", "\u{ff11}\u{ff12}"],
        vec!["facts", "18446744073709551616"],
        vec!["facts", "6", "0"],
        vec!["collatz"],
        vec!["collatz", "27", "x"],
        vec!["count"],
        vec!["count", "a", "b"],
    ] {
        assert!(run(&bad).is_err(), "{bad:?} should be refused");
    }
}

#[test]
fn the_program_prints_nothing_partial_and_exits_64() {
    let exe = env!("CARGO_BIN_EXE_numplay");
    for bad in [vec!["facts", "6", "0"], vec!["nope"], vec![]] {
        let output = Command::new(exe).args(&bad).output().expect("run numplay");
        assert_eq!(output.status.code(), Some(64), "{bad:?}");
        assert!(output.stdout.is_empty(), "{bad:?} printed a partial result");
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(stderr.starts_with("error: "), "{bad:?}: {stderr}");
        assert!(stderr.contains("USAGE:"), "{bad:?}: {stderr}");
    }

    let good = Command::new(exe).args(["facts", "6"]).output().expect("run numplay");
    assert_eq!(good.status.code(), Some(0));
    assert!(String::from_utf8_lossy(&good.stdout).starts_with("n: 6\n"));
    assert!(good.stderr.is_empty());

    let version = Command::new(exe).arg("--version").output().expect("run numplay");
    assert_eq!(String::from_utf8_lossy(&version.stdout), "numplay 0.1.0\n");
    let help = Command::new(exe).arg("--help").output().expect("run numplay");
    assert_eq!(help.status.code(), Some(0));
    assert_eq!(String::from_utf8_lossy(&help.stdout), numplay::USAGE);
}

#[test]
fn the_program_counts_standard_input() {
    let mut child = Command::new(env!("CARGO_BIN_EXE_numplay"))
        .args(["count", "-"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("run numplay");
    child
        .stdin
        .take()
        .expect("stdin")
        .write_all(POST.as_bytes())
        .expect("write stdin");
    let output = child.wait_with_output().expect("wait");
    assert_eq!(output.status.code(), Some(0));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("words: 17 (prime)\n"));
    assert!(stdout.contains("characters: 89 (prime), line breaks not counted\n"));
}
