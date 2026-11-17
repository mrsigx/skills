//! aimcount: passive raw-input mouse counter for measuring counts per in-game turn.
//!
//! Registers for raw mouse and keyboard input with RIDEV_INPUTSINK, so it keeps
//! receiving input while a game has focus. It only reads: it never injects or
//! moves input, never touches a game process, and ignores every key except
//! F8 (quit), F9 (start/reset) and F10 (stop and report). Windows only, no crates.

use std::env;
use std::ffi::c_void;
use std::fs::OpenOptions;
use std::io::Write;
use std::process::ExitCode;
use std::time::{SystemTime, UNIX_EPOCH};

type Hwnd = isize;

#[repr(C)]
struct RawInputDevice {
    us_usage_page: u16,
    us_usage: u16,
    dw_flags: u32,
    hwnd_target: Hwnd,
}

#[repr(C)]
struct Point {
    x: i32,
    y: i32,
}

#[repr(C)]
struct Msg {
    hwnd: Hwnd,
    message: u32,
    w_param: usize,
    l_param: isize,
    time: u32,
    pt: Point,
    l_private: u32,
}

#[link(name = "user32")]
extern "system" {
    fn CreateWindowExW(
        ex_style: u32,
        class: *const u16,
        name: *const u16,
        style: u32,
        x: i32,
        y: i32,
        w: i32,
        h: i32,
        parent: Hwnd,
        menu: isize,
        instance: isize,
        param: *mut c_void,
    ) -> Hwnd;
    fn RegisterRawInputDevices(devices: *const RawInputDevice, count: u32, size: u32) -> i32;
    fn GetRawInputData(
        raw: isize,
        command: u32,
        data: *mut c_void,
        size: *mut u32,
        header_size: u32,
    ) -> u32;
    fn GetMessageW(msg: *mut Msg, hwnd: Hwnd, min: u32, max: u32) -> i32;
    fn DispatchMessageW(msg: *const Msg) -> isize;
    fn MessageBeep(kind: u32) -> i32;
}

const HWND_MESSAGE: Hwnd = -3;
const RIDEV_INPUTSINK: u32 = 0x100;
const RID_INPUT: u32 = 0x1000_0003;
const WM_INPUT: u32 = 0x00FF;
const RIM_TYPEMOUSE: u32 = 0;
const RIM_TYPEKEYBOARD: u32 = 1;
const MOUSE_MOVE_ABSOLUTE: u16 = 1;
const RI_KEY_BREAK: u16 = 1;
const VK_F8: u16 = 0x77;
const VK_F9: u16 = 0x78;
const VK_F10: u16 = 0x79;
const HEADER_SIZE: u32 = 24; // RAWINPUTHEADER on 64-bit Windows
const CM_PER_IN: f64 = 2.54;

fn wide(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}

struct Options {
    dpi: Option<f64>,
    label: String,
    log: Option<String>,
}

fn parse_args() -> Result<Options, String> {
    let mut o = Options {
        dpi: None,
        label: String::new(),
        log: None,
    };
    let mut it = env::args().skip(1);
    while let Some(k) = it.next() {
        let mut val = || it.next().ok_or_else(|| format!("{k} needs a value"));
        match k.as_str() {
            "--dpi" => {
                let v: f64 = val()?
                    .parse()
                    .map_err(|_| "--dpi must be a number".to_string())?;
                if !(v.is_finite() && v > 0.0) {
                    return Err("--dpi must be positive".into());
                }
                o.dpi = Some(v);
            }
            "--label" => o.label = val()?,
            "--log" => o.log = Some(val()?),
            "--help" | "-h" => return Err(String::new()),
            _ => return Err(format!("unknown option '{k}'")),
        }
    }
    Ok(o)
}

const USAGE: &str = "aimcount [--dpi D] [--label TEXT] [--log FILE.jsonl]
  F9  start (or restart) counting      F10 stop and report      F8 quit
Turn in game between F9 and F10; corrections back and forth net out.
Reports raw counts on X and Y; with --dpi also cm and inches of mouse travel.
--log appends one JSON line per measurement. Read-only: never sends input.";

fn report(o: &Options, n: u32, x: i64, y: i64) {
    let mut line = format!(
        "#{n} {}counts_x={x} counts_y={y}",
        if o.label.is_empty() {
            String::new()
        } else {
            format!("[{}] ", o.label)
        }
    );
    let mut cm = None;
    if let Some(dpi) = o.dpi {
        let c = x.unsigned_abs() as f64 / dpi * CM_PER_IN;
        line.push_str(&format!(
            "  |x| = {c:.2} cm = {:.2} in at {dpi} DPI",
            c / CM_PER_IN
        ));
        cm = Some(c);
    }
    println!("{line}");
    if let Some(path) = &o.log {
        let ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        let label = o.label.replace('\\', "\\\\").replace('"', "\\\"");
        let json = format!(
            "{{\"unix_time\":{ts},\"label\":\"{label}\",\"counts_x\":{x},\"counts_y\":{y},\"dpi\":{},\"cm_x\":{}}}",
            o.dpi.map_or("null".into(), |d| d.to_string()),
            cm.map_or("null".into(), |c| format!("{c:.4}"))
        );
        match OpenOptions::new().create(true).append(true).open(path) {
            Ok(mut f) => {
                if let Err(e) = writeln!(f, "{json}") {
                    eprintln!("log write failed: {e}");
                }
            }
            Err(e) => eprintln!("log open failed: {e}"),
        }
    }
}

fn main() -> ExitCode {
    let o = match parse_args() {
        Ok(o) => o,
        Err(e) => {
            if !e.is_empty() {
                eprintln!("error: {e}\n");
            }
            eprintln!("{USAGE}");
            return ExitCode::from(64);
        }
    };

    let class = wide("STATIC");
    let name = wide("aimcount");
    // SAFETY: plain Win32 calls with valid, NUL-terminated UTF-16 strings and owned buffers.
    let hwnd = unsafe {
        CreateWindowExW(
            0,
            class.as_ptr(),
            name.as_ptr(),
            0,
            0,
            0,
            0,
            0,
            HWND_MESSAGE,
            0,
            0,
            std::ptr::null_mut(),
        )
    };
    if hwnd == 0 {
        eprintln!("could not create the message window");
        return ExitCode::from(1);
    }
    let devices = [
        RawInputDevice {
            us_usage_page: 1,
            us_usage: 2,
            dw_flags: RIDEV_INPUTSINK,
            hwnd_target: hwnd,
        },
        RawInputDevice {
            us_usage_page: 1,
            us_usage: 6,
            dw_flags: RIDEV_INPUTSINK,
            hwnd_target: hwnd,
        },
    ];
    let ok = unsafe {
        RegisterRawInputDevices(
            devices.as_ptr(),
            devices.len() as u32,
            std::mem::size_of::<RawInputDevice>() as u32,
        )
    };
    if ok == 0 {
        eprintln!("could not register for raw input");
        return ExitCode::from(1);
    }

    println!(
        "aimcount ready. F9 start, F10 stop, F8 quit.{}",
        o.dpi.map_or(String::new(), |d| format!(" DPI {d}."))
    );
    let (mut counting, mut x, mut y, mut n) = (false, 0i64, 0i64, 0u32);
    let mut buf = [0u64; 16]; // 128 bytes, 8-aligned; RAWINPUT for mouse/keyboard is 48/40 bytes
    let mut msg = Msg {
        hwnd: 0,
        message: 0,
        w_param: 0,
        l_param: 0,
        time: 0,
        pt: Point { x: 0, y: 0 },
        l_private: 0,
    };

    while unsafe { GetMessageW(&mut msg, 0, 0, 0) } > 0 {
        if msg.message == WM_INPUT {
            let mut size = std::mem::size_of_val(&buf) as u32;
            let got = unsafe {
                GetRawInputData(
                    msg.l_param,
                    RID_INPUT,
                    buf.as_mut_ptr().cast(),
                    &mut size,
                    HEADER_SIZE,
                )
            };
            if got != u32::MAX && got >= HEADER_SIZE {
                let b: &[u8] =
                    unsafe { std::slice::from_raw_parts(buf.as_ptr().cast::<u8>(), got as usize) };
                let u16_at = |o: usize| u16::from_le_bytes([b[o], b[o + 1]]);
                let i32_at = |o: usize| i32::from_le_bytes([b[o], b[o + 1], b[o + 2], b[o + 3]]);
                let kind = u32::from_le_bytes([b[0], b[1], b[2], b[3]]);
                if kind == RIM_TYPEMOUSE && b.len() >= 44 {
                    if counting && u16_at(24) & MOUSE_MOVE_ABSOLUTE == 0 {
                        x += i64::from(i32_at(36));
                        y += i64::from(i32_at(40));
                    }
                } else if kind == RIM_TYPEKEYBOARD
                    && b.len() >= 32
                    && u16_at(26) & RI_KEY_BREAK == 0
                {
                    match u16_at(30) {
                        VK_F9 => {
                            (counting, x, y) = (true, 0, 0);
                            println!("counting...");
                            unsafe { MessageBeep(0) };
                        }
                        VK_F10 if counting => {
                            counting = false;
                            n += 1;
                            report(&o, n, x, y);
                            unsafe { MessageBeep(0x40) };
                        }
                        VK_F8 => break,
                        _ => {}
                    }
                }
            }
        }
        unsafe { DispatchMessageW(&msg) };
    }
    println!("aimcount stopped after {n} measurement(s).");
    ExitCode::SUCCESS
}
