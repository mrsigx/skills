# Game-model evidence

Reviewed 2026-10-02; amended 2026-10-04. Versioned game behavior can change. A model's mathematical consistency is distinct from an in-game measurement.

## Evidence to prefer

Use a repeated physical measurement for the precise game/build/state, or a current documented input equation with its assumptions checked. A file value is useful evidence of configuration, but two file/menu pairs alone do not establish degrees per count. If the model is uncertain, label the calculation as conditional.

## CS2 and related games

[Valve's Source SDK mouse implementation](https://github.com/ValveSoftware/source-sdk-2013/blob/master/src/game/client/in_mouse.cpp) is primary source code for Source SDK input behavior, including a configurable yaw factor and separate acceleration paths. It is not proof of the current CS2 or Apex implementation. Only pass `--yaw 0.022` when that assumption is supported for the actual setup; otherwise calibrate. The skill does not equate an engine-family name with a verified current scale.

## WARDOGS: provisional inherited estimate

The interrupted draft used this model, retained solely for explicit comparison:

```text
menu range: 0.1 .. 2.0
degrees_per_count = 0.007 + (menu - 0.1) * 0.063 / 1.9
estimated_ini_MouseSensitivity = degrees_per_count / 0.07
```

It reproduces the recorded pairs menu 0.73 -> file 0.398421 and menu 0.56 -> file 0.317895 to their stored precision. Those pairs were recorded from one player's settings, not from a newly performed game measurement. The endpoints' angular units and ADS behavior have not been independently verified. No official source for this WARDOGS equation has been found.

Use `--game wardogs-estimate` only when an estimated comparison is appropriate and keep the emitted warning in the answer. `--game wardogs` is rejected. Results outside the inherited menu range are rejected rather than extrapolated. A source conversion using this profile also stays unverified even if the target yaw is known.

Recorded on one machine in October 2026: `AimingFovSensitivityMode=Legacy` (previously `Relative`) with `MouseSensitivity=0.526316`, which the estimate decodes to menu 1.00. Whether `Legacy` changes hipfire gain, ADS gain or both is unknown; do not apply the estimate across a mode change without a fresh measurement.

**Contradicted (stated, unmeasured):** one player ran exactly that setup (Legacy, menu 1.0, ADS 1.0, FOV 103, 1050 DPI) and puts it near 35 cm/360. The estimate predicts 23.6 cm. That is too far apart to be an estimation error, so treat the estimate as wrong, at least in Legacy mode. Calibrate from a ruler measurement instead (`aimcalc calibrate`).

Changing FOV preserves physical cm/360 only if the game's input path leaves degrees/count unchanged. The skill cannot certify that WARDOGS does this. Names such as `Relative`, `Affected` and a coefficient of `100` do not establish its zoom equation. Measure before prescribing an ADS setting.

## Escape from Tarkov and PUBG

No built-in scale is supplied. Earlier sources contained conflicting Tarkov coefficients and a rough physical estimate; neither is a verified calibration. Tarkov's `OpticSensitivity` is not a look-gain factor (stated by a player, 2026-10-02: it sets how responsive a variable scope's zoom is); leave it out of conversions. Use a repeatable measurement with the exact aim state. Equal hip/aim slider numbers in either game do not establish equal physical gain after engine processing. Interpret individual configuration keys only with current evidence.

## Aimlabs

[Aimlabs' official sensitivity guide](https://aimlabs.com/articles/aimlabs/how-to-configure-and-convert-your-sensitivity-in-aimlabs/) explains selection of a game profile, manual CPI/DPI entry and FOV matching, and notes that game updates can change sensitivity scales. Treat the trainer as a useful cross-check, with the actual profile and DPI recorded; its displayed conversion does not replace an in-game measurement.

The inherited example used vertical 70/50 FOV and stored sensitivities 0.6064115/0.4042743. Their ratio is about 0.666666612, whereas the rectilinear 0% monitor-distance calculation is about 0.665956353. The difference is about 0.000710259, or 0.107% relative to the geometric value. Its cause is unresolved. Do not describe this stored pair as an exact independent validation of the geometric equation, or guess that engine behavior or rounding caused it.
