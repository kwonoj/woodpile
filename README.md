### Woodpile

Woodpile is a utility library to traverse [SWC](https://github.com/swc-project/swc) ASTs in Javascript. It is a thin interop layer to the SWC's rust implementation of its visitor macro, attempt to provide consistent, synchronized mechanism to traverse ASTs in Javascript as well. This package could be useful for ducktyping, or prototyping SWC plugin in Javascript as its interface aims to provide similar experience as SWC's visitor macro.

For those reason, this package aims correctness over performance. There are inevitable costs to exchange data between Javascript to Rust, and vice versa. If you want to achieve peak performace, you should use SWC's visitor macro directly.


### Usage

`visit` is a main interface to traverse AST.

Currently, `visit` accepts an object with visitor properties have corresponding callbacks to traverse. `visit` property's callback will be called with corresponding node.

```ts
const { visit } = require('woodpile');
const { parseSync } = require('@swc/core');

const ast = parseSync('console.log("Hello, World!")');

visit(ast, {
    visit: {
        // Callbacks with visit${NodeName} will be called recursively for the node
        visitProgram: (node, self) => {
            console.log('visitProgram', node);
        },
        visitExpr: (node) => {
            console.log('visitExpr', node);
        }
    },
});
```

It is possible to return node in each callback which attempts to replace given node.

```ts
  visitProgram: (node) => {
    node.Span = ...;
    return node
  }
```

However, it doesn't check if the returned node is valid or not but will hard fail if the returned node is not valid. Callback also passes `self` as a second parameter. This is a context to the visitor object itself.

There is also `visitWithPath` property. This visitor's callback will supply `path` to the current node. This is useful to determin what kind of parent nodes are there from existing node. The parent nodes passed into path is not a full AST node, but a subset of the nodes indicates its types.

```ts
visit(ast, {
    visitWithPath: {
        visitExpr: (node, path, self) => {
        },
    }
})
```

There are also another utility function `compat`, attempts to provide conversion to the estree-compatble AST from SWC. Note this is the _closest attempt_ to generate compatible AST, likely will have some differences.

```ts
const { compat } = require('woodpile');
const { parseSync } = require('@swc/core');

const ast = parseSync('console.log("Hello, World!")');

const compat_ast = compat(ast, {
  source: "" // optional, original source code to the input ast
  flavor: "babel" | "acorn" // optional, default to babel
})
```

### Bundler Configuration

Usage of this package with projects (e.g. Next.js) that use Webpack may require some configuration changes.

See [this issue](https://github.com/vercel/next.js/issues/29362) and [this repo](https://github.com/hasharchives/wasm-ts-esm-in-node-jest-and-nextjs) for more information.

Tldr; to fix it, add the following snippet to your Webpack (or next.config.js) config:

```js
// next.config.js

/** @type {import("next").NextConfig} */
module.exports = {
  webpack(config, { isServer, dev }) {
    config.experiments = {
      asyncWebAssembly: true,
      layers: true,
    };

    if (!dev && isServer) {
      config.output.webassemblyModuleFilename = "chunks/[id].wasm";
      config.plugins.push(new WasmChunksFixPlugin());
    }

    config.module.rules.push({
      test: /\.svg$/,
      use: ["@svgr/webpack"],
    });

    return config;
  },
};

class WasmChunksFixPlugin {
  apply(compiler) {
    compiler.hooks.thisCompilation.tap("WasmChunksFixPlugin", (compilation) => {
      compilation.hooks.processAssets.tap(
        { name: "WasmChunksFixPlugin" },
        (assets) =>
          Object.entries(assets).forEach(([pathname, source]) => {
            if (!pathname.match(/\.wasm$/)) return;
            compilation.deleteAsset(pathname);

            const name = pathname.split("/")[1];
            const info = compilation.assetsInfo.get(pathname);
            compilation.emitAsset(name, source, info);
          })
      );
    });
  }
}
```
