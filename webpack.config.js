const path = require("path");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const webpack = require("webpack");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

module.exports = (env) => {
  const TRACE = env && env.trace;

  return {
    entry: "./src/index.js",
    output: {
      path: path.resolve(__dirname, "dist"),
      filename: "index.js",
    },
    plugins: [
      new HtmlWebpackPlugin({
        template: "src/index.html",
      }),
      new WasmPackPlugin({
        crateDirectory: path.resolve(__dirname, "."),
        extraArgs: `--target web -- ${
          TRACE ? "" : "--features multi-threading"
        }`,
      }),
    ],
    mode: "development",
    experiments: {
      asyncWebAssembly: true,
    },
    module: {
      rules: [
        {
          test: /\.s[ac]ss$/i,
          use: ["style-loader", "css-loader", "sass-loader"],
        },
      ],
    },
    devServer: {
      headers: {
        "Cross-Origin-Opener-Policy": "same-origin",
        "Cross-Origin-Embedder-Policy": "require-corp",
      },
    },
  };
};
