# Speedpost developer tasks.
#
#   make dev     Watch sources, recompile and restart the app on every change.
#   make watch   Dependency-free fallback: recompile on change (no restart).
#   make run     Run the app once.
#   make build   Build the app.
#   make check   Type-check only (fast).
#   make ci      Format check, clippy and build.

CARGO ?= cargo

.DEFAULT_GOAL := help

.PHONY: help dev watch run build check fmt clippy ci install-watch

help:
	@echo "Targets:"
	@echo "  dev           Watch sources and recompile + restart the app (uses cargo-watch)"
	@echo "  watch         Recompile on change without extra tools (polling fallback)"
	@echo "  run           Run the app once"
	@echo "  build         Build the app"
	@echo "  check         Type-check only"
	@echo "  install-watch Install cargo-watch (required by 'dev')"
	@echo "  fmt           Format the code"
	@echo "  clippy        Run the linter"
	@echo "  ci            fmt-check + clippy + build"

# Watch + restart: hot-reloads the running app on every save.
dev:
	@if command -v cargo-watch >/dev/null 2>&1; then \
		echo "Watching for changes (Ctrl-C to stop)..."; \
		$(CARGO) watch -x run; \
	elif $(CARGO) install cargo-watch; then \
		echo "Watching for changes (Ctrl-C to stop)..."; \
		$(CARGO) watch -x run; \
	else \
		echo "cargo-watch unavailable; falling back to polling watcher."; \
		$(MAKE) watch; \
	fi

# Fallback for environments without cargo-watch: poll mtimes and rebuild.
watch:
	@echo "Watching for changes (Ctrl-C to stop)..."
	@stamp=""; \
	while true; do \
		new=$$(find src Cargo.toml -type f -printf '%T@ %p\n' 2>/dev/null | sort | cksum); \
		if [ -n "$$stamp" ] && [ "$$new" != "$$stamp" ]; then \
			echo "==> change detected: rebuilding"; \
			$(CARGO) build; \
		fi; \
		stamp="$$new"; \
		sleep 1; \
	done

install-watch:
	$(CARGO) install cargo-watch

run:
	$(CARGO) run

build:
	$(CARGO) build

check:
	$(CARGO) check

fmt:
	$(CARGO) fmt

clippy:
	$(CARGO) clippy --all-targets -- -D warnings

ci:
	$(CARGO) fmt --check
	$(CARGO) clippy --all-targets -- -D warnings
	$(CARGO) build