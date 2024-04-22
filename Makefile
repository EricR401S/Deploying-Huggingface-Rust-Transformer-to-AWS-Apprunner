install:
		cargo clean &&\
			cargo build &&\
				cargo run

build:
	docker build -t autocomplete_actix .

rundocker:
	docker run -it --rm -p 8080:8080 autocomplete_actix

format:
	cargo fmt --quiet

lint:
	cargo clippy --quiet

test:
	cargo test --quiet

deploy-aws:
	aws ecr get-login-password --region us-east-1 | docker login --username AWS --password-stdin 667719398048.dkr.ecr.us-east-1.amazonaws.com
	docker build -t autocompleteme .
	docker tag autocompleteme:latest 667719398048.dkr.ecr.us-east-1.amazonaws.com/autocompleteme:latest
	docker push 667719398048.dkr.ecr.us-east-1.amazonaws.com/autocompleteme:latest

local-run: install

local-docker-run: build rundocker

all: format lint test run