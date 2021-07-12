import { ServerRequest } from "https://deno.land/std@0.90.0/http/server.ts"
import getReleaseURL from "https://deno.land/x/get_release_url@1.0.0/mod.ts"

export default async (req: ServerRequest) => {
    const [url] = await getReleaseURL({
        provider: "github",
        user: "khrj",
        repo: "repl.deploy",
    })

    req.respond({
        status: 302,
        headers: new Headers({
            "Location": url,
            "Cache-Control": "s-maxage=300",
        }),
    })
}
