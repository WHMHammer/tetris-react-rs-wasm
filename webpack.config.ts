import HtmlWebpackPlugin from "html-webpack-plugin";
import path from "path";
import { fileURLToPath } from "url";
import { Configuration } from "webpack";
import { Configuration as DevServerConfiguration } from "webpack-dev-server";

const __dirname = path.dirname(fileURLToPath(import.meta.url));

const config: Configuration & DevServerConfiguration = {
  devServer: {
    hot: true,
    open: true,
  },
  devtool: "inline-source-map",
  entry: path.resolve(__dirname, "src", "index.tsx"),
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
  },
  plugins: [
    new HtmlWebpackPlugin({
      hash: true,
      scriptLoading: "module",
      template: path.resolve(__dirname, "src", "index.html"),
      title: "Tetris",
    }),
  ],
  resolve: {
    extensions: [".js", ".mjs", ".cjs", ".ts", ".jsx", ".tsx"],
  },
};

export default config;
