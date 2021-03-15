import { Config, InvalidConfigError, sampleConfig } from "./types"

// ! Do not convert to arrow function, `asserts` doesn't play nicely with it
export function validateConfig(config: unknown): asserts config is Config {
    const configKeys = Object.keys(sampleConfig)

    if (typeof config !== "object" || !config) {
        throw new InvalidConfigError(`JSON is not Record<string | number | symbol, unknown>`)
    }

    for (const key of configKeys) {
        if (!(key in config)) {
            throw new InvalidConfigError(`Missing ${key}`)
        }
    }
}