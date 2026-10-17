//! The garden diorama shared by every scene: sky, background, striped lawn and the cut ground face.

use crate::props;
use crate::util::*;
use crate::w;

pub const LAWN_BACK: f32 = -430.0;
pub const XL: f32 = -3000.0;
pub const XR: f32 = 5500.0;
pub const L_SOIL: f32 = 22.0;
pub const L_GRAVEL: f32 = 400.0;
pub const L_AQ1: f32 = 585.0;
pub const L_CLAY: f32 = 700.0;
pub const L_AQ2: f32 = 1700.0;
pub const L_ROCK: f32 = 2150.0;
pub const L_BOTTOM: f32 = 6000.0;

/// Screen-space sky with soft clouds; the gradient follows the horizon.
pub fn sky(s: &mut String, cam: Cam, t: f32) {
    let hs = (LAWN_BACK - 120.0 - cam.y) * cam.z + 960.0;
    w!(s, r##"<rect width="1080" height="1920" fill="#3f72d6"/>"##);
    w!(s, r#"<rect x="0" y="{:.1}" width="1080" height="1800" fill="url(#sky)"/>"#, hs - 1700.0);
    w!(s, r##"<rect x="0" y="{:.1}" width="1080" height="4000" fill="#c8dcf5"/>"##, hs + 99.0);
    let clouds = [
        (120.0, -1650.0, 1.0),
        (820.0, -1500.0, 1.25),
        (430.0, -1250.0, 0.8),
        (1350.0, -1350.0, 1.1),
        (-350.0, -1350.0, 1.0),
        (1900.0, -1700.0, 1.3),
        (2500.0, -1300.0, 0.9),
        (1100.0, -2100.0, 1.4),
        (300.0, -2300.0, 1.2),
    ];
    for (i, (cx, cy, sc)) in clouds.iter().enumerate() {
        let drift = t * (6.0 + i as f32 * 1.5);
        let x = (cx + drift - cam.x * 0.6) * cam.z * 0.75 + 540.0;
        let y = (cy - cam.y * 0.6) * cam.z * 0.75 + 960.0;
        let k = sc * cam.z.powf(0.5) * 1.1;
        if y < -300.0 || y > hs + 200.0 || x < -500.0 || x > 1600.0 {
            continue;
        }
        w!(s, r#"<g transform="translate({x:.1} {y:.1}) scale({k:.3})" opacity="0.9">"#);
        for (dx, dy, rx, ry) in [
            (0.0, 0.0, 170.0, 70.0),
            (-110.0, 18.0, 120.0, 52.0),
            (115.0, 15.0, 130.0, 55.0),
            (30.0, -40.0, 110.0, 70.0),
            (-40.0, -25.0, 90.0, 55.0),
        ] {
            w!(s, r#"<ellipse cx="{dx}" cy="{dy}" rx="{rx}" ry="{ry}" fill="url(#cloud)"/>"#);
        }
        s.push_str("</g>");
    }
}

/// Hills, trees and the fence behind the lawn. `garden` fades the big house (first act only).
pub fn background(s: &mut String, garden: f32, t: f32) {
    // Distant hills.
    w!(
        s,
        r#"<path d="M{XL},{:.0} L{} L{XR},{:.0} Z" fill="url(#hill)"/>"#,
        LAWN_BACK + 20.0,
        wavy(LAWN_BACK - 120.0, XL, XR, 45.0, 1.3),
        LAWN_BACK + 20.0
    );
    w!(
        s,
        r#"<rect x="{XL}" y="{:.0}" width="{}" height="200" fill="url(#haze)"/>"#,
        LAWN_BACK - 200.0,
        XR - XL
    );
    // Back row of trees.
    let trees = [
        (-700.0, 1.0, 1u64),
        (-320.0, 1.15, 2),
        (110.0, 1.05, 3),
        (960.0, 1.1, 4),
        (1420.0, 0.95, 5),
        (1700.0, 1.2, 6),
        (2330.0, 1.1, 7),
        (2700.0, 1.0, 8),
        (3100.0, 1.2, 9),
    ];
    for (x, sc, seed) in trees {
        props::tree(s, x, LAWN_BACK - 10.0, sc, seed, t);
    }
    if garden > 0.01 {
        props::big_house(s, garden);
    }
    // Fence along the back of the lawn.
    w!(
        s,
        r##"<rect x="{XL}" y="{:.0}" width="{}" height="9" fill="#8a5a30"/><rect x="{XL}" y="{:.0}" width="{}" height="9" fill="#8a5a30"/>"##,
        LAWN_BACK - 72.0,
        XR - XL,
        LAWN_BACK - 30.0,
        XR - XL
    );
    w!(
        s,
        r#"<rect x="{XL}" y="{:.0}" width="{}" height="100" fill="url(#pFence)"/>"#,
        LAWN_BACK - 100.0,
        XR - XL
    );
}

/// Lawn in light and dark mowing stripes converging to the camera's vanishing point.
pub fn lawn(s: &mut String, cam: Cam) {
    let vp_x = cam.x;
    let vp_y = -1700.0;
    let k = 1.0 - (-LAWN_BACK) / (-vp_y);
    w!(
        s,
        r##"<rect x="{XL}" y="{LAWN_BACK}" width="{}" height="{}" fill="#4a9124"/>"##,
        XR - XL,
        -LAWN_BACK + 1.0
    );
    let step = 170.0;
    let mut x = (vp_x / step).floor() * step - step * 30.0 + 85.0;
    let mut i = ((vp_x / step).floor() as i64).rem_euclid(2);
    s.push_str(r##"<path fill="#5aa52a" d=""##);
    let mut dark = String::from(r##"<path fill="#3f8520" d=""##);
    while x < vp_x + step * 30.0 {
        let x2 = x + step;
        let b1 = vp_x + (x - vp_x) * k;
        let b2 = vp_x + (x2 - vp_x) * k;
        let seg = format!("M{x:.1},0 L{x2:.1},0 L{b2:.1},{LAWN_BACK} L{b1:.1},{LAWN_BACK} Z ");
        if i % 2 == 0 {
            s.push_str(&seg);
        } else {
            dark.push_str(&seg);
        }
        i += 1;
        x = x2;
    }
    s.push_str(r#""/>"#);
    s.push_str(&dark);
    s.push_str(r#""/>"#);
    w!(
        s,
        r#"<rect x="{XL}" y="{LAWN_BACK}" width="{}" height="{}" fill="url(#lawnShade)"/>"#,
        XR - XL,
        -LAWN_BACK
    );
}

/// The vertical cut through the ground: soil, gravel, shallow aquifer, clay, deep aquifer, rock.
pub fn face(s: &mut String) {
    let layers: [(f32, &str, &str, &str, f32); 6] = [
        (L_SOIL, "url(#pSoil)", "#8a5226", "#5e3313", 0.0),
        (L_GRAVEL, "url(#pGravel)", "#8b8781", "#5c5853", 6.0),
        (L_AQ1, "url(#pAq1)", "#4a6fd6", "#2f4fa8", 6.0),
        (L_CLAY, "url(#pClay)", "#8a8a88", "#6c6c6a", 8.0),
        (L_AQ2, "url(#pAq2)", "#6aa6cc", "#4f88b0", 10.0),
        (L_ROCK, "url(#pRock)", "#333438", "#202124", 12.0),
    ];
    for (i, (y, fill, base, edge, amp)) in layers.iter().enumerate() {
        let pts = wavy(*y, XL, XR, *amp, i as f32 * 1.7);
        // The flat base colour hides hairline seams between pattern tiles.
        w!(
            s,
            r#"<path d="M{XL},{y} L{pts} L{XR},{L_BOTTOM} L{XL},{L_BOTTOM} Z" fill="{base}"/><path d="M{XL},{y} L{pts} L{XR},{L_BOTTOM} L{XL},{L_BOTTOM} Z" fill="{fill}"/>"#
        );
        if i > 0 {
            w!(
                s,
                r#"<path d="M{pts}" fill="none" stroke="{edge}" stroke-width="5" opacity="0.55"/>"#
            );
            // Soft shadow just under each boundary gives the layers some depth.
            w!(
                s,
                r#"<path d="M{pts}" fill="none" stroke="{edge}" stroke-width="22" opacity="0.12" transform="translate(0 9)"/>"#
            );
        }
    }
    // Sod band and grass blades along the front edge of the lawn.
    w!(s, r##"<rect x="{XL}" y="0" width="{}" height="{L_SOIL}" fill="#2f6a17"/>"##, XR - XL);
    w!(s, r#"<rect x="{XL}" y="{L_SOIL}" width="{}" height="70" fill="url(#faceTop)"/>"#, XR - XL);
    w!(s, r#"<rect x="{XL}" y="-26" width="{}" height="26" fill="url(#pBlades)"/>"#, XR - XL);
}
