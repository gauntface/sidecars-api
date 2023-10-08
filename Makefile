build:
	sam build --beta-features

server: build
	sam local start-api \
		--port 3001 \
		--force-image-build \
		--config-env=local \
		--config-file="samconfig.toml"
