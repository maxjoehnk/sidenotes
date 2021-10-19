use druid::{AppLauncher, WindowDesc};

use crate::sync::SyncThread;

mod providers;
pub mod config;
mod models;
mod ui;
mod sync;
pub(crate) mod rich_text;

fn main() -> anyhow::Result<()> {
    let window = WindowDesc::new(ui::ui_builder()).title("Sidenotes");
    let launcher = AppLauncher::with_window(window).log_to_console();

    let event_sink = launcher.get_external_handle();

    SyncThread::new(event_sink).start();

    launcher.launch(Default::default())?;

    Ok(())
}
