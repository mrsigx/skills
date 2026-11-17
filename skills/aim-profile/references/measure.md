# Measuring counts per 360 without a ruler (G2)

`bin/aimcount.exe` counts the raw mouse counts the game receives between two hotkeys. Counts per 360 is the quantity to match: two games turned the same number of counts per 360 feel identical in physical travel, whatever the mouse's true DPI, because a DPI error multiplies both games' counts equally. `--dpi` only converts counts into cm for display.

It only listens. It registers for raw input as a background sink, the same Windows mechanism overlays and mouse utilities use, and never sends or alters input. Anti-cheat behavior is the game vendor's; say so, and prefer offline or practice modes.

## Procedure

1. Start it where its output can be read afterwards (a second monitor, or alt-tab when done):
   `aimcount --dpi 1050 --label "tarkov 0.658 fov70" --log <profile folder>\measurements.jsonl`
   Put the game, setting and aim state in the label.
2. In game, stand still in an offline or practice area. Put the crosshair exactly on a sharp vertical edge.
3. Press **F9** (beep). Turn one full rotation the same direction until the edge is back under the crosshair. Overshoot and correct freely; movement in both directions nets out. Press **F10** (beep); the line prints and is logged.
4. Repeat at least three times, slow and medium speed. Repeats should agree within about 1%. A spread that grows with speed means turn-rate limiting or acceleration: use the slow results and note it. Turn slowly in games with inertia (Tarkov).
5. **F8** quits.

Check before trusting it: with nothing else touching the mouse, `counts_y` should stay small relative to `counts_x`.

## Using the results

- Same-game matching: target counts = reference counts.
- Display: cm/360 = counts / DPI x 2.54 (printed with `--dpi`).
- One setting measured: `aimcalc calibrate --sens S --dpi D --cm C` gives that operating point's gain.
- An offset slider needs two or more measured settings; fit degrees/count = 360 / counts against the menu value, and keep the fit's inputs in the answer.
- Log lines are JSON with `unix_time`, `label`, `counts_x`, `counts_y`, `dpi`, `cm_x`.

Verified 2026-10-04 on Windows 11 with a desktop self-test: injected F9, ten +100 relative moves, F10, F8 gave `counts_x=1000 counts_y=0` on two runs.
