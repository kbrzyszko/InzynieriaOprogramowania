//! "How a hole in your yard becomes running water" – a cut-away animation drawn as SVG and
//! rendered with fframes. 1080×1920, 60 fps, 66.5 s, no text.

mod defs;
mod ground;
mod house;
mod props;
mod story;
mod util;
mod wells;

use fframes::pix_fmt::YuvMatrix;
use fframes::{AudioMap, Duration, EncoderOptions, FFramesContext, Frame, RenderOptions, Svgr, Video};
use util::Write;

struct WellVideo {
    defs: String,
}

impl Video for WellVideo {
    const FPS: usize = 60;
    const WIDTH: usize = 1080;
    const HEIGHT: usize = 1920;

    fn duration(&self) -> Duration<'_> {
        Duration::Seconds(66.5)
    }

    fn audio(&self) -> AudioMap<'_> {
        AudioMap::none()
    }

    fn render_frame<'a>(&'a self, frame: Frame, _ctx: &FFramesContext<'a, '_>) -> Svgr<'a> {
        let t = frame.seconds();
        let cam = story::camera(t);
        let mut s = String::with_capacity(400_000);
        s.push_str(r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1080 1920" width="1080" height="1920">"#);
        s.push_str(&self.defs);
        ground::sky(&mut s, cam, t);
        w!(
            s,
            r#"<g transform="translate(540 960) scale({:.5}) translate({:.3} {:.3})">"#,
            cam.z,
            -cam.x,
            -cam.y
        );
        story::world(&mut s, t, cam);
        s.push_str("</g></svg>");
        s.into()
    }
}

fn main() -> std::process::ExitCode {
    let video = WellVideo { defs: defs::build() };
    let params: &[(&str, &str)] = &[("crf", "18"), ("preset", "medium"), ("tune", "animation")];
    let options = RenderOptions {
        video_encoder_options: EncoderOptions {
            preferred_encoder: Some("libx264"),
            color_matrix: YuvMatrix::Bt709,
            codec_params: Some(params),
            gop_size: 60,
            ..Default::default()
        },
        ..Default::default()
    };
    fframes::cli::new(&video, options).run()
}
