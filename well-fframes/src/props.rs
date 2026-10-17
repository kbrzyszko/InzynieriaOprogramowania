//! Objects placed in the scene. Every function draws in world units at the given position.

use crate::util::*;
use crate::w;

pub fn shadow(s: &mut String, cx: f32, cy: f32, rx: f32, ry: f32, op: f32) {
    w!(
        s,
        r#"<ellipse cx="{cx:.1}" cy="{cy:.1}" rx="{rx:.1}" ry="{ry:.1}" fill="url(#shadow)" opacity="{op:.2}"/>"#
    );
}

pub fn tree(s: &mut String, x: f32, base: f32, sc: f32, seed: u64, t: f32) {
    let mut r = Rng::new(seed * 31 + 5);
    let sway = (t * 0.9 + seed as f32).sin() * 1.2;
    w!(s, r#"<g transform="translate({x:.1} {base:.1}) scale({sc:.2})">"#);
    shadow(s, 0.0, 0.0, 120.0, 20.0, 0.6);
    s.push_str(r#"<path d="M-14,0 L-10,-150 Q-30,-190 -48,-205 L-40,-212 Q-14,-196 -2,-176 Q8,-205 34,-222 L40,-214 Q16,-190 10,-150 L14,0 Z" fill="url(#cTrunk)"/>"#);
    w!(s, r#"<g transform="rotate({sway:.2} 0 -150)">"#);
    let greens = ["#2f6e1a", "#3a7d1e", "#4a9124", "#5aa52a", "#68b236"];
    for i in 0..9 {
        let a = i as f32 / 9.0 * std::f32::consts::TAU;
        let cx = a.cos() * r.r(40.0, 75.0);
        let cy = -280.0 + a.sin() * r.r(40.0, 70.0);
        let rr = r.r(70.0, 95.0);
        let c = greens[i % 3];
        w!(s, r#"<circle cx="{cx:.1}" cy="{cy:.1}" r="{rr:.1}" fill="{c}"/>"#);
    }
    for i in 0..6 {
        let cx = r.r(-60.0, 40.0);
        let cy = r.r(-360.0, -260.0);
        let rr = r.r(40.0, 60.0);
        let c = greens[3 + i % 2];
        w!(s, r#"<circle cx="{cx:.1}" cy="{cy:.1}" r="{rr:.1}" fill="{c}" opacity="0.85"/>"#);
    }
    s.push_str(r##"<ellipse cx="20" cy="-200" rx="120" ry="60" fill="#1d4a0f" opacity="0.25"/>"##);
    s.push_str("</g></g>");
}

/// Two-storey cream house with an orange hip roof, standing behind the fence.
pub fn big_house(s: &mut String, op: f32) {
    let (x0, x1, base, top) = (250.0, 830.0, -440.0, -790.0);
    w!(s, r#"<g opacity="{op:.3}">"#);
    // Side wall in perspective.
    w!(
        s,
        r##"<path d="M{x1},{base} L{:.0},{:.0} L{:.0},{:.0} L{x1},{top} Z" fill="#d8c79f"/>"##,
        x1 + 90.0,
        base - 40.0,
        x1 + 90.0,
        top - 25.0
    );
    w!(
        s,
        r#"<rect x="{x0}" y="{top}" width="{}" height="{}" fill="url(#houseWall)"/>"#,
        x1 - x0,
        base - top
    );
    // Floor divider and windows (two rows of five).
    w!(
        s,
        r##"<rect x="{x0}" y="{:.0}" width="{}" height="10" fill="#e6d7b2"/>"##,
        (base + top) / 2.0 - 5.0,
        x1 - x0
    );
    for row in 0..2 {
        let wy = top + 45.0 + row as f32 * 175.0;
        for i in 0..5 {
            let wx = x0 + 40.0 + i as f32 * 106.0;
            if row == 1 && i == 2 {
                // Front door.
                w!(s, r##"<rect x="{:.0}" y="{:.0}" width="76" height="130" fill="#7a4a2a"/><rect x="{:.0}" y="{:.0}" width="62" height="118" fill="#8f5a34"/><circle cx="{:.0}" cy="{:.0}" r="4" fill="#e0c060"/>"##, wx - 4.0, wy + 2.0, wx + 3.0, wy + 10.0, wx + 54.0, wy + 70.0);
                continue;
            }
            w!(s, r##"<rect x="{wx:.0}" y="{wy:.0}" width="68" height="96" fill="#fdfbf4"/><rect x="{:.0}" y="{:.0}" width="56" height="84" fill="url(#glass)"/><path d="M{:.0},{:.0} V{:.0} M{:.0},{:.0} H{:.0}" stroke="#fdfbf4" stroke-width="5"/><path d="M{:.0},{:.0} L{:.0},{:.0} L{:.0},{:.0} Z" fill="#fff" opacity="0.25"/><rect x="{:.0}" y="{:.0}" width="80" height="9" fill="#e8dcc0"/>"##,
                wx + 6.0, wy + 6.0,
                wx + 34.0, wy + 6.0, wy + 90.0, wx + 6.0, wy + 48.0, wx + 62.0,
                wx + 8.0, wy + 40.0, wx + 30.0, wy + 8.0, wx + 8.0, wy + 8.0,
                wx - 6.0, wy + 96.0);
        }
    }
    // Hip roof.
    let ry = top - 200.0;
    w!(
        s,
        r##"<path d="M{:.0},{top} L{:.0},{ry:.0} L{:.0},{ry:.0} L{:.0},{top} Z" fill="url(#roofOr)"/><path d="M{:.0},{top} L{:.0},{ry:.0} L{:.0},{ry:.0} L{:.0},{top} Z" fill="url(#pShingle)"/>"##,
        x0 - 40.0,
        x0 + 120.0,
        x1 - 120.0,
        x1 + 40.0,
        x0 - 40.0,
        x0 + 120.0,
        x1 - 120.0,
        x1 + 40.0
    );
    w!(
        s,
        r##"<path d="M{:.0},{top} L{:.0},{:.0} L{:.0},{ry:.0} Z" fill="#a8401c"/><rect x="{:.0}" y="{top}" width="{:.0}" height="12" fill="#8a3214"/><rect x="{:.0}" y="{:.0}" width="46" height="90" fill="url(#pBrick)"/><rect x="{:.0}" y="{:.0}" width="56" height="12" fill="#6e6e6e"/>"##,
        x1 + 40.0,
        x1 + 110.0,
        top - 30.0,
        x1 - 120.0,
        x0 - 40.0,
        x1 - x0 + 80.0,
        x1 - 200.0,
        ry - 30.0,
        x1 - 205.0,
        ry - 40.0
    );
    s.push_str("</g>");
}

/// Simple garden gnome: red pointed hat, white beard, blue coat. `(x, y)` is between its feet.
pub fn gnome(s: &mut String, x: f32, y: f32, sc: f32, rot: f32, op: f32) {
    w!(
        s,
        r#"<g transform="translate({x:.1} {y:.1}) rotate({rot:.1}) scale({sc:.2})" opacity="{op:.2}">"#
    );
    s.push_str(concat!(
        r##"<ellipse cx="-13" cy="-6" rx="13" ry="8" fill="#4a2c16"/><ellipse cx="13" cy="-6" rx="13" ry="8" fill="#4a2c16"/>"##,
        r##"<path d="M-26,-10 Q-30,-52 -18,-66 L18,-66 Q30,-52 26,-10 Z" fill="url(#cCoat)"/>"##,
        r##"<rect x="-27" y="-30" width="54" height="8" fill="#2b1a0e"/><rect x="-6" y="-31" width="12" height="10" fill="#e8c34a"/>"##,
        r##"<ellipse cx="-30" cy="-44" rx="9" ry="16" fill="#2a58a8" transform="rotate(18 -30 -44)"/><ellipse cx="30" cy="-44" rx="9" ry="16" fill="#2a58a8" transform="rotate(-18 30 -44)"/>"##,
        r##"<circle cx="-33" cy="-30" r="7" fill="#f2c4a0"/><circle cx="33" cy="-30" r="7" fill="#f2c4a0"/>"##,
        r##"<circle cx="0" cy="-80" r="20" fill="#f2c4a0"/>"##,
        r##"<path d="M-20,-78 Q-24,-40 0,-28 Q24,-40 20,-78 Q10,-66 0,-68 Q-10,-66 -20,-78 Z" fill="#f7f7f2"/>"##,
        r##"<path d="M-14,-74 Q0,-82 14,-74" stroke="#e8e8e2" stroke-width="5" fill="none"/>"##,
        r##"<circle cx="0" cy="-78" r="6" fill="#e8907a"/><circle cx="-8" cy="-87" r="2.5" fill="#2a1a10"/><circle cx="8" cy="-87" r="2.5" fill="#2a1a10"/>"##,
        r##"<path d="M-25,-92 Q-20,-100 0,-101 Q20,-100 25,-92 Q14,-130 4,-150 Q-2,-158 -4,-150 Q-14,-125 -25,-92 Z" fill="url(#cHat)"/>"##,
        r##"<path d="M-25,-92 Q0,-86 25,-92" stroke="#a51d14" stroke-width="4" fill="none"/>"##,
        "</g>"
    ));
}

pub fn rock(s: &mut String, x: f32, y: f32, sc: f32) {
    w!(s, r#"<g transform="translate({x:.1} {y:.1}) scale({sc:.2})">"#);
    shadow(s, 4.0, 0.0, 70.0, 14.0, 0.8);
    s.push_str(concat!(
        r##"<path d="M-58,0 Q-66,-30 -40,-48 Q-10,-66 22,-58 Q56,-48 60,-18 Q62,0 50,4 L-50,4 Z" fill="#8e8f8c"/>"##,
        r##"<path d="M-40,-46 Q-10,-64 22,-56 Q40,-50 46,-38 Q12,-46 -20,-30 Q-36,-24 -50,-20 Q-52,-36 -40,-46 Z" fill="#b4b5b1"/>"##,
        r##"<path d="M20,-10 Q40,-14 58,-6 L52,4 L10,4 Z" fill="#6c6d6a"/>"##,
        "</g>"
    ));
}

/// Garden spade; `(x, y)` is the blade tip, `rot` in degrees.
pub fn shovel(s: &mut String, x: f32, y: f32, rot: f32) {
    w!(s, r#"<g transform="translate({x:.1} {y:.1}) rotate({rot:.1})">"#);
    s.push_str(concat!(
        r##"<rect x="-5" y="-330" width="10" height="290" rx="4" fill="url(#cWoodLt)"/>"##,
        r##"<rect x="-22" y="-342" width="44" height="12" rx="6" fill="#6b4423"/>"##,
        r##"<path d="M-26,-44 L26,-44 L24,-6 Q12,8 0,12 Q-12,8 -24,-6 Z" fill="url(#cSteel)"/>"##,
        r##"<rect x="-8" y="-60" width="16" height="18" fill="#6f777f"/>"##,
        "</g>"
    ));
}

/// Wooden bucket standing on the ground; `fill` 0..1 shows the water near the rim.
pub fn bucket(s: &mut String, cx: f32, base: f32, wd: f32, h: f32, fill: f32, water: &str) {
    let top = base - h;
    let (tw, bw) = (wd / 2.0, wd * 0.4);
    shadow(s, cx + 6.0, base, wd * 0.75, wd * 0.14, 0.8);
    w!(
        s,
        r#"<path d="M{:.1},{top:.1} L{:.1},{base:.1} Q{cx:.1},{:.1} {:.1},{base:.1} L{:.1},{top:.1} Z" fill="url(#cWood)"/>"#,
        cx - tw,
        cx - bw,
        base + wd * 0.12,
        cx + bw,
        cx + tw
    );
    for k in [0.22, 0.75] {
        let yy = top + h * k;
        let half = lerp(tw, bw, k) + 1.0;
        w!(
            s,
            r##"<path d="M{:.1},{yy:.1} Q{cx:.1},{:.1} {:.1},{yy:.1}" stroke="#4a4f55" stroke-width="5" fill="none"/>"##,
            cx - half,
            yy + 6.0,
            cx + half
        );
    }
    w!(
        s,
        r##"<ellipse cx="{cx:.1}" cy="{top:.1}" rx="{tw:.1}" ry="{:.1}" fill="#5a3a1c"/><ellipse cx="{cx:.1}" cy="{:.1}" rx="{:.1}" ry="{:.1}" fill="#3a2410"/>"##,
        wd * 0.13,
        top + 2.0,
        tw - 5.0,
        wd * 0.1
    );
    if fill > 0.01 {
        let r = (tw - 6.0) * (0.75 + 0.25 * fill);
        w!(
            s,
            r#"<ellipse cx="{cx:.1}" cy="{:.1}" rx="{r:.1}" ry="{:.1}" fill="{water}" opacity="{:.2}"/>"#,
            top + 3.0 + (1.0 - fill) * 4.0,
            r * 0.24,
            0.4 + 0.6 * fill
        );
    }
}

/// Small hanging pail with a handle; `(cx, top)` is the middle of its rim.
pub fn pail(s: &mut String, cx: f32, top: f32) {
    w!(
        s,
        r##"<path d="M{:.1},{top:.1} Q{cx:.1},{:.1} {:.1},{top:.1}" stroke="#3a3f45" stroke-width="3" fill="none"/>"##,
        cx - 22.0,
        top - 30.0,
        cx + 22.0
    );
    bucket(s, cx, top + 40.0, 46.0, 40.0, 0.0, "");
}

pub fn arrow(s: &mut String, x: f32, y: f32, up: bool, size: f32, op: f32) {
    let d = if up { -1.0 } else { 1.0 };
    let (hw, sw, l) = (size * 0.5, size * 0.18, size);
    w!(
        s,
        r##"<path d="M{:.1},{:.1} L{:.1},{:.1} L{:.1},{:.1} L{:.1},{:.1} L{:.1},{:.1} L{:.1},{:.1} L{:.1},{:.1} Z" fill="#ffffff" opacity="{op:.2}"/>"##,
        x,
        y + d * l * 0.5,
        x - hw,
        y,
        x - sw,
        y,
        x - sw,
        y - d * l * 0.5,
        x + sw,
        y - d * l * 0.5,
        x + sw,
        y,
        x + hw,
        y
    );
}

/// Red cast-iron pitcher pump. `base` is the underside of its flange, `lever` in degrees.
pub fn red_pump(s: &mut String, cx: f32, base: f32, lever: f32) {
    w!(s, r#"<g transform="translate({cx:.1} {base:.1})">"#);
    // Lever behind the body, pivoting at the top.
    w!(
        s,
        r##"<g transform="rotate({lever:.1} -10 -150)"><rect x="-120" y="-156" width="114" height="12" rx="6" fill="url(#cRed)"/><circle cx="-118" cy="-150" r="9" fill="#8e0f0a"/></g>"##
    );
    s.push_str(concat!(
        r##"<rect x="-30" y="-14" width="60" height="14" rx="3" fill="url(#cRed)"/>"##,
        r##"<rect x="-20" y="-120" width="40" height="108" fill="url(#cRed)"/>"##,
        r##"<rect x="-24" y="-60" width="48" height="8" fill="#8e0f0a"/>"##,
        r##"<path d="M18,-108 L64,-104 Q72,-103 72,-94 L72,-82 L62,-82 L62,-92 L18,-90 Z" fill="url(#cRed)"/>"##,
        r##"<rect x="-26" y="-134" width="52" height="16" rx="4" fill="url(#cRed)"/>"##,
        r##"<path d="M-18,-134 Q0,-156 18,-134 Z" fill="#a8120c"/>"##,
        r##"<rect x="-14" y="-170" width="8" height="40" fill="#7a0c08"/>"##,
        r##"<circle cx="-10" cy="-150" r="6" fill="#5a0805"/>"##,
        r##"<rect x="-12" y="-118" width="5" height="100" fill="#fff" opacity="0.18"/>"##,
        "</g>"
    ));
}

/// Wooden well frame with a gable roof and a windlass; grows out of the ground with `g`.
pub fn well_frame(s: &mut String, g: f32, pail_top: f32, crank: f32) {
    if g <= 0.0 {
        return;
    }
    w!(
        s,
        r#"<g transform="translate(0 -12) scale(1 {g:.3}) translate(0 12)" opacity="{:.2}">"#,
        clamp01(g * 3.0)
    );
    s.push_str(concat!(
        r##"<rect x="432" y="-330" width="16" height="320" fill="url(#cWood)"/>"##,
        r##"<rect x="632" y="-330" width="16" height="320" fill="url(#cWood)"/>"##,
        r##"<path d="M440,-250 L470,-330 M640,-250 L610,-330" stroke="#7a5230" stroke-width="8"/>"##,
        r##"<rect x="436" y="-280" width="208" height="16" rx="8" fill="#8a5a30"/>"##,
        r##"<rect x="436" y="-279" width="208" height="5" rx="2" fill="#c08a58"/>"##,
        r##"<rect x="500" y="-286" width="80" height="28" rx="6" fill="#7a4e2a"/>"##,
        r##"<path d="M502,-280 H578 M502,-272 H578 M502,-264 H578" stroke="#5a381c" stroke-width="2"/>"##,
        r##"<path d="M540,-430 L392,-322 L688,-322 Z" fill="#c9a06a"/>"##,
        r##"<path d="M540,-430 L392,-322 L688,-322 Z" fill="url(#pPlank)" opacity="0.45"/>"##,
        r##"<path d="M540,-446 L376,-326 L394,-312 L540,-418 L686,-312 L704,-326 Z" fill="#7a3418"/>"##,
        r##"<path d="M540,-446 L376,-326 L384,-320 L540,-436 Z" fill="#a0482a"/>"##,
    ));
    // Crank on the right post.
    let (ca, sa) = (crank.to_radians().cos(), crank.to_radians().sin());
    w!(
        s,
        r##"<path d="M648,-272 L{:.1},{:.1}" stroke="#4a4f55" stroke-width="6" stroke-linecap="round"/><rect x="{:.1}" y="{:.1}" width="8" height="22" rx="3" fill="#6b4423"/>"##,
        648.0 + ca * 26.0,
        -272.0 + sa * 26.0,
        644.0 + ca * 26.0,
        -272.0 + sa * 26.0 - 4.0
    );
    w!(
        s,
        r##"<path d="M540,-262 V{:.1}" stroke="#d8c9a4" stroke-width="3"/>"##,
        pail_top - 28.0
    );
    pail(s, 540.0, pail_top);
    s.push_str("</g>");
}

/// Yellow drilling tripod over x = 540. `drop` lifts it (0 = standing).
pub fn tripod(s: &mut String, lift: f32, spin: f32) {
    w!(s, r#"<g transform="translate(0 {:.1})">"#, -lift);
    s.push_str(concat!(
        r##"<path d="M540,-600 L568,-110" stroke="#b07e06" stroke-width="20" stroke-linecap="round"/>"##,
        r##"<path d="M540,-600 L372,-8" stroke="#f2b10a" stroke-width="24" stroke-linecap="round"/>"##,
        r##"<path d="M540,-600 L708,-8" stroke="#f2b10a" stroke-width="24" stroke-linecap="round"/>"##,
        r##"<path d="M540,-600 L372,-8" stroke="#ffd65a" stroke-width="7" stroke-linecap="round" transform="translate(-6 0)"/>"##,
        r##"<path d="M540,-600 L708,-8" stroke="#ffd65a" stroke-width="7" stroke-linecap="round" transform="translate(-6 0)"/>"##,
        r##"<path d="M420,-180 H660 M455,-300 H625" stroke="#d99a06" stroke-width="10"/>"##,
        r##"<rect x="356" y="-14" width="34" height="14" fill="#3c3f45"/><rect x="690" y="-14" width="34" height="14" fill="#3c3f45"/>"##,
        r##"<circle cx="540" cy="-604" r="26" fill="#3c3f45"/><circle cx="540" cy="-604" r="12" fill="#9aa3ab"/>"##,
        r##"<rect x="612" y="-250" width="80" height="64" rx="6" fill="url(#cYellow)"/><rect x="622" y="-240" width="60" height="10" fill="#3c3f45"/>"##,
        r##"<path d="M652,-186 Q600,-140 548,-210" stroke="#2a2d33" stroke-width="5" fill="none"/>"##,
    ));
    w!(
        s,
        r##"<circle cx="540" cy="-604" r="20" fill="none" stroke="#2a2d33" stroke-width="4" stroke-dasharray="10 8" stroke-dashoffset="{:.1}"/>"##,
        spin
    );
    s.push_str("</g>");
}

/// Drill string from the tripod head down to `bit`, with a rotating tri-cone bit.
pub fn drill(s: &mut String, top: f32, bit: f32, spin: f32) {
    w!(
        s,
        r#"<rect x="533" y="{top:.1}" width="14" height="{:.1}" fill="url(#cSteel)"/>"#,
        (bit - 40.0 - top).max(0.0)
    );
    w!(
        s,
        r##"<path d="M540,{top:.1} V{:.1}" stroke="#2a2d33" stroke-width="3" stroke-dasharray="14 18" stroke-dashoffset="{spin:.1}"/>"##,
        bit - 40.0
    );
    w!(
        s,
        r##"<g transform="translate(540 {bit:.1})"><path d="M-30,-44 H30 L36,-20 Q20,8 0,12 Q-20,8 -36,-20 Z" fill="url(#cSteel)"/><path d="M-30,-30 H30 M-28,-16 H28" stroke="#4d555d" stroke-width="4" stroke-dasharray="8 7" stroke-dashoffset="{spin:.1}"/></g>"##
    );
}

/// Brass garden faucet on the end of a pipe; the spout outlet is at `(x + 26, y + 36)`.
pub fn brass_faucet(s: &mut String, x: f32, y: f32) {
    w!(s, r#"<g transform="translate({x:.1} {y:.1})">"#);
    s.push_str(concat!(
        r##"<rect x="-6" y="-14" width="40" height="28" rx="8" fill="url(#cBrass)"/>"##,
        r##"<rect x="18" y="6" width="16" height="30" rx="4" fill="url(#cBrass)"/>"##,
        r##"<rect x="14" y="-30" width="8" height="18" fill="#a07a24"/>"##,
        r##"<ellipse cx="18" cy="-32" rx="22" ry="6" fill="#b8282a"/><ellipse cx="18" cy="-34" rx="16" ry="3" fill="#e05050"/>"##,
        r##"<rect x="-4" y="-10" width="36" height="5" rx="2" fill="#fff3c0" opacity="0.5"/>"##,
        "</g>"
    ));
}

/// Falling water stream along a quadratic curve with moving highlights.
pub fn stream(s: &mut String, d: &str, wd: f32, t: f32, op: f32) {
    w!(
        s,
        r##"<path d="{d}" stroke="#5cc8f0" stroke-width="{wd:.1}" fill="none" stroke-linecap="round" opacity="{:.2}"/><path d="{d}" stroke="#d8f4ff" stroke-width="{:.1}" fill="none" stroke-dasharray="14 22" stroke-dashoffset="{:.1}" opacity="{:.2}"/>"##,
        0.9 * op,
        wd * 0.35,
        -t * 260.0,
        0.8 * op
    );
}

/// Expanding ripple rings on a water surface.
pub fn ripples(s: &mut String, cx: f32, y: f32, maxr: f32, t: f32, op: f32) {
    for k in 0..3 {
        let p = (t * 0.8 + k as f32 / 3.0).fract();
        let r = maxr * (0.2 + 0.8 * p);
        w!(
            s,
            r##"<ellipse cx="{cx:.1}" cy="{y:.1}" rx="{r:.1}" ry="{:.1}" fill="none" stroke="#ffffff" stroke-width="2" opacity="{:.2}"/>"##,
            r * 0.28,
            op * (1.0 - p) * 0.7
        );
    }
}
