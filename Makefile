all: core ui
init: core ui-init
core:
	cd core && make
ui:
	cd ui && deno task build
ui-init:
	cd ui && deno task init
