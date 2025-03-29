# DJI Tello Drone Controller

A simple interactive command-line application written in Rust for controlling DJI Tello drones.

## Overview

This application provides a GDB-like interactive shell interface for controlling a DJI Tello drone. It allows you to send commands to the drone through a simple command-line interface, with support for multiple commands in a single input.

## Requirements

- Rust and Cargo installed on your system
- DJI Tello or Tello EDU drone
- Computer with Wi-Fi capability

## Getting Started

1. Power on your DJI Tello drone
2. Connect to the drone's Wi-Fi network from your computer
3. Clone this repository and navigate to the project directory
4. Run the application:

```
cargo run
```

## Features

### Interactive Command Mode

The application provides a shell-like interface where you can enter commands to control the drone. The command prompt looks like this:

```
> 
```

### Available Commands

Currently, the application supports the following commands:

- `takeoff [height]`: Take off with an optional height parameter in meters
  - Default height: 1 meter
  - Maximum allowed height: 8 meters
  - Example: `takeoff 2` (take off and hover at 2 meters)
  
- `land`: Land the drone safely
  - Example: `land`

- `exit`: Exit the application

### Multiple Commands

You can execute multiple commands in sequence by separating them with semicolons (`;`):

```
> takeoff 2; land
```

This will instruct the drone to take off to a height of 2 meters and then land.

## Technical Details

The application connects to the Tello drone via UDP on port 8889, following the official [Tello SDK 2.0 Protocol](https://dl-cdn.ryzerobotics.com/downloads/Tello/Tello%20SDK%202.0%20User%20Guide.pdf).

Key components:

- `src/main.rs`: Contains the main application logic and interactive command loop
- `src/tello.rs`: Implements the Tello struct and methods for communicating with the drone

## Testing

The project includes both unit tests and integration tests:

### Unit Tests

Unit tests are located within the `src/tello.rs` file and include:

- Tests for core Tello functionality
- Mock implementation for testing without actual drone hardware
- Tests for command processing and validation
- Tests for error handling

Run the unit tests with:

```
make test
```

### Integration Tests

Integration tests are located in the `tests/` directory and test the application as a whole:

- `integration_tests.rs`: Tests for multiple command execution (semicolon-separated commands)
- Tests for parameter validation
- Tests for error handling in the command interface

These tests are marked as `#[ignore]` by default since they require special setup. To run them:

```
cargo test -- --include-ignored
```

## Error Handling

The application provides helpful error messages when:
- The connection to the drone fails
- A command fails to execute
- An invalid parameter is provided (e.g., excessive height)

## Future Enhancements

Planned features for future versions:
- Support for more drone commands (flip, rotate, move in specific directions)
- Video streaming support
- Telemetry data display
- Command history and auto-completion

## License

This project is open-source and available under the MIT License.

## Contribution

Contributions are welcome! Feel free to submit issues or pull requests.