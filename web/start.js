const HtmlWebpackPlugin = require("html-webpack-plugin");
const path = require("path");
const webpack = require("webpack");
const WebpackDevServer = require("webpack-dev-server");

const TEN_MEGABYTES = 10 * 1024 * 1024;

const webpackConfig = {
  mode: "production",
  cache: false,
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
  watchOptions: {
    ignored: path.resolve(__dirname, "../web-emulator/pkg"),
  },
};

const compiler = webpack(webpackConfig);

const devServerOptions = { ...webpackConfig.devServer, open: true };
const server = new WebpackDevServer(devServerOptions, compiler);

const runServer = async () => {
  console.log("Starting server...");
  await server.start();
};

runServer();
