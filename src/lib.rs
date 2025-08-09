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
    let clean = if hex.starts_with("0x") || hex.starts_with("0X") {
        &hex[2..]
    } else if hex.starts_with('#') {
        &hex[1..]
    } else {
        hex
    };

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

/// Converts a hex color string to RGB format
/// 
/// # Examples
/// ```
/// use hextorgb::hextorgb;
/// assert_eq!(hextorgb("#FF0000"), "RGB(255, 0, 0)");
/// ```
pub fn hextorgb(hex: &str) -> String {
    let parsed = match parse_hex(&hex) {
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
    };

    parsed
}