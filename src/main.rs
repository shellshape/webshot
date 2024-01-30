mod config;
use anyhow::Result;
use clap::{command, Parser};
use config::Config;
use console::{style, Term};
use headless_chrome::{
    protocol::cdp::{
        Page::{CaptureScreenshotFormatOption, Viewport},
        Target::CreateTarget,
    },
    Browser,
};
use std::{
    fs,
    io::{self, Write},
    path::{Path, PathBuf},
};

/// Simply screenshot websites from your terminal
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Path to a config file
    #[arg(short, long)]
    config: Option<String>,

    /// URL of the web page to be captured
    url: String,

    /// Output directory or file name
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// Screen width
    #[arg(short = 'W', long)]
    width: Option<u32>,

    /// Screen height
    #[arg(short = 'H', long)]
    height: Option<u32>,

    /// Scale factor
    #[arg(short, long)]
    scale: Option<f64>,

    /// Wait for DOM element (query selector)
    #[arg(long, default_value = "body")]
    wait_for: String,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let cfg = Config::parse(cli.config)?;

    let term = Term::stdout();

    let orig_width = cli.width.or(cfg.default_width).unwrap_or(1920);
    let orig_height = cli.height.or(cfg.default_height).unwrap_or(1080);
    let scale = cli.scale.or(cfg.default_scale).unwrap_or(1.0);

    let width = orig_width as f64 / scale;
    let height = orig_height as f64 / scale;

    write_line(&term, &style("Open browser ...").italic().to_string());

    let browser = Browser::default()?;

    write_line(&term, &style("Open new tab ...").italic().to_string());

    let tab = browser.new_tab_with_options(CreateTarget {
        url: cli.url.clone(),
        width: Some(width as u32),
        height: Some(height as u32),
        browser_context_id: None,
        enable_begin_frame_control: None,
        new_window: None,
        background: None,
    })?;

    write_line(
        &term,
        &style(format!(
            "Waiting for element '{}' ...",
            style(&cli.wait_for).cyan()
        ))
        .italic()
        .to_string(),
    );

    tab.wait_for_element(&cli.wait_for)?;

    let viewport = Viewport {
        x: 0.0,
        y: 0.0,
        width,
        height,
        scale,
    };

    let output_path = match cli.output {
        Some(p) if p.is_dir() => p.join(default_file_name(&cli.url, orig_width, orig_height)),
        Some(p) => p,
        _ => default_file_name(&cli.url, orig_width, orig_height),
    };

    write_line(
        &term,
        &style("Capturing screenshot ...").italic().to_string(),
    );

    let image_data = tab.capture_screenshot(
        image_type_from_file_name(&output_path),
        None,
        Some(viewport),
        true,
    )?;

    write_line(
        &term,
        &style(format!(
            "Writing screenshot to '{}' ...",
            style(&output_path.to_string_lossy()).cyan()
        ))
        .italic()
        .to_string(),
    );

    fs::write(&output_path, image_data)?;

    write_line(
        &term,
        &style(format!(
            "Successfully saved screenshot to '{}'.\n",
            style(&output_path.to_string_lossy()).cyan()
        ))
        .green()
        .to_string(),
    );

    Ok(())
}

fn image_type_from_file_name<P: AsRef<Path>>(path: P) -> CaptureScreenshotFormatOption {
    match path.as_ref().extension().and_then(|ext| ext.to_str()) {
        Some("png") => CaptureScreenshotFormatOption::Png,
        Some("jpg") | Some("jpeg") => CaptureScreenshotFormatOption::Jpeg,
        Some("webp") => CaptureScreenshotFormatOption::Webp,
        None | Some(_) => CaptureScreenshotFormatOption::Png,
    }
}

fn canonicalize_url(url: &str) -> &str {
    let mut url = url
        .strip_prefix("https://")
        .or_else(|| url.strip_prefix("http://"))
        .unwrap_or(url);

    if let Some(i) = url.find('/') {
        url = &url[..i];
    }

    url
}

fn default_file_name(url: &str, width: u32, height: u32) -> PathBuf {
    format!("{}-{width}x{height}.png", canonicalize_url(url)).into()
}

fn write_line(term: &Term, v: &str) {
    _ = term.clear_line();
    print!("{v}");
    _ = io::stdout().flush();
}
