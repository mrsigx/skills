---
name: aim-profile
description: Find and carry a player's aim profile across games — mouse sensitivity, cm/360, FOV and ADS feel — using their own historical settings, measured turn distance, explicit math and Aim Labs as instruments, advancing through evidence gates. Use for "best sensitivity for this game", converting sens between games or aim trainers, cm/360 calibration, FOV conversion, or separating physical turn matching from perceived ADS matching.
---

# Aim Profile

Find the setting the player's hands already trust, then carry it into the target game with visible assumptions. Separate physical mouse-to-camera gain, FOV geometry and the game's ADS behavior. Equal menu numbers do not establish equal movement or feel.

Keep evidence levels distinct in every table and answer:

- **Measured:** repeated physical mouse travel for a known camera rotation, with DPI and aiming state recorded.
- **Recorded:** a dated configuration value, launch-log entry or screenshot; this proves the setting, not its engine interpretation.
- **Calculated:** arithmetic under an explicit model, with its inputs and units.
- **Estimated:** an assumed scale, approximate measurement, or unverified game behavior.
- **Stated:** what the player says a setting does or how something feels. Their account of their own controls outranks a third-party converter page.

## Gates

Advance in order. Each gate names what must hold before the next; say which gate you are at and what is missing. A later gate never back-fills an earlier one.

**G0 — Harvest history (read-only).** Before asking for numbers, look for them. Inventory installed games, then read each relevant game's settings files and launch logs per [history sources](references/history.md). Launch logs can give a dated per-session series that shows when a value changed; a single file gives only the endpoint. Build a reference table: game, key, value, date, evidence level. DPI is usually not in any game file — ask for it. Exit: the table exists, and every unresolved key is listed.

**G1 — Interpret with the player.** Ask what ambiguous keys do before treating them as aim gain (a "scope sensitivity" may control zoom speed, not look speed). Establish which game the player calls dialed in, and what "ADS same as hipfire" means to them (below). Exit: one reference game and one ADS goal, both stated by the player.

**G2 — Measure the reference.** Measure the reference game's counts per 360 with `bin/aimcount.exe` per [measuring without a ruler](references/measure.md), or its cm/360 with a ruler (procedure under *Calibrate*). If the player can only estimate, carry it as **Estimated** and say the answer is conditional. Exit: a measured or explicitly estimated reference turn distance.

**G3 — Calculate.** Use `aimcalc` for every number. Show the command or its inputs. Produce the target value plus a `steps` table of neighbors, because equal slider increments need not give equal cm/360 differences. Exit: target values with their model named.

**G4 — Validate.** Check the result in the target game and in Aim Labs per [Aim Labs as an instrument](references/aimlabs.md): spin test against the hand measurement, then A/B the neighboring steps on one drill. Iterate in `steps` increments on the player's feedback. Exit: the player accepts a value, or the gap is named.

**G5 — Apply (only on explicit request).** Confirm the game is closed so it cannot overwrite the file. Back up the original, show the diff, make the bounded edit, re-read the saved value. Warn that a later menu Apply may round or re-encode it. Analysis or packaging alone never authorizes editing game files or mouse settings; do not re-ask for an approval already given.

**G6 — Record the profile.** With the player's agreement, write or update a dated JSON profile following [profile template](references/profile.template.json) in the workspace the player chooses. Live files outrank a stored profile; re-read them before reusing it.

## Choose what to preserve

For **hipfire physical matching**, preserve cm/360. This uses DPI and angular gain, not FOV, provided the game does not change that gain with FOV or other state.

For **ADS matching**, resolve what "same as hipfire" means:

- Equal physical turn distance: angular ADS/hip ratio 1.
- Equal near-crosshair screen movement: the rectilinear monitor-distance 0% limit.
- A particular flick distance: monitor-distance matching at a specified fraction of the half-screen, on a named axis.

A player's history can reveal which one they have trained (for example, equal hip/aim values in several games versus an Aim Labs zoom profile at MD 0%). When the history conflicts, present both and test.

Use actual rendered FOVs on the same axis. Convert a documented 4:3-normalized horizontal FOV to vertical at 4:3 first, then to rendered horizontal at the display aspect. A menu label alone does not establish the convention. Geometric ADS ratios are targets, **not verified game slider coefficients**; account for the game's built-in zoom scaling before mapping them to settings. Matching the reference game's rendered FOV keeps on-screen motion comparable; it is the player's call whether to match it.

## Calculate and calibrate

Use `bin/aimcalc.exe` on Windows; build it once with `build.ps1` (see the README), since no executable is committed. Portable Rust source is in `aimcalc/`; [CLI instructions and examples](references/cli.md) include offline build and test commands. The executable reads arguments and writes stdout/stderr only; it never edits settings or detects DPI. Run `--help` for the exact accepted options. Keep full precision and round only the displayed recommendation.

Read [game-model evidence](references/games.md) when a named game's scale or ADS behavior is needed. Do not infer a current coefficient from the engine family or another game's preset. The calculator has no silent per-game presets; WARDOGS's provisional model requires the explicit `wardogs-estimate` name and prints a warning — keep it in the answer.

When the scale is unknown, measure it. Align with a fixed landmark and measure a 360-degree turn; a measured 180-degree turn can be doubled. Repeat at least three times, slow and faster, without changing DPI, sensitivity, aim state or sight. Turn slowly in games with turn-rate limits or inertia. Inconsistent distances call for diagnosing changing gain or measurement error before calculating an "exact" conversion. Enter a repeatable result into `calibrate`; a single-point fit describes that operating point only.

Use [geometry and units](references/geometry.md) to explain a calculation or diagnose a discrepancy. Passing arithmetic tests does not validate a game's current input implementation. Do not alter a formula to force agreement with a stored configuration ratio.

## Deliver

Report the reference inputs with evidence levels and dates, the gate reached, the target sensitivity and turn distance, the FOV convention, and the chosen ADS goal. Name the next measurement if a game-specific mapping is unresolved; avoid false precision from estimated inputs.
