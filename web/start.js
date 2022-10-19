const HtmlWebpackPlugin = require("html-webpack-plugin");
const path = require("path");
const webpack = require("webpack");
const WebpackDevServer = require("webpack-dev-server");
const { execSync } = require("child_process");
const chokidar = require("chokidar");

const TEN_MEGABYTES = 10 * 1024 * 1024;

const webpackConfig = {
  mode: "production",
  cache: false,
  entry: "./src/index.tsx",
  output: {
    path: path.resolve(__dirname, "./dist"),
    filename: "index_bundle.js",
  },
  optimization: {
    minimize: false,
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

const compiler = webpack(webpackConfig);

const server = new WebpackDevServer({ open: true, hot: false }, compiler);

const watcher = chokidar.watch(path.resolve(__dirname, "../web-emulator"), {
  ignored: /web-emulator\/pkg\//,
});

const runServer = async () => {
  console.log("building wasm");
  execSync("./node_modules/.bin/wasm-pack build ../web-emulator", {
    cwd: __dirname,
  });
  console.log("Starting server...");
  await server.start();
  watcher.on("change", () => {
    console.log("rebuilding wasm");
    execSync("./node_modules/.bin/wasm-pack build ../web-emulator", {
      cwd: __dirname,
    });
    server.middleware.invalidate();
  });
};

runServer().catch((err) => {
  console.error(err);
});
