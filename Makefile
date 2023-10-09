clean:
	rm -rf .aws-sam
	cd lambda && cargo clean

build:
	sam build --beta-features

server: build
	sam local start-api \
		--port 3001 \
		--force-image-build \
		--config-env=local \
		--config-file="samconfig.toml"

watch:
	npx nodemon --ext rs --watch './lambda/src/' --exec 'make build'

test:
	cd lambda && cargo test --verbose

test-coverage:
	cd lambda && cargo llvm-cov --html

lint:
	cd lambda && cargo fmt --check
	cd lambda && cargo clippy -- -Dwarnings

lint-fix:
	cd lambda && cargo fmt

install-deps:
	pip3 install cargo-lambda
