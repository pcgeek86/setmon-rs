# SetMon

The `setmon` CLI tool is used to set LCD monitor brightness.

## Installation

Make sure [Rust is installed](https://rustup.rs) before you attempt to install this utility.
Then, use the following command.

```
cargo install setmon
```

## Usage

### List Monitor IDs

The following command will enumerate all attached displays.
Decide which monitor ID you want to set the brightness for, and use it in the next command.

```
setmon list
```

### Set Monitor Brightness

The `setmon set` command updates the brightness, for the specified monitor ID, to the value `0 - 100`.

```
setmon set <monitor_id> <brightness>
```

## License

MIT - 2023 Trevor Sullivan