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

#### Basic Flight Controls

- `takeoff [height]`: Take off with an optional height parameter in meters
  - Default height: 1 meter
  - Maximum allowed height: 8 meters
  - Example: `takeoff 2` (take off and hover at 2 meters)
  
- `land`: Land the drone safely
  - Example: `land`

#### Camera Controls

- `photo`: Take a photo with the drone's camera
  - Example: `photo`

- `video start`: Start recording video
  - Example: `video start`

- `video stop`: Stop recording video
  - Example: `video stop`

#### Media Management

- `media list`: List all media files stored on the drone
  - Example: `media list`

- `media download <filename>`: Download a specific file from the drone
  - Example: `media download photo_01.jpg`

- `media delete <filename>`: Delete a specific file from the drone
  - Example: `media delete photo_01.jpg`

- `media deleteall`: Delete all media files from the drone
  - Example: `media deleteall`

- `media path <path>`: Set the local directory path where downloaded files are stored
  - Example: `media path /home/user/tello_photos`
  - Default path: `./tello_media`

#### State and Telemetry

- `state`: Get the current state and telemetry data of the drone
  - Example: `state`

#### Application Control

- `exit`: Exit the application

### Multiple Commands

You can execute multiple commands in sequence by separating them with semicolons (`;`):

```
> takeoff 2; photo; land
```

This will instruct the drone to take off to a height of 2 meters, take a photo, and then land.

## Technical Details

The application connects to the Tello drone via UDP on port 8889, following the official [Tello SDK 2.0 Protocol](https://dl-cdn.ryzerobotics.com/downloads/Tello/Tello%20SDK%202.0%20User%20Guide.pdf).

Key components:

- `src/main.rs`: Contains the main application logic and interactive command loop
- `src/tello.rs`: Implements the Tello struct and methods for communicating with the drone

### Media Files

The media files captured by the drone are:
- Temporarily stored on the drone's internal memory
- Can be downloaded to your computer using the `media download` command
- Can be deleted from the drone to free up space

By default, downloaded files are stored in the `./tello_media` directory, but you can change this using the `media path` command.

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
- Media operations fail (e.g., file not found, disk full)

## Future Enhancements

Planned features for future versions:
- Support for more drone commands (flip, rotate, move in specific directions)
- Live video streaming preview
- Telemetry data visualization
- Command history and auto-completion
- Predefined flight patterns and automated sequences

## License

This project is open-source and available under the MIT License.

## Contribution

Contributions are welcome! Feel free to submit issues or pull requests.