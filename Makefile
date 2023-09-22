release:
	cargo build --release

test:
	cargo test

run:
	cargo run

wasm:
	wasm-pack build --release --target web --out-dir server/pkg

publish:
	cargo test
	cargo publish

docker: wasm
	docker build ./server -t git.betalupi.com/mark/daisy --no-cache
	docker push git.betalupi.com/mark/daisy
