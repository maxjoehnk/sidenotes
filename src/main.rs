use druid::{AppLauncher, Color, WindowDesc};

use crate::sync::SyncThread;
use crate::ui::theme::*;

mod calendar;
pub mod config;
mod models;
mod providers;
pub(crate) mod rich_text;
mod sync;
mod ui;

fn main() -> anyhow::Result<()> {
    let window = WindowDesc::new(ui::ui_builder()).title("Sidenotes");
    let launcher = AppLauncher::with_window(window)
        .log_to_console()
        .configure_env(|env, _| {
            env.set(LINK_COLOR, Color::rgb8(94, 129, 172));
            env.set(CARD_COLOR, Color::rgba8(0, 0, 0, 32));
            env.set(STATUS_COLOR, Color::rgb8(163, 190, 140));
            env.set(MEETING_TIME_COLOR, Color::rgba(1., 1., 1., 0.2));
            env.set(MEETING_JOIN_COLOR, Color::rgb8(163, 190, 140));
        });

    let event_sink = launcher.get_external_handle();

    SyncThread::new(event_sink).start();

    launcher.launch(Default::default())?;

    Ok(())
}
