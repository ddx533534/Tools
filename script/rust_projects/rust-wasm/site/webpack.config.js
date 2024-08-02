const path = require("path");
module.exports = {
  entry: "./index.js",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "index.js",
  },
  devServer: {
    contentBase: path.join(__dirname, '/'), // 根据你实际的静态文件目录修改
    compress: true,
    port: 8080,
    host: '0.0.0.0', // 监听所有网络接口
  },
  mode: "development",
};
