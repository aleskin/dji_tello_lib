# DJI Tello Library Development Roadmap

This document outlines the planned features and improvements for the DJI Tello Library project.

## Action Plan for Expanding DJI Tello Application Functionality

### Basic Flight and Control Commands

**Movement Commands with Stubs Only**:
   - [ ] `flip [direction]` - perform a flip in the specified direction (l, r, f, b)
   - [ ] `speed [value]` - set speed between 1-100 cm/s

**Camera Settings**:
   - [ ] `exposure [level]` - set camera exposure level
   - [ ] `jpeg [quality]` - set JPEG quality

### Automated Flights

**Automatic Missions**:
   - [ ] `mission [filename]` - execute a sequence of commands from a file
   - [ ] `square [size]` - fly in a square pattern of specified size

**Interactive Controller Mode**:
   - [ ] `joystick` - control mode using joystick/gamepad

### Safety and Monitoring

**Safety Commands**:
   - [ ] `emergency` - emergency motor stop
   - [ ] `hover` - hover in place
   - [ ] `return` - automatic return to takeoff point

**Enhanced Status Monitoring**:
   - [ ] `temp` - check drone temperature

### Command Aliases

**Alias Management**:
   - [ ] `alias [name] [commands]` - create a named alias for a sequence of commands
   - [ ] `aliases` - list all available aliases
   - [ ] `unalias [name]` - remove an alias
   - [ ] Load/save aliases from/to a configuration file in user's home directory
   - [ ] Support for parameters in aliases

**Example Use Cases**:
   - Creating a "patrol" alias that combines multiple movement commands
   - Setting up camera presets with specific exposure and quality settings
   - Defining complex flight patterns as simple commands

### Package Building
   - [ ] Create Debian (.deb) package for easy installation on Ubuntu/Debian systems
   - [ ] Create RPM package for Fedora/Red Hat systems
   - [ ] Package dependencies and configuration properly

### Extended Testing
   - [ ] Create and test simple missions (e.g., square flight pattern)

## Short-term Goals (1-2 months)

### Core Functionality
- [ ] Implement mission planning system (pre-programmed sequences of movements)
- [ ] Improve error handling and recovery mechanisms
- [ ] Implement automatic emergency landing on low battery

### User Experience
- [ ] Create visual feedback for drone's current position and orientation
- [ ] Develop a configuration file system for storing preferences and aliases

### Media Management
- [ ] Add video streaming to a local port for viewing in VLC or browser
- [ ] Implement automatic media syncing to local storage
- [ ] Create thumbnail generation for downloaded media
- [ ] Implement direct video capture through UDP port used by the DJI Tello video stream
- [ ] Add robust mechanisms for handling files coming from the drone
- [ ] Create screenshot capture functionality from video stream for models that don't support internal photo storage

### Testing
- [ ] Create a comprehensive test suite with mocked hardware responses
- [ ] Add integration tests for all main flight sequences
- [ ] Implement connection resilience tests
- [ ] Create a simulated flight mode for development without a physical drone

## Mid-term Goals (3-6 months)

### Enhanced Control
- [ ] Implement multi-drone control capabilities
- [ ] Add support for swarm choreography
- [ ] Create a waypoint navigation system with visual map
- [ ] Implement follow-me functionality using computer vision

### Advanced Features
- [ ] Implement computer vision for object tracking
- [ ] Add obstacle avoidance using built-in sensors
- [ ] Create a panorama photo stitching feature
- [ ] Develop 3D mapping capability using drone's camera

### Documentation and Tutorials
- [ ] Create comprehensive API documentation
- [ ] Develop video tutorials for common operations
- [ ] Write guides for extending the library functionality
- [ ] Publish example projects for different use cases

## Long-term Goals (6+ months)

### Professional Features
- [ ] Implement autonomous flight planning with mission objectives
- [ ] Add AI-powered flight patterns for cinematography
- [ ] Develop environmental sensing and mapping tools
- [ ] Create a plugin system for third-party extensions

### Performance Optimization
- [ ] Optimize battery consumption for longer flights
- [ ] Improve real-time video processing performance
- [ ] Enhance stability in challenging weather conditions
- [ ] Implement predictive positioning for smoother flights

### Integration
- [ ] Add support for third-party controllers and joysticks
- [ ] Implement integration with popular mapping platforms
- [ ] Create connections to cloud storage services for media
- [ ] Develop integration with photo/video editing software

### Package Distribution
- [ ] Create installation packages for major Linux distributions
- [ ] Provide automated build system for packages
- [ ] Set up continuous integration for package building
- [ ] Create installer for Windows and macOS platforms

## Technical Debt and Maintenance
- [ ] Regular code refactoring for clarity and performance
- [ ] Dependency updates and security patches
- [ ] Cross-platform testing and compatibility
- [ ] Performance benchmarking and optimization

## Next Immediate Tasks

1. Add `exposure` and `jpeg` commands for camera configuration
2. Create a basic mission system for reading commands from files
3. Implement safety commands (`emergency`, `hover`, `return`)
4. Add `temp` command for monitoring drone temperature
5. Develop alias system for custom command sequences
6. Set up package building for Debian and RPM systems

## Additional Features

### Debug and Diagnostics
- [ ] Implement debug mode with detailed information about sent commands and received data
- [ ] Display drone version and system information during connection
- [ ] Improve the `info` command to fully report all available drone information
- [ ] Remove the standard 3-second wait time for commands, use command status instead

### Status Monitoring and Control Flow
- [ ] Add command status checking to verify if a command has completed execution
- [ ] Implement command queuing based on drone readiness status
- [ ] Create battery status monitoring with warnings at critical levels
- [ ] Enhance status command to return comprehensive drone state information

### Media and Video Streaming
- [ ] Fix existing media functionality
- [ ] Document and implement the video streaming protocol
- [ ] Enable real-time image transmission
- [ ] Add capability to save video stream to files in a designated folder
- [ ] Implement real-time camera feed display in a small window on the monitor
- [ ] Create adjustable window size for the live camera feed

### Positioning and Environmental Data
- [ ] Display current coordinates during flight
- [ ] Monitor and report wind conditions
- [ ] Create visualization of current position and orientation

### Advanced Movement Patterns
- [ ] Implement circular movement around a central point ("wheel" pattern)
- [ ] Scale circular movement based on current altitude
- [ ] Create advanced feature for circular movement in arbitrary planes
- [ ] Add rotation during circular movement execution