const path = require('path');
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');

const wasmPlugin = {
  plugin: {
    overrideWebpackConfig: ({webpackConfig}) => {
      const wasmExtensionRegExp = /\.wasm$/;

      // derived from here
      // https://tomtongue.com/blog/2019/03/07/react-rust-wasm.html
      // Make file-loader ignore WASM files
      webpackConfig.module.rules.forEach((rule) => {
        (rule.oneOf || []).forEach((oneOf) => {
          if (oneOf.loader && oneOf.loader.indexOf('file-loader') >= 0) {
            // Make file-loader ignore WASM files
            oneOf.exclude.push(wasmExtensionRegExp);
          }
        });
      });

      webpackConfig.plugins.push(
        new WasmPackPlugin({
          crateDirectory: path.resolve(__dirname, '../crate'),

          // Check https://rustwasm.github.io/wasm-pack/book/commands/build.html for
          // the available set of arguments.
          //
          // Default arguments are `--typescript --target browser --mode normal`.
          // extraArgs: '--no-typescript',
          withTypeScript: true,

          // Optional array of absolute paths to directories, changes to which
          // will trigger the build.
          // watchDirectories: [
          //   path.resolve(__dirname, "another-crate/src")
          // ],

          // The same as the `--out-dir` option for `wasm-pack`
          outDir: path.resolve(__dirname, './src/crate'),

          // The same as the `--out-name` option for `wasm-pack`
          // outName: "index",

          // If defined, `forceWatch` will force activate/deactivate watch mode for
          // `.rs` files.
          //
          // The default (not set) aligns watch mode for `.rs` files to Webpack's
          // watch mode.
          // forceWatch: true,

          // If defined, `forceMode` will force the compilation mode for `wasm-pack`
          //
          // Possible values are `development` and `production`.
          //
          // the mode `development` makes `wasm-pack` build in `debug` mode.
          // the mode `production` makes `wasm-pack` build in `release` mode.
          // forceMode: "development",
        }),
      );
      return webpackConfig;
    },
  },
};

module.exports = function({env}) {
  return {
    eslint: {
      configure: {
        overrides: [
          {
            files: ['src/crate/*.js'],
            rules: {eqeqeq: 'off', 'no-unused-vars': 'off'},
          },
        ],
      },
    },
    plugins: [wasmPlugin],
  };
};
