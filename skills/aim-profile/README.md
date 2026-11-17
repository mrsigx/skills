# aim-profile

Find the mouse sensitivity your hands already trust, then carry it into another game with every assumption written down.

aim-profile is an agent skill for Claude Code and Codex, plus two small Rust tools with no dependencies:

- **`aimcalc`** works out sensitivity, cm/360, FOV and ADS ratios under a model you name. It reads arguments and prints results. It never edits settings and never detects DPI.
- **`aimcount`** is a passive mouse counter for Windows. Press F9, turn in game, press F10, and it reports the raw counts, plus centimetres of mouse travel if you pass `--dpi`. It reads raw input only and never sends any.

## Why

The same menu number doesn't mean the same movement:
- Games scale input differently.
- FOV conventions differ.
- "ADS the same as hipfire" can mean equal turn distance or equal on-screen movement.

The skill keeps three things apart: physical turn distance, FOV geometry, and the game's own ADS behavior. It labels every number as **measured**, **recorded**, **calculated**, **estimated** or **stated**. A converter's guess never passes as a measurement.

## How the skill works

It moves through gates in order, and a later gate never fills in for an earlier one. Details are in [SKILL.md](SKILL.md).

| Gate | What happens |
|---|---|
| G0 | Read the player's history (read-only): game settings files and launch logs, per [history sources](references/history.md) |
| G1 | Interpret ambiguous keys with the player; agree on one reference game and one ADS goal |
| G2 | Measure the reference turn, with `aimcount` or a ruler ([measuring](references/measure.md)) |
| G3 | Calculate with `aimcalc`, showing the inputs and a table of neighboring steps |
| G4 | Validate in the game and in Aim Labs ([Aim Labs as an instrument](references/aimlabs.md)) |
| G5 | Apply the setting only on explicit request: game closed, original backed up, diff shown |
| G6 | Record a dated profile from the [template](references/profile.template.json) |

## Install

Copy this folder into your skills folder:

- **Claude Code:** `~/.claude/skills/aim-profile`
- **Codex:** `~/.codex/skills/aim-profile`. `agents/openai.yaml` holds Codex's display text.

Then build the two tools once. This needs Windows and a Rust toolchain:

```powershell
pwsh -File ~/.claude/skills/aim-profile/build.ps1
```

`build.ps1` builds both crates, runs `aimcalc`'s tests, and copies `aimcalc.exe` and `aimcount.exe` into `bin\`. No executable is committed to this repository.

## Using the tools directly

These are the documented examples from [references/cli.md](references/cli.md); the numbers are example inputs:

```powershell
bin\aimcalc.exe --help
bin\aimcalc.exe fov --vfov 70 --aspect 16:9
bin\aimcalc.exe calibrate --sens 0.658 --dpi 1050 --cm 38
bin\aimcalc.exe convert --from-yaw 0.022 --sens 2 --dpi 800 --to-yaw 0.044 --to-dpi 1600
bin\aimcalc.exe zoom --hip-fov 70 --ads-fov 50 --axis vertical --md 0
```

Invalid requests exit 64 and print no partial result.

`aimcount`:

```powershell
bin\aimcount.exe --dpi 800 --label "cs2 sens 1.0 hipfire" --log measurements.jsonl
```

- **Keys:** F9 starts or restarts counting, F10 stops and reports, F8 quits.
- **Log:** `--log` appends one JSON line per measurement.
- **Procedure:** [references/measure.md](references/measure.md) has the full no-ruler method.

## Game models

The calculator has no silent per-game presets:
- **Source-engine yaw** comes from [Valve's Source SDK](https://github.com/ValveSoftware/source-sdk-2013/blob/master/src/game/client/in_mouse.cpp).
- **Aim Labs behavior** comes from [Aim Labs' own guide](https://aimlabs.com/articles/aimlabs/how-to-configure-and-convert-your-sensitivity-in-aimlabs/).
- **WARDOGS** has only a provisional `wardogs-estimate` model, which prints a warning and is contradicted in Legacy mode.

See [references/games.md](references/games.md). When a game's scale is unknown, measure it.

## Platform

Built and tested on Windows 11:
- `aimcount` uses the Windows Raw Input API, so it is Windows-only.
- `aimcalc` is portable Rust, but builds on other platforms are untested.

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT), at your option.
