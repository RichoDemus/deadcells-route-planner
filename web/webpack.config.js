const MiniCssExtractPlugin = require('mini-css-extract-plugin');
const path = require('path');
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const CopyPlugin = require('copy-webpack-plugin');

const mode = process.env.NODE_ENV || 'development';
const prod = mode === 'production';
const dist = path.resolve(__dirname, "public");

module.exports = {
	entry: {
		bundle: ['./src/main.js']
	},
	resolve: {
		alias: {
			svelte: path.resolve('node_modules', 'svelte')
		},
		extensions: ['.mjs', '.js', '.svelte'],
		mainFields: ['svelte', 'browser', 'module', 'main']
	},
	output: {
		path: __dirname + '/dist',
		filename: '[name].js',
		chunkFilename: '[name].[id].js'
	},
	module: {
		rules: [
			{
				test: /\.svelte$/,
				use: {
					loader: 'svelte-loader',
					options: {
						emitCss: true,
						hotReload: true
					}
				}
			},
			{
				test: /\.css$/,
				use: [
					/**
					 * MiniCssExtractPlugin doesn't support HMR.
					 * For developing, use 'style-loader' instead.
					 * */
					prod ? MiniCssExtractPlugin.loader : 'style-loader',
					'css-loader'
				]
			}
		]
	},
	mode,
	plugins: [
		new MiniCssExtractPlugin({
			filename: '[name].css'
		}),
		new CopyPlugin([
			{ from: 'public', to: '.' },
		]),
		new WasmPackPlugin({
			crateDirectory: "../rust/",
			outDir: "../web/pkg",
		}),
	],
	devtool: prod ? false: 'source-map'
};
