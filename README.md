# Spotify Auto Renewal

This is a Rust tool for logging into a Spotify account from the India region using a proxy. 
The tool starts a Docker container with a pre-configured proxy, reads a Spotify authentication cookie token from a `token.txt` file, and performs an HTTP GET request to Spotify's servers every 3 days.

## Prerequisites

To use this tool, you will need to have the following installed on your system:

- Docker
- Rust

## Installation

1. Clone this repository to your local machine using:

```
git clone https://github.com/headneZzz/spotify-auto-renewal.git
```

2. Change to the directory where the repository was cloned:

```
cd spotify-auto-renewal
```

3. Build the Rust project using:

```
cargo build --release
```

## Usage

1. Make sure that you have a `token.txt` file in the project directory that contains your Spotify authentication cookie token.

2. Start the Docker container with the pre-configured proxy using:

```
docker run -p 9050:9050 -e LOCATION=IN -d --name torproxy dperson/torproxy
```

3. Run the Rust tool using:

```
cargo run --release
```

This will start the tool and perform an initial HTTP GET request to Spotify's servers.

4. The tool will automatically perform subsequent HTTP GET requests to Spotify's servers every 3 days as long as it is running.

## Contributing

If you would like to contribute to this project, please feel free to fork the repository and submit a pull request.

## License

This project is licensed under the MIT License.