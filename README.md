# DJI Tello Drone Controller

A simple interactive command-line application written in Rust for controlling DJI Tello drones.

## Overview

This application provides a GDB-like interactive shell interface for controlling a DJI Tello drone. It allows you to send commands to the drone through a simple command-line interface, with support for multiple commands in a single input.

## Requirements

- Rust and Cargo installed on your system
- DJI Tello or Tello EDU drone
- Computer with Wi-Fi capability

## Compatibility

This library has been tested with:
- DJI Tello (basic model)
- DJI Tello EDU

Please note that different Tello models may have varying levels of support for certain commands:
- Basic Tello drones may have limited photo storage capabilities
- Some functions might work differently based on firmware version
- Media handling capabilities vary between models

## Getting Started

1. Power on your DJI Tello drone
2. Connect to the drone's Wi-Fi network from your computer
3. Clone this repository and navigate to the project directory
4. Run the application:

```
cargo run
```

## Features

### Enhanced Interactive Command Mode

The application provides a shell-like interface with advanced features for an improved user experience:

- **Command History**: Use the up and down arrow keys to navigate through previously entered commands
- **Command Editing**: Use left and right arrow keys to move within the current command for editing
- **Tab Completion**: Press the Tab key to autocomplete commands
- **History Search**: Use Ctrl+R to search through command history
- **Persistent History**: Command history is saved between sessions in the ~/.tello_history file

The command prompt looks like this:

```
> 
```

### Available Commands

Currently, the application supports the following commands:

#### Application Control

- `help`: Show a list of all available commands
  - Example: `help` (displays all commands with descriptions)
  - Can be used at any time during operation

- `wait <seconds>`: Insert a specific delay between commands
  - Example: `wait 2.5` (wait for 2.5 seconds before executing the next command)
  - Useful for creating more precise flight sequences
  - Can be used in command chains with semicolons: `takeoff; wait 5; land`

- `exit`: Exit the application

#### Basic Flight Controls

- `takeoff [height]`: Take off with an optional height parameter in meters
  - Default height: 1 meter
  - Maximum allowed height: 8 meters
  - Example: `takeoff 2` (take off and hover at 2 meters)
  
- `land`: Land the drone safely
  - Example: `land`

#### Movement Controls

- `forward <distance>`: Move the drone forward by the specified distance in centimeters
  - Range: 1-500 cm
  - Example: `forward 100` (move forward 1 meter)

- `back <distance>`: Move the drone backward by the specified distance in centimeters
  - Range: 1-500 cm
  - Example: `back 50` (move backward 0.5 meters)

- `left <distance>`: Move the drone left by the specified distance in centimeters
  - Range: 1-500 cm
  - Example: `left 30` (move left 0.3 meters)

- `right <distance>`: Move the drone right by the specified distance in centimeters
  - Range: 1-500 cm
  - Example: `right 80` (move right 0.8 meters)

- `up <distance>`: Move the drone up by the specified distance in centimeters
  - Range: 1-500 cm
  - Example: `up 100` (move up 1 meter)

- `down <distance>`: Move the drone down by the specified distance in centimeters
  - Range: 1-500 cm
  - Example: `down 50` (move down 0.5 meters)

#### Rotation Controls

- `rotate_cw <degrees>`: Rotate the drone clockwise by the specified number of degrees
  - Example: `rotate_cw 90` (rotate 90 degrees clockwise)

- `rotate_ccw <degrees>`: Rotate the drone counter-clockwise by the specified number of degrees
  - Example: `rotate_ccw 45` (rotate 45 degrees counter-clockwise)

- `camera_to_center <x> <y>`: Point the camera towards a specific center point
  - Example: `camera_to_center 0 0` (point camera towards the center point at coordinates (0,0))

- `camera_from_center <x> <y>`: Point the camera away from a specific center point
  - Example: `camera_from_center 0 0` (point camera away from the center point)

#### Position Management

- `position <x> <y> <z>`: Set the current position of the drone for camera positioning calculations
  - Example: `position 1 2 3` (set drone position to coordinates (1,2,3))
  
- `get_position`: Get the current tracked position of the drone
  - Example: `get_position` (displays current X, Y, Z coordinates)

#### Media Management

- `media list`: List all media files stored on the drone
  - Example: `media list`
  - Note: Some Tello models have limited media listing capabilities

- `media download <filename>`: Download a specific file from the drone
  - Example: `media download photo_01.jpg`

- `media direct <filename>`: Download a specific file using direct TCP connection
  - Example: `media direct photo_01.jpg`
  - Uses port 8888 for more efficient file transfer
  - This is a more reliable method for large files

- `media delete <filename>`: Delete a specific file from the drone
  - Example: `media delete photo_01.jpg`

- `media deleteall`: Delete all media files from the drone
  - Example: `media deleteall`

- `media path <path>`: Set the local directory path where downloaded files are stored
  - Example: `media path /home/user/tello_photos`
  - Default path: `./tello_media`

#### Camera Controls

- `photo`: Take a photo with the drone's camera
  - Example: `photo`
  - The library will try multiple commands to ensure compatibility with different Tello models
  - Note: Some models may not store photos internally and require the official app

- `video start`: Start recording video
  - Example: `video start`
  - Enables the video stream which can be captured

- `video stop`: Stop recording video
  - Example: `video stop`

### Multiple Commands

You can execute multiple commands in sequence by separating them with semicolons (`;`):

```
> takeoff 2; forward 100; photo; back 100; land
```

This will instruct the drone to:
1. Take off to a height of 2 meters
2. Move forward by 1 meter
3. Take a photo
4. Move back by 1 meter
5. Land

## Example: Flying in a Square Pattern

Here's how to make the drone fly in a square pattern of 1 meter per side, taking a photo at each corner:

```
> takeoff 1.5
> forward 100
> photo
> right 100
> photo
> back 100
> photo
> left 100
> photo
> land
```

## Example: Flying in a Circle with Camera Pointing to Center

Here's an example of how to make the drone fly in a circle with a radius of 5 meters at a height of 3 meters, with the camera always pointed towards the center of the circle, while recording video of the entire flight:

```
> takeoff 3
> video start
> position 5 0 3; camera_to_center 0 0; rotate_cw 45; position 3.5 3.5 3; camera_to_center 0 0; rotate_cw 45; position 0 5 3; camera_to_center 0 0; rotate_cw 45; position -3.5 3.5 3; camera_to_center 0 0; rotate_cw 45; position -5 0 3; camera_to_center 0 0; rotate_cw 45; position -3.5 -3.5 3; camera_to_center 0 0; rotate_cw 45; position 0 -5 3; camera_to_center 0 0; rotate_cw 45; position 3.5 -3.5 3; camera_to_center 0 0; rotate_cw 45; position 5 0 3
> video stop
> media list
> media download circle_flight.mp4
> land
```

## Technical Details

The application connects to the Tello drone via UDP on port 8889, following the official [Tello SDK 2.0 Protocol](https://dl-cdn.ryzerobotics.com/downloads/Tello/Tello%20SDK%202.0%20User%20Guide.pdf).

Key components:

- `src/main.rs`: Contains the main application logic and interactive command loop
- `src/tello.rs`: Implements the Tello struct and methods for communicating with the drone
- `src/tello_movement.rs`: Contains movement-related methods for the Tello struct

### Network Communication

The application uses several UDP ports for different purposes:
- Port 8889: Command communication with the drone
- Port 8890: Local port for receiving responses
- Port 8891: Receiving state/telemetry information
- Port 8888: Reserved for direct file transfers

### Telemetry and Response Handling

The library intelligently handles drone responses:
- Distinguishes between command responses and telemetry data
- Automatically processes telemetry data when received instead of command responses
- Provides meaningful feedback even when the drone's response format varies

### Media Files

The media files captured by the drone are:
- Temporarily stored on the drone's internal memory
- Can be downloaded to your computer using the `media download` command
- Can be downloaded more efficiently using the `media direct` command for TCP-based transfers
- Can be deleted from the drone to free up space

By default, downloaded files are stored in the `./tello_media` directory, but you can change this using the `media path` command.

### Photo Capture Compatibility

Different Tello models use different commands for photo capture. This library automatically:
1. Attempts to use the `snapshot` command first
2. If that fails, tries the `takepic` command for Tello EDU models
3. Provides clear feedback about which method worked

Note that some Tello models don't support internal photo storage and only work with the official app.

## Known Limitations

- Some Tello models have limited media handling capabilities
- Photo capture might not work on all drone models or firmware versions
- The drone must be connected to the computer's Wi-Fi, which means you can't use Wi-Fi for internet connection simultaneously without additional hardware
- Media listing may not be fully supported on some models

## Troubleshooting

### Connection Issues
- Ensure you're connected to the Tello's Wi-Fi network
- Try restarting both the drone and the application
- Check that no other application is using the required UDP ports (8889, 8890, 8891)

### Command Response Issues
- If commands are failing, check battery level with `state` command
- Ensure you're within the operational range of the drone
- Some commands may not be available on certain Tello models

## Future Enhancements

Planned features for future versions:
- Support for more drone commands (flip, complex movement patterns)
- Live video streaming preview
- Telemetry data visualization
- Command history and auto-completion
- Predefined flight patterns and automated sequences

## License

This project is open-source and available under the MIT License.

## Contribution

Contributions are welcome! Feel free to submit issues or pull requests.