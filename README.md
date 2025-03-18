
# parsezilla 🦖

Parsezilla is a Rust-based address parsing API that leverages [libpostal](https://github.com/openvenues/libpostal) via Rust bindings. It exposes an HTTP endpoint to parse addresses into structured components. Built with Actix-Web for asynchronous performance, Parsezilla also supports customizable bind addresses and ports through CLI options.

## Features

- **Address Parsing:** Uses libpostal to split addresses into components.
- **Web API:** Exposes a `POST /parse` endpoint accepting JSON with an address.
- **Auto-Capitalization:** Capitalizes parsed address components.
- **Configurable Bind & Port:** Easily configure the host and port via CLI.
- **Rust-Powered:** Built with Actix-Web, Clap, and Colored for a robust experience.

## Prerequisites

- **Rust:** Latest stable version ([install Rust](https://www.rust-lang.org/tools/install))
- **libpostal:** Ensure libpostal is installed on your system. See the [libpostal installation guide](https://github.com/openvenues/libpostal).

## Installation

1. **Clone the repository:**

   ```bash
   git clone https://github.com/yourusername/parsezilla.git
   cd parsezilla
   ```

2. **Build the project:**

   ```bash
   cargo build --release
   ```

## Usage

### Running the Server

By default, the server binds to `127.0.0.1:8080`. You can customize the bind address and port via CLI arguments:

```bash
cargo run -- --bind 0.0.0.0 --port 8080
```

When the server starts, you'll see a colorful log message, for example:

```
🦖 Parsezilla is roaring at http://0.0.0.0:8080
```

### Testing the API

Send a POST request to the `/parse` endpoint with a JSON payload containing the address. For example, using `curl`:

```bash
curl -X POST http://localhost:8080/parse      -H "Content-Type: application/json"      -d '{"address": "Apt 4 18 Downey Street, 95926 Chico"}'
```

Expected response:

```json
[
  {"component": "unit", "value": "Apt 4"},
  {"component": "road", "value": "Downey Street"},
  {"component": "house_number", "value": "18"},
  {"component": "postcode", "value": "95926"},
  {"component": "city", "value": "Chico"}
]
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request for bug fixes or improvements.

Happy parsing with Parsezilla! 🦖
