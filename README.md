# rusty_minesweeper

## Local Run

```bash
make run BOARD_PATH=./example.txt
```

The directive above has an optional argument `BOARD_PATH` which is the path to the board file. If not provided, the default board file is used (`./example.txt`).

## Container Run

```bash
make run_image
```

## TODOs

- `From<Vec<Vec<Cell>>>` should be a `TryFrom<Vec<Vec<Cell>>>` because the matrix could not be square
    - `From<SquareMatrix>` where `SquareMatrix` is a Trait that should be implemented by `C`.
