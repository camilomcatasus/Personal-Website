const path = require('path');

module.exports = {
  mode: 'development',
  entry: {
        main:'./main/main.ts',
        "htmx-snake":'./htmx-snake/htmx-snake.ts',
        "landing": './landing/landing.ts',
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
