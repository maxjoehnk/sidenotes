use crate::config::Config;
use crate::models::{AppState, Navigation};
use crate::providers::{ProviderConfig, ProviderConfigEntry, ProviderId};
use crate::ui::commands;
use crate::ui::prism::ProviderConfigPrism;
use crate::ui::widgets::{button_builder, header_builder};
use druid::widget::*;
use druid::{Color, Command, Data, Insets, Target, Widget};
use druid_widget_nursery::enum_switcher::Switcher;

#[cfg(feature = "confluence")]
pub mod confluence;
#[cfg(feature = "github")]
pub mod github;
#[cfg(feature = "gitlab")]
pub mod gitlab;
#[cfg(feature = "jira")]
pub mod jira;
#[cfg(feature = "joplin")]
pub mod joplin;
#[cfg(feature = "nextcloud")]
pub mod nextcloud;
#[cfg(feature = "taskwarrior")]
pub mod taskwarrior;
#[cfg(feature = "upsource")]
pub mod upsource;

pub fn provider_settings_builder() -> impl Widget<AppState> {
    let header = header_builder("Providers");
    let add_btn = button_builder("Add Provider").on_click(|ctx: _, _, _: &_| {
        ctx.submit_command(Command::new(
            commands::NAVIGATE,
            Navigation::NewProvider,
            Target::Auto,
        ))
    });
    let provider_list = List::new(view_provider_builder)
        .with_spacing(8.0)
        .lens(Config::providers)
        .lens(AppState::config);
    let provider_list = Scroll::new(provider_list).vertical();

    Flex::column()
        .must_fill_main_axis(true)
        .with_child(header)
        .with_child(add_btn)
        .with_flex_child(provider_list, 1.0)
}

pub fn edit_provider() -> impl Widget<ProviderConfig> {
    let cancel_btn = button_builder("Cancel").on_click(|event_ctx, _, _: &_| {
        event_ctx.submit_command(Command::new(commands::NAVIGATE_BACK, (), Target::Auto))
    });
    let confirm_btn = button_builder("Save").on_click(|event_ctx, _, _: &_| {
        event_ctx.submit_command(Command::new(commands::SAVE_PROVIDER, (), Target::Auto))
    });
    let delete_btn = button_builder("Delete").on_click(|event_ctx, _, _: &_| {
        event_ctx.submit_command(Command::new(commands::DELETE_PROVIDER, (), Target::Auto))
    });

    let actions = Flex::row()
        .with_flex_child(cancel_btn, 1.0)
        .with_spacer(8.)
        .with_flex_child(confirm_btn, 1.0)
        .padding(4.);

    Flex::column()
        .with_child(edit_provider_builder())
        .with_child(delete_btn)
        .with_child(actions)
}

pub fn new_provider_selector() -> impl Widget<AppState> {
    let mut selector = Flex::column();

    if cfg!(feature = "github") {
        add_provider::<crate::providers::github::GithubConfig, _>(&mut selector, "Github");
    }
    if cfg!(feature = "gitlab") {
        add_provider::<crate::providers::gitlab::GitlabConfig, _>(&mut selector, "Gitlab");
    }
    if cfg!(feature = "jira") {
        add_provider::<crate::providers::jira::JiraConfig, _>(&mut selector, "Jira");
    }
    if cfg!(feature = "confluence") {
        add_provider::<crate::providers::confluence::ConfluenceConfig, _>(
            &mut selector,
            "Confluence",
        );
    }
    if cfg!(feature = "joplin") {
        add_provider::<crate::providers::joplin::JoplinConfig, _>(&mut selector, "Joplin");
    }
    if cfg!(feature = "taskwarrior") {
        add_provider::<crate::providers::taskwarrior::TaskwarriorConfig, _>(
            &mut selector,
            "Taskwarrior",
        );
    }
    if cfg!(feature = "upsource") {
        add_provider::<crate::providers::upsource::UpsourceConfig, _>(&mut selector, "Upsource");
    }
    if cfg!(feature = "nextcloud") {
        add_provider::<crate::providers::nextcloud::deck::NextcloudDeckProviderConfig, _>(
            &mut selector,
            "Nextcloud Deck",
        );
    }

    let header = header_builder("Add Provider");

    Flex::column()
        .must_fill_main_axis(true)
        .cross_axis_alignment(CrossAxisAlignment::Fill)
        .with_child(header)
        .with_flex_child(selector, 1.0)
}

fn add_provider<C: Default + Into<ProviderConfig>, T: Data>(selector: &mut Flex<T>, title: &str) {
    selector.add_child(button_builder(title).on_click(|event_ctx, _, _: &_| {
        event_ctx.submit_command(Command::new(
            commands::NAVIGATE,
            Navigation::EditProvider((ProviderId::default(), C::default().into())),
            Target::Auto,
        ))
    }));
}

pub fn view_provider_builder() -> impl Widget<ProviderConfigEntry> {
    let mut switcher = Switcher::new();
    if cfg!(feature = "github") {
        switcher = switcher.with_variant(ProviderConfigPrism, github::view());
    }
    if cfg!(feature = "gitlab") {
        switcher = switcher.with_variant(ProviderConfigPrism, gitlab::view());
    }
    if cfg!(feature = "joplin") {
        switcher = switcher.with_variant(ProviderConfigPrism, joplin::view());
    }
    if cfg!(feature = "confluence") {
        switcher = switcher.with_variant(ProviderConfigPrism, confluence::view());
    }
    if cfg!(feature = "jira") {
        switcher = switcher.with_variant(ProviderConfigPrism, jira::view());
    }
    if cfg!(feature = "taskwarrior") {
        switcher = switcher.with_variant(ProviderConfigPrism, taskwarrior::view());
    }
    if cfg!(feature = "upsource") {
        switcher = switcher.with_variant(ProviderConfigPrism, upsource::view());
    }
    if cfg!(feature = "nextcloud") {
        switcher = switcher.with_variant(ProviderConfigPrism, nextcloud::view());
    }

    switcher
        .padding(4.)
        .background(Color::rgba8(0, 0, 0, 16))
        .rounded(2.0)
        .padding(Insets::uniform_xy(0., 2.))
        .expand_width()
        .lens(ProviderConfigEntry::provider)
        .on_click(|event_ctx, provider: &mut ProviderConfigEntry, _: &_| {
            event_ctx.submit_command(Command::new(
                commands::NAVIGATE,
                Navigation::EditProvider((provider.id, provider.provider.clone())),
                Target::Auto,
            ))
        })
}

pub fn edit_provider_builder() -> impl Widget<ProviderConfig> {
    let mut switcher = Switcher::new();
    if cfg!(feature = "github") {
        switcher = switcher.with_variant(ProviderConfigPrism, github::edit());
    }
    if cfg!(feature = "gitlab") {
        switcher = switcher.with_variant(ProviderConfigPrism, gitlab::edit());
    }
    if cfg!(feature = "joplin") {
        switcher = switcher.with_variant(ProviderConfigPrism, joplin::edit());
    }
    if cfg!(feature = "confluence") {
        switcher = switcher.with_variant(ProviderConfigPrism, confluence::edit());
    }
    if cfg!(feature = "jira") {
        switcher = switcher.with_variant(ProviderConfigPrism, jira::edit());
    }
    if cfg!(feature = "taskwarrior") {
        switcher = switcher.with_variant(ProviderConfigPrism, taskwarrior::edit());
    }
    if cfg!(feature = "upsource") {
        switcher = switcher.with_variant(ProviderConfigPrism, upsource::edit());
    }
    if cfg!(feature = "nextcloud") {
        switcher = switcher.with_variant(ProviderConfigPrism, nextcloud::edit());
    }

    switcher
}
