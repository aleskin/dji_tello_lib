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

#### Rotation Controls

- `rotate_cw <degrees>`: Rotate the drone clockwise by the specified number of degrees
  - Example: `rotate_cw 90` (rotate 90 degrees clockwise)

- `rotate_ccw <degrees>`: Rotate the drone counter-clockwise by the specified number of degrees
  - Example: `rotate_ccw 45` (rotate 45 degrees counter-clockwise)

- `camera_to_center <x> <y>`: Point the camera towards a specific center point
  - Example: `camera_to_center 0 0` (point camera towards the center point at coordinates (0,0))

- `camera_from_center <x> <y>`: Point the camera away from a specific center point
  - Example: `camera_from_center 0 0` (point camera away from the center point)

- `position <x> <y> <z>`: Set the current position of the drone for camera positioning calculations
  - Example: `position 1 2 3` (set drone position to coordinates (1,2,3))

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

## Example: Flying in a Circle with Camera Pointing to Center

Here's an example of how to make the drone fly in a circle with a radius of 5 meters at a height of 3 meters, with the camera always pointed towards the center of the circle, while recording video of the entire flight:

```
> takeoff 3
> video start
> position 5 0 3
> camera_to_center 0 0
> rotate_cw 45
> position 3.5 3.5 3
> camera_to_center 0 0
> rotate_cw 45
> position 0 5 3
> camera_to_center 0 0
> rotate_cw 45
> position -3.5 3.5 3
> camera_to_center 0 0
> rotate_cw 45
> position -5 0 3
> camera_to_center 0 0
> rotate_cw 45
> position -3.5 -3.5 3
> camera_to_center 0 0
> rotate_cw 45
> position 0 -5 3
> camera_to_center 0 0
> rotate_cw 45
> position 3.5 -3.5 3
> camera_to_center 0 0
> rotate_cw 45
> position 5 0 3
> video stop
> media list
> media download circle_flight.mp4
> land
```

This sequence of commands will:
1. Take off to a height of 3 meters
2. Start video recording
3. Fly in an 8-point circle with a radius of 5 meters
4. At each position, point the camera towards the center (0,0)
5. Stop the video recording after completing the circle
6. Download the recorded video to your computer
7. Land the drone

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
- Tests for rotation and camera positioning commands

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
- An invalid parameter is provided (e.g., excessive height, invalid rotation degrees)
- Media operations fail (e.g., file not found, disk full)

## Future Enhancements

Planned features for future versions:
- Support for more drone commands (flip, move in specific directions)
- Live video streaming preview
- Telemetry data visualization
- Command history and auto-completion
- Predefined flight patterns and automated sequences

## License

This project is open-source and available under the MIT License.

## Contribution

Contributions are welcome! Feel free to submit issues or pull requests.