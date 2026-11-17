# Calculator use

`aimcalc` 0.2.0 is dependency-free Rust. Build the Windows x64 executable once with `build.ps1` at the skill root. Run it on demand; no service or installation is needed. Resolve `bin/aimcalc.exe` relative to this skill folder, not the current shell directory. Arguments are separate `--flag value` pairs. Help/version and successful calculations exit 0; invalid requests exit 64 with no partial result on stdout.

PowerShell example:

```powershell
$aimcalc = '<skill-root>\bin\aimcalc.exe'
& $aimcalc --help
```

Replace `<skill-root>` with the actual skill directory. Do not silently change the example values into a user's current settings.

## Physical matching

```powershell
# Conditional calculation using an explicitly supplied linear scale.
& $aimcalc cm360 --yaw 0.022 --sens 1.004282 --dpi 1050

# Estimate a local scale from a physical measurement; 38 cm is an example input.
& $aimcalc calibrate --sens 0.658 --dpi 1050 --cm 38

# A synthetic unit example: the result is sensitivity 0.5 at 1600 DPI.
& $aimcalc convert --from-yaw 0.022 --sens 2 --dpi 800 --to-yaw 0.044 --to-dpi 1600

# Find a target sensitivity under an explicitly supplied scale.
& $aimcalc sens --yaw 0.022 --dpi 1050 --cm 38

# Explicitly provisional WARDOGS comparison, not a verified recommendation.
& $aimcalc sens --game wardogs-estimate --dpi 1050 --cm 38
& $aimcalc cm360 --game wardogs-estimate --dpi 1050 --file 0.317895
& $aimcalc steps --game wardogs-estimate --dpi 1050 --from 0.57 --to 0.59 --step 0.01
```

`steps` prints a CSV table after model/warning lines and is limited to 1,000 rows. It includes a reachable upper endpoint and never rounds up to a sensitivity above `--to`. Its delta column is the signed change from the previous row, not a universal per-step constant. `convert` preserves physical cm/360, including changes in DPI. `--file` is a numeric configuration value, not a file path or permission to read a game file.

## FOV and ADS

```powershell
& $aimcalc fov --vfov 70 --aspect 16:9
& $aimcalc fov --hfov 96 --aspect 2560x1440

# Equal physical turn distance.
& $aimcalc zoom --mode turn

# Equal infinitesimal near-crosshair screen movement in a rectilinear camera.
& $aimcalc zoom --hip-fov 70 --ads-fov 50 --axis vertical --md 0

# Match hand travel from screen center to the edge on the horizontal axis.
& $aimcalc zoom --hip-fov 120 --ads-fov 60 --axis horizontal --md 100
```

Aspect is required for FOV conversion; `W:H` and `WxH` are accepted. ADS FOVs must use the same declared axis. These commands return angular ratios, not a game's named coefficient. `zoom --mode turn` accepts no extra FOV/monitor options because those would imply an unused constraint.

## Rebuild or use another platform

With an existing Rust toolchain, use an allowed, task-owned output directory:

```text
cargo test --offline --manifest-path <skill-root>/aimcalc/Cargo.toml --target-dir <build-dir>
cargo build --release --offline --manifest-path <skill-root>/aimcalc/Cargo.toml --target-dir <build-dir>
```

The resulting program is `<build-dir>/release/aimcalc.exe` on Windows or `aimcalc` elsewhere. The executable was built and exercised on Windows; other platforms have source available but are not certified by that run. Do not install a toolchain or write to a restricted location just to execute this skill.
