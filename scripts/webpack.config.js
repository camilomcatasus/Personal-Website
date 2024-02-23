const path = require('path');

module.exports = {
  mode: 'development',
  entry: {
        main:'./main/main.ts',
        htmx_snake:'./htmx_snake/htmx_snake.ts',
  },
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
    filename: '[name].js',
    path: path.resolve(__dirname, '../static'),
  },
};
