use clap::Parser;
use colored::*;

#[derive(Parser)]
#[command(name = "hextorgb")]
#[command(about = "🎨 Convert hex color codes to RGB values")]
#[command(version = "0.5.0")]
#[command(author = "Luke Hannan")]
struct Args {
    /// Hex color code (e.g., #FF0000, 0xAABBCC, FFAABBCC)
    hex_color: Option<String>,

    /// Output format
    #[arg(short, long, value_enum, default_value_t = hextorgb::OutputFormat::Standard)]
    format: hextorgb::OutputFormat,

    /// Run performance benchmark
    #[arg(short, long)]
    benchmark: bool,

    /// Interactive mode
    #[arg(short, long)]
    interactive: bool,

    /// Show color preview (requires true color terminal)
    #[arg(short, long)]
    preview: bool,
}

fn run_interactive_mode() {
    println!(
        "{}",
        "🎨 Hex to RGB Converter - Interactive Mode"
            .bright_cyan()
            .bold()
    );
    println!("{}", "Enter hex colors (type 'quit' to exit)".dimmed());

    loop {
        print!("{} ", "hex>".bright_green().bold());
        std::io::Write::flush(&mut std::io::stdout()).unwrap();

        let mut input = String::new();
        if std::io::stdin().read_line(&mut input).is_err() {
            break;
        }

        let input = input.trim();
        if input.is_empty() {
            continue;
        }
        if input == "quit" || input == "exit" {
            println!("{}", "Goodbye! 👋".bright_cyan());
            break;
        }

        match hextorgb::convert_with_format(input, &hextorgb::OutputFormat::Standard, true) {
            Ok(result) => println!("  {}", result),
            Err(e) => println!("  {} {}", "Error:".red().bold(), e),
        }
    }
}

fn run_benchmark() {
    println!(
        "{}",
        "🚀 Running Performance Benchmark...".bright_yellow().bold()
    );

    let test_colors = vec!["#FF0000", "0x00FF00", "0000FF", "#FFAABBCC"];
    let iterations = 1_000_000;

    let start = std::time::Instant::now();
    for _ in 0..iterations {
        for color in &test_colors {
            let _ = hextorgb::hextorgb(color);
        }
    }
    let duration = start.elapsed();

    let total_conversions = iterations * test_colors.len();
    let avg_per_conversion = duration / total_conversions as u32;

    println!(
        "  {} conversions in {:?}",
        total_conversions.to_string().bright_white().bold(),
        duration
    );
    println!(
        "  {} per conversion",
        format!("{:?}", avg_per_conversion).bright_green()
    );
    println!(
        "  {} conversions/sec",
        ((total_conversions as f64 / duration.as_secs_f64()) as u64)
            .to_string()
            .bright_cyan()
            .bold()
    );
}

fn main() {
    let args = Args::parse();

    if args.benchmark {
        run_benchmark();
        return;
    }

    if args.interactive {
        run_interactive_mode();
        return;
    }

    let hex_color: String = match args.hex_color {
        Some(color) => color,
        None => {
            eprintln!(
                "{}",
                "Error: Please provide a hex color or use --interactive mode".red()
            );
            std::process::exit(1);
        }
    };

    match hextorgb::convert_with_format(hex_color.trim(), &args.format, args.preview) {
        Ok(result) => println!("{}", result),
        Err(e) => {
            eprintln!("{} {}", "Error:".red().bold(), e);
            std::process::exit(1);
        }
    }
}
