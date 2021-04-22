# rust-api

simple api server implemented in Rust...

### Dependencies:

- [actix-web][actix-web]
- [actix-rt][actix-rt]
- [serde_json][serde-json]
- [sudoku][sudoku]


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
[serde-json]: https://crates.io/crates/serde_json
[sudoku]: https://crates.io/crates/sudoku
[local]: http://0.0.0.0:8080
[puzzle]: http://0.0.0.0:8080/api/puzzle
