//! PiperWorkbench - Piper linguistic development tool
//! Main entry point for the GUI application

slint::include_modules!();

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ui = AppWindow::new()?;
    ui.run()?;
    Ok(())
}
