# Changelog

All notable changes to this project will be documented in this file.
The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Add basic azure devops pull request support
- Support colorized backgrounds for rich-text elements
- Add support for links in jira markup
- Add support for more jira syntax
- Add support to load and display comments
- Add actions to providers to mark todos as done from detail view
- Add space between tags
- Add support for tags on todos
- Add basic ews calendar support (#10)
- Add option to show/hide draft prs
- Add common settings to providers
- Support color tags in panels and CRLF line endings
- Add new upsource provider
- Support basic jira lists
- Add basic detail view for todos
- Add gitlab provider
- Add basic joplin provider support

### Changed

- Update rust crate toml to 0.7.1 (#60)
- Update rust crate pulldown-cmark to 0.9.2
- Update rust crate uuid to 1.3.0 (#61)
- Update rust crate base64 to 0.21
- Update rust crate gitlab to 0.1508.0 (#59)
- Update rust crate gitlab to 0.1507.0 (#55)
- Update rust crate toml to 0.6.0 (#58)
- Update rust crate enum_dispatch to 0.3.11 (#56)
- Update rust crate toml to 0.5.11 (#57)
- Update rust crate enum_dispatch to 0.3.10 (#54)
- Update rust crate once_cell to 1.17 (#49)
- Update rust crate enum_dispatch to 0.3.9 (#50)
- Update rust crate derive_builder to 0.12 (#42)
- Update rust crate task-hookrs to 0.8.0 (#48)
- Show project as tag (#45)
- Update rust crate toml to 0.5.10 (#47)
- Update rust crate base64 to 0.20 (#46)
- Update rust crate open to 3.2.0 (#41)
- Update rust crate gitlab to 0.1506.0 (#44)
- Update rust crate smol to 1.3.0 (#43)
- Update rust crate open to v3 (#39)
- Update rust crate chrono to 0.4.23 (#25)
- Update rust crate gitlab to 0.1505.0 (#33)
- Update rust crate once_cell to 1.16 (#34)
- Update rust crate uuid to v1 (#40)
- Update rust crate toml to 0.5.9 (#28)
- Update rust crate derive_builder to 0.11 (#32)
- Update rust crate enum_dispatch to 0.3.8 (#27)
- Load github notifications
- Show due date in todo list and sort todos by due date
- Parse and render tables in jira markup
- Add appointment list view for rest of the day
- Improve parsing of inline styles
- Provide a settings ui as alternative to manual configuration in config files (#15)
- Show notebook name in todo tags
- Show progress indicator for current appointment
- Add provider for confluence inline tasks
- Don't crash sync thread when provider setup has failed
- Only parse svg icons once
- Reduce brightness of collapsed provider header
- Allow collapsing of providers and show todo count
- Load config from xdg_home (#3)
- Allow configuration of custom search query
- Implement support for nextcloud deck api
- Improve color palette and detail view
- Display panel title as heading
- Show link to todo in detail view when available
- Initial implementation (#1)
- Basic jira parsing
- Render subset of markdown in detail view
- Hide empty providers
- Show review state and draft state of prs
- Show ticket number in front of summary
- Improve layout and styling of todos

### Fixed

- Don't open console when starting on platform windows
- Set default for show_project config
- Fix wrong config path in installation section
- Fix jira_markup formatting
- Due date is shown on wrong todos after order has changed
- Time until meeting doesn't update
- Minor linting issues
- Sync job was never running because add_idle_callback never got called
- Ignore spaces after closing panel bracket
- Color tags don't work in lists
- Update upcoming appointment timer and meeting indicator continuously
- Empty config fails to load
- Full hour only shows one minute digit
- Long meeting titles overflow beyond the window
- Don't fail when no reviews are available
- Taskwarrior provider does not compile
- Sync done log message references notes instead of prs

### Removed

- Remove unused comments field in app state

