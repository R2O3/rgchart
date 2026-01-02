# rgchart
A library for parsing and writing charts for various rhythm games. It supports cross-platform usage including Web and Node.js environments via WebAssembly (WASM).

## Table of Contents

- [Rust Usage](#rust-usage)
    - [Installation](#installation)
    - [API Reference](#api-reference)
        - [Parsing Charts](#parsing-charts)
        - [Writing Charts](#writing-charts)
        - [Chart Structure](#chart-structure)
- [JavaScript/TypeScript Usage](#javascripttypescript-usage)
    - [Installation](#installation-1)
    - [API Reference](#api-reference-1)
        - [Initialization](#initialization)
        - [Parsing Charts](#parsing-charts-1)
        - [Writing Charts](#writing-charts-1)
        - [TypeScript Types](#typescript-types)
- [Building](#building)
    - [Rust Library](#rust-library)
    - [WASM Bindings](#wasm-bindings)
- [License](#license)

## Rust Usage

### Installation
Add this to your `Cargo.toml`:
```toml
[dependencies]
rgchart = "0.0.11"
```

Or run:
```sh
cargo add rgchart
```

### API Reference

#### Parsing Charts
```rust
use rgchart::parse;

// Parse an osu! chart from string to a generic mania chart
let osu_chart = parse::from_osu_generic(raw_osu_string).expect("Failed to parse osu! chart");

// Parse a Stepmania chart from string to a generic mania chart
let sm_chart = parse::from_sm_generic(raw_sm_string).expect("Failed to parse Stepmania chart");

// Parse a Quaver chart from string to a generic mania chart
let qua_chart = parse::from_qua_generic(raw_qua_string).expect("Failed to parse Quaver chart");

// Parse a fluXis chart from string to a generic mania chart
let fsc_chart = parse::from_fsc_generic(raw_qua_string).expect("Failed to parse fluXis chart");
```

to parse charts in their original structures:
```rust
use rgchart::FscFile;
use rgchart::OsuFile;
use rgchart::QuaFile;

// Parse an osu! chart from string
let osu_chart = OsuFile::from_str(raw_osu_string).expect("Failed to parse osu! chart");

// Parse a Quaver chart from string
let qua_chart = QuaFile::from_str(raw_qua_string).expect("Failed to parse Quaver chart");

// Parse a fluXis chart from string
let fsc_chart = FscFile::from_str(raw_fsc_string).expect("Failed to parse fluXis chart");
```

#### Writing Charts
```rust
use rgchart::parse;
use rgchart::write;
use rgchart::GenericManiaChart;

let chart: GenericManiaChart = parse::from_osu_generic(raw_osu_string).expect("Failed to parse osu! chart");

// Write from generic mania chart to to osu! format
let osu_string = write::to_osu_generic(&chart);

// Write from generic mania chart to Stepmania format
let sm_string = write::to_sm_generic(&chart);

// Write from generic mania chart to Quaver format
let qua_string = write::to_qua_generic(&chart);

// Write from generic mania chart to fluXis format
let fsc_string = write::To_fsc_generic(&chart);
```

to write charts from their original structures:
```rust
use rgchart::FscFile;
use rgchart::OsuFile;
use rgchart::QuaFile;

// Write from OsuFile to to osu! format
let osu_string = osu_chart.to_osu_format_mania(soundbank);

// assuming you don't have a soundbank
let osu_string = osu_chart.to_osu_format_mania_no_soundbank();

// other modes for osu!, it will interprete the hit objects values as is for the mode you're writing to.
let osu_string = osu_chart.to_osu_format();
// or
let osu_string = osu_chart.to_osu_format_standard(soundbank);

let osu_string = osu_chart.to_osu_format_taiko(soundbank);
let osu_string = osu_chart.to_osu_format_catch(soundbank);

// Write from QuaFile to Quaver format
let qua_string = qua_chart.to_str().expect("Failed to write Quaver chart");

// Write from FscFile to fluXis format
let fsc_string = fsc_chart.to_str().expect("Failed to write fluXis chart");
```

as of now you can't parse/write Sm files in their original structures.

#### Generic Mania Chart Structure
The `GenericManiaChart` contains all the relevant chart information:
```rust
pub struct GenericManiaChart {
    pub metadata: Metadata,
    pub chartinfo: ChartInfo,
    pub timing_points: TimingPoints,
    pub hitobjects: HitObjects,
    pub soundbank: Option<SoundBank>,
}
```

The `Metadata` contains all the metadata related information about a specific chart, a lot of all of these can be empty:
```rust
pub struct Metadata {
    pub title: String,
    pub alt_title: String,
    pub artist: String,
    pub alt_artist: String,
    pub creator: String,
    pub genre: String,
    pub tags: Vec<String>,
    pub source: String,
}
```

The `ChartInfo` contains all the gameplay information about a specific chart:
```rust
pub struct ChartInfo {
    pub difficulty_name: String,
    pub od: f32,
    pub hp: f32,
    pub bg_path: String,
    pub video_path: String,
    pub song_path: String,
    pub audio_offset: i32,
    pub preview_time: i32,
    pub key_count: u8,
}
```

The `TimingPoints` contains all the timing information such as bpm changes and sv:
```rust
pub enum TimingChangeType {
    Bpm,
    Sv,
    Stop
}

pub struct TimingChange {
    pub change_type: TimingChangeType,
    pub value: f32,
}

pub struct TimingPoint {
    pub time: i32,
    pub beat: f32,
    pub change: TimingChange,
}

pub struct TimingPoints {
    pub points: Vec<TimingPoint>,
}
```

The `HitObjects` struct contains all the hitobject information:
```rust
pub struct HitObject {
    pub time: i32,
    pub beat: f32,
    pub keysound: KeySound,
    pub key: Key,
    pub lane: u8,
}

pub struct HitObjects {
    pub objects: Vec<HitObject>,
}
```

Here is how sounds are handled for Mania.
``SoundBank`` contains all the sounds effects as well as a lookup for samples, it's done this way to be compatible with Quaver.
```rust
pub enum HitSoundType {
    Normal,
    Clap,
    Whistle,
    Finish,
}

pub struct SoundEffect {
    pub time: i32,
    pub volume: u8,
    pub sample: usize,
}
pub struct KeySound {
    pub volume: u8,
    pub hitsound_type: HitSoundType,
    pub sample: Option<usize>,
    pub has_custom: bool,
}

pub struct SoundBank {
    pub audio_tracks: Vec<String>,
    sound_sample_paths: Vec<String>,
    pub sound_effects: Vec<SoundEffect>,
    sample_map: HashMap<String, usize>,
}
```

## JavaScript/TypeScript Usage

### Installation
For Node.js:
```sh
npm install @r2o3/rgchart-nodejs
```

For web projects:
```html
<script src="https://unpkg.com/@r2o3/rgchart-browser@latest/rgchart.js"></script>
```
or
```javascript
npm install @r2o3/rgchart-browser
```
then use as an ES module

### API Reference

#### Initialization
```javascript
// For ES modules
import * as rgchart from '@r2o3/rgchart'; // or if not on node use the path to rgchart.js

// or alternatively
const rgchart = await import('path/to/rgchart.js')

// For CommonJS
const rgchart = require('rgchart');
```

you may need to do ``await rgchart.default()`` after importing if you've imported it in a script tag (with type="module") or you get an error like ``Uncaught TypeError: Cannot read properties of undefined (reading '__wbindgen_malloc')``

As of now you can't parse/write using the original structures in JS/TS, will be supported in the *near* future.

#### Parsing Charts
```javascript
// Parse an osu! chart from string to a generic mania chart
const OsuChart = rgchart.parseFromOsuGeneric(rawOsuString);

// Parse a Stepmania chart from string to a generic mania chart
const SmChart = rgchart.parseFromSmGeneric(rawSmString);

// Parse a Quaver chart from string to a generic mania chart
const QuaChart = rgchart.parseFromQuaGeneric(rawQuaString);

// Parse a fluXis chart from string to a generic mania chart
const FscChart = rgchart.parseFromFscGeneric(rawFscString);
```

#### Writing Charts
```javascript
// write from generic mania chart to osu! format
const osuString = rgchart.writeToOsuGeneric(chart);

// write from generic mania chart to Stepmania format
const smString = rgchart.writeToSmGeneric(chart);

// write from generic mania chart to Quaver format
const quaString = rgchart.writeToQuaGeneric(chart);

// write from generic mania chart to fluXis format
const fscString = rgchart.writeToFscGeneric(chart);
```

#### TypeScript Types
The core chart library is written in Rust, but *most* types in the WASM bindings are generated for TypeScript.

[See Chart Structure](#chart-structure).
## Building

### Rust Library
```sh
cargo build
```

### WASM Bindings
1. Install wasm-pack:
```sh
cargo install wasm-pack
```
> [!IMPORTANT]  
> It's really recommended to have [wasm-opt](https://github.com/WebAssembly/binaryen) installed and added to path for the wasm build.

2. Build the package:
```sh
npm run build # debug build
npm run build-release # release build
```

3. This will build it for both node and browser and the output will be in `dist-web` and `dist-node` directory.

## License
RGC uses the MIT License for all its sibiling projects.
See [LICENSE](https://github.com/menvae/rgchart/blob/master/LICENSE) for more information
