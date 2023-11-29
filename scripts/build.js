const { execSync } = require("child_process");
const fs = require("fs");
const path = require("path");

const packagesDir = path.join(__dirname, "../packages");
const crateDir = path.join(packagesDir, "woodpile");
const outDir = path.join(packagesDir, "woodpile/pkg");

function build(target, mode) {
  execSync(
    `wasm-pack build --target ${target} --out-dir "pkg/${target}" --${mode} ${crateDir}`
  );
  const targetDir = path.join(outDir, target);
  fs.renameSync(
    path.join(targetDir, "woodpile.d.ts"),
    path.join(outDir, "woodpile.d.ts")
  );
  fs.unlinkSync(path.join(targetDir, "package.json"));
}

function clean() {
  fs.rmSync(outDir, { recursive: true, force: true });
}

function main() {
  clean();
  let mode = "release";
  if (process.argv.length > 2) {
    mode = process.argv[2];
  }

  const targets = ["bundler", "nodejs"];
  targets.forEach((target) => build(target, mode));

  const packageJsonContent = `{
	"name": "woodpile",
	"version": "0.1.0",
	"files": [
		"bundler/woodpile_bg.wasm",
		"bundler/woodpile.js",
		"bundler/woodpile_bg.js",
		"bundler/woodpile.d.ts",
		"nodejs/woodpile_bg.wasm",
		"nodejs/woodpile.js",
		"nodejs/woodpile.d.ts"
	],
	"exports": {
		".": {
			"import": "./bundler/woodpile.js",
			"default": "./nodejs/woodpile.js",
			"types": "./woodpile.d.ts"
		}
	},
	"main": "nodejs/woodpile.js",
	"module": "bundler/woodpile.js",
	"types": "woodpile.d.ts",
	"sideEffects": [
		"./bundler/woodpile.js",
		"./bundler/snippets/*"
	]
}`;
  fs.writeFileSync("packages/woodpile/pkg/package.json", packageJsonContent);
}

main();
