# IdleBolt

IdleBolt is a cross-platform utility designed to monitor mouse activity and lock the user account after a period of inactivity. It ensures that users remain interactive, making it especially useful for shared and public computers or workstations where security is a priority.

## Features

- 🌍 **Cross-Platform**: Works seamlessly on Windows, MacOS, and Linux.
- ⏰ **Customizable Timeout**: Set the inactivity duration after which the system will be locked.
- 📏 **Customizable Movement Threshold**: Specify how much the mouse must move to be considered active.
- 📜 **Configuration File**: Easily tailor the tool's behavior via a configuration file.
- 🔄 **Auto Recovery**: The utility is designed to handle errors gracefully, ensuring continuous monitoring.

## Installation

### From Source

```bash
git clone https://github.com/richardsondev/IdleBolt.git
cd IdleBolt
cargo build --release
```

The executable will be in the `target/release` directory.

### Download

You can also download pre-compiled binaries from the [Releases](https://github.com/richardsondev/IdleBolt/releases) page.

## Usage

1. Run the `IdleBolt` executable.
2. To customize the inactivity duration and movement threshold, create a `config.toml` in the same directory as the executable. Example:

```toml
timeout = 300 # in seconds, 300 seconds is 5 minutes
movement_threshold = 0.05 # 5% of the screen width/height
```

3. The system will automatically lock if the mouse doesn't move beyond the set threshold within the specified timeout.

## Contribution

Contributions are welcome! Please fork the repository and open a pull request with your changes or open an issue to discuss any ideas.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
