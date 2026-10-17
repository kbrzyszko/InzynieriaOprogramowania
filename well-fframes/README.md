# well-fframes

A cut-away animation of a well, drawn entirely as SVG shapes in Rust and rendered with
[fframes](https://crates.io/crates/fframes): from digging a hole in the garden, through a stone well
and a hand pump, a drilled well with casing and a submersible pump, up to the water tower and the
kitchen tap. No text on screen.

- 1080×1920, 60 fps, 66.5 s, H.264, no audio
- result: `well-fframes.mp4`

## Structure

| file | contents |
| --- | --- |
| `src/main.rs` | `Video` implementation, encoder settings (libx264, CRF 18, BT.709) |
| `src/story.rs` | camera keyframes and the draw order of the world |
| `src/ground.rs` | sky, background, striped lawn, cut ground face (soil, gravel, aquifers, clay, rock) |
| `src/wells.rs` | act 1: dug well (0–21 s), act 2: drilled well (21–41 s) |
| `src/house.rs` | act 3: pipes, tank/water tower, kitchen, water heater (41–66.5 s) |
| `src/props.rs` | gnome, trees, house, pumps, buckets, tripod, faucet… |
| `src/defs.rs` | gradients and textures (`<pattern>`) built once at start-up |

## Build and render

### Windows

FFmpeg 9 (shared, GPL build with libx264), LLVM/libclang and the VS Build Tools have to be
installed. If the shell does not see the variables yet, set them in the session:

```powershell
$env:FFMPEG_DIR = "C:\Users\Kacper\tools\ffmpeg-n9.0-latest-win64-gpl-shared-9.0"
$env:LIBCLANG_PATH = "C:\Users\Kacper\tools\LLVM\bin"
$env:PATH = "$env:USERPROFILE\.cargo\bin;$env:FFMPEG_DIR\bin;$env:LIBCLANG_PATH;$env:PATH"

cargo build --release
.\target\release\well-fframes.exe render -o well-fframes.mp4
```

### Linux / macOS

FFmpeg is linked statically and libx264 is compiled in (`libx264-dev`, `nasm`, `clang`). When the
downloaded prebuilt FFmpeg does not match the system x264 version, force a source build:

```sh
FFMPEG_FORCE_BUILD=1 cargo build --release
./target/release/well-fframes render -o well-fframes.mp4
```

## Handy commands while iterating

```sh
# contact sheet at given times
well-fframes strip "0s,2s,4s,6s" --columns 4 --width 240 -o sheet.png
# single frames / a range at half resolution
well-fframes frame 18.5s -o frames
well-fframes render 40s..44s --scale 0.5 -o draft.mp4
```
