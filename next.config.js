// DOCS: https://www.npmjs.com/package/next-with-less
const withPlugins = require("next-compose-plugins");
const withLess    = require("next-with-less");
const plugins = [
  [withLess, {
    lessLoaderOptions: {}
  }]
];

/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: true
}

module.exports = withPlugins(plugins, nextConfig);
