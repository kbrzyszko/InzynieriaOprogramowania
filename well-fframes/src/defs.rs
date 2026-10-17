//! Gradients and texture patterns shared by every frame. Built once at start-up.

use crate::util::*;
use crate::w;

/// Horizontal "cylinder" shading: dark edges, a highlight left of centre.
fn cyl(s: &mut String, id: &str, base: &str) {
    w!(
        s,
        r#"<linearGradient id="{id}" x1="0" y1="0" x2="1" y2="0"><stop offset="0" stop-color="{}"/><stop offset="0.2" stop-color="{}"/><stop offset="0.36" stop-color="{}"/><stop offset="0.6" stop-color="{base}"/><stop offset="1" stop-color="{}"/></linearGradient>"#,
        shade(base, 0.5),
        shade(base, 0.85),
        shade(base, 1.3),
        shade(base, 0.48)
    );
}

fn vgrad(s: &mut String, id: &str, stops: &[(f32, &str, f32)]) {
    w!(s, r#"<linearGradient id="{id}" x1="0" y1="0" x2="0" y2="1">"#);
    for (o, c, a) in stops {
        w!(s, r#"<stop offset="{o}" stop-color="{c}" stop-opacity="{a}"/>"#);
    }
    s.push_str("</linearGradient>");
}

fn hgrad(s: &mut String, id: &str, stops: &[(f32, &str, f32)]) {
    w!(s, r#"<linearGradient id="{id}" x1="0" y1="0" x2="1" y2="0">"#);
    for (o, c, a) in stops {
        w!(s, r#"<stop offset="{o}" stop-color="{c}" stop-opacity="{a}"/>"#);
    }
    s.push_str("</linearGradient>");
}

fn radial(s: &mut String, id: &str, stops: &[(f32, &str, f32)]) {
    w!(s, r#"<radialGradient id="{id}" cx="0.5" cy="0.5" r="0.5">"#);
    for (o, c, a) in stops {
        w!(s, r#"<stop offset="{o}" stop-color="{c}" stop-opacity="{a}"/>"#);
    }
    s.push_str("</radialGradient>");
}

fn pattern_open(s: &mut String, id: &str, w_: f32, h: f32, base: &str) {
    w!(
        s,
        r#"<pattern id="{id}" patternUnits="userSpaceOnUse" x="0" y="0" width="{w_}" height="{h}"><rect width="{w_}" height="{h}" fill="{base}"/>"#
    );
}

/// Scatters pebbles inside a tile; pebbles near an edge are repeated on the other side.
fn pebbles(
    s: &mut String,
    rng: &mut Rng,
    tw: f32,
    th: f32,
    n: usize,
    r: (f32, f32),
    cols: &[&str],
    outline: Option<&str>,
) {
    for _ in 0..n {
        let x = rng.r(0.0, tw);
        let y = rng.r(0.0, th);
        let rx = rng.r(r.0, r.1);
        let ry = rx * rng.r(0.55, 0.85);
        let c = cols[(rng.f() * cols.len() as f32) as usize % cols.len()];
        let rot = rng.r(-25.0, 25.0);
        for dx in [-tw, 0.0, tw] {
            for dy in [-th, 0.0, th] {
                let (cx, cy) = (x + dx, y + dy);
                if cx + rx < 0.0 || cx - rx > tw || cy + ry < 0.0 || cy - ry > th {
                    continue;
                }
                let st = outline
                    .map(|o| format!(r#" stroke="{o}" stroke-width="1.6""#))
                    .unwrap_or_default();
                w!(
                    s,
                    r##"<g transform="translate({cx:.1} {cy:.1}) rotate({rot:.0})"><ellipse rx="{rx:.1}" ry="{ry:.1}" fill="{c}"{st}/><ellipse cx="{:.1}" cy="{:.1}" rx="{:.1}" ry="{:.1}" fill="#fff" opacity="0.22"/></g>"##,
                    -rx * 0.25,
                    -ry * 0.35,
                    rx * 0.45,
                    ry * 0.3
                );
            }
        }
    }
}

pub fn build() -> String {
    let mut s = String::with_capacity(120_000);
    s.push_str("<defs>");

    // Sky and atmosphere.
    vgrad(&mut s, "sky", &[(0.0, "#3f72d6", 1.0), (0.55, "#7fa6e6", 1.0), (1.0, "#c8dcf5", 1.0)]);
    radial(&mut s, "cloud", &[(0.0, "#ffffff", 0.95), (0.55, "#ffffff", 0.7), (1.0, "#ffffff", 0.0)]);
    radial(&mut s, "shadow", &[(0.0, "#000000", 0.4), (1.0, "#000000", 0.0)]);
    radial(&mut s, "glow", &[(0.0, "#fff3c4", 0.9), (1.0, "#fff3c4", 0.0)]);
    vgrad(&mut s, "haze", &[(0.0, "#e4eefb", 0.55), (1.0, "#e4eefb", 0.0)]);
    vgrad(&mut s, "lawnShade", &[(0.0, "#ffffff", 0.18), (1.0, "#000000", 0.08)]);
    vgrad(&mut s, "faceShade", &[(0.0, "#000000", 0.0), (1.0, "#000000", 0.22)]);
    vgrad(&mut s, "faceTop", &[(0.0, "#000000", 0.35), (1.0, "#000000", 0.0)]);
    hgrad(&mut s, "shaft", &[(0.0, "#000000", 0.72), (0.5, "#000000", 0.38), (1.0, "#000000", 0.72)]);
    hgrad(&mut s, "shaftLite", &[(0.0, "#000000", 0.45), (0.5, "#000000", 0.12), (1.0, "#000000", 0.45)]);
    vgrad(&mut s, "holeTop", &[(0.0, "#2a170a", 1.0), (1.0, "#4a2a12", 1.0)]);
    hgrad(&mut s, "water", &[(0.0, "#2f9fd0", 0.92), (0.35, "#8fe0fa", 0.9), (0.6, "#5cc8f0", 0.88), (1.0, "#2a92c4", 0.92)]);
    hgrad(&mut s, "murk", &[(0.0, "#7a5a16", 1.0), (0.35, "#d6b04e", 1.0), (0.6, "#b8902e", 1.0), (1.0, "#6a4c12", 1.0)]);
    vgrad(&mut s, "waterV", &[(0.0, "#8fe0fa", 0.9), (0.25, "#5cc8f0", 0.88), (1.0, "#2f8fc4", 0.92)]);
    vgrad(&mut s, "window", &[(0.0, "#5b8fe0", 1.0), (0.7, "#a9c9f2", 1.0), (1.0, "#d6e6fa", 1.0)]);
    vgrad(&mut s, "houseWall", &[(0.0, "#f6eed6", 1.0), (1.0, "#e2d4b0", 1.0)]);
    vgrad(&mut s, "kitchenWall", &[(0.0, "#efe9dc", 1.0), (1.0, "#f8f4ea", 1.0)]);
    vgrad(&mut s, "glass", &[(0.0, "#9fc1e8", 1.0), (0.5, "#5f86bf", 1.0), (1.0, "#3d5f97", 1.0)]);
    vgrad(&mut s, "roofOr", &[(0.0, "#e0683a", 1.0), (1.0, "#a8401c", 1.0)]);
    vgrad(&mut s, "roofGray", &[(0.0, "#7a8089", 1.0), (1.0, "#4a4f57", 1.0)]);
    vgrad(&mut s, "hill", &[(0.0, "#7fae63", 1.0), (1.0, "#5e9447", 1.0)]);
    vgrad(&mut s, "tankInner", &[(0.0, "#55606b", 1.0), (1.0, "#8d98a3", 1.0)]);

    for (id, c) in [
        ("cWater", "#5cc8f0"),
        ("cRed", "#c0140e"),
        ("cNavy", "#1c2b55"),
        ("cPipe", "#8fc4e0"),
        ("cCopper", "#c87533"),
        ("cBrass", "#d4a73a"),
        ("cHeater", "#f05a1a"),
        ("cSteel", "#a9b1b9"),
        ("cTank", "#d3dbe2"),
        ("cWood", "#9a6a3a"),
        ("cWoodLt", "#b98552"),
        ("cStone", "#9a9a97"),
        ("cConcrete", "#bdb9b0"),
        ("cYellow", "#f2b10a"),
        ("cInner", "#dfe4e8"),
        ("cRoofNavy", "#22336b"),
        ("cTrunk", "#6b4423"),
        ("cWhite", "#f4f6f8"),
        ("cFridge", "#e9edf1"),
        ("cRust", "#9a4a1c"),
        ("cHat", "#d42a1f"),
        ("cCoat", "#2f62b8"),
        ("cTerra", "#c0643a"),
        ("cCab", "#1f4fa3"),
    ] {
        cyl(&mut s, id, c);
    }

    let mut rng = Rng::new(7);

    // Soil: orange-brown with vertical streaks.
    let (tw, th) = (420.0, 320.0);
    pattern_open(&mut s, "pSoil", tw, th, "#8a5226");
    for i in 0..90 {
        let x = rng.r(0.0, tw);
        let y = rng.r(0.0, th);
        let h = rng.r(25.0, 150.0);
        let wd = rng.r(1.5, 6.0);
        let (c, a) = if i % 4 == 0 { ("#a8693a", 0.5) } else { ("#5e3313", rng.r(0.3, 0.75)) };
        for dy in [-th, 0.0] {
            w!(
                s,
                r#"<rect x="{x:.1}" y="{:.1}" width="{wd:.1}" height="{h:.1}" rx="{:.1}" fill="{c}" opacity="{a:.2}"/>"#,
                y + dy,
                wd / 2.0
            );
        }
    }
    for _ in 0..60 {
        w!(
            s,
            r##"<circle cx="{:.1}" cy="{:.1}" r="{:.1}" fill="#4a2810" opacity="0.5"/>"##,
            rng.r(0.0, tw),
            rng.r(0.0, th),
            rng.r(1.0, 3.0)
        );
    }
    s.push_str("</pattern>");

    // Grey gravel layer under the soil.
    pattern_open(&mut s, "pGravel", 300.0, 220.0, "#8b8781");
    pebbles(&mut s, &mut rng, 300.0, 220.0, 90, (4.0, 13.0), &["#a7a39c", "#77736d", "#6a665f", "#b5b0a8", "#96918a"], None);
    s.push_str("</pattern>");

    // Shallow aquifer: blue sand and pebbles.
    pattern_open(&mut s, "pAq1", 300.0, 200.0, "#4a6fd6");
    pebbles(&mut s, &mut rng, 300.0, 200.0, 70, (4.0, 12.0), &["#5f82e0", "#3b5cc4", "#6d8fe6", "#4266cf"], None);
    s.push_str("</pattern>");

    // Grey clay with horizontal smears.
    pattern_open(&mut s, "pClay", 520.0, 300.0, "#8a8a88");
    for _ in 0..70 {
        let x = rng.r(-100.0, 520.0);
        let y = rng.r(0.0, 300.0);
        let c = if rng.f() < 0.5 { "#7a7a78" } else { "#9c9c9a" };
        w!(
            s,
            r#"<rect x="{x:.1}" y="{y:.1}" width="{:.1}" height="{:.1}" rx="2" fill="{c}" opacity="{:.2}"/>"#,
            rng.r(40.0, 220.0),
            rng.r(2.0, 6.0),
            rng.r(0.4, 0.8)
        );
    }
    for _ in 0..80 {
        w!(
            s,
            r##"<circle cx="{:.1}" cy="{:.1}" r="{:.1}" fill="#6f6f6d" opacity="0.6"/>"##,
            rng.r(0.0, 520.0),
            rng.r(0.0, 300.0),
            rng.r(0.8, 2.2)
        );
    }
    s.push_str("</pattern>");

    // Deep aquifer: light blue, rocky.
    pattern_open(&mut s, "pAq2", 360.0, 260.0, "#5f98c0");
    pebbles(&mut s, &mut rng, 360.0, 260.0, 55, (14.0, 30.0), &["#7fb8d9", "#8cc2e0", "#74aed2", "#93c7e3"], Some("#4f88b0"));
    s.push_str("</pattern>");

    // Bedrock with grain.
    pattern_open(&mut s, "pRock", 300.0, 300.0, "#333438");
    for _ in 0..320 {
        let c = ["#3f4045", "#2a2b2e", "#4a4b51", "#26272a", "#5a5b62"][(rng.f() * 5.0) as usize % 5];
        w!(
            s,
            r#"<ellipse cx="{:.1}" cy="{:.1}" rx="{:.1}" ry="{:.1}" fill="{c}" transform="rotate({:.0} 150 150)"/>"#,
            rng.r(0.0, 300.0),
            rng.r(0.0, 300.0),
            rng.r(1.0, 6.0),
            rng.r(0.8, 3.0),
            rng.r(-20.0, 20.0)
        );
    }
    for _ in 0..30 {
        w!(
            s,
            r##"<circle cx="{:.1}" cy="{:.1}" r="{:.1}" fill="#8a8b92" opacity="0.7"/>"##,
            rng.r(0.0, 300.0),
            rng.r(0.0, 300.0),
            rng.r(0.6, 1.6)
        );
    }
    s.push_str("</pattern>");

    // Stone blocks for the well lining.
    pattern_open(&mut s, "pStone", 120.0, 60.0, "#5f5f5d");
    for row in 0..2 {
        let off = if row == 0 { 0.0 } else { -30.0 };
        for i in 0..3 {
            let x = off + i as f32 * 60.0 + 2.0;
            let y = row as f32 * 30.0 + 2.0;
            let c = ["#9a9a97", "#8c8c89", "#a6a6a2", "#929290"][(rng.f() * 4.0) as usize % 4];
            w!(
                s,
                r##"<rect x="{x:.1}" y="{y:.1}" width="56" height="26" rx="6" fill="{c}"/><rect x="{:.1}" y="{:.1}" width="46" height="5" rx="2.5" fill="#fff" opacity="0.2"/>"##,
                x + 5.0,
                y + 3.0
            );
        }
    }
    s.push_str("</pattern>");

    // Bricks.
    pattern_open(&mut s, "pBrick", 48.0, 24.0, "#d9ccb8");
    for (x, y) in [(1.0, 1.0), (25.0, 1.0), (-11.0, 13.0), (13.0, 13.0), (37.0, 13.0)] {
        let c = ["#a6462c", "#9a3e26", "#b04f33"][(rng.f() * 3.0) as usize % 3];
        w!(s, r#"<rect x="{x}" y="{y}" width="22" height="10" fill="{c}"/>"#);
    }
    s.push_str("</pattern>");

    // Fence pickets.
    pattern_open(&mut s, "pFence", 34.0, 100.0, "none");
    s.push_str(r#"<path d="M5,100 V14 L15,2 L25,14 V100 Z" fill="url(#cWoodLt)"/>"#);
    s.push_str("</pattern>");

    // Roof shingles (orange) and wall tiles.
    pattern_open(&mut s, "pShingle", 40.0, 24.0, "none");
    s.push_str(r##"<path d="M0,11.5 H40 M0,23.5 H40 M10,0 V12 M30,12 V24" stroke="#7e2e12" stroke-width="2" opacity="0.45"/>"##);
    s.push_str("</pattern>");
    pattern_open(&mut s, "pTile", 22.0, 22.0, "#fbfbfb");
    s.push_str(r##"<path d="M0,0.5 H22 M0.5,0 V22" stroke="#c9ccd0" stroke-width="1.2"/>"##);
    s.push_str("</pattern>");
    pattern_open(&mut s, "pPlank", 90.0, 20.0, "#c49a6c");
    s.push_str(r##"<path d="M0,0.5 H90 M30,0 V10 M75,10 V20 M0,10.5 H90" stroke="#9b7146" stroke-width="1.3"/>"##);
    s.push_str("</pattern>");
    pattern_open(&mut s, "pRoofGray", 36.0, 18.0, "none");
    s.push_str(r##"<path d="M0,17.5 H36 M18,0 V9 M0,9 H36" stroke="#2f343a" stroke-width="1.6" opacity="0.5"/>"##);
    s.push_str("</pattern>");

    // Grass blades on the front edge of the lawn.
    pattern_open(&mut s, "pBlades", 60.0, 26.0, "none");
    for i in 0..12 {
        let x = i as f32 * 5.0 + rng.r(0.0, 3.0);
        let h = rng.r(10.0, 24.0);
        let c = if i % 2 == 0 { "#4f9a24" } else { "#3a7d1e" };
        w!(s, r#"<path d="M{:.1},26 L{:.1},{:.1} L{:.1},26 Z" fill="{c}"/>"#, x, x + rng.r(-2.0, 3.0), 26.0 - h, x + 4.0);
    }
    s.push_str("</pattern>");

    s.push_str("</defs>");
    s
}
