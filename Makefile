test:
	cd ./gamelib && cargo test && cd -

compile_gamelib_to_wasm:
	cd ./wasm &&\
	wasm-pack build --target web || true &&\
	cd -

run_dev:
	cd ./frontend && pnpm run dev || true && cd -

front_build:
	cd ./frontend && pnpm run build || true && cd -
