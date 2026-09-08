mod audio;

slint::include_modules!();

// HUGE diff here, as I prepare to move into Slint. I just don't think that iced is
// necessarily what I want to use for this project, and slint may be better if I want to use
// some C.
fn main() -> Result<(), slint::PlatformError> {
    let main_window = MainWindow::new()?;

    main_window.run()
}
