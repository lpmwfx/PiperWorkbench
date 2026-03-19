# PiperWorkbench

A professional linguistic development workbench for Piper TTS (text-to-speech) voice modeling and processing.

## Features (Planned)

- **Voice Processing**: TTS engine integration with real-time audio playback
- **Audio Analysis**: Spectrum analyzer, waveform visualization, phoneme timeline
- **Equalization**: 3-band/parametric EQ with visual feedback
- **Synthesis**: Oscillator waveforms, envelope control (ADSR)
- **Modulation**: LFO, tremolo, vibrato effects
- **Presets**: Save/load voice configurations

## Quick Start

### Prerequisites

- Rust 1.70+ ([install](https://rustup.rs/))
- Slint UI framework (pulled via Cargo)

### Build

```bash
cargo build --release
```

### Run

```bash
cargo run
```

## Project Structure

```
PiperWorkbench/
├── src/           # Rust source code
├── ui/            # Slint UI definitions
├── proj/          # Project metadata (PROJECT, TODO, RULES, etc.)
├── tools/         # Build scripts and utilities
├── Cargo.toml     # Rust dependencies
└── build.rs       # Build configuration
```

## Development

### Current Phase

Phase 1: Setup & Architecture

### Architecture

- **Audio Pipeline**: Modular audio processing with trait-based design
- **UI Framework**: Slint (modern, lightweight, cross-platform)
- **Integration**: Rust core with C++/Python for specialized algorithms

See `proj/PROJECT` for phases and roadmap.

## Documentation

- `proj/PROJECT` — Project state and phases
- `proj/UIUX` — UI/UX guidelines
- `proj/TODO` — Current tasks
- `proj/RULES` — Active coding rules

## License

TBD

## Contact

- Lead: mathi


---

<!-- LARS:START -->
<a href="https://lpmathiasen.com">
  <img src="https://carousel.lpmathiasen.com/carousel.svg?slot=5" alt="Lars P. Mathiasen"/>
</a>
<!-- LARS:END -->
