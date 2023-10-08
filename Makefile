build:
	sam build --beta-features

server: build
	sam local start-api \
		--port 3001 \
		--force-image-build \
		--config-env=local \
		--config-file="samconfig.toml"

lint:
	cd lambda && cargo fmt --check
	cd lambda && cargo clippy -- -Dwarnings

lint-fix:
	cd lambda && cargo fmt

install-deps:
	pip3 install cargo-lambda
