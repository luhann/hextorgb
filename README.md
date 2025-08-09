# 🎨 HexToRGB

A fast and flexible command-line tool and Rust library for converting hex color codes to RGB values.

[![Rust](https://github.com/luhann/hextorgb/workflows/CI/badge.svg)](https://github.com/luhann/hextorgb/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## ✨ Features

- 🚀 **Blazing fast** - Optimized with LTO and minimal allocations
- 🎯 **Multiple input formats** - Support for `#FF0000`, `0xAABBCC`, `FFAABBCC`
- 🌈 **Alpha channel support** - Handle 8-digit hex codes with transparency
- 📋 **Multiple output formats** - Standard, CSS, JSON, Hex, and Compact
- 🖥️ **Terminal color preview** - See colors directly in your terminal
- ⚡ **Interactive mode** - REPL-style interface for multiple conversions
- 📊 **Built-in benchmarking** - Performance testing included
- 📚 **Library support** - Use as a dependency in your Rust projects

## 🚀 Installation

### From Source
```bash
git clone https://github.com/luhann/hextorgb.git
cd hextorgb
cargo install --path . --features cli
```

### From GitHub (Latest)
```bash
cargo install --git https://github.com/luhann/hextorgb --features cli
```

### Library Only
```toml
[dependencies]
hextorgb = { git = "https://github.com/luhann/hextorgb" }
# No CLI dependencies included - zero external deps!
```

## 📖 Usage

### Command Line Interface

#### Basic conversion
```bash
# Different input formats
hextorgb "#FF0000"          # RGB(255, 0, 0)
hextorgb "0x00FF00"         # RGB(0, 255, 0)
hextorgb "0000FF"           # RGB(0, 0, 255)

# With alpha channel
hextorgb "#FF0000AA"        # RGB(255, 0, 0, 0.67)
```

#### Output formats
```bash
# CSS format
hextorgb "#FF0000" --format css     # rgb(255, 0, 0)

# JSON format  
hextorgb "#FF0000" --format json    # {"r": 255, "g": 0, "b": 0}

# Hex format (normalize)
hextorgb "ff0000" --format hex      # #FF0000

# Compact format
hextorgb "#FF0000" --format compact # 255,0,0
```

#### Interactive mode
```bash
hextorgb --interactive
```
```
🎨 Hex to RGB Converter - Interactive Mode
Enter hex colors (type 'quit' to exit)
hex> #FF0000
  RGB(255, 0, 0)
hex> 0x00FF00  
  RGB(0, 255, 0)
hex> quit
Goodbye! 👋
```

#### Color preview
```bash
hextorgb "#FF0000" --preview        # Shows color block in terminal
```

#### Performance benchmark
```bash
hextorgb --benchmark
```
```
🚀 Running Performance Benchmark...
  4000000 conversions in 267.599371ms
  66ns per conversion
  14947718 conversions/sec
```

### Help
```bash
hextorgb --help
```

## 📚 Library Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
hextorgb = { git = "https://github.com/luhann/hextorgb" }
# Zero external dependencies for library usage!
```

### Basic usage
```rust
use hextorgb::{hextorgb, parse_hex};

fn main() {
    // Simple conversion
    let result = hextorgb("#FF0000");
    println!("{}", result); // RGB(255, 0, 0)
    
    // Parse hex to raw values
    let (rgb, alpha) = parse_hex("#FF0000AA").unwrap();
    println!("RGB: {:?}, Alpha: {:?}", rgb, alpha); // RGB: [255, 0, 0], Alpha: Some(170)

}
```

### Advanced usage
```rust
use hextorgb::{RGB, parse_hex};

fn process_color(hex: &str) -> Result<RGB, String> {
    let (rgb, alpha) = parse_hex(hex).map_err(|e| e.to_string())?;
    
    Ok(RGB {
        r: rgb[0],
        g: rgb[1], 
        b: rgb[2],
        a: alpha.map(|a| a as f64 / 255.0).unwrap_or(1.0),
    })
}
```

## 🎯 Supported Input Formats

| Format | Example | Description |
|--------|---------|-------------|
| Hash prefix | `#FF0000` | Standard web format |
| Hex prefix | `0xFF0000` | Programming style |
| Raw hex | `FF0000` | No prefix |
| With alpha | `#FF0000AA` | 8-digit with transparency |
| Case insensitive | `#ff0000` | Lowercase works too |

## 📤 Output Formats

| Format | Example | Use Case |
|--------|---------|----------|
| Standard | `RGB(255, 0, 0)` | Human readable |
| CSS | `rgb(255, 0, 0)` | Web development |
| JSON | `{"r": 255, "g": 0, "b": 0}` | APIs and data exchange |
| Hex | `#FF0000` | Normalize format |
| Compact | `255,0,0` | CSV or minimal output |

## ⚡ Performance

Typical performance: **~60ns per conversion** on modern hardware.

## 🧪 Development

### Building
```bash
git clone https://github.com/luhann/hextorgb.git
cd hextorgb

# Build library only
cargo build --release

# Build CLI tool
cargo build --release --features cli

# Run CLI tool
cargo run --features cli -- "#FF0000"
```

### Testing
```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run integration tests
cargo test --test conversions

# Test CLI features
cargo test --features cli
```

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Built with [clap](https://github.com/clap-rs/clap) for CLI argument parsing
- Colored output powered by [colored](https://github.com/mackwic/colored)
- Inspired by the need for fast, reliable color conversion tools

---

<p align="center">Made with ❤️ and 🦀 Rust</p>