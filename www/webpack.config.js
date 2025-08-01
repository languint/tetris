const CopyWebpackPlugin = require("copy-webpack-plugin");
const path = require('path');
const { experiments } = require("webpack");

module.exports = {
  entry: "./bootstrap.js",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "bootstrap.js",
  },
  experiments: {
    asyncWebAssembly: true,
  },
  mode: "development",
  plugins: [
    new CopyWebpackPlugin(
      [
        { from: 'index.html' },
        { from: 'style.css' },
        { from: '../pkg', to: 'pkg' }
      ]
    )
  ],
};