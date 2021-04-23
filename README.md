# rust-api

simple api server implemented in Rust...

### Dependencies:

[![actix-web][badge-actix-web]][actix-web]
[![actix-rt][badge-actix-rt]][actix-rt]
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
  "blanks": 57,
  "puzzle": "1.......2.47.6..5...3......56...28.......91.....1..46.9.68..2...3........5.9....1",
  "solution": "185794632247361958693528714561432879374689125829175463916853247438217596752946381",
  "solved": 112097,
  "time": 6171687,
  "units": "ns"
}
```


[actix-web]: https://crates.io/crates/actix-web
[actix-rt]: https://crates.io/crates/actix-rt
[base64]: https://crates.io/crates/base64
[serde-json]: https://crates.io/crates/serde_json
[sudoku]: https://crates.io/crates/sudoku
[local]: http://0.0.0.0:8080
[puzzle]: http://0.0.0.0:8080/api/puzzle

[badge-actix-web]: https://img.shields.io/badge/crates.io-actix--web-orange
[badge-actix-rt]: https://img.shields.io/badge/crates.io-actix--rt-orange
[badge-base64]: https://img.shields.io/badge/crates.io-base64-orange
[badge-serde-json]: https://img.shields.io/badge/crates.io-serde__json-orange
[badge-sudoku]: https://img.shields.io/badge/crates.io-sudoku-orange
