use color_eyre::eyre::Result;
use crossterm::terminal;
use rascii_art::{render_image_to, RenderOptions};

const GOOSE_IMAGE: &[u8] = include_bytes!("../goose.jpg");
const FALLBACK_WIDTH: u32 = 60;
const FALLBACK_HEIGHT: u32 = 24;

pub fn goose() -> Result<()> {
    let (width, height) = terminal::size()
        .map(|(cols, rows)| {
            let w = (cols.saturating_sub(1) as u32).max(20);
            let h = (rows.saturating_sub(2) as u32).max(10);
            (w, h)
        })
        .unwrap_or((FALLBACK_WIDTH, FALLBACK_HEIGHT));

    let mut buffer = String::new();
    let image = image::load_from_memory(GOOSE_IMAGE)?;
    render_image_to(
        &image,
        &mut buffer,
        &RenderOptions::new()
            .width(width)
            .height(height)
            .colored(true)
            .charset(&[".", ",", "-", "*", "£", "$", "#"]),
    )?;
    print!("{buffer}");
    print!("\x1b[0m\nstop goosin around\n");
    Ok(())
}
