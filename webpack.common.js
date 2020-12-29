const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const CopyWebpackPlugin = require('copy-webpack-plugin');

const distPath = path.resolve(__dirname, "dist");
module.exports = {
    entry: './src/index.js',
    output: {
        path: distPath,
        filename: 'index.js',
    },
    module: {
        rules: [
            {
                test: /\.css$/i,
                use: ['style-loader', 'css-loader'],
            },
        ],
    },
    experiments: { asyncWebAssembly: true },
    plugins: [
        new CopyWebpackPlugin({
            patterns: [
                { from: "static", to: distPath },
            ],
        }),
        new HtmlWebpackPlugin({
            title: 'Kira Web Demo'
        }),
        new HtmlWebpackPlugin({
            filename: 'underwater-demo/index.html',
            title: 'Kira Web Demo'
        }),
        new WasmPackPlugin({
            crateDirectory: path.resolve(__dirname, ".")
        }),
    ]
};
