# hi
Greeting before you starting, taking a lunch break, or finishing work.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://github.com/kyoshidajp/hi/blob/main/LICENSE)
![hi](https://github.com/kyoshidajp/hi/workflows/hi/badge.svg)

## Examples

```sh
# Before starting to work
% hi --time-slot morning
🏢 おはようございます。 (09:10:28-18:10:28)

# Before taking a lunch break
% hi --time-slot lunch
🍱 休憩します。 (12:12:23-13:12:23)

# Before finishing work
% hi --time-slot evening
♨️ お疲れさまでした。
```

## Post to Slack

`--post-to-slack` option enable you to post message to your Slack channel.

1. Create Slack Incoming WebHook in your workspace, and get the web hook url.
1. Create `.env` in a directory where you run `hi` command.
1. Run `hi` command without `--post-to-slack` for cheking the message.
1. If the message is good, then run the command with `--post-to-slack`.

Sample of `.env`.

```sh
SLACK_WEB_HOOK_URL=https://hooks.slack.com/services/xxx
SLACK_CHANNEL=#channel-name
SLACK_USER_NAME=bot-kyoshida
SLACK_ICON_EMOJI=:bot-kyoshida:
```
