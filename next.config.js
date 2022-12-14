/** @type {import('next').NextConfig} */
const nextTranslate = require("next-translate");
const intercept = require("intercept-stdout");

const nextConfig = nextTranslate({
  reactStrictMode: true,
  swcMinify: true,
  output: "standalone",
  publicRuntimeConfig: {
    TEAM_NAME: process.env.TEAM_NAME || "UNKNOWN",
    IS_DEV: process.env.NODE_ENV !== "production",
  },
  webpack: (config, { webpack }) => {
    // this will override the experiments
    config.experiments = {
      ...config.experiments,
      ...{ syncWebAssembly: true },
    };
    // this will just update topLevelAwait property of config.experiments
    // config.experiments.topLevelAwait = true

    return config;
  },
});

/**
 * Hide warning of RecoilJS when hot reload
 */
intercept((text) => (text.includes("Duplicate atom key") ? "" : text));

module.exports = nextConfig;
