# Changelog

## Moved into the Null Limit skills collection (2026-11-17)

The skill and both tools are unchanged from 0.3.0. One README line changed: the install step now says to copy this folder, where it said to clone the repository.

## aim-profile 0.3.0, with aimcalc 0.2.0 and aimcount 0.1.0 (2026-11-03)

First public release.

- **Gated workflow.** The skill runs gates G0 to G6: harvest history, interpret with the player, measure, calculate, validate, apply only on request, and record a profile. Five evidence levels keep the numbers apart: measured, recorded, calculated, estimated and stated.
- **References.** Where settings and launch-log history live for CS2, Escape from Tarkov, PUBG, WARDOGS and Aim Labs. Also Aim Labs as a measuring instrument, game-model evidence, geometry and units, measuring without a ruler, and a profile template.
- **`aimcalc` 0.2.0.** Sensitivity, cm/360, calibration, step tables, FOV conversion and ADS ratios, with no per-game presets.
- **`aimcount` 0.1.0.** A passive Windows raw-input counter that measures counts per 360.
- **Build from source.** No prebuilt executables are committed; `build.ps1` builds them.
- **License.** `MIT OR Apache-2.0`.
