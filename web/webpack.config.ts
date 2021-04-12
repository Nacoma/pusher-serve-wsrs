// @ts-ignore
import HtmlWebpackPlugin from 'html-webpack-plugin';
// @ts-ignore
import webpack from 'webpack';
// @ts-ignore
import path from 'path';

export default {
  mode: 'development',
  devtool: 'source-map',
  entry: './src/index.tsx',
  output: {
    publicPath: '/',
  },
  module: {
    rules: [
      {
        test: /\.(ts|js)x?$/i,
        exclude: /node_modules/,
        use: {
          loader: 'babel-loader',
          options: {
            presets: [
              '@babel/preset-env',
              '@babel/preset-react',
              '@babel/preset-typescript',
            ],
            plugins: [
              '@babel/plugin-proposal-class-properties',
            ],
          },
        },
      },
    ],
  },
  resolve: {
    extensions: ['.ts', '.js', '.json', '.tsx'],
  },
  devServer: {
    contentBase: path.join(__dirname, 'build'),
    historyApiFallback: true,
    port: 4000,
    open: true,
    hot: true,
  },
  plugins: [
    new HtmlWebpackPlugin({
      template: 'src/index.html',
    }),

    new webpack.HotModuleReplacementPlugin(),
  ],
};
