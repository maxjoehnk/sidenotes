<img src="assets/logo.svg" width="256px"/>

# Sidenotes

A small desktop app to view github PRs, gitlab MRs, jira issues, taskwarrior tasks, joplin notebooks and upsource tasks in a sidebar.

Overview-Page:

![Image Overview Page](docs/img/overview.png)

Detail-Page:

![Image Detail Page](docs/img/detail.png)

## Features

All features are optional but will be installed by default.

Available:

* github
* gitlab
* jira
* taskwarrior
* joplin
* upsource
* confluence
* nextcloud
* devops

## Installation

Create a `settings.toml` in `$XDG_CONFIG_HOME/sidenotes/`

Example:

```toml
sync_timeout = 30

[[provider]]
name = "Github"
type = "github"
token = "<github token>"
repos = ["maxjoehnk/sidenotes"]

[[provider]]
name = "Gitlab"
type = "gitlab"
url = "your.gitlab.url"
token = "<gitlab token>"
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

[[provider]]
type = "joplin"
token = "<Web Clipper token>"
show_notebook_names = false # Show the notebook title as a tag below the todo
# Notebook Ids
notebooks = ["bc956e0e43b74c678817a1e82f468127", "d705bc49caa34927926a3c8018bf593d", "cc1fe66cbf384c60b65978dec330f364", "5002ad0da82f4e6e8b3c3735ae205c41", "8a537e1c29e14884a32efd28c629652c"]

[[provider]]
type = "upsource"
url = "https://your-upsource-instance"
token = "auth token"
# optional, default is "state: open"
query = "an upsource query"

[[provider]]
type = "confluence"
username = "your-usernamee"
password = "your-password"
url = "https://your.confluence.url"

[[provider]]
type = "nextcloudDeck"
host = "https://nextcloud.url"
username = "username"
password = "password"
[[provider.boards]] # multiple supported
title = "Infrastruktur" # limit todos to cards in this board
stacks = ["TODO", "in Bearbeitung"] # limit todos to cards in these stacks

[[provider]]
type = "azureDevops"
token = "your private access token"
organization = "organization name"
project = "project name"
```
