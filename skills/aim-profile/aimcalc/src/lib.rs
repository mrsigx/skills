//! Deterministic sensitivity and perspective geometry. No file or network I/O.

use std::collections::HashMap;

const CM_PER_IN: f64 = 2.54;
const MAX_ROWS: usize = 1000;
const WARDOGS_DIVISOR: f64 = 0.07;

fn positive(value: f64, name: &str) -> Result<f64, String> {
    if value.is_finite() && value > 0.0 {
        Ok(value)
    } else {
        Err(format!("{name} must be finite and greater than zero"))
    }
}

fn angle(value: f64, name: &str) -> Result<f64, String> {
    positive(value, name)?;
    if value >= 180.0 {
        return Err(format!("{name} must be less than 180 degrees"));
    }
    Ok(value)
}

#[derive(Clone, Copy, Debug)]
enum Scale {
    Linear(f64),
    WardogsEstimate,
}

impl Scale {
    fn sensitivity(self, value: f64) -> Result<f64, String> {
        positive(value, "sensitivity")?;
        if matches!(self, Self::WardogsEstimate) && !(0.1..=2.0).contains(&value) {
            return Err("WARDOGS estimate supports menu sensitivity 0.1 through 2.0 only".into());
        }
        Ok(value)
    }

    fn deg_per_count(self, sens: f64) -> Result<f64, String> {
        self.sensitivity(sens)?;
        positive(
            match self {
                Self::Linear(yaw) => yaw * sens,
                Self::WardogsEstimate => 0.007 + (sens - 0.1) * 0.063 / 1.9,
            },
            "degrees per count",
        )
    }

    fn sens_for(self, deg: f64) -> Result<f64, String> {
        positive(deg, "degrees per count")?;
        self.sensitivity(match self {
            Self::Linear(yaw) => deg / yaw,
            Self::WardogsEstimate => 0.1 + (deg - 0.007) * 1.9 / 0.063,
        })
    }

    fn label(self) -> &'static str {
        match self {
            Self::Linear(_) => "caller_supplied_linear_yaw",
            Self::WardogsEstimate => "UNVERIFIED_wardogs_estimate",
        }
    }
}

fn cm360(deg: f64, dpi: f64) -> Result<f64, String> {
    positive(deg, "degrees per count")?;
    positive(dpi, "DPI")?;
    positive(360.0 / deg / dpi * CM_PER_IN, "cm per 360 result")
}

fn deg_for_cm(cm: f64, dpi: f64) -> Result<f64, String> {
    positive(cm, "cm per 360")?;
    positive(dpi, "DPI")?;
    positive(360.0 * CM_PER_IN / cm / dpi, "degrees per count result")
}

fn parse_aspect(text: &str) -> Result<f64, String> {
    let (w, h) = text
        .split_once(':')
        .or_else(|| text.split_once('x'))
        .ok_or("aspect must be W:H or WxH, for example 16:9 or 2560x1440")?;
    let w: f64 = w.trim().parse().map_err(|_| "invalid aspect width")?;
    let h: f64 = h.trim().parse().map_err(|_| "invalid aspect height")?;
    positive(w, "aspect width")?;
    positive(h, "aspect height")?;
    positive(w / h, "aspect ratio")
}

fn h_from_v(v: f64, aspect: f64) -> Result<f64, String> {
    angle(v, "vertical FOV")?;
    positive(aspect, "aspect ratio")?;
    angle(
        2.0 * ((v.to_radians() / 2.0).tan() * aspect).atan().to_degrees(),
        "horizontal FOV result",
    )
}

fn v_from_h(h: f64, aspect: f64) -> Result<f64, String> {
    angle(h, "horizontal FOV")?;
    positive(aspect, "aspect ratio")?;
    angle(
        2.0 * ((h.to_radians() / 2.0).tan() / aspect).atan().to_degrees(),
        "vertical FOV result",
    )
}

fn zoom_multiplier(hip: f64, ads: f64, md: f64) -> Result<f64, String> {
    angle(hip, "hip FOV")?;
    angle(ads, "ADS FOV")?;
    if !md.is_finite() || !(0.0..=100.0).contains(&md) {
        return Err("monitor distance must be 0 through 100 percent of the half-screen".into());
    }
    let hip_tan = (hip.to_radians() / 2.0).tan();
    let ads_tan = (ads.to_radians() / 2.0).tan();
    let value = if md == 0.0 {
        ads_tan / hip_tan
    } else {
        (md / 100.0 * ads_tan).atan() / (md / 100.0 * hip_tan).atan()
    };
    positive(value, "ADS/hip multiplier result")
}

struct Args(HashMap<String, String>);

impl Args {
    fn parse(raw: &[String]) -> Result<Self, String> {
        let mut values = HashMap::new();
        let mut iter = raw.iter();
        while let Some(flag) = iter.next() {
            let key = flag
                .strip_prefix("--")
                .filter(|k| !k.is_empty())
                .ok_or_else(|| format!("expected --flag, got '{flag}'"))?;
            let value = iter
                .next()
                .filter(|v| !v.starts_with("--"))
                .ok_or_else(|| format!("--{key} needs a value"))?;
            if values.insert(key.to_owned(), value.clone()).is_some() {
                return Err(format!("duplicate --{key}"));
            }
        }
        Ok(Self(values))
    }

    fn allow(&self, allowed: &[&str]) -> Result<(), String> {
        let mut unknown: Vec<_> = self
            .0
            .keys()
            .filter(|k| !allowed.contains(&k.as_str()))
            .collect();
        unknown.sort();
        if let Some(key) = unknown.first() {
            return Err(format!("unknown or inapplicable --{key}"));
        }
        Ok(())
    }

    fn get(&self, key: &str) -> Option<&str> {
        self.0.get(key).map(String::as_str)
    }

    fn required(&self, key: &str) -> Result<&str, String> {
        self.get(key).ok_or_else(|| format!("missing --{key}"))
    }

    fn num(&self, key: &str) -> Result<f64, String> {
        let text = self.required(key)?;
        let value: f64 = text
            .parse()
            .map_err(|_| format!("--{key} '{text}' is not a number"))?;
        if !value.is_finite() {
            return Err(format!("--{key} must be finite"));
        }
        Ok(value)
    }

    fn pos(&self, key: &str) -> Result<f64, String> {
        positive(self.num(key)?, &format!("--{key}"))
    }

    fn optional(&self, key: &str, default: f64) -> Result<f64, String> {
        if self.get(key).is_some() {
            self.num(key)
        } else {
            Ok(default)
        }
    }

    fn scale(&self, prefix: &str) -> Result<Scale, String> {
        let game = format!("{prefix}game");
        let yaw = format!("{prefix}yaw");
        match (self.get(&game), self.get(&yaw)) {
            (None, Some(_)) => Ok(Scale::Linear(self.pos(&yaw)?)),
            (Some("wardogs-estimate"), None) => Ok(Scale::WardogsEstimate),
            (Some(name), None) => Err(format!("no verified built-in scale for '{name}'; supply --{yaw} from a checked scale or measurement. WARDOGS's provisional model requires --{game} wardogs-estimate")),
            _ => Err(format!("pass exactly one of --{game} or --{yaw}")),
        }
    }
}

fn describe(scale: Scale, sens: f64, dpi: f64) -> Result<String, String> {
    let deg = scale.deg_per_count(sens)?;
    let cm = cm360(deg, dpi)?;
    let mut out = format!("model={}\nsensitivity={sens}\ndpi={dpi}\ndeg_per_count={deg}\ncm_per_360={cm}\nin_per_360={}\n", scale.label(), positive(cm / CM_PER_IN, "inches per 360 result")?);
    if matches!(scale, Scale::WardogsEstimate) {
        out.push_str(&format!("estimated_ini_MouseSensitivity={}\nwarning=WARDOGS model is unverified; validate in game before adopting\n", deg / WARDOGS_DIVISOR));
    }
    Ok(out)
}

fn run(cmd: &str, a: &Args) -> Result<String, String> {
    match cmd {
        "cm360" => {
            a.allow(&["game", "yaw", "dpi", "sens", "file"])?;
            let scale = a.scale("")?;
            let dpi = a.pos("dpi")?;
            let sens = match (a.get("sens"), a.get("file")) {
                (Some(_), None) => a.pos("sens")?,
                (None, Some(_)) if matches!(scale, Scale::WardogsEstimate) => {
                    scale.sens_for(a.pos("file")? * WARDOGS_DIVISOR)?
                }
                (None, Some(_)) => return Err("--file requires --game wardogs-estimate".into()),
                _ => return Err("pass exactly one of --sens or --file".into()),
            };
            describe(scale, sens, dpi)
        }
        "sens" => {
            a.allow(&["game", "yaw", "dpi", "cm"])?;
            let scale = a.scale("")?;
            let dpi = a.pos("dpi")?;
            describe(scale, scale.sens_for(deg_for_cm(a.pos("cm")?, dpi)?)?, dpi)
        }
        "calibrate" => {
            a.allow(&["sens", "dpi", "cm"])?;
            let dpi = a.pos("dpi")?;
            let cm = a.pos("cm")?;
            let sens = a.pos("sens")?;
            let yaw = positive(deg_for_cm(cm, dpi)? / sens, "calibrated yaw")?;
            Ok(format!("model=local_linear_calibration\nyaw={yaw}\nsensitivity={sens}\ndpi={dpi}\ncm_per_360={cm}\nassumption=constant gain, no offset; one measurement does not verify the whole slider\n"))
        }
        "convert" => {
            a.allow(&[
                "from-game",
                "from-yaw",
                "sens",
                "to-game",
                "to-yaw",
                "dpi",
                "to-dpi",
            ])?;
            let source = a.scale("from-")?;
            let target = a.scale("to-")?;
            let dpi = a.pos("dpi")?;
            let to_dpi = positive(a.optional("to-dpi", dpi)?, "target DPI")?;
            let cm = cm360(source.deg_per_count(a.pos("sens")?)?, dpi)?;
            let sens = target.sens_for(deg_for_cm(cm, to_dpi)?)?;
            let warning = if matches!(source, Scale::WardogsEstimate) {
                "source_warning=WARDOGS model is unverified; validate in game before adopting\n"
            } else {
                ""
            };
            Ok(format!(
                "source_model={}\nsource_cm_per_360={cm}\n{warning}{}",
                source.label(),
                describe(target, sens, to_dpi)?
            ))
        }
        "steps" => {
            a.allow(&["game", "yaw", "dpi", "from", "to", "step"])?;
            let scale = a.scale("")?;
            let dpi = a.pos("dpi")?;
            let from = scale.sensitivity(a.pos("from")?)?;
            let to = scale.sensitivity(a.pos("to")?)?;
            let step = positive(a.optional("step", 0.01)?, "step")?;
            if to < from {
                return Err("need --from <= --to".into());
            }
            if to > from && from + step <= from {
                return Err("step is too small at this magnitude".into());
            }
            let span = (to - from) / step;
            if !span.is_finite() || span >= MAX_ROWS as f64 {
                return Err(format!("table exceeds {MAX_ROWS} rows; increase --step"));
            }
            let mut out = format!("model={}\n", scale.label());
            if matches!(scale, Scale::WardogsEstimate) {
                out.push_str(
                    "warning=WARDOGS model is unverified; validate in game before adopting\n",
                );
            }
            out.push_str("sensitivity,cm_per_360,delta_cm\n");
            let mut prev = None;
            let mut prev_s = None;
            for i in 0..=(span.ceil() as usize).min(MAX_ROWS - 1) {
                let mut s = from + step * i as f64;
                if s > to {
                    if s - to <= 16.0 * f64::EPSILON * to.abs().max(from.abs()) {
                        s = to;
                    } else {
                        break;
                    }
                }
                if prev_s.is_some_and(|previous| s <= previous) {
                    return Err("step cannot produce distinct floating-point rows".into());
                }
                let cm = cm360(scale.deg_per_count(s)?, dpi)?;
                let delta = prev.map(|p: f64| (cm - p).to_string()).unwrap_or_default();
                out.push_str(&format!("{s},{cm},{delta}\n"));
                prev = Some(cm);
                prev_s = Some(s);
            }
            Ok(out)
        }
        "fov" => {
            a.allow(&["vfov", "hfov", "aspect"])?;
            let aspect = parse_aspect(a.required("aspect")?)?;
            let (v, h) = match (a.get("vfov"), a.get("hfov")) {
                (Some(_), None) => {
                    let v = a.num("vfov")?;
                    (v, h_from_v(v, aspect)?)
                }
                (None, Some(_)) => {
                    let h = a.num("hfov")?;
                    (v_from_h(h, aspect)?, h)
                }
                _ => return Err("pass exactly one of --vfov or --hfov".into()),
            };
            Ok(format!("projection=rectilinear\naspect_ratio={aspect}\nvertical_fov_degrees={v}\nhorizontal_fov_degrees={h}\n"))
        }
        "zoom" => match a.get("mode").unwrap_or("monitor") {
            "turn" => {
                a.allow(&["mode"])?;
                Ok("mode=equal_turn_distance\nads_per_hip_angular_multiplier=1\nwarning=angular ratio, not an engine UI coefficient\n".into())
            }
            "monitor" => {
                a.allow(&["mode", "hip-fov", "ads-fov", "axis", "md"])?;
                let axis = a.required("axis")?;
                if axis != "horizontal" && axis != "vertical" {
                    return Err("axis must be horizontal or vertical".into());
                }
                let md = a.optional("md", 0.0)?;
                let m = zoom_multiplier(a.num("hip-fov")?, a.num("ads-fov")?, md)?;
                Ok(format!("mode=monitor_distance\naxis={axis}\nmonitor_distance_percent_of_half_screen={md}\nads_per_hip_angular_multiplier={m}\nwarning=geometric target, not a verified engine UI coefficient\n"))
            }
            _ => Err("mode must be monitor or turn".into()),
        },
        _ => Err(format!("unknown command '{cmd}'")),
    }
}

pub const USAGE: &str = "aimcalc <command> [--flag value ...]
  cm360     (--yaw Y | --game wardogs-estimate) --dpi D (--sens S | --file F)
  sens      (--yaw Y | --game wardogs-estimate) --dpi D --cm C
  calibrate --sens S --dpi D --cm MEASURED_CM
  convert   (--from-yaw Y | --from-game wardogs-estimate) --sens S
            (--to-yaw Y | --to-game wardogs-estimate) --dpi D [--to-dpi D2]
  steps     (--yaw Y | --game wardogs-estimate) --dpi D --from S --to S2 [--step 0.01]
  fov       (--vfov V | --hfov H) --aspect W:H
  zoom      --mode turn
  zoom      [--mode monitor] --hip-fov A --ads-fov B --axis horizontal|vertical [--md 0]

Yaw is degrees/count at sensitivity 1, supplied from evidence or physical calibration.
WARDOGS is an explicitly unverified estimate; --file means its provisional ini encoding.
FOVs must be rectilinear degrees on the same axis. MD is 0..100% of the half-screen.
Zoom reports an angular ratio, never a game-specific menu coefficient. No files are read or written.
Use --help or --version. Invalid input exits 64; valid calculations exit 0.";

pub fn execute(raw: &[String]) -> Result<String, String> {
    match raw {
        [flag] if flag == "--help" || flag == "help" => Ok(format!("{USAGE}\n")),
        [flag] if flag == "--version" => Ok(format!("aimcalc {}\n", env!("CARGO_PKG_VERSION"))),
        [cmd, rest @ ..] => run(cmd, &Args::parse(rest)?),
        _ => Err("a command is required".into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn near(a: f64, b: f64) {
        assert!((a - b).abs() < 1e-10 * b.abs().max(1.0), "{a} != {b}");
    }

    #[test]
    fn known_physical_units() {
        near(cm360(1.0, 254.0).unwrap(), 3.6);
    }

    #[test]
    fn round_trips_physical_distance_over_scales_and_dpi() {
        for dpi in [400.0, 800.0, 1050.0, 1600.0, 3200.0] {
            for cm in [5.0, 25.4, 38.0, 80.0] {
                near(cm360(deg_for_cm(cm, dpi).unwrap(), dpi).unwrap(), cm);
            }
        }
    }

    #[test]
    fn dpi_and_scale_conversion_preserve_turn_distance() {
        let cm = cm360(0.022 * 2.0, 800.0).unwrap();
        let target = Scale::Linear(0.044)
            .sens_for(deg_for_cm(cm, 1600.0).unwrap())
            .unwrap();
        near(target, 0.5);
    }

    #[test]
    fn fov_round_trips_and_square_identity() {
        for aspect in [1.0, 4.0 / 3.0, 16.0 / 9.0, 21.0 / 9.0] {
            for v in [30.0, 70.0, 90.0, 120.0] {
                near(v_from_h(h_from_v(v, aspect).unwrap(), aspect).unwrap(), v);
            }
        }
        near(h_from_v(90.0, 1.0).unwrap(), 90.0);
        near(h_from_v(70.0, 16.0 / 9.0).unwrap(), 102.44785777798825);
    }

    #[test]
    fn monitor_distance_independent_anchors() {
        near(
            zoom_multiplier(70.0, 50.0, 0.0).unwrap(),
            0.6659563525226446,
        );
        near(zoom_multiplier(120.0, 60.0, 100.0).unwrap(), 0.5);
        near(zoom_multiplier(70.0, 70.0, 0.0).unwrap(), 1.0);
    }

    #[test]
    fn recorded_aimlabs_ratio_does_not_prove_exact_geometric_match() {
        let difference = (zoom_multiplier(70.0, 50.0, 0.0).unwrap() - 0.4042743 / 0.6064115).abs();
        assert!(difference > 0.0007 && difference < 0.00072);
    }

    #[test]
    fn provisional_wardogs_encoding_matches_inherited_examples_only() {
        let w = Scale::WardogsEstimate;
        assert!((w.deg_per_count(0.73).unwrap() / WARDOGS_DIVISOR - 0.398421).abs() < 1e-6);
        assert!((w.deg_per_count(0.56).unwrap() / WARDOGS_DIVISOR - 0.317895).abs() < 1e-6);
        assert!(w.sens_for(deg_for_cm(1.0, 1050.0).unwrap()).is_err());
    }
}
