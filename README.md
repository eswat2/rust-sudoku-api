# rust-api

simple api server implemented in Rust...

### Dependencies:

[![actix-cors][badge-actix-cors]][actix-cors]
[![actix-web][badge-actix-web]][actix-web]
[![base64][badge-base64]][base64]
[![serde_json][badge-serde-json]][serde-json]
[![sudoku][badge-sudoku]][sudoku]


### Usage:

`cargo run`

- [localhost:8080][local]
- [localhost:8080/api/puzzle][puzzle]

### Sample:

```
{
  "metrics": {
    "counts": {
      "blanks": 56,
      "clues": 25
    },
    "nanos": {
      "generate": 278502,
      "solve": 5358
    }
  },
  "puzzle": "51..6...28.2......6..3.8.9.4..9..8.........5.1564............192...9...8...54....",
  "ref": "NTE5NzY0MzgyODMyMTU5NjQ3Njc0MzI4MTk1NDI3OTM1ODYxOTgzMjE2NzU0MTU2NDg3OTIzMzY1ODcyNDE5MjQxNjkzNTc4Nzk4NTQxMjM2"
}
```

| Property | Description |
| ---: | :--- |
| **metrics** | _insight into the puzzle counts & times_ |
| **puzzle** | _a string representing the puzzle board (dots are blank cells)_ |
| **ref** | _the base64 encoded solution to the puzzle_ |

[actix-cors]: https://crates.io/crates/actix-cors
[actix-web]: https://crates.io/crates/actix-web
[base64]: https://crates.io/crates/base64
[serde-json]: https://crates.io/crates/serde_json
[sudoku]: https://crates.io/crates/sudoku
[local]: http://0.0.0.0:8080
[puzzle]: http://0.0.0.0:8080/api/puzzle

[badge-actix-cors]: https://img.shields.io/badge/crates.io-actix--cors-orange
[badge-actix-web]: https://img.shields.io/badge/crates.io-actix--web-orange
[badge-base64]: https://img.shields.io/badge/crates.io-base64-orange
[badge-serde-json]: https://img.shields.io/badge/crates.io-serde__json-orange
[badge-sudoku]: https://img.shields.io/badge/crates.io-sudoku-orange
