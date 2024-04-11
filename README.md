## Weather

---

Command line tool for getting weather information

### Prerequisites

Rust is required.
Install rust via [rustup](https://rustup.rs/).

Dependencies

- [clap](https://crates.io/crates/clap)
- [comfy-table](https://crates.io/crates/comfy-table)
- [dotenv](https://crates.io/crates/dotenv)
- [reqwest](https://crates.io/crates/reqwest)
- [serde](https://crates.io/crates/serde)
- [serde_json](https://crates.io/crates/serde_json)
- [tokio](https://crates.io/crates/tokio)

### Build

Run `cargo build --release` to build the binary.

### Register your API key

Register your API key at [OpenWeatherMap](https://home.openweathermap.org/users/sign_up)

Copy `.env.sample` to `.env` file and add your API

```
API_KEY=<YOUR_API_KEY>
```

### Run

```sh
./target/release/weather <CITY>
```

Or

```sh
cargo run --release <CITY>
```
