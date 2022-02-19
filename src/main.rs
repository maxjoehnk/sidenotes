use crate::jobs::{ConfigLoadJob, SyncTimerJob};
use druid::{AppLauncher, Color, WindowDesc};

use crate::ui::theme::*;
use crate::ui::SidenotesDelegate;

mod calendar;
pub mod config;
mod jobs;
mod models;
mod providers;
pub(crate) mod rich_text;
mod ui;

fn main() -> anyhow::Result<()> {
    let window = WindowDesc::new(ui::ui_builder()).title("Sidenotes");
    let launcher = AppLauncher::with_window(window)
        .delegate(SidenotesDelegate::default())
        .log_to_console()
        .configure_env(|env, _| {
            env.set(LINK_COLOR, Color::rgb8(94, 129, 172));
            env.set(CARD_COLOR, Color::rgba8(0, 0, 0, 32));
            env.set(STATUS_COLOR, Color::rgb8(163, 190, 140));
        });

    let event_sink = launcher.get_external_handle();

    ConfigLoadJob::new(event_sink.clone()).run();
    SyncTimerJob::new(event_sink).run();

    launcher.launch(Default::default())?;

    Ok(())
}
