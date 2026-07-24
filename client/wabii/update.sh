#!/usr/bin/env sh

cd ../../
wabii="$PWD/client/wabii"
cd host

for input in "$wabii"/*.ron; do
	cargo run -p js-bindgen-dev -- codegen \
		--input "$input" \
		--output-dir "$wabii/src"
done
