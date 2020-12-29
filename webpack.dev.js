const { merge } = require('webpack-merge');
const common = require('./webpack.common.js');
const path = require("path");

module.exports = merge(common, {
    devServer: {
        historyApiFallback: true,
        contentBase: path.resolve(__dirname, "dist"),
        port: 9000
    },
    mode: 'development'
});