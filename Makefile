build:
	wasm-pack build --target web

run-web:
	python3 -m http.server

open-web:
	open http://localhost:8000