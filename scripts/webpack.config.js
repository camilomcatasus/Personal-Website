const path = require('path');

module.exports = {
  mode: 'development',
  entry: './main.ts',
  devtool: 'inline-source-map',
  module: {
    rules: [
      {
        test: /\.tsx?$/,
        use: 'ts-loader',
        exclude: /node_modules/,
      },
    ],
  },
  resolve: {
    extensions: ['.tsx', '.ts', '.js'],
  },
  output: {
    filename: 'fun_nonsense.js',
    path: path.resolve(__dirname, '../static'),
  },
};
