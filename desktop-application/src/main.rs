/// Designing the UI
use druid::widget::{Label, Button, Flex};
use druid::{AppLauncher, WindowDesc, Widget, PlatformError};
use druid::WidgetExt;

/// Function to build UI using druid's widgets
fn build_ui() -> impl Widget<()> {
    let mut col = Flex::column();

    col.add_child(Label::new("Hello, Rust World!").center());
    col.add_child(Button::new("Click me!").on_click(|_ctx, _data, _env| {
        println!("Button clicked!");
    }).center());

    col
}

fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(build_ui())
        .title("A simple Rust Desktop App")
        .window_size((400.0, 400.0));

    AppLauncher::with_window(main_window)
        .launch(())?;
    Ok(())
}