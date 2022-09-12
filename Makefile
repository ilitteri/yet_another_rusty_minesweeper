run:
	cargo run --release

build_image:
	docker build -t minesweeper .

run_image: build_image
	docker run --rm minesweeper
