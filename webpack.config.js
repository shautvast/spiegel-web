const path = require("path");
const HtmlWebpackPlugin = require("html-webpack-plugin");

const dist = path.resolve(__dirname, "dist");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

var mainConfig = {
  mode: "development",
  entry: "./src/js/index.js",
  output: {
    path: dist,
    filename: "bundle.js",
  },
  devServer: {
    static: dist,
  },
  module: {
    rules: [
      {
        test: /\.css$/,
        use: ["style-loader", "css-loader"],
      },
      {
        test: /\.(png|svg|jpg|gif)$/,
        use: ["file-loader"],
      },
    ],
  },
  plugins: [
    new HtmlWebpackPlugin({
      template: "index.html",
    }),

    new WasmPackPlugin({
      crateDirectory: path.resolve(__dirname, "webclient"),
      // WasmPackPlugin defaults to compiling in "dev" profile. To change that, use forceMode: 'release':
      forceMode: "release",
    }),
  ],
  experiments: {
    asyncWebAssembly: true,
  },
};

module.exports = [mainConfig];
