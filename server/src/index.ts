import { createSign } from "crypto"
import fetch from "node-fetch"
import { Probot } from "probot"

import { getConfigFileURL } from "./constants"
import { validateConfig } from "./helpers"
import { ConfigFetchFailedError, InvalidConfigError, ReplRequestFailedError } from "./types"

export default function replDeployBot(app: Probot) {
    app.on("push", async (context) => {
        const owner = context.payload.repository.owner.name,
            repo = context.payload.repository.name,
            slug = context.payload.repository.full_name,
            commitID = context.payload.after

        const configResponse = await fetch(
            getConfigFileURL(
                slug,
                commitID,
            ),
        )

        // Ignore repository if config doesn't exist
        if (configResponse.status === 404) return

        const check = await context.octokit.checks.create({
            owner,
            repo,
            head_sha: commitID,
            name: "Deploying to Repl.it",
            status: "in_progress",
        })

        try {
            // If the config response returns any non-okay code other than 404,
            // throw an error
            if (!configResponse.ok) {
                throw new ConfigFetchFailedError(`HTTP request failed with code: ${configResponse.status.toString()}`)
            }

            const config: unknown = await (async () => {
                try {
                    return await configResponse.json()
                } catch {
                    throw new InvalidConfigError("Error parsing JSON")
                }
            })()

            validateConfig(config)

            const request = JSON.stringify({
                timestamp: Date.now(),
                endpoint: config.endpoint,
            })

            const signer = createSign("sha256")
            signer.update(request)

            const signature = signer.sign({
                key: process.env.REPL_DEPLOY_KEY!,
                passphrase: process.env.REPL_DEPLOY_KEY_PASSPHRASE!,
            }).toString("base64")

            try {
                const response = await fetch(config.endpoint, {
                    method: "POST",
                    body: request,
                    headers: {
                        "Signature": signature,
                    },
                })

                if (!response.ok) throw "Request not OK"
            } catch {
                throw new ReplRequestFailedError()
            }

            await context.octokit.checks.update({
                owner,
                repo,
                check_run_id: check.data.id,
                conclusion: "success",
                output: {
                    title: "Completed request to redeploy repl",
                    summary: "Changes should reflect soon (once the repl finishes restarting)",
                },
            })
        } catch (e) {
            if (e.name === "InvalidConfigError" || e.name === "ConfigFetchFailedError") {
                await context.octokit.checks.update({
                    owner,
                    repo,
                    check_run_id: check.data.id,
                    conclusion: "failure",
                    output: {
                        title: "Configuration error",
                        summary: `${e.name}: ${e.message}`,
                    },
                })
            } else if (e.name = "ReplRequestFailedError") {
                await context.octokit.checks.update({
                    owner,
                    repo,
                    check_run_id: check.data.id,
                    conclusion: "failure",
                    output: {
                        title: "HTTP Request to Repl failed",
                        summary: `${e.name}: ${e.message}`,
                    },
                })
            } else {
                throw e
            }
        }
    })
}
