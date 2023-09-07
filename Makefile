all: core ui
prepare: core ui-prepare
core:
	cd core && make
ui:
	cd ui && deno task build
ui-deps:
	cd ui && deno task deps
ui-prepare:
	cd ui && deno task prepare
ui-dev:
	cd ui && deno task dev
ui-wasm:
	cd ui && deno task wasm
ui-wasm-watch:
	cd ui && deno task wasm-watch
ui-eslint:
	cd ui && deno task eslint
ui-preview:
	cd ui && deno task preview
ui-serve:
	cd ui && deno task serve
