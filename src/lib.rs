pub enum Color {
    Rgb { r: u8, g: u8, b: u8 },
    Rgba { r: u8, g: u8, b: u8, a: f64 },
}

impl Color {
    pub fn to_string(&self) -> String {
        match self {
            Color::Rgb { r, g, b } => {
                format!("RGB({}, {}, {})", r, g, b)
            }
            Color::Rgba { r, g, b, a } => {
                format!("RGBA({}, {}, {}, {:.2})", r, g, b, a)
            }
        }
    }
}

pub fn parse_hex(hex: &str) -> Result<([u8; 3], Option<u8>), &'static str> {
    let clean = hex
        .strip_prefix("0x")
        .or_else(|| hex.strip_prefix("0X"))
        .or_else(|| hex.strip_prefix("#"))
        .unwrap_or(hex);

    match clean.len() {
        6 => {
            let r = u8::from_str_radix(&clean[0..2], 16).map_err(|_| "Invalid hex")?;
            let g = u8::from_str_radix(&clean[2..4], 16).map_err(|_| "Invalid hex")?;
            let b = u8::from_str_radix(&clean[4..6], 16).map_err(|_| "Invalid hex")?;
            Ok(([r, g, b], None))
        }
        8 => {
            let r = u8::from_str_radix(&clean[0..2], 16).map_err(|_| "Invalid hex")?;
            let g = u8::from_str_radix(&clean[2..4], 16).map_err(|_| "Invalid hex")?;
            let b = u8::from_str_radix(&clean[4..6], 16).map_err(|_| "Invalid hex")?;
            let a = u8::from_str_radix(&clean[6..8], 16).map_err(|_| "Invalid hex")?;
            Ok(([r, g, b], Some(a)))
        }
        _ => Err("Invalid hex length"),
    }
}

// Core conversion function - zero dependencies
pub fn convert_hex_to_format(hex: &str, format: &str) -> Result<String, String> {
    let (rgb, alpha) = parse_hex(hex).map_err(|e| e.to_string())?;

    let converted = match alpha {
        Some(a) => Color::Rgba {
            r: (rgb[0]),
            g: (rgb[1]),
            b: (rgb[2]),
            a: (a as f64 / 255.0),
        },
        None => Color::Rgb {
            r: (rgb[0]),
            g: (rgb[1]),
            b: (rgb[2]),
        },
    };

    let output = match format {
        "standard" => converted.to_string(),
        "css" => match &converted {
            Color::Rgba { r, g, b, a } => format!("rgba({}, {}, {}, {:.2})", r, g, b, a),
            Color::Rgb { r, g, b } => format!("rgb({}, {}, {})", r, g, b),
        },
        "json" => match &converted {
            Color::Rgba { r, g, b, a } => {
                format!(r#"{{"r": {}, "g": {}, "b": {}, "a": {:.2}}}"#, r, g, b, a)
            }
            Color::Rgb { r, g, b } => format!(r#"{{"r": {}, "g": {}, "b": {}}}"#, r, g, b),
        },
        "hex" => match &converted {
            Color::Rgba { r, g, b, a } => {
                format!("#{:02X}{:02X}{:02X}{:02X}", r, g, b, (*a * 255.0) as u8)
            }
            Color::Rgb { r, g, b } => format!("#{:02X}{:02X}{:02X}", r, g, b),
        },
        "compact" => match &converted {
            Color::Rgba { r, g, b, a } => format!("{},{},{},{:.2}", r, g, b, a),
            Color::Rgb { r, g, b } => format!("{},{},{}", r, g, b),
        },
        _ => return Err(format!("Unknown format: {}", format)),
    };

    Ok(output)
}

// CLI-specific wrapper with enum and preview support
#[cfg(feature = "cli")]
pub use clap::ValueEnum;

#[cfg(feature = "cli")]
#[derive(Clone, clap::ValueEnum)]
pub enum OutputFormat {
    Standard,
    Css,
    Json,
    Hex,
    Compact,
}

#[cfg(feature = "cli")]
impl OutputFormat {
    pub fn as_str(&self) -> &'static str {
        match self {
            OutputFormat::Standard => "standard",
            OutputFormat::Css => "css",
            OutputFormat::Json => "json",
            OutputFormat::Hex => "hex",
            OutputFormat::Compact => "compact",
        }
    }
}

#[cfg(feature = "cli")]
pub fn convert_with_format(
    hex: &str,
    format: &OutputFormat,
    show_preview: bool,
) -> Result<String, String> {
    use colored::*;

    let mut output = convert_hex_to_format(hex, format.as_str())?;

    if show_preview {
        let (rgb, _) = parse_hex(hex).map_err(|e| e.to_string())?;
        let preview = "   ".on_truecolor(rgb[0], rgb[1], rgb[2]);
        output = format!("{} {}", preview, output);
    }

    Ok(output)
}

/// Converts a hex color string to RGB format
///
/// # Examples
/// ```
/// use hextorgb::hextorgb;
/// assert_eq!(hextorgb("#FF0000"), "RGB(255, 0, 0)");
/// ```
pub fn hextorgb(hex: &str) -> String {
    match parse_hex(hex) {
        Ok((rgb, alpha)) => {
            let color = match alpha {
                Some(a) => Color::Rgba {
                    r: rgb[0],
                    g: rgb[1],
                    b: rgb[2],
                    a: a as f64 / 255.0,
                },
                None => Color::Rgb {
                    r: rgb[0],
                    g: rgb[1],
                    b: rgb[2],
                },
            };
            color.to_string()
        }
        Err(e) => e.to_string(),
    }
}
