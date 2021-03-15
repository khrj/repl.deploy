const configFileName = "replit-deploy.json"
export const getConfigFileURL = (slug: string, commitID: string) =>
    `https://raw.githubusercontent.com/${slug}/${commitID}/${configFileName}`
