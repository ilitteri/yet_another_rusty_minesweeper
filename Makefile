# Couldn't use PATH
BOARD_PATH?=./example.txt

run:
	cargo run --release ${BOARD_PATH}

build_image:
	docker build -t minesweeper .

run_image: build_image
	docker run --rm minesweeper
