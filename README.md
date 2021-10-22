# Sidenotes

A small desktop app to view github prs, jira issues and taskwarrior tasks in a sidebar.

Overview-Page:

![Image Overview Page](docs/img/overview.png?raw=true "Overview Page")

Detail-Page:

![Image Detail Page](docs/img/detail.png?raw=true "Detail Page")

## Installation

Create a `settings.toml` in `$XDG_HOME/sitenotes/`

Example:
```toml
sync_timeout = 30

[[provider]]
name = "Github"
type = "github"
token = "<github token>"
repos = ["maxjoehnk/sidenotes"]

[[provider]]
name = "Jira"
type = "Jira"
url = "https://your.jira.url"
username = "your-username"
password = "your-password"
jql = "assignee = currentUser() and statusCategory != Done"

[[provider]]
name = "Tasks"
type = "taskwarrior"
query = "status:pending"
```