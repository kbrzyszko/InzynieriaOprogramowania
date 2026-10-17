//! Act 1 (0–21 s): the hand-dug well. Act 2 (21–41 s): the drilled well with casing and pumps.

use crate::props::{self, arrow, ripples, stream};
use crate::util::*;
use crate::w;

// Dug hole and stone lining (x range).
const HX0: f32 = 460.0;
const HX1: f32 = 620.0;
const LX0: f32 = 482.0;
const LX1: f32 = 598.0;
const PARAPET: f32 = -90.0;
const HOLE: f32 = 575.0;

pub fn hole_depth(t: f32) -> f32 {
    30.0 + (HOLE - 30.0) * smooth(prog(t, 0.0, 2.0))
}

pub fn water_level(t: f32) -> f32 {
    let a = lerp(HOLE, 548.0, ease_out(prog(t, 2.1, 4.2)));
    lerp(a, 470.0, ease_io(prog(t, 7.8, 11.0)))
}

fn murk(t: f32) -> f32 {
    smooth(prog(t, 10.5, 11.4)) * (1.0 - smooth(prog(t, 13.3, 14.1)))
}

fn half_ellipse(s: &mut String, x0: f32, x1: f32, y: f32, ry: f32, fill: &str, op: f32) {
    let rx = (x1 - x0) / 2.0;
    w!(
        s,
        r#"<path d="M{x0:.1},{y:.1} A{rx:.1},{ry:.1} 0 0 1 {x1:.1},{y:.1} Z" fill="{fill}" opacity="{op:.2}"/>"#
    );
}

/// Things standing on the lawn in the first act (behind the hole).
pub fn lawn_props_a(s: &mut String, t: f32) {
    let fade = 1.0 - smooth(prog(t, 21.0, 21.6));
    if fade <= 0.0 {
        return;
    }
    w!(s, r#"<g opacity="{fade:.3}">"#);
    props::rock(s, 236.0, -150.0, 0.95);
    // Dirt pile from digging.
    let h = 14.0 + 78.0 * smooth(prog(t, 0.1, 2.1));
    let pile = 1.0 - smooth(prog(t, 12.8, 13.6));
    if pile > 0.0 {
        props::shadow(s, 790.0, -36.0, 120.0, 16.0, 0.7 * pile);
        w!(
            s,
            r##"<g opacity="{pile:.2}"><path d="M680,-34 Q700,-38 730,{:.1} Q785,{:.1} 840,{:.1} Q880,-40 900,-34 Z" fill="url(#pSoil)"/><path d="M680,-34 Q700,-38 730,{:.1} Q785,{:.1} 840,{:.1} Q880,-40 900,-34 Z" fill="#000" opacity="0.18"/></g>"##,
            -34.0 - h * 0.7,
            -34.0 - h * 1.25,
            -34.0 - h * 0.6,
            -34.0 - h * 0.7,
            -34.0 - h * 1.25,
            -34.0 - h * 0.6
        );
    }
    s.push_str("</g>");
}

pub fn act_a(s: &mut String, t: f32) {
    let fade = 1.0 - smooth(prog(t, 21.0, 21.6));
    if fade <= 0.0 {
        return;
    }
    w!(s, r#"<g opacity="{fade:.3}">"#);
    let d = hole_depth(t);
    let wl = water_level(t);
    let m = murk(t);
    let lin = smooth(prog(t, 13.1, 14.2));
    let lined = t >= 13.1;

    // Hole opening on the lawn and the shaft cut in half by the section plane.
    half_ellipse(s, HX0, HX1, 0.0, 26.0, "url(#holeTop)", 1.0);
    w!(s, r#"<rect x="{HX0}" y="0" width="{}" height="{d:.1}" fill="url(#shaft)"/>"#, HX1 - HX0);
    half_ellipse(s, HX0, HX1, d, 26.0, "#2a170a", 0.55);
    w!(
        s,
        r##"<path d="M{HX0},0 V{d:.1} M{HX1},0 V{d:.1}" stroke="#2a170a" stroke-width="4"/>"##
    );

    // Stone lining back wall (built from the bottom up).
    if lined {
        let top = lerp(d, PARAPET, lin);
        w!(
            s,
            r#"<clipPath id="lin"><rect x="440" y="{top:.1}" width="200" height="{:.1}"/></clipPath><g clip-path="url(#lin)">"#,
            d - top + 2.0
        );
        w!(
            s,
            r##"<rect x="{LX0}" y="{PARAPET}" width="{}" height="{:.1}" fill="#8f8f8c"/>"##,
            LX1 - LX0,
            d - PARAPET
        );
        w!(
            s,
            r#"<rect x="{LX0}" y="{PARAPET}" width="{}" height="{:.1}" fill="url(#pStone)"/><rect x="{LX0}" y="{PARAPET}" width="{}" height="{:.1}" fill="url(#shaft)"/>"#,
            LX1 - LX0,
            d - PARAPET,
            LX1 - LX0,
            d - PARAPET
        );
        for k in 0..5 {
            let y = -60.0 + k as f32 * 140.0;
            w!(
                s,
                r##"<path d="M{LX0},{y:.1} Q540,{:.1} {LX1},{y:.1}" stroke="#7e3e18" stroke-width="8" fill="none"/><path d="M{LX0},{:.1} Q540,{:.1} {LX1},{:.1}" stroke="#c4733a" stroke-width="2" fill="none"/>"##,
                y - 30.0,
                y - 3.0,
                y - 33.0,
                y - 3.0
            );
        }
        s.push_str("</g>");
    }

    // Wooden frame with a roof and pail (4 s), pail lowered on its rope (16 s).
    let g = if t < 3.5 { 0.0 } else { ease_back(prog(t, 3.5, 4.3)) };
    let frame_op = 1.0 - smooth(prog(t, 17.3, 17.8));
    if g > 0.0 && frame_op > 0.0 {
        let sway = (t * 2.2).sin() * 3.0;
        let low = ease_io(prog(t, 15.9, 17.3));
        let pail_top = lerp(-170.0 + sway, wl - 26.0, low);
        let crank = t * 40.0 + low * 900.0;
        w!(s, r#"<g opacity="{frame_op:.2}">"#);
        props::well_frame(s, g, pail_top, crank);
        s.push_str("</g>");
    }

    // Hand-pump pipe reaching down into the water (18 s).
    let pump_in = prog(t, 17.6, 18.1);
    if pump_in > 0.0 {
        let grow = ease_io(prog(t, 17.9, 18.5));
        let bottom = lerp(PARAPET, wl + 60.0, grow);
        w!(
            s,
            r#"<rect x="534" y="{PARAPET}" width="12" height="{:.1}" fill="url(#cSteel)"/>"#,
            bottom - PARAPET
        );
    }

    // Gnome: stands on the lawn, hops into the hole at 9.5 s and sinks.
    let gop = 1.0 - smooth(prog(t, 12.2, 13.0));
    if gop > 0.0 && t >= 9.9 {
        let (x, y, rot) = gnome_fall(t, wl);
        props::gnome(s, x, y, 0.75, rot, gop);
    }

    // Water body inside the shaft.
    if t >= 2.1 {
        let (x0, x1) = if lined { (LX0, LX1) } else { (HX0, HX1) };
        let op = smooth(prog(t, 2.1, 2.5));
        if m < 1.0 {
            w!(
                s,
                r#"<rect x="{x0}" y="{wl:.1}" width="{}" height="{:.1}" fill="url(#water)" opacity="{:.2}"/>"#,
                x1 - x0,
                d - wl,
                op * (1.0 - m) * 0.85
            );
        }
        if m > 0.0 {
            w!(
                s,
                r#"<rect x="{x0}" y="{wl:.1}" width="{}" height="{:.1}" fill="url(#murk)" opacity="{:.2}"/>"#,
                x1 - x0,
                d - wl,
                op * m
            );
        }
        let surf = mix("#a8e8fb", "#d8c07a", m);
        half_ellipse(s, x0, x1, wl, 24.0, &surf, 0.9 * op);
        ripples(s, 540.0, wl - 9.0, (x1 - x0) * 0.42, t, op);
    }

    // Splash when the gnome hits the water.
    let sp = prog(t, 10.45, 11.1);
    if sp > 0.0 && sp < 1.0 {
        let c = mix("#bfeeff", "#d8c07a", m);
        for k in 0..10 {
            let a = (k as f32 / 9.0 - 0.5) * 2.4;
            let v = 220.0 + (k % 3) as f32 * 60.0;
            let x = 540.0 + a.sin() * v * sp * 0.6;
            let y = wl - 20.0 - a.cos() * v * sp + 520.0 * sp * sp;
            w!(
                s,
                r#"<circle cx="{x:.1}" cy="{y:.1}" r="{:.1}" fill="{c}" opacity="{:.2}"/>"#,
                7.0 - 3.0 * sp,
                1.0 - sp
            );
        }
    }

    // Cut faces of the stone lining with rusty hoops, parapet rim.
    if lined {
        let top = lerp(d, PARAPET, lin);
        w!(
            s,
            r#"<clipPath id="lin2"><rect x="440" y="{top:.1}" width="200" height="{:.1}"/></clipPath><g clip-path="url(#lin2)">"#,
            d - top + 2.0
        );
        for x in [HX0, LX1] {
            w!(
                s,
                r#"<rect x="{x}" y="{PARAPET}" width="22" height="{:.1}" fill="url(#pStone)"/>"#,
                d - PARAPET
            );
        }
        for k in 0..5 {
            let y = -60.0 + k as f32 * 140.0;
            for x in [HX0 - 3.0, LX1 - 3.0] {
                w!(
                    s,
                    r##"<rect x="{x}" y="{:.1}" width="28" height="10" rx="2" fill="url(#cRust)"/>"##,
                    y - 5.0
                );
            }
        }
        s.push_str("</g>");
        let rim = smooth(prog(t, 13.9, 14.3));
        w!(
            s,
            r##"<path d="M{HX0},{PARAPET} A80,26 0 0 1 {HX1},{PARAPET} L{LX1},{PARAPET} A58,19 0 0 0 {LX0},{PARAPET} Z" fill="#b5b5b1" opacity="{rim:.2}"/>"##
        );
    }

    // Concrete cover and red hand pump with water arcing into a wooden bucket (18–21 s).
    if pump_in > 0.0 {
        let drop = -320.0 * (1.0 - ease_back(pump_in));
        w!(s, r#"<g transform="translate(0 {drop:.1})">"#);
        half_ellipse(s, 446.0, 634.0, -104.0, 30.0, "#d3d0c8", 1.0);
        s.push_str(r#"<rect x="446" y="-104" width="188" height="16" fill="url(#cConcrete)"/>"#);
        let pumping = window(t, 18.3, 18.5, 20.9, 21.2);
        let lever = -6.0 + pumping * 16.0 * (t * std::f32::consts::TAU / 0.9).sin();
        props::red_pump(s, 540.0, -104.0, lever);
        s.push_str("</g>");
        let b = smooth(prog(t, 17.8, 18.2));
        if b > 0.0 {
            let fill = smooth(prog(t, 18.7, 20.8));
            w!(s, r#"<g opacity="{b:.2}">"#);
            props::bucket(s, 704.0, -30.0, 78.0, 70.0, fill, "#5cc8f0");
            s.push_str("</g>");
        }
        let flow = window(t, 18.5, 18.75, 20.9, 21.2);
        if flow > 0.0 {
            stream(s, "M614,-200 Q664,-206 700,-104", 9.0, t, flow);
        }
    }
    s.push_str("</g>");

    // The gnome stands on the lawn before its fall (drawn here so it sits over the lawn props).
    if t < 9.9 {
        let (x, y, rot) = gnome_fall(t, wl);
        props::gnome(s, x, y, 0.75, rot, 1.0);
    }
}

fn gnome_fall(t: f32, wl: f32) -> (f32, f32, f32) {
    let (sx, sy) = (372.0, -84.0);
    if t < 9.4 {
        return (sx, sy, (t * 1.3).sin() * 1.5);
    }
    if t < 9.9 {
        let p = prog(t, 9.4, 9.9);
        let x = lerp(sx, 540.0, p);
        let y = lerp(sy, -40.0, p) - 110.0 * (std::f32::consts::PI * p).sin();
        return (x, y, 35.0 * p);
    }
    if t < 10.45 {
        let p = ease_in(prog(t, 9.9, 10.45));
        return (540.0, lerp(-40.0, wl + 30.0, p), 35.0 + 140.0 * p);
    }
    let p = smooth(prog(t, 10.45, 11.6));
    (540.0 + 8.0 * (t * 3.0).sin() * (1.0 - p), wl + 30.0 + 30.0 * p, 175.0 - 70.0 * p)
}

/// Shovel digging the hole in the first two seconds, with clods flying to the pile.
pub fn shovel(s: &mut String, t: f32) {
    if t > 2.5 {
        return;
    }
    let period = 0.5;
    let ph = (t / period).fract();
    let (mut x, mut y, mut rot);
    if ph < 0.45 {
        let p = smooth(ph / 0.45);
        x = 540.0;
        y = lerp(-70.0, 70.0, p);
        rot = -8.0;
    } else if ph < 0.75 {
        let p = smooth((ph - 0.45) / 0.3);
        x = lerp(540.0, 600.0, p);
        y = lerp(70.0, -120.0, p);
        rot = lerp(-8.0, 38.0, p);
    } else {
        let p = smooth((ph - 0.75) / 0.25);
        x = lerp(600.0, 540.0, p);
        y = lerp(-120.0, -70.0, p);
        rot = lerp(38.0, -8.0, p);
    }
    let out = smooth(prog(t, 2.0, 2.5));
    x += 360.0 * out;
    y -= 600.0 * out;
    rot += 30.0 * out;
    props::shovel(s, x, y, rot);
    if ph > 0.6 && t < 2.0 {
        let p = (ph - 0.6) / 0.4;
        for k in 0..4 {
            let kx = k as f32 * 9.0;
            let cx = lerp(612.0, 770.0 + kx, p);
            let cy = lerp(-150.0, -60.0, p) - 120.0 * (std::f32::consts::PI * p).sin() + kx;
            w!(
                s,
                r##"<circle cx="{cx:.1}" cy="{cy:.1}" r="{:.1}" fill="#6e3f1c"/>"##,
                8.0 - k as f32
            );
        }
    }
}

// ---------------------------------------------------------------------------------------------
// Act 2: drilled well.

const CX0: f32 = 480.0;
const CX1: f32 = 600.0;
const CW: f32 = 10.0;
const IX0: f32 = 527.0;
const IX1: f32 = 553.0;
pub const CASING_WATER: f32 = 1650.0;
const STALL: f32 = 1050.0;
const PIPE_BOTTOM: f32 = 1840.0;
const PIPE_TOP: f32 = -250.0;

pub fn drill_depth(t: f32) -> f32 {
    2060.0 * ease_io(prog(t, 22.4, 26.0))
}

fn casing_bottom(t: f32) -> f32 {
    (drill_depth(t) - 50.0).max(0.0)
}

/// Height reached by the water pushed up by the submersible pump.
pub fn push_front(t: f32) -> f32 {
    lerp(1990.0, PIPE_TOP - 4.0, ease_io(prog(t, 36.6, 39.6)))
}

fn inner_level(t: f32) -> f32 {
    if t < 30.0 {
        return CASING_WATER;
    }
    let suck = lerp(CASING_WATER, STALL, ease_out(prog(t, 30.0, 32.4)));
    let wobble = if t > 32.4 { (t * 7.0).sin() * 3.0 * (1.0 - prog(t, 32.4, 34.0)) } else { 0.0 };
    (suck + wobble).min(push_front(t).max(PIPE_TOP))
}

pub fn act_b(s: &mut String, t: f32) {
    if t < 21.5 {
        return;
    }
    let d = drill_depth(t);
    let cb = casing_bottom(t);

    // Borehole and casing (cut open to show its inside).
    if d > 0.0 {
        w!(
            s,
            r##"<rect x="470" y="0" width="140" height="{d:.1}" fill="#140c06" opacity="0.6"/>"##
        );
    }
    if cb > 0.0 {
        w!(
            s,
            r##"<rect x="{}" y="-30" width="{}" height="{:.1}" fill="#0c1430"/>"##,
            CX0 + CW,
            CX1 - CX0 - 2.0 * CW,
            cb + 30.0
        );
        w!(
            s,
            r#"<rect x="{}" y="-30" width="{}" height="{:.1}" fill="url(#shaft)" opacity="0.5"/>"#,
            CX0 + CW,
            CX1 - CX0 - 2.0 * CW,
            cb + 30.0
        );
    }

    // Water entering the casing through the screen at the bottom.
    let cw_level = lerp(cb, CASING_WATER, ease_io(prog(t, 26.0, 27.7)));
    if t > 25.9 && cb > cw_level {
        w!(
            s,
            r#"<rect x="{}" y="{cw_level:.1}" width="{}" height="{:.1}" fill="url(#water)"/>"#,
            CX0 + CW,
            CX1 - CX0 - 2.0 * CW,
            cb - cw_level
        );
        w!(
            s,
            r##"<rect x="{}" y="{:.1}" width="{}" height="4" fill="#bdf0ff" opacity="0.8"/>"##,
            CX0 + CW,
            cw_level - 2.0,
            CX1 - CX0 - 2.0 * CW
        );
        let inflow = window(t, 25.8, 26.2, 27.4, 27.9);
        if inflow > 0.0 {
            for k in 0..14 {
                let p = (t * 1.6 + k as f32 * 0.37).fract();
                let y = 1780.0 + (k as f32 * 53.0) % 200.0;
                let (xa, xb) = if k % 2 == 0 { (420.0, 494.0) } else { (660.0, 586.0) };
                w!(
                    s,
                    r##"<circle cx="{:.1}" cy="{:.1}" r="5" fill="#5cc8f0" stroke="#ffffff" stroke-width="1.5" opacity="{:.2}"/>"##,
                    lerp(xa, xb, p),
                    y - 10.0 * p,
                    inflow * (1.0 - p * 0.5)
                );
            }
        }
    }

    // Inner (drop) pipe: translucent so the water inside is visible.
    let pipe_grow = ease_io(prog(t, 27.6, 28.4));
    let pipe_bottom = lerp(-60.0, PIPE_BOTTOM, pipe_grow);
    if t >= 27.6 {
        let top = if t < 35.9 { -60.0 } else { PIPE_TOP };
        w!(
            s,
            r##"<rect x="{IX0}" y="{top}" width="{}" height="{:.1}" fill="#ffffff" opacity="0.24"/>"##,
            IX1 - IX0,
            pipe_bottom - top
        );
        let il = inner_level(t);
        if pipe_bottom > il {
            w!(
                s,
                r#"<rect x="{}" y="{il:.1}" width="{}" height="{:.1}" fill="url(#water)"/>"#,
                IX0 + 4.0,
                IX1 - IX0 - 8.0,
                pipe_bottom - il
            );
            w!(
                s,
                r##"<rect x="{}" y="{:.1}" width="{}" height="3" fill="#d8f6ff"/>"##,
                IX0 + 4.0,
                il - 1.0,
                IX1 - IX0 - 8.0
            );
        }
        w!(
            s,
            r##"<rect x="{IX0}" y="{top}" width="5" height="{:.1}" fill="#f4f7f9"/><rect x="{}" y="{top}" width="5" height="{:.1}" fill="#c9d1d8"/>"##,
            pipe_bottom - top,
            IX1 - 5.0,
            pipe_bottom - top
        );
    }

    // Casing walls with the slotted screen at the aquifer.
    if cb > 0.0 {
        for x in [CX0, CX1 - CW] {
            w!(
                s,
                r#"<rect x="{x}" y="-30" width="{CW}" height="{:.1}" fill="url(#cNavy)"/>"#,
                cb + 30.0
            );
        }
        let mut y = 1770.0;
        while y < cb.min(2000.0) {
            for x in [CX0 + 1.0, CX1 - CW + 1.0] {
                w!(s, r##"<rect x="{x}" y="{y}" width="8" height="4" fill="#8fa6d8"/>"##);
            }
            y += 16.0;
        }
    }

    // Drilling: tripod drops in, drill string turns, cuttings rise; then the rig leaves.
    let rig_in = ease_back(prog(t, 21.6, 22.2));
    let rig_out = ease_in(prog(t, 27.1, 27.8));
    if rig_in > 0.0 && rig_out < 1.0 {
        let spin = -t * 300.0;
        let lift = 700.0 * (1.0 - rig_in) + 1800.0 * rig_out;
        if t < 26.9 {
            let retract = ease_in(prog(t, 26.2, 26.9));
            let bit = lerp(d.max(-20.0), -560.0, retract);
            if bit > -540.0 {
                props::drill(s, -600.0 - lift * 0.0, bit, spin);
            }
            if d > 30.0 && t < 26.1 {
                for k in 0..12 {
                    let p = (t * 2.0 + k as f32 / 12.0).fract();
                    let x = 540.0 + if k % 2 == 0 { -42.0 } else { 42.0 } + (k as f32 * 1.7).sin() * 4.0;
                    let y = d - 30.0 - p * 160.0;
                    let c = if d > crate::ground::L_CLAY { "#b9b9b6" } else { "#a8693a" };
                    w!(
                        s,
                        r#"<circle cx="{x:.1}" cy="{y:.1}" r="{:.1}" fill="{c}" opacity="{:.2}"/>"#,
                        3.0 + (k % 3) as f32,
                        1.0 - p
                    );
                }
            }
        }
        props::tripod(s, lift, spin);
    }

    // Well cap on top of the casing.
    if t >= 27.4 {
        s.push_str(r##"<rect x="470" y="-50" width="140" height="22" rx="4" fill="#3c4048"/><rect x="470" y="-50" width="140" height="6" rx="3" fill="#5f656e"/>"##);
    }

    // Hand pump on the casing (28–36 s), arrows pressing down on the water (30–33 s).
    let hp_in = prog(t, 27.6, 28.1);
    let hp_out = smooth(prog(t, 35.6, 35.9));
    if hp_in > 0.0 && hp_out < 1.0 {
        let drop = -320.0 * (1.0 - ease_back(hp_in));
        let pumping = window(t, 28.9, 29.2, 33.4, 33.8);
        let lever = -6.0 + pumping * 16.0 * (t * std::f32::consts::TAU / 0.9).sin();
        w!(s, r#"<g transform="translate(0 {drop:.1})" opacity="{:.2}">"#, 1.0 - hp_out);
        props::red_pump(s, 540.0, -50.0, lever);
        s.push_str("</g>");
    }
    let down = window(t, 30.1, 30.5, 33.4, 34.0);
    if down > 0.0 {
        let bob = (t * 3.0).fract() * 10.0;
        for x in [508.5, 571.5] {
            arrow(s, x, CASING_WATER - 30.0 + bob, false, 30.0, down);
        }
    }
    // Height of the lifted column: white dimension line, no label.
    let dim = window(t, 32.4, 33.2, 35.4, 35.9);
    if dim > 0.0 {
        let top = lerp(CASING_WATER, STALL, ease_io(prog(t, 32.4, 33.4)));
        w!(
            s,
            r##"<g opacity="{dim:.2}" stroke="#ffffff" stroke-width="4" fill="#ffffff"><path d="M648,{CASING_WATER} V{top:.1}"/><path d="M634,{CASING_WATER} H662 M634,{top:.1} H662"/><path d="M648,{:.1} L639,{:.1} L657,{:.1} Z M648,{:.1} L639,{:.1} L657,{:.1} Z" stroke="none"/></g>"##,
            top + 2.0,
            top + 18.0,
            top + 18.0,
            CASING_WATER - 2.0,
            CASING_WATER - 18.0,
            CASING_WATER - 18.0
        );
    }

    // Submersible pump at the bottom of the drop pipe (36 s) pushing water up.
    let sp = smooth(prog(t, 35.8, 36.3));
    if sp > 0.0 {
        let y0 = PIPE_BOTTOM + (1.0 - sp) * -120.0;
        w!(
            s,
            r##"<g opacity="{sp:.2}" transform="translate(0 {:.1})"><rect x="521" y="{PIPE_BOTTOM}" width="38" height="150" rx="8" fill="url(#cSteel)"/><rect x="521" y="{}" width="38" height="22" fill="#2f353c"/><path d="M523,{} H557 M523,{} H557 M523,{} H557" stroke="#8d969f" stroke-width="2"/><rect x="519" y="{}" width="42" height="10" rx="3" fill="#6f7882"/><path d="M559,{} Q572,{} 570,{} V-40" stroke="#1d1f22" stroke-width="3" fill="none"/></g>"##,
            y0 - PIPE_BOTTOM,
            PIPE_BOTTOM + 52.0,
            PIPE_BOTTOM + 57.0,
            PIPE_BOTTOM + 63.0,
            PIPE_BOTTOM + 69.0,
            PIPE_BOTTOM + 96.0,
            PIPE_BOTTOM + 30.0,
            PIPE_BOTTOM + 20.0,
            PIPE_BOTTOM - 10.0
        );
        let up = window(t, 36.5, 36.8, 39.7, 40.1);
        if up > 0.0 {
            let f = push_front(t);
            for k in 0..6 {
                let y = f + 30.0 + k as f32 * 70.0 - ((t * 2.5).fract() * 70.0);
                if y < PIPE_BOTTOM {
                    arrow(s, 540.0, y, true, 14.0, up * (1.0 - k as f32 / 7.0));
                }
            }
            let mut r = Rng::new(99);
            for k in 0..14 {
                let p = (t * 0.9 + r.f()).fract();
                let y = f + 20.0 + p * 420.0;
                let x = 534.0 + r.f() * 12.0;
                if y < PIPE_BOTTOM && k % 1 == 0 {
                    w!(
                        s,
                        r##"<circle cx="{x:.1}" cy="{y:.1}" r="{:.1}" fill="#ffffff" opacity="{:.2}"/>"##,
                        1.5 + r.f() * 2.0,
                        up * 0.8
                    );
                }
            }
            for k in 0..10 {
                let p = (t * 0.7 + k as f32 * 0.137).fract();
                let x = if k % 2 == 0 { 506.0 } else { 574.0 } + (t * 4.0 + k as f32).sin() * 3.0;
                let y = PIPE_BOTTOM + 140.0 - p * 260.0;
                if y > CASING_WATER + 8.0 {
                    w!(
                        s,
                        r##"<circle cx="{x:.1}" cy="{y:.1}" r="3" fill="none" stroke="#ffffff" stroke-width="1.5" opacity="{:.2}"/>"##,
                        up * 0.7
                    );
                }
            }
        }
    }

    // Faucet head and bucket at the surface (from 36 s).
    if t >= 35.9 {
        w!(
            s,
            r##"<rect x="{IX0}" y="{}" width="{}" height="26" fill="#dfe4e8"/><rect x="{IX0}" y="{}" width="{}" height="6" fill="#ffffff" opacity="0.6"/><rect x="{IX0}" y="{PIPE_TOP}" width="{}" height="{}" fill="#dfe4e8" opacity="0.25"/>"##,
            PIPE_TOP - 13.0,
            612.0 - IX0,
            PIPE_TOP - 10.0,
            612.0 - IX0,
            IX1 - IX0,
            PIPE_TOP.abs() - 50.0
        );
        props::brass_faucet(s, 606.0, PIPE_TOP);
        let fill = smooth(prog(t, 39.8, 42.0));
        props::bucket(s, 632.0, -30.0, 78.0, 70.0, fill, "#5cc8f0");
        let flow = window(t, 39.55, 39.75, 42.2, 42.6);
        if flow > 0.0 {
            let y0 = PIPE_TOP + 38.0;
            stream(s, &format!("M632,{y0:.1} L632,-102"), 8.0, t, flow);
        }
    }
}
