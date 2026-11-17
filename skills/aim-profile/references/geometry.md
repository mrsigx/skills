# Geometry and units

For a constant linear input path, let `d` be camera degrees per mouse count, `D` be DPI (counts per inch), and `s` be sensitivity. A linear yaw model has `d = yaw * s`. Then:

```text
cm_per_360 = 360 * 2.54 / (D * d)
d = 360 * 2.54 / (D * cm_per_360)
calibrated_yaw = d / s
```

The last equation is a local linear fit. Offset scales need their inverse mapping rather than multiplying menu values. With acceleration or variable gain, one yaw constant is inadequate. Changes in effective DPI must be included even when the menu sensitivity is unchanged.

For rectilinear perspective, display aspect `a = width / height` and full vertical/horizontal angles `V`, `H`:

```text
H = 2 * atan(tan(V / 2) * a)
V = 2 * atan(tan(H / 2) / a)
```

Trig operates in radians; the CLI accepts and returns degrees. Angles must lie strictly between 0 and 180 degrees. These equations concern the rendered camera FOV, not necessarily an engine's menu convention or a picture-in-picture scope.

For normalized distance `p` from screen center, measured as a fraction of the half-screen on the same axis as FOV `F`, the camera rotation to that position is `atan(p * tan(F/2))`. Therefore the angular ADS/hip ratio that preserves hand travel to the same normalized distance is:

```text
m = atan(p * tan(ADS_FOV/2)) / atan(p * tan(HIP_FOV/2))
as p -> 0: m = tan(ADS_FOV/2) / tan(HIP_FOV/2)
at p = 1: m = ADS_FOV / HIP_FOV
```

The CLI's `--md` is `100*p`, supported from 0 to 100. A game's coefficient named “0”, “100”, “1.3” or “133%” may use another convention; do not substitute it automatically. Equal physical turn distance is a separate choice, `m = 1`. Translating `m` to an ADS menu value requires the target game's actual mapping and any automatic zoom compensation.
