use std::process::{Command, Output};

fn run(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_aimcalc"))
        .args(args)
        .output()
        .expect("execute aimcalc")
}

fn success(args: &[&str]) -> String {
    let out = run(args);
    assert!(
        out.status.success(),
        "{:?}: {}",
        args,
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(out.stderr.is_empty());
    String::from_utf8(out.stdout).unwrap()
}

fn number(output: &str, key: &str) -> f64 {
    output
        .lines()
        .find_map(|s| s.strip_prefix(&format!("{key}=")))
        .expect("numeric output field")
        .parse()
        .unwrap()
}

fn reject(args: &[&str]) {
    let out = run(args);
    assert_eq!(
        out.status.code(),
        Some(64),
        "accepted invalid args: {args:?}"
    );
    assert!(
        out.stdout.is_empty(),
        "partial result for invalid input: {args:?}"
    );
    assert!(String::from_utf8_lossy(&out.stderr).starts_with("error:"));
}

#[test]
fn help_and_version() {
    assert!(success(&["--help"]).contains("calibrate"));
    assert_eq!(success(&["--version"]), "aimcalc 0.2.0\n");
    reject(&[]);
}

#[test]
fn calibrated_scale_and_changed_dpi_round_trip_through_cli() {
    let result = success(&[
        "calibrate",
        "--sens",
        "0.658",
        "--dpi",
        "1050",
        "--cm",
        "38",
    ]);
    let yaw = number(&result, "yaw").to_string();
    let converted = success(&[
        "convert",
        "--from-yaw",
        &yaw,
        "--sens",
        "0.658",
        "--dpi",
        "1050",
        "--to-yaw",
        "0.022",
        "--to-dpi",
        "800",
    ]);
    assert!((number(&converted, "cm_per_360") - 38.0).abs() < 1e-10);
    let sens = number(&converted, "sensitivity").to_string();
    let back = success(&["cm360", "--yaw", "0.022", "--dpi", "800", "--sens", &sens]);
    assert!((number(&back, "cm_per_360") - 38.0).abs() < 1e-10);
}

#[test]
fn invalid_numeric_domains_and_overflow_fail_without_a_result() {
    for bad in ["0", "-1", "NaN", "inf", "-inf", "nonsense"] {
        reject(&["cm360", "--yaw", "0.022", "--dpi", bad, "--sens", "1"]);
        reject(&["cm360", "--yaw", bad, "--dpi", "1050", "--sens", "1"]);
        reject(&["calibrate", "--dpi", "1050", "--sens", "0.658", "--cm", bad]);
    }
    reject(&[
        "cm360", "--yaw", "1e308", "--dpi", "1050", "--sens", "1e308",
    ]);
    reject(&[
        "cm360", "--yaw", "1e-300", "--dpi", "1050", "--sens", "1e-300",
    ]);
}

#[test]
fn ambiguous_missing_and_ignored_options_are_errors() {
    for args in [
        &[
            "cm360", "--yaw", "0.022", "--dpi", "1050", "--sens", "1", "--sens", "2",
        ][..],
        &[
            "cm360", "--yaw", "0.022", "--dpi", "1050", "--sens", "1", "--fov", "90",
        ],
        &[
            "cm360",
            "--yaw",
            "0.022",
            "--game",
            "wardogs-estimate",
            "--dpi",
            "1050",
            "--sens",
            "1",
        ],
        &[
            "cm360",
            "--game",
            "wardogs-estimate",
            "--dpi",
            "1050",
            "--sens",
            "1",
            "--file",
            "0.3",
        ],
        &["cm360", "--yaw", "0.022", "--dpi", "1050", "--file", "0.3"],
        &["cm360", "--dpi", "--sens", "1"],
        &["cm360", "--yaw", "0.022", "--dpi", "1050"],
        &["zoom", "--mode", "turn", "--md", "0"],
    ] {
        reject(args);
    }
}

#[test]
fn unsupported_games_cannot_silently_select_a_scale() {
    for game in ["wardogs", "cs2", "apex", "tarkov", "pubg", "aimlabs"] {
        reject(&["cm360", "--game", game, "--dpi", "1050", "--sens", "0.658"]);
    }
}

#[test]
fn provisional_profile_is_labeled_on_source_and_target() {
    let result = success(&[
        "sens",
        "--game",
        "wardogs-estimate",
        "--dpi",
        "1050",
        "--cm",
        "38",
    ]);
    assert!(result.contains("UNVERIFIED") && result.contains("unverified"));
    // Independently evaluated with 40-digit decimal arithmetic for this provisional model.
    assert!((number(&result, "sensitivity") - 0.5800453514739229).abs() < 1e-12);
    let source = success(&[
        "convert",
        "--from-game",
        "wardogs-estimate",
        "--sens",
        "0.58",
        "--to-yaw",
        "0.022",
        "--dpi",
        "1050",
    ]);
    assert!(source.contains("source_warning=WARDOGS model is unverified"));
    reject(&[
        "sens",
        "--game",
        "wardogs-estimate",
        "--dpi",
        "1050",
        "--cm",
        "1",
    ]);
}

#[test]
fn finite_table_never_exceeds_requested_range() {
    let output = success(&[
        "steps", "--yaw", "0.022", "--dpi", "1050", "--from", "0.5", "--to", "0.515", "--step",
        "0.01",
    ]);
    let rows: Vec<&str> = output.lines().skip(2).collect();
    assert_eq!(rows.len(), 2);
    assert!(rows
        .iter()
        .all(|s| s.split(',').next().unwrap().parse::<f64>().unwrap() <= 0.515));
    let output = success(&[
        "steps", "--yaw", "0.022", "--dpi", "1050", "--from", "0.57", "--to", "0.59", "--step",
        "0.01",
    ]);
    assert_eq!(output.lines().skip(2).count(), 3);
    let rows: Vec<f64> = output
        .lines()
        .skip(2)
        .map(|s| s.split(',').nth(1).unwrap().parse().unwrap())
        .collect();
    assert!(rows[0] > rows[1] && rows[1] > rows[2]);
    assert!((rows[0] - rows[1]) > (rows[1] - rows[2]));
    reject(&[
        "steps", "--yaw", "0.022", "--dpi", "1050", "--from", "0.5", "--to", "1", "--step",
        "0.000001",
    ]);
    reject(&[
        "steps", "--yaw", "0.022", "--dpi", "1050", "--from", "1", "--to", "0.5",
    ]);
    reject(&[
        "steps", "--yaw", "0.022", "--dpi", "1050", "--from", "0.5", "--to", "1", "--step", "0",
    ]);
}

#[test]
fn aspect_and_fov_domains_are_explicit() {
    let output = success(&["fov", "--vfov", "70", "--aspect", "2560x1440"]);
    assert!((number(&output, "horizontal_fov_degrees") - 102.44785777798825).abs() < 1e-10);
    for aspect in [
        "0:9",
        "16:0",
        "-16:-9",
        "NaN:9",
        "16",
        "16:9:2",
        "1e308:1e-308",
    ] {
        reject(&["fov", "--vfov", "70", "--aspect", aspect]);
    }
    for fov in ["0", "180", "181", "-1", "NaN", "inf"] {
        reject(&["fov", "--vfov", fov, "--aspect", "16:9"]);
    }
    reject(&["fov", "--vfov", "70"]);
    reject(&["fov", "--vfov", "70", "--hfov", "102", "--aspect", "16:9"]);
}

#[test]
fn row_limit_is_enforced_at_the_boundary() {
    let output = success(&[
        "steps", "--yaw", "0.022", "--dpi", "1050", "--from", "1", "--to", "1000", "--step", "1",
    ]);
    assert_eq!(output.lines().skip(2).count(), 1000);
    reject(&[
        "steps", "--yaw", "0.022", "--dpi", "1050", "--from", "1", "--to", "1001", "--step", "1",
    ]);
}

#[test]
fn equal_turn_and_monitor_matching_are_different_explicit_operations() {
    assert_eq!(
        number(
            &success(&["zoom", "--mode", "turn"]),
            "ads_per_hip_angular_multiplier"
        ),
        1.0
    );
    let result = success(&[
        "zoom",
        "--hip-fov",
        "70",
        "--ads-fov",
        "50",
        "--axis",
        "vertical",
        "--md",
        "0",
    ]);
    assert!((number(&result, "ads_per_hip_angular_multiplier") - 0.6659563525226446).abs() < 1e-12);
    reject(&["zoom", "--hip-fov", "70", "--ads-fov", "50"]);
    for md in ["-1", "101", "NaN", "inf"] {
        reject(&[
            "zoom",
            "--hip-fov",
            "70",
            "--ads-fov",
            "50",
            "--axis",
            "vertical",
            "--md",
            md,
        ]);
    }
}
