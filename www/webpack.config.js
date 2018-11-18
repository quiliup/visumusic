const CopyWebpackPlugin = require("copy-webpack-plugin");
const path = require('path');

module.exports = {
  entry: "./bootstrap.js",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "bootstrap.js",
  },
  mode: "development",
  plugins: [
    new CopyWebpackPlugin([
      'index.html',
      'bootstrap.min.css',
      'bootstrap-4.1.min.js',
      'canvasjs.min.js',
      'jquery-3.3.1.min.js'
    ])
  ],
};
