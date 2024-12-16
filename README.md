# Loklang

Loklang is a local audio streaming service that can be used to access audio files on the host from any device on the network.

## Configuration

Create a file named `config.ini` in the same directory as the `loklang.exe` file and add any supported config options to that file.
```ini
[config]
rootdir = "path/to/music"
```

### Currently Supported Options

- `rootdir`: Root directory for the music files

## Development

1. Clone this repo
```shell
git clone git@github.com:Nemesis-AS/loklang-rs.git
```
2. Make any changes you wish
```shell
cargo run
```

## Building

1. Clone this repo
```shell
git clone git@github.com:Nemesis-AS/loklang-rs.git
```
2. Run cargo build
```shell
cargo build --release
```
