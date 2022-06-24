// DOCS: https://www.npmjs.com/package/next-with-less
const withPlugins    = require("next-compose-plugins");
const withLess       = require("next-with-less");
// const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin")
// const path           = require("path");

const plugins = [
  [withLess, {
    lessLoaderOptions: {}
  }]
];

/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: true,

  // // BUG:  HookWebpackError: Cannot read properties of undefined (reading 'traceChild')
  // // DOCS: https://nextjs.org/docs/api-reference/next.config.js/custom-webpack-config
  // webpack: (config, { buildId, dev, isServer, defaultLoaders, webpack }) => {
  //   // Important: return the modified config
  //   return {
  //     ...config,
  //     plugins: [
  //       new WasmPackPlugin({
  //         // crateDirectory: path.resolve(__dirname, "rust"),
  //         crateDirectory: path.resolve(__dirname, "wasm-react-webpack-template"),
  //         forceMode: "production",
  //         // outDir: "public/wasm/",
  //       })
  //     ],
  //   }
  // }
}

module.exports = withPlugins(plugins, nextConfig);
