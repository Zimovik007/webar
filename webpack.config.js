const path = require("path");

module.exports = {
  entry: "./index.js",
  output: {
    filename: "./bundle.js",
    path: path.resolve()
  },
  module: {
    rules: [
      {
        test: /\.(js|jsx)$/,
        exclude: /node_modules/
      },
    ]
  }
};