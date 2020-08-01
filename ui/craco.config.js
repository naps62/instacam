const path = require("path");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

const wasmPlugin = {
  plugin: {
    overrideWebpackConfig: ({ webpackConfig }) => {
      const wasmExtensionRegExp = /\.wasm$/;

      // derived from here
      // https://tomtongue.com/blog/2019/03/07/react-rust-wasm.html
      // Make file-loader ignore WASM files
      webpackConfig.module.rules.forEach((rule) => {
        (rule.oneOf || []).forEach((oneOf) => {
          if (oneOf.loader && oneOf.loader.indexOf("file-loader") >= 0) {
            // Make file-loader ignore WASM files
            oneOf.exclude.push(wasmExtensionRegExp);
          }
        });
      });

      webpackConfig.plugins.push(
        new WasmPackPlugin({
          crateDirectory: path.resolve(__dirname, "../wasm/pkg"),
          withTypeScript: true,
          outDir: path.resolve(__dirname, "./src/wasm"),
        })
      );
      console.log(webpackConfig);
      return webpackConfig;
    },
  },
};

module.exports = function () {
  return { plugins: [wasmPlugin] };
};
