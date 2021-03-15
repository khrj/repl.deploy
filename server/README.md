# repl-deploy-bot

> A GitHub App built with [Probot](https://github.com/probot/probot) that Automatically deploy to repl.it from GitHub

## Setup

```sh
# Install dependencies
yarn

# Run the bot
yarn start
```

## Docker

```sh
# 1. Build container
docker build -t repl-deploy-bot .

# 2. Start container
docker run -e APP_ID=<app-id> -e PRIVATE_KEY=<pem-value> repl-deploy-bot
```

Ï

## License

[MIT](LICENSE) © 2021 Khushraj Rathod <khushraj.rathod@gmail.com>
