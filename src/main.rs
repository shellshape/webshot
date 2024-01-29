mod config;
use anyhow::Result;
use clap::{command, Parser};
use config::Config;
use headless_chrome::{
    protocol::cdp::{
        Page::{CaptureScreenshotFormatOption, Viewport},
        Target::CreateTarget,
    },
    Browser,
};
use std::{
    fs,
    path::{Path, PathBuf},
};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Path to a config file
    #[arg(short, long)]
    config: Option<String>,

    url: String,

    #[arg(short, long, default_value = "screenshot.png")]
    output: PathBuf,

    #[arg(short = 'W', long)]
    width: Option<u32>,

    #[arg(short = 'H', long)]
    height: Option<u32>,

    #[arg(short, long)]
    scale: Option<f64>,

    #[arg(long, default_value = "body")]
    wait_for: String,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let cfg = Config::parse(cli.config)?;

    let width = cli.width.or(cfg.default_width).unwrap_or(1920);
    let height = cli.width.or(cfg.default_height).unwrap_or(1089);
    let scale = cli.scale.or(cfg.default_scale).unwrap_or(1.0);

    let width = width as f64 / scale;
    let height = height as f64 / scale;

    let browser = Browser::default()?;

    let tab = browser.new_tab_with_options(CreateTarget {
        url: cli.url,
        width: Some(width as u32),
        height: Some(height as u32),
        browser_context_id: None,
        enable_begin_frame_control: None,
        new_window: None,
        background: None,
    })?;

    tab.wait_for_element(&cli.wait_for)?;

    let viewport = Viewport {
        x: 0.0,
        y: 0.0,
        width,
        height,
        scale,
    };

    let image_data = tab.capture_screenshot(
        image_type_from_file_name(&cli.output),
        None,
        Some(viewport),
        true,
    )?;

    fs::write(&cli.output, image_data)?;

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
