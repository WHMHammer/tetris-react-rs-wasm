/* eslint-disable @typescript-eslint/no-require-imports */
/* eslint-disable no-undef */

const path = require("path");
const HtmlWebpackPlugin = require("html-webpack-plugin");

module.exports = {
  devServer: {
    static: path.resolve(__dirname, "dist"),
  },
  devtool: "inline-source-map",
  entry: path.resolve(__dirname, "tetris-react-ts", "index.tsx"),
  experiments: {
    asyncWebAssembly: true,
  },
  module: {
    rules: [
      {
        exclude: /node_modules/,
        test: /\.tsx?$/,
        use: "ts-loader",
      },
      {
        test: /\.scss$/i,
        use: [
          "style-loader", // Creates `style` nodes from JS strings
          "css-loader", // Translates CSS into CommonJS
          "sass-loader", // Compiles Sass to CSS
        ],
      },
    ],
  },
  output: {
    filename: "bundle.js",
    path: path.resolve(__dirname, "dist"),
    publicPath: "./",
  },
  plugins: [
    new HtmlWebpackPlugin({
      hash: true,
      scriptLoading: "module",
      template: path.resolve(__dirname, "tetris-react-ts", "index.html"),
      title: "Tetris",
    }),
  ],
  resolve: {
    extensions: [".js", ".mjs", ".cjs", ".ts", ".jsx", ".tsx"],
  },
};
