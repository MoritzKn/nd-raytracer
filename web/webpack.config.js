const path = require("path");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const webpack = require("webpack");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

const output = {
  path: path.resolve(__dirname, "dist"),
  filename: "[name].js"
};

const plugins = [
  new HtmlWebpackPlugin({
    template: "index.html"
  }),

  new WasmPackPlugin({
    crateDirectory: path.resolve(__dirname, "."),
    forceMode: "production"
  })
];

const mode = "development"; // "development";

module.exports = [
  {
    entry: "./index.js",
    output: {
      ...output,
      filename: "index.js"
    },
    plugins,
    mode
  },
  {
    entry: "./worker.js",
    target: "webworker",
    output: {
      ...output,
      filename: "worker.js",
      globalObject: "this"
    },
    plugins,
    mode
  }
];
