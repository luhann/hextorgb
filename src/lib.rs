pub struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: f64,
}

impl RGB {
    fn to_string(&self, format_type: &str) -> String {
        if format_type == "alpha" {
            format!("RGB({}, {}, {}, {:.2})", self.r, self.g, self.b, self.a)
        } else {
            format!("RGB({}, {}, {})", self.r, self.g, self.b)
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

    let converted = RGB {
        r: rgb[0],
        g: rgb[1],
        b: rgb[2],
        a: match alpha {
            Some(a) => a as f64 / 255.0,
            None => 1.0,
        },
    };

    let output = match format {
        "standard" => {
            if alpha.is_some() {
                format!(
                    "RGBA({}, {}, {}, {:.2})",
                    converted.r, converted.g, converted.b, converted.a
                )
            } else {
                format!("RGB({}, {}, {})", converted.r, converted.g, converted.b)
            }
        }
        "css" => {
            if alpha.is_some() {
                format!(
                    "rgba({}, {}, {}, {:.2})",
                    converted.r, converted.g, converted.b, converted.a
                )
            } else {
                format!("rgb({}, {}, {})", converted.r, converted.g, converted.b)
            }
        }
        "json" => {
            if alpha.is_some() {
                format!(
                    r#"{{"r": {}, "g": {}, "b": {}, "a": {:.2}}}"#,
                    converted.r, converted.g, converted.b, converted.a
                )
            } else {
                format!(
                    r#"{{"r": {}, "g": {}, "b": {}}}"#,
                    converted.r, converted.g, converted.b
                )
            }
        }
        "hex" => {
            if alpha.is_some() {
                format!(
                    "#{:02X}{:02X}{:02X}{:02X}",
                    converted.r,
                    converted.g,
                    converted.b,
                    (converted.a * 255.0) as u8
                )
            } else {
                format!("#{:02X}{:02X}{:02X}", converted.r, converted.g, converted.b)
            }
        }
        "compact" => {
            if alpha.is_some() {
                format!(
                    "{},{},{},{:.2}",
                    converted.r, converted.g, converted.b, converted.a
                )
            } else {
                format!("{},{},{}", converted.r, converted.g, converted.b)
            }
        }
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
            let converted = RGB {
                r: rgb[0],
                g: rgb[1],
                b: rgb[2],
                a: match alpha {
                    Some(a) => a as f64 / 255.0,
                    None => -1.0,
                },
            };

            if converted.a < 0.0 {
                converted.to_string("noalpha")
            } else {
                converted.to_string("alpha")
            }
        }
        Err(e) => e.to_string(),
    }
}
