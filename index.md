A small desktop app to view github PRs, gitlab MRs, jira issues, taskwarrior tasks, joplin notebooks and upsource tasks in a sidebar.

### Supported Providers

* Github
* Gitlab
* Jira
* Taskwarrior
* Joplin
* Upsource
* Confluence

#### Github

The Github provider supports a list of open pull requests as well as a running a search for issues and prs.

![Github Configuration](assets/img/github_config.png)

##### Config

```toml
[[provider]]
type = "github" # required
token = "<github token>" # required
repos = ["maxjoehnk/sidenotes"] # optional
query = "type:pr is:open draft:false author:@me review:changes_requested" # optional
```

At least one of `repos` or `query` is required for the Github provider to show anything.

#### Gitlab

The Gitlab provider supports a list of open pull requests.

![Gitlab Configuration](assets/img/gitlab_config.png)

##### Config

```toml
[[provider]]
type = "gitlab" # required
url = "your.gitlab.url" # required
token = "<gitlab token>" # required
repos = ["maxjoehnk/sidenotes"] # required
show_drafts = true # optional (default: false)
```

#### Jira

The Jira provider supports any valid JQL query.

![Jira Configuration](assets/img/jira_config.png)

##### Config

```toml
[[provider]]
type = "jira" # required
url = "https://your.jira.url" # required
username = "your username" # required
password = "your password" # required
jql = "assignee = currentUser() and statusCategory != Done" # required
```
