const path = require("path");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const webpack = require("webpack");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

const output = {
  path: path.resolve(__dirname, "dist"),
  filename: "[name].js"
};

const plugins = [
  new WasmPackPlugin({
    crateDirectory: path.resolve(__dirname, "."),
    forceMode: "production"
  })
];

const mode = "production"; // "development";

module.exports = [
  {
    entry: "./src/index.js",
    output: {
      ...output,
      filename: "index.js"
    },
    plugins: [
      ...plugins,
      new HtmlWebpackPlugin({
        template: "./src/index.html"
      })
    ],
    devServer: { compress: true },
    mode
  },
  {
    entry: "./src/worker.js",
    target: "webworker",
    output: {
      ...output,
      filename: "worker.js",
      globalObject: "this"
    },
    plugins,
    devServer: { compress: true },
    mode
  }
];
