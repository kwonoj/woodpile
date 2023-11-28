#!/bin/bash
set -ex

function build() {
	wasm-pack build \
		--target $1 \
		--out-dir "pkg/$1" \
		--$mode \
		packages/woodpile
	rm packages/woodpile/pkg/$1/package.json
	mv packages/woodpile/pkg/$1/woodpile.d.ts ./packages/woodpile/pkg
}

function clean() {
	rm -rf packages/woodpile/pkg
}

function main() {
	clean
	mode=release
	if [ -n "$1" ]; then
			mode=$1
	fi

	targets=("bundler" "nodejs")
	for target in "${targets[@]}"
	do
		build $target
	done

	cat << EOF > packages/woodpile/pkg/package.json
{
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
  "main": "nodejs/woodpile.js",
  "module": "bundler/woodpile.js",
  "types": "woodpile.d.ts",
  "sideEffects": [
    "./bundler/woodpile.js",
    "./bundler/snippets/*"
  ]
}
EOF
}

main
