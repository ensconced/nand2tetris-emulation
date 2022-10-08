import HtmlWebpackPlugin from "html-webpack-plugin";
import path from "path";

const TEN_MEGABYTES = 10 * 1024 * 1024;

module.exports = {
  mode: "production",
  entry: "./src/index.tsx",
  output: {
    path: path.resolve(__dirname, "./dist"),
    filename: "index_bundle.js",
  },
  module: {
    rules: [
      {
        test: /\.tsx?$/,
        use: {
          loader: "babel-loader",
          options: {
            presets: ["@babel/preset-typescript", "@babel/preset-react"],
          },
        },
      },
      {
        test: /\.css$/,
        use: ["style-loader", "css-loader"],
      },
    ],
  },
  performance: {
    maxAssetSize: TEN_MEGABYTES,
    maxEntrypointSize: TEN_MEGABYTES,
  },
  resolve: {
    extensions: [".js", ".json", ".ts", ".tsx"],
  },
  plugins: [new HtmlWebpackPlugin({ title: "template project" })],
  experiments: {
    asyncWebAssembly: true,
  },
};
