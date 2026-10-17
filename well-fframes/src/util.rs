//! Small helpers: easing, keyframes, colours, deterministic noise and the camera.

pub use std::fmt::Write;

/// `w!(s, "...", args)` appends formatted SVG markup to a String.
#[macro_export]
macro_rules! w {
    ($s:expr, $($a:tt)*) => {{ let _ = write!($s, $($a)*); }};
}

pub fn clamp01(x: f32) -> f32 {
    x.clamp(0.0, 1.0)
}

pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

pub fn smooth(t: f32) -> f32 {
    let t = clamp01(t);
    t * t * (3.0 - 2.0 * t)
}

pub fn ease_io(t: f32) -> f32 {
    let t = clamp01(t);
    if t < 0.5 {
        4.0 * t * t * t
    } else {
        1.0 - (-2.0 * t + 2.0).powi(3) / 2.0
    }
}

pub fn ease_out(t: f32) -> f32 {
    1.0 - (1.0 - clamp01(t)).powi(3)
}

pub fn ease_in(t: f32) -> f32 {
    clamp01(t).powi(2)
}

/// Overshooting ease used for objects that pop into place.
pub fn ease_back(t: f32) -> f32 {
    let t = clamp01(t);
    let c1 = 1.70158;
    let c3 = c1 + 1.0;
    1.0 + c3 * (t - 1.0).powi(3) + c1 * (t - 1.0).powi(2)
}

/// Linear progress of `t` inside `[a, b]`, clamped to 0..1.
pub fn prog(t: f32, a: f32, b: f32) -> f32 {
    clamp01((t - a) / (b - a))
}

/// Opacity that fades in over `[a, b]` and out over `[c, d]`.
pub fn window(t: f32, a: f32, b: f32, c: f32, d: f32) -> f32 {
    smooth(prog(t, a, b)) * (1.0 - smooth(prog(t, c, d)))
}

#[derive(Clone, Copy)]
pub struct Cam {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

/// Camera keyframes `(time, x, y, zoom)`; zoom is interpolated in log space.
pub fn cam_at(t: f32, k: &[(f32, f32, f32, f32)]) -> Cam {
    if t <= k[0].0 {
        return Cam { x: k[0].1, y: k[0].2, z: k[0].3 };
    }
    for p in k.windows(2) {
        let a = p[0];
        let b = p[1];
        if t <= b.0 {
            let e = ease_io(prog(t, a.0, b.0));
            return Cam {
                x: lerp(a.1, b.1, e),
                y: lerp(a.2, b.2, e),
                z: lerp(a.3.ln(), b.3.ln(), e).exp(),
            };
        }
    }
    let l = k[k.len() - 1];
    Cam { x: l.1, y: l.2, z: l.3 }
}

pub fn hex(c: &str) -> (f32, f32, f32) {
    let v = u32::from_str_radix(c.trim_start_matches('#'), 16).unwrap_or(0);
    (
        ((v >> 16) & 255) as f32,
        ((v >> 8) & 255) as f32,
        (v & 255) as f32,
    )
}

pub fn rgb(r: f32, g: f32, b: f32) -> String {
    let c = |x: f32| x.round().clamp(0.0, 255.0) as u8;
    format!("#{:02x}{:02x}{:02x}", c(r), c(g), c(b))
}

/// Multiplies a colour: f < 1 darkens, f > 1 lightens towards white.
pub fn shade(c: &str, f: f32) -> String {
    let (r, g, b) = hex(c);
    if f <= 1.0 {
        rgb(r * f, g * f, b * f)
    } else {
        let k = f - 1.0;
        rgb(r + (255.0 - r) * k, g + (255.0 - g) * k, b + (255.0 - b) * k)
    }
}

pub fn mix(a: &str, b: &str, t: f32) -> String {
    let (r1, g1, b1) = hex(a);
    let (r2, g2, b2) = hex(b);
    rgb(lerp(r1, r2, t), lerp(g1, g2, t), lerp(b1, b2, t))
}

/// Deterministic xorshift generator for textures and scattered details.
pub struct Rng(u64);

impl Rng {
    pub fn new(seed: u64) -> Self {
        Rng(seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1)
    }
    pub fn f(&mut self) -> f32 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        ((self.0 >> 40) as f32) / ((1u64 << 24) as f32)
    }
    pub fn r(&mut self, a: f32, b: f32) -> f32 {
        lerp(a, b, self.f())
    }
}

/// Wavy horizontal boundary from x0 to x1 around `y`, as path commands (without the leading M).
pub fn wavy(y: f32, x0: f32, x1: f32, amp: f32, seed: f32) -> String {
    let mut s = String::new();
    let mut x = x0;
    while x <= x1 {
        let yy = y
            + amp * ((x * 0.004 + seed).sin() * 0.6 + (x * 0.011 + seed * 2.3).sin() * 0.4);
        w!(s, "{:.0},{:.1} ", x, yy);
        x += 80.0;
    }
    s
}
