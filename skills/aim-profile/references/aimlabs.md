# Aim Labs as an instrument (G4)

Aim Labs is useful here for three jobs: reading what the player has trained with, cross-checking a conversion, and comparing nearby settings on identical drills. It does not replace an in-game measurement. [Aimlabs' sensitivity guide](https://aimlabs.com/articles/aimlabs/how-to-configure-and-convert-your-sensitivity-in-aimlabs/) describes game-profile selection, manual CPI/DPI entry and FOV matching, and notes that game updates can change scales. Menu wording changes between versions; describe intent and let the player find the control.

## 1. Read the trained setup

The settings file (see history.md) records the game profile, hip sensitivity, look FOV, zoom profile, ADS sensitivity, zoom FOV and the monitor-distance conversion. A `Custom` zoom profile with explicit FOVs and `zoomMonitorDistance` 0 means the player trained ADS to the near-crosshair (MD 0%) goal. Note the file date; it may predate the player's current game.

## 2. Cross-check a conversion

1. Set the Aim Labs profile to the reference game, enter its sensitivity, the player's DPI and the reference FOV with its axis.
2. Record the turn distance Aim Labs reports, if it shows one.
3. Spin test: the player aligns on a landmark and moves the mouse the expected distance against a ruler. Landing on the landmark confirms Aim Labs' scale for that game; a miss means Aim Labs and the hand disagree, and the hand wins. Converter sites and trainers can carry the same stale scale.

## 3. A/B nearby settings

1. Take the candidate value and its two neighbors from `aimcalc steps`.
2. Use one drill the player already knows, with identical FOV and ADS settings for every run.
3. Run each value at least three times, in an interleaved order (A B C B A C …) so warm-up does not favor one value.
4. Compare the player's feel first and scores second; a single high score is noise.
5. Move one step toward the preferred value and repeat until the player settles.

## 4. ADS comparison

Use the custom zoom profile to enter the actual hip and ADS FOVs on one axis. Run the same drill with ADS at the angular ratio for each candidate goal (1 for equal turn distance; `aimcalc zoom` for MD 0% or another MD). The goal the player prefers is the ADS goal for G1 if it was not already settled.
