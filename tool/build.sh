#! /bin/bash
cd ./web/Niming
pnpm build
cp -rf dist/* ../dist
cd ../../
cargo run
