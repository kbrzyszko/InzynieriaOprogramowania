//! Act 3 (41–66.5 s): pipes to an elevated tank and a small house with a kitchen.

use crate::props::{self, stream};
use crate::util::*;
use crate::w;

const TX0: f32 = 1000.0;
const TX1: f32 = 1300.0;
const TANK_H: f32 = 300.0;
const RISER_X: f32 = 1060.0;
const OUT_X: f32 = 1240.0;

// Kitchen geometry.
const HX0: f32 = 1500.0;
const HX1: f32 = 2150.0;
const FLOOR: f32 = -30.0;
const BACK: f32 = -75.0;
const CEIL: f32 = -455.0;
const COUNTER: f32 = -205.0;
const SINK_X0: f32 = 1772.0;
const SINK_X1: f32 = 1878.0;
const FEED_X: f32 = 2000.0;
const TAP_Y: f32 = -250.0;

/// Extra height of the truss in the final shot (tank becomes a water tower).
pub fn tower_ext(t: f32) -> f32 {
    800.0 * ease_io(prog(t, 62.4, 64.9))
}

fn tank_bottom(t: f32) -> f32 {
    -700.0 - tower_ext(t)
}

fn tank_water(t: f32) -> f32 {
    let tb = tank_bottom(t);
    lerp(tb - 70.0, tb - TANK_H + 108.0, ease_io(prog(t, 49.9, 52.0)))
}

fn valve_open(t: f32) -> bool {
    t > 43.5 && t < 52.0
}

fn sink_level(t: f32) -> f32 {
    lerp(COUNTER + 60.0, COUNTER + 8.0, smooth(prog(t, 47.7, 56.5)))
}

fn plen(p: &[(f32, f32)]) -> f32 {
    p.windows(2).map(|w| ((w[1].0 - w[0].0).powi(2) + (w[1].1 - w[0].1).powi(2)).sqrt()).sum()
}

fn pdata(p: &[(f32, f32)]) -> String {
    let mut d = String::new();
    for (i, (x, y)) in p.iter().enumerate() {
        w!(d, "{}{x:.1},{y:.1} ", if i == 0 { "M" } else { "L" });
    }
    d
}

/// Pipe as layered strokes; `grow` reveals it along its length, `flow` animates water dashes.
fn pipe(s: &mut String, p: &[(f32, f32)], grow: f32, flow: f32, t: f32, colors: (&str, &str, &str), wd: f32) {
    if grow <= 0.0 {
        return;
    }
    let l = plen(p);
    let d = pdata(p);
    let dash = format!(r#"stroke-dasharray="{:.1} 99999""#, l * grow);
    w!(
        s,
        r#"<path d="{d}" fill="none" stroke="{}" stroke-width="{:.1}" stroke-linejoin="round" {dash}/><path d="{d}" fill="none" stroke="{}" stroke-width="{:.1}" stroke-linejoin="round" {dash}/><path d="{d}" fill="none" stroke="{}" stroke-width="{:.1}" stroke-linejoin="round" opacity="0.8" {dash}/>"#,
        colors.0,
        wd + 6.0,
        colors.1,
        wd,
        colors.2,
        wd * 0.25
    );
    if flow > 0.0 && grow >= 1.0 {
        w!(
            s,
            r##"<path d="{d}" fill="none" stroke="#2f9fd8" stroke-width="{:.1}" stroke-dasharray="18 30" stroke-dashoffset="{:.1}" opacity="{:.2}"/>"##,
            wd * 0.4,
            -t * 160.0,
            flow * 0.75
        );
    }
}

const BLUE: (&str, &str, &str) = ("#4f86a6", "#8fc4e0", "#d5ecf7");
const COPPER: (&str, &str, &str) = ("#7a3f16", "#c87533", "#f0b07a");

fn p1(t: f32) -> Vec<(f32, f32)> {
    vec![(600.0, 150.0), (RISER_X, 150.0), (RISER_X, tank_bottom(t) + 2.0)]
}

fn p2(t: f32) -> Vec<(f32, f32)> {
    vec![
        (OUT_X, tank_bottom(t) + 2.0),
        (OUT_X, 190.0),
        (FEED_X, 190.0),
        (FEED_X, TAP_Y),
        (SINK_X1 - 10.0, TAP_Y),
    ]
}

fn leg(s: &mut String, a: (f32, f32), b: (f32, f32), wd: f32, back: bool) {
    let (o, c) = if back { ("#7d8791", "#c9d0d7") } else { ("#8c969f", "#f4f6f8") };
    w!(
        s,
        r#"<path d="M{:.1},{:.1} L{:.1},{:.1}" stroke="{o}" stroke-width="{:.1}" stroke-linecap="round"/><path d="M{:.1},{:.1} L{:.1},{:.1}" stroke="{c}" stroke-width="{:.1}" stroke-linecap="round"/>"#,
        a.0,
        a.1,
        b.0,
        b.1,
        wd + 5.0,
        a.0,
        a.1,
        b.0,
        b.1,
        wd
    );
}

fn truss(s: &mut String, tb: f32, back: bool) {
    let (fl0, fl1, fr0, fr1) = if back {
        ((1000.0, -150.0), (1045.0, tb - 20.0), (1300.0, -150.0), (1255.0, tb - 20.0))
    } else {
        ((950.0, -15.0), (1020.0, tb), (1350.0, -15.0), (1280.0, tb))
    };
    let wd = if back { 9.0 } else { 14.0 };
    leg(s, fl0, fl1, wd, back);
    leg(s, fr0, fr1, wd, back);
    let bays = ((fl0.1 - fl1.1) / 180.0).round().max(2.0) as usize;
    let at = |a: (f32, f32), b: (f32, f32), k: f32| (lerp(a.0, b.0, k), lerp(a.1, b.1, k));
    for i in 0..bays {
        let k0 = i as f32 / bays as f32;
        let k1 = (i + 1) as f32 / bays as f32;
        let (l0, l1, r0, r1) = (at(fl0, fl1, k0), at(fl0, fl1, k1), at(fr0, fr1, k0), at(fr0, fr1, k1));
        leg(s, l1, r1, wd * 0.6, back);
        leg(s, l0, r1, wd * 0.45, back);
        leg(s, r0, l1, wd * 0.45, back);
    }
}

fn tank(s: &mut String, t: f32) {
    let tb = tank_bottom(t);
    let tt = tb - TANK_H;
    let cut = window(t, 49.3, 49.9, 52.6, 53.2);
    let cx = (TX0 + TX1) / 2.0;
    // Platform and railing.
    w!(
        s,
        r##"<rect x="975" y="{tb:.1}" width="350" height="14" fill="#59606a"/><path d="M980,{tb:.1} V{:.1} M1320,{tb:.1} V{:.1} M980,{:.1} H1320" stroke="#e9edf0" stroke-width="4" fill="none"/>"##,
        tb - 36.0,
        tb - 36.0,
        tb - 36.0
    );
    // Interior (visible when the shell is cut open).
    if cut > 0.0 {
        let tw = tank_water(t);
        w!(s, r#"<g opacity="{cut:.2}">"#);
        w!(
            s,
            r##"<rect x="{TX0}" y="{tt:.1}" width="{}" height="{TANK_H}" fill="url(#tankInner)"/><path d="M{TX0},{tb:.1} A150,26 0 0 1 {TX1},{tb:.1} Z" fill="#46505a"/>"##,
            TX1 - TX0
        );
        // Riser with the float valve.
        let vy = tt + 62.0;
        w!(
            s,
            r##"<rect x="{}" y="{vy:.1}" width="18" height="{:.1}" fill="url(#cPipe)"/><rect x="{}" y="{:.1}" width="34" height="24" rx="5" fill="url(#cBrass)"/><rect x="{}" y="{:.1}" width="12" height="16" fill="#b08a2c"/>"##,
            RISER_X - 9.0,
            tb - vy,
            RISER_X - 17.0,
            vy - 12.0,
            RISER_X - 6.0,
            vy + 10.0
        );
        if valve_open(t) {
            stream(s, &format!("M{RISER_X},{:.1} L{RISER_X},{tw:.1}", vy + 26.0), 9.0, t, 1.0);
        }
        // Water body.
        w!(
            s,
            r##"<rect x="{TX0}" y="{tw:.1}" width="{}" height="{:.1}" fill="url(#water)"/><path d="M{TX0},{tw:.1} A150,24 0 0 1 {TX1},{tw:.1} Z" fill="#a8e8fb" opacity="0.9"/>"##,
            TX1 - TX0,
            tb - tw
        );
        props::ripples(s, RISER_X + 10.0, tw - 8.0, 60.0, t, if valve_open(t) { 1.0 } else { 0.0 });
        // Float arm and copper ball.
        let (px, py) = (RISER_X + 14.0, vy);
        let arm = 165.0;
        let by = tw - 6.0;
        let a = ((by - py) / arm).clamp(-1.0, 1.0).asin();
        let bx = px + arm * a.cos();
        w!(
            s,
            r##"<path d="M{px:.1},{py:.1} L{bx:.1},{by:.1}" stroke="#7d868f" stroke-width="6" stroke-linecap="round"/><circle cx="{bx:.1}" cy="{by:.1}" r="28" fill="url(#cCopper)"/><circle cx="{:.1}" cy="{:.1}" r="9" fill="#ffd2a8" opacity="0.6"/><circle cx="{px:.1}" cy="{py:.1}" r="6" fill="#5a6068"/>"##,
            bx - 9.0,
            by - 10.0
        );
        w!(
            s,
            r##"<rect x="{TX0}" y="{tt:.1}" width="9" height="{TANK_H}" fill="#9aa6b1"/><rect x="{}" y="{tt:.1}" width="9" height="{TANK_H}" fill="#9aa6b1"/>"##,
            TX1 - 9.0
        );
        s.push_str("</g>");
    }
    // Outer shell.
    let shell = 1.0 - cut;
    if shell > 0.0 {
        w!(
            s,
            r##"<g opacity="{shell:.2}"><path d="M{TX0},{tt:.1} V{tb:.1} A150,26 0 0 0 {TX1},{tb:.1} V{tt:.1} Z" fill="url(#cTank)"/><path d="M{TX0},{:.1} A150,26 0 0 0 {TX1},{:.1} M{TX0},{:.1} A150,26 0 0 0 {TX1},{:.1}" stroke="#9aa6b1" stroke-width="3" fill="none"/></g>"##,
            tt + 100.0,
            tt + 100.0,
            tt + 200.0,
            tt + 200.0
        );
    }
    // Navy conical roof.
    w!(
        s,
        r##"<path d="M{:.1},{tt:.1} L{cx:.1},{:.1} L{:.1},{tt:.1} A160,28 0 0 1 {:.1},{tt:.1} Z" fill="url(#cRoofNavy)"/><rect x="{:.1}" y="{:.1}" width="24" height="18" fill="#1a2650"/><ellipse cx="{cx:.1}" cy="{:.1}" rx="18" ry="5" fill="#2c3f80"/>"##,
        TX0 - 12.0,
        tt - 110.0,
        TX1 + 12.0,
        TX0 - 12.0,
        cx - 12.0,
        tt - 126.0,
        tt - 126.0
    );
}

/// Small house cut open: kitchen with blue cabinets, fridge, window, lamp and plant.
fn house_back(s: &mut String, t: f32) {
    // Foundation in the ground and floor slab.
    w!(
        s,
        r##"<rect x="{HX0}" y="0" width="{}" height="70" fill="url(#cConcrete)"/><rect x="{HX0}" y="{FLOOR}" width="{}" height="{}" fill="#a9a59c"/>"##,
        HX1 - HX0,
        HX1 - HX0,
        -FLOOR
    );
    // Floor in perspective, back wall, ceiling.
    w!(
        s,
        r##"<rect x="{}" y="{BACK}" width="{}" height="{}" fill="url(#pPlank)"/><rect x="{}" y="{CEIL}" width="{}" height="{}" fill="url(#kitchenWall)"/>"##,
        HX0 + 30.0,
        HX1 - HX0 - 60.0,
        FLOOR - BACK,
        HX0 + 30.0,
        HX1 - HX0 - 60.0,
        BACK - CEIL
    );
    w!(
        s,
        r##"<rect x="{}" y="{BACK}" width="{}" height="6" fill="#000" opacity="0.15"/>"##,
        HX0 + 30.0,
        HX1 - HX0 - 60.0
    );
    // Backsplash tiles.
    w!(
        s,
        r##"<rect x="1655" y="-345" width="335" height="110" fill="#f4f4f4"/><rect x="1655" y="-345" width="335" height="110" fill="url(#pTile)"/>"##
    );
    // Window with sky and a tree outside.
    s.push_str(concat!(
        r##"<rect x="1764" y="-448" width="122" height="104" fill="#ffffff"/>"##,
        r##"<rect x="1772" y="-440" width="106" height="88" fill="url(#window)"/>"##,
        r##"<circle cx="1850" cy="-372" r="26" fill="#4a9124"/><circle cx="1832" cy="-360" r="18" fill="#3a7d1e"/>"##,
        r##"<rect x="1772" y="-372" width="106" height="20" fill="#5aa52a" opacity="0.8"/>"##,
        r##"<path d="M1825,-440 V-352 M1772,-396 H1878" stroke="#ffffff" stroke-width="5"/>"##,
        r##"<rect x="1758" y="-350" width="134" height="8" fill="#e8e4dc"/>"##,
    ));
    // Upper cabinets.
    for (x, wd) in [(1655.0, 104.0), (1892.0, 98.0)] {
        w!(
            s,
            r##"<rect x="{x}" y="-450" width="{wd}" height="96" fill="#1f4fa3"/><rect x="{}" y="-444" width="{}" height="84" fill="#2a5db5"/><rect x="{}" y="-400" width="5" height="18" rx="2" fill="#d9dde2"/>"##,
            x + 5.0,
            wd - 10.0,
            x + wd - 16.0
        );
    }
    // Fridge.
    s.push_str(concat!(
        r##"<rect x="1545" y="-388" width="100" height="313" rx="8" fill="url(#cFridge)"/>"##,
        r##"<path d="M1547,-268 H1643" stroke="#b9c1c9" stroke-width="3"/>"##,
        r##"<rect x="1625" y="-350" width="6" height="60" rx="3" fill="#9aa3ab"/><rect x="1625" y="-250" width="6" height="70" rx="3" fill="#9aa3ab"/>"##,
    ));
    // Base cabinets and countertop.
    s.push_str(concat!(
        r##"<rect x="1655" y="-205" width="335" height="130" fill="#1f4fa3"/>"##,
        r##"<rect x="1655" y="-85" width="335" height="10" fill="#14366f"/>"##,
        r##"<path d="M1738,-200 V-90 M1822,-200 V-90 M1906,-200 V-90" stroke="#163d80" stroke-width="4"/>"##,
        r##"<rect x="1661" y="-198" width="70" height="104" fill="#2a5db5"/><rect x="1745" y="-198" width="70" height="104" fill="#2a5db5"/>"##,
        r##"<rect x="1829" y="-198" width="70" height="104" fill="#2a5db5"/><rect x="1913" y="-198" width="70" height="104" fill="#2a5db5"/>"##,
        r##"<circle cx="1722" cy="-150" r="4" fill="#d9dde2"/><circle cx="1754" cy="-150" r="4" fill="#d9dde2"/><circle cx="1890" cy="-150" r="4" fill="#d9dde2"/><circle cx="1922" cy="-150" r="4" fill="#d9dde2"/>"##,
        r##"<rect x="1648" y="-235" width="349" height="22" fill="#f1eee7"/><rect x="1648" y="-215" width="349" height="12" fill="#d6d1c6"/>"##,
    ));
    // Plant on the counter.
    s.push_str(concat!(
        r##"<path d="M1688,-262 H1720 L1716,-235 H1692 Z" fill="url(#cTerra)"/>"##,
        r##"<ellipse cx="1694" cy="-282" rx="8" ry="22" fill="#3a7d1e" transform="rotate(-30 1694 -282)"/>"##,
        r##"<ellipse cx="1714" cy="-284" rx="8" ry="22" fill="#4a9124" transform="rotate(28 1714 -284)"/>"##,
        r##"<ellipse cx="1704" cy="-292" rx="8" ry="26" fill="#5aa52a"/>"##,
        r##"<ellipse cx="1688" cy="-270" rx="6" ry="14" fill="#5aa52a" transform="rotate(-60 1688 -270)"/>"##,
    ));
    // Ceiling lamp with a warm glow.
    let flick = 0.85 + 0.05 * (t * 9.0).sin();
    w!(
        s,
        r##"<circle cx="1720" cy="-392" r="90" fill="url(#glow)" opacity="{flick:.2}"/><path d="M1720,{CEIL} V-412" stroke="#3a3a3a" stroke-width="3"/><path d="M1700,-412 H1740 L1752,-392 H1688 Z" fill="#2f4f8f"/><ellipse cx="1720" cy="-391" rx="10" ry="5" fill="#fff6d0"/>"##
    );
}

/// Sink basin (cut open), faucet and the walls/ceiling/roof drawn over the pipes.
fn house_front(s: &mut String, t: f32) {
    // Sink basin cut open in front of the cabinet.
    let sl = sink_level(t);
    w!(
        s,
        r##"<path d="M{SINK_X0},{COUNTER} V{:.1} Q{SINK_X0},{:.1} {:.1},{:.1} H{:.1} Q{SINK_X1},{:.1} {SINK_X1},{:.1} V{COUNTER} Z" fill="url(#cSteel)"/><rect x="{:.1}" y="{COUNTER}" width="{:.1}" height="56" fill="#d9dee3"/>"##,
        COUNTER - 2.0 + 64.0,
        COUNTER + 72.0,
        SINK_X0 + 10.0,
        COUNTER + 72.0,
        SINK_X1 - 10.0,
        COUNTER + 72.0,
        COUNTER + 62.0,
        SINK_X0 + 7.0,
        SINK_X1 - SINK_X0 - 14.0
    );
    let filling = t >= 47.5;
    if filling {
        let bottom = COUNTER + 62.0;
        let top = sl.min(bottom - 1.0);
        w!(
            s,
            r##"<rect x="{:.1}" y="{top:.1}" width="{:.1}" height="{:.1}" fill="url(#waterV)"/><rect x="{:.1}" y="{:.1}" width="{:.1}" height="3" fill="#d8f6ff"/>"##,
            SINK_X0 + 7.0,
            SINK_X1 - SINK_X0 - 14.0,
            bottom - top,
            SINK_X0 + 7.0,
            top - 1.0,
            SINK_X1 - SINK_X0 - 14.0
        );
        props::ripples(s, 1816.0, top + 2.0, 30.0, t, 1.0);
    }
    w!(
        s,
        r##"<rect x="{:.1}" y="{:.1}" width="{:.1}" height="7" rx="3" fill="#c3cad1"/>"##,
        SINK_X0 - 6.0,
        COUNTER - 6.0,
        SINK_X1 - SINK_X0 + 12.0
    );
    // Drain under the sink.
    w!(s, r##"<path d="M1825,{:.1} V-100 Q1825,-82 1845,-82 H1900" stroke="#9aa3ab" stroke-width="8" fill="none"/>"##, COUNTER + 72.0);
    // Faucet (gooseneck) with lever.
    s.push_str(concat!(
        r##"<path d="M1858,-236 V-326 Q1858,-346 1838,-346 H1828 Q1816,-346 1816,-334 V-318" stroke="#8d969f" stroke-width="11" fill="none" stroke-linecap="round"/>"##,
        r##"<path d="M1858,-236 V-326 Q1858,-346 1838,-346 H1828 Q1816,-346 1816,-334 V-318" stroke="#e6ebef" stroke-width="5" fill="none" stroke-linecap="round"/>"##,
        r##"<rect x="1848" y="-246" width="20" height="12" rx="3" fill="#a9b1b9"/>"##,
        r##"<path d="M1866,-262 L1886,-272" stroke="#a9b1b9" stroke-width="5" stroke-linecap="round"/>"##,
    ));
    if filling {
        let bottom = sl;
        stream(s, &format!("M1816,-316 L1816,{bottom:.1}"), 6.0, t, smooth(prog(t, 47.4, 47.6)));
    }
    // Steam over the sink once the hot water flows.
    let steam = window(t, 58.4, 59.2, 64.0, 65.0);
    if steam > 0.0 {
        for k in 0..4 {
            let p = (t * 0.45 + k as f32 * 0.25).fract();
            let x = 1800.0 + k as f32 * 14.0 + (t * 2.0 + k as f32).sin() * 8.0;
            let y = sl - 10.0 - p * 120.0;
            w!(
                s,
                r##"<ellipse cx="{x:.1}" cy="{y:.1}" rx="{:.1}" ry="{:.1}" fill="#ffffff" opacity="{:.2}"/>"##,
                8.0 + 10.0 * p,
                6.0 + 6.0 * p,
                steam * 0.45 * (1.0 - p)
            );
        }
    }

    // Cut walls, ceiling, attic, gray roof and brick chimney.
    w!(
        s,
        r##"<rect x="{HX0}" y="{CEIL}" width="30" height="{}" fill="url(#cWhite)"/><rect x="{}" y="{CEIL}" width="30" height="{}" fill="url(#cWhite)"/><rect x="{HX0}" y="{:.0}" width="{}" height="22" fill="#e6e2da"/>"##,
        FLOOR - CEIL,
        HX1 - 30.0,
        FLOOR - CEIL,
        CEIL - 22.0,
        HX1 - HX0
    );
    let ridge = CEIL - 270.0;
    let mx = (HX0 + HX1) / 2.0;
    w!(
        s,
        r##"<path d="M{HX0},{:.0} L{mx},{:.0} L{HX1},{:.0} Z" fill="#6e5a46"/><path d="M{},{:.0} L{mx},{:.0} L{},{:.0}" stroke="#5a4838" stroke-width="10" fill="none"/>"##,
        CEIL - 22.0,
        ridge + 30.0,
        CEIL - 22.0,
        HX0 + 70.0,
        CEIL - 22.0,
        ridge + 60.0,
        HX1 - 70.0,
        CEIL - 22.0
    );
    // Chimney behind the right roof slope.
    w!(
        s,
        r##"<rect x="1985" y="{:.0}" width="58" height="240" fill="url(#pBrick)"/><rect x="1978" y="{:.0}" width="72" height="14" fill="#6e6e6e"/>"##,
        ridge - 50.0,
        ridge - 62.0
    );
    w!(
        s,
        r##"<path d="M{:.0},{:.0} L{mx},{:.0} L{:.0},{:.0} L{:.0},{:.0} L{mx},{:.0} L{:.0},{:.0} Z" fill="url(#roofGray)"/><path d="M{:.0},{:.0} L{mx},{:.0} L{:.0},{:.0} L{:.0},{:.0} L{mx},{:.0} L{:.0},{:.0} Z" fill="url(#pRoofGray)"/>"##,
        HX0 - 40.0,
        CEIL - 2.0,
        ridge,
        HX1 + 40.0,
        CEIL - 2.0,
        HX1 + 40.0,
        CEIL + 28.0 - 2.0,
        ridge + 36.0,
        HX0 - 40.0,
        CEIL + 26.0,
        HX0 - 40.0,
        CEIL - 2.0,
        ridge,
        HX1 + 40.0,
        CEIL - 2.0,
        HX1 + 40.0,
        CEIL + 26.0,
        ridge + 36.0,
        HX0 - 40.0,
        CEIL + 26.0
    );
}

/// Orange water heater with a copper hot-water line to the faucet (57–58 s).
fn heater(s: &mut String, t: f32) {
    let g = ease_back(prog(t, 56.9, 57.5));
    if g <= 0.0 {
        return;
    }
    w!(
        s,
        r##"<g transform="translate(2062 {BACK}) scale({g:.3}) translate(-2062 {})"><rect x="2016" y="-335" width="92" height="262" rx="12" fill="url(#cHeater)"/><path d="M2016,-330 Q2062,-360 2108,-330 Z" fill="#f4f6f8"/><rect x="2040" y="-210" width="44" height="30" rx="4" fill="#2b2f36"/><circle cx="2052" cy="-195" r="5" fill="#ff5a2a"/><circle cx="2072" cy="-195" r="5" fill="#9ad7ff"/><rect x="2024" y="-322" width="8" height="240" rx="4" fill="#fff" opacity="0.2"/></g>"##,
        -BACK
    );
    let cold = ease_io(prog(t, 57.2, 57.6));
    pipe(s, &[(FEED_X, -120.0), (2016.0, -120.0)], cold, 0.0, t, BLUE, 10.0);
    let hot = ease_io(prog(t, 57.6, 58.5));
    let path = [(2040.0, -338.0), (2040.0, -360.0), (1990.0, -360.0), (1990.0, -272.0), (1866.0, -272.0), (1866.0, -248.0)];
    pipe(s, &path, hot, if hot >= 1.0 { 1.0 } else { 0.0 }, t, COPPER, 10.0);
}

pub fn act_c(s: &mut String, t: f32) {
    if t < 40.5 {
        return;
    }
    let tb = tank_bottom(t);
    truss(s, tb, true);
    // Pipes grow one after another, water runs through them.
    let g1 = ease_io(prog(t, 42.6, 44.4));
    let g2 = ease_io(prog(t, 44.4, 46.4));
    let f1 = if valve_open(t) { 1.0 } else { 0.0 };
    pipe(s, &p1(t), g1, f1, t, BLUE, 18.0);
    pipe(s, &p2(t), g2, 1.0, t, BLUE, 18.0);
    truss(s, tb, false);
    tank(s, t);
    house_back(s, t);
    // The house hides the part of the feed pipe that runs behind the cabinets.
    let all = p2(t);
    let (head, tail) = (plen(&all[..3]), plen(&all[2..]));
    pipe(s, &all[2..], clamp01((g2 * (head + tail) - head) / tail), 1.0, t, BLUE, 18.0);
    heater(s, t);
    house_front(s, t);
}
