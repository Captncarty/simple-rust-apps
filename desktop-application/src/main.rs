// Imports
use ansi_term::Colour;
use std::thread;
use std::time::Duration;

/// Designing the UI
use druid::widget::{Label, Button, Flex};
use druid::{AppLauncher, WindowDesc, Widget, PlatformError};
use druid::WidgetExt;


//Drawing a Ferris
fn draw() {
    let ferris: String = Colour::Red.paint(r"
    .
     .
      .
       █ █           █ █
        ▀█  ▄█████▄  █▀
         ▀▄███▀█▀███▄▀
         ▄▀███▀▀▀███▀▄
         █ ▄▀▀▀▀▀▀▀▄ █
    ").to_string();
    println!("{}", ferris);
}

/// Function to build UI using druid's widgets
fn build_ui() -> impl Widget<()> {
    let mut col = Flex::column();

    col.add_child(Label::new("Draw a Ferris!").center());
    col.add_child(Button::new("Click to Draw!").on_click(|_ctx, _data, _env| {
        draw();
        thread::sleep(Duration::from_millis(2000));
    }).center());

    col
}

fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(build_ui())
        .title("A simple Rust Desktop App")
        .window_size((800.0, 400.0));

    AppLauncher::with_window(main_window)
        .launch(())?;
    Ok(())
}