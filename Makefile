all: core ui
core:
	cd core && make
ui:
	cd ui && deno task build
