export type Config = typeof sampleConfig

export const sampleConfig = {
    endpoint: "",
}

export class InvalidConfigError extends Error {
    constructor(message?: string | undefined) {
        super(message)
        this.name = "InvalidConfigError"
    }
}

export class ConfigFetchFailedError extends Error {
    constructor(message?: string | undefined) {
        super(message)
        this.name = "ConfigFetchFailedError"
    }
}

export class ReplRequestFailedError extends Error {
    constructor(message?: string | undefined) {
        super(message)
        this.name = "ReplRequestFailedError"
    }
}
