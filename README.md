<div align="center">
    <img src="assets/logo.svg" width="400" height="400" alt="blueprint illustration">
    <h1>repl.deploy</h1>
    <p>
        <b>Automatically deploy from GitHub to Replit, lightning fast ⚡️</b>
    </p>
    <p>
        <img alt="build" src="https://img.shields.io/github/workflow/status/khrj/repl.deploy/release">
        <img alt="language" src="https://img.shields.io/github/languages/top/khrj/repl.deploy" >
        <img alt="code size" src="https://img.shields.io/github/languages/code-size/khrj/repl.deploy">
        <img alt="issues" src="https://img.shields.io/github/issues/khrj/repl.deploy" >
        <img alt="license" src="https://img.shields.io/github/license/khrj/repl.deploy">
        <img alt="version" src="https://img.shields.io/github/v/release/khrj/repl.deploy">
    </p>
    <p>
        <a href="https://www.producthunt.com/posts/repl-deploy?utm_source=badge-featured&utm_medium=badge&utm_souce=badge-repl-deploy" target="_blank"><img src="https://api.producthunt.com/widgets/embed-image/v1/featured.svg?post_id=288767&theme=dark" alt="repl.deploy - Automatically deploy from GitHub to Replit, lightning fast | Product Hunt" style="width: 250px; height: 54px;" width="250" height="54" /></a>
    </p>
    <br>
    <br>
    <br>
</div>

`repl.deploy` is split into

- A GitHub app, which listens for code changes and sends events to your repl
- A daemon, which runs on your repl, listens for events, fetches changes from
  GitHub and restarts your application 

## Table of Contents

- [Usage](#usage)
- [How?](#how)
- [FAQ](#faq)
- [Supporters](#supporters)

## Usage

1. [Authorize
   repl.deploy](https://github.com/apps/repl-deploy/installations/new) to get
   events from GitHub

2. Make sure you have a `main` branch on your `origin` remote

3. Add `replit-deploy.json` to your git repository with a single `endpoint` key,
   which is the address of your repl + `/refresh`. E.g.
```json
{
    "endpoint": "https://my-amazing-application.my-username.repl.co/refresh"    
}
```

4. Clone your git repository to your repl

5. Download `repl.deploy` to the root of your repl -- Open the shell, and run
```bash
curl -sL https://repl-deploy.vercel.app/ -o repl.deploy
chmod +x ./repl.deploy
```

> **WARNING**: Proceeding will overwrite any local changes and reset from your GitHub repo. Commit AND push any local changes BEFORE running repl.deploy

6. For repls that do not use an HTTP server in their code [(See
   example)](https://github.com/KhushrajSandbox/repl.deploy-standalone-example)
    - Create/modify the `.replit` file in the root of your repl and change
      `run=` to run `./repl.deploy --standalone <command to run your code
      here>`. E.g.
    ```
    run="./repl.deploy --standalone node index.js"
    ```

7. For repls that use an HTTP server in their code [(See
   example)](https://github.com/KhushrajSandbox/repl.deploy-http-example)
    - Create/modify the `.replit` file in the root of your repl and change
      `run=` to run `./repl.deploy <command to run your code here>`. E.g.
    ```
    run="./repl.deploy node index.js"
    ```
    - Set up the `/refresh` endpoint, and log a line in the following format to
      `stdout` when a request is recieved: `repl.deploy<insert json body here
      (don't include the angle brackets)><insert "Signature" header here (don't
      include the angle brackets)>`. E.g.
    ```
    repl.deploy{"timestamp":1615896087141,"endpoint":"https://8c051d0fbc4b.ngrok.io/refresh"}ostjM6/jGmHbRWcHazxKWSPmvgvoIryI9XxLgNKgxPCKRW==
    ```
    - Your application will recieve JSON via `stdin`. E.g.
    ```
    {"status":"403","body":"Invalid Signature"}
    ```
    simply respond with the given status and body (see example)
    - Once you've responded, log (to `stdout`)
    ```
    repl.deploy-success
    ```

8. Click `Run` once. Make sure your repl is set to always-on or has a [pinging
   service](https://uptimerobot.com) set up (otherwise, the daemon will be
   stopped by repl once you close your browser tab)

9. That's it! Repl.it should automatically pull changes from GitHub the next
   time you commit

## How? 

- When you commit, GitHub sends an event to a hosted instance of the
  `repl.deploy` server

- A payload consisting of both the endpoint and the current time is prepared and
  signed with an RSA private key

- The daemon running on the repl recieves the payload, and
    - Verifies the signature
    - Checks that the endpoint matches (this is to prevent someone from just
      forwarding a signed request to your repl and causing it to restart)
    - Checks that the timestamp is less than 15 seconds old (this is to prevent
      someone from abusing a signature in the event that a signed request is
      leaked by you)

- If the request is valid, the daemon 
    - runs `git fetch --all` and then `git reset --hard origin/main`
    - restarts your program

## FAQ

- **Q: What does `run="./repl.deploy --standalone node index.js"` do?**  
  A: It makes the Replit `Run` button run the daemon instead of executing the
  program directly, which then executes/re-executes the program on `git push`
 
- **Q: Does repl.deploy need to be downloaded every time the program is run?**  
  A: No, repl.deploy only needs to be downloaded once per repl
  
- **Q: What's `https://repl-deploy.vercel.app/`?**  
  A: A hosted version of
  [get-release-url](https://github.com/khrj/get-release-url), which saves
  you the time of manually finding the latest release and linking it.

## Supporters

[![Stargazers repo roster for @khrj/repl.deploy](https://reporoster.com/stars/khrj/repl.deploy)](https://github.com/khrj/repl.deploy/stargazers)

[![Forkers repo roster for @khrj/repl.deploy](https://reporoster.com/forks/khrj/repl.deploy)](https://github.com/khrj/repl.deploy/network/members)
