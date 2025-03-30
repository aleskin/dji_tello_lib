/*
 * DJI Tello Drone Controller Library
 *
 * Copyright (c) 2025 aleskin
 *
 * This file is part of dji_tello_lib.
 *
 * dji_tello_lib is free software: you can redistribute it and/or modify
 * it under the terms of the MIT License as published.
 *
 * Created: March 30, 2025
 */

use std::io::{self};
use std::thread;
use std::time::Duration;
use std::collections::HashMap;
use std::path::PathBuf;
use rustyline::error::ReadlineError;
use rustyline::{Editor, Config, CompletionType};
use rustyline::completion::{Completer, Pair};
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::validate::Validator;
use rustyline::Helper;
use crate::tello::Tello;

/// Structure for command auto-completion
pub struct CommandCompleter {
    commands: Vec<String>,
}

impl CommandCompleter {
    fn new() -> Self {
        let commands = vec![
            "takeoff", "land", "state", "forward", "back", "left", "right", "up", "down",
            "wait", "photo", "video start", "video stop", 
            "media list", "media download", "media direct", "media delete", "media deleteall", "media path",
            "position", "get_position", "rotate_cw", "rotate_ccw", 
            "camera_to_center", "camera_from_center", "exit"
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        
        CommandCompleter { commands }
    }
}

impl Completer for CommandCompleter {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        _pos: usize,
        _ctx: &rustyline::Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Self::Candidate>)> {
        // Split the line by semicolons to support completion for multiple commands
        let parts: Vec<&str> = line.split(';').collect();
        
        // Get the last part that is being typed
        let current_part = parts.last().unwrap_or(&"").trim();
        let start_pos = line.len() - current_part.len();
        
        // Find matching commands
        let matches: Vec<Pair> = self
            .commands
            .iter()
            .filter(|cmd| cmd.starts_with(current_part))
            .map(|cmd| Pair {
                display: cmd.clone(),
                replacement: cmd.clone() + " ",
            })
            .collect();
        
        Ok((start_pos, matches))
    }
}

// Define a custom helper that implements necessary traits for rustyline
pub struct CommandHelper {
    completer: CommandCompleter,
}

impl CommandHelper {
    fn new() -> Self {
        CommandHelper {
            completer: CommandCompleter::new(),
        }
    }
}

// Implement necessary traits for CommandHelper
impl Helper for CommandHelper {}
impl Hinter for CommandHelper {
    type Hint = String;
    fn hint(&self, _line: &str, _pos: usize, _ctx: &rustyline::Context<'_>) -> Option<Self::Hint> {
        None
    }
}
impl Highlighter for CommandHelper {}
impl Validator for CommandHelper {}

// Implement the Completer trait for CommandHelper by delegating to CommandCompleter
impl Completer for CommandHelper {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        ctx: &rustyline::Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Self::Candidate>)> {
        self.completer.complete(line, pos, ctx)
    }
}

/// Structure for managing command-specific delays
pub struct CommandDelay {
    delays: HashMap<&'static str, u64>,
}

impl CommandDelay {
    pub fn new() -> Self {
        let mut delays = HashMap::new();
        // Movement commands need moderate delay
        delays.insert("forward", 800);
        delays.insert("back", 800);
        delays.insert("left", 800);
        delays.insert("right", 800);
        delays.insert("up", 800);
        delays.insert("down", 800);
        
        // Flight commands need longer delay
        delays.insert("takeoff", 3000);
        delays.insert("land", 3000);
        
        // Rotation commands
        delays.insert("rotate_cw", 1000);
        delays.insert("rotate_ccw", 1000);
        
        // Camera commands need minimal delay
        delays.insert("photo", 500);
        delays.insert("video", 500);
        
        // Media commands can be quick
        delays.insert("media", 200);
        delays.insert("state", 100);
        delays.insert("position", 100);
        delays.insert("get_position", 100);
        
        // Camera pointing commands
        delays.insert("camera_to_center", 1000);
        delays.insert("camera_from_center", 1000);
        
        CommandDelay { delays }
    }
    
    pub fn get_delay(&self, command: &str) -> u64 {
        *self.delays.get(command).unwrap_or(&500)
    }
}

/// Run the interactive command-line interface with enhanced editing capabilities
pub fn run_command_line(mut drone: Tello) -> io::Result<()> {
    // Create command delay settings
    let command_delays = CommandDelay::new();
    
    // Setup rustyline with configuration
    let config = Config::builder()
        .history_ignore_space(true)
        .completion_type(CompletionType::List)
        .build();
    
    // Create editor with history and command completion
    let helper = CommandHelper::new();
    let mut rl = match Editor::with_config(config) {
        Ok(editor) => editor,
        Err(err) => {
            return Err(io::Error::new(io::ErrorKind::Other, 
                format!("Failed to initialize command line editor: {}", err)))
        }
    };
    
    rl.set_helper(Some(helper));
    
    // Tab completion is enabled by default with the helper
    // No need to explicitly bind keys
    
    // Try to load history from previous sessions
    let history_path = get_history_file_path();
    if rl.load_history(&history_path).is_err() {
        println!("No previous history found.");
    }
    
    println!("Tello Control - Interactive Mode");
    println!("Type commands to control the drone. Separate multiple commands with semicolons (;)");
    println!("Available commands:");
    print_available_commands();
    println!("Use arrow keys to navigate, Tab for completion, Ctrl+R to search history");
    
    // Main command loop
    loop {
        // Read line with editing capabilities
        let readline = rl.readline("> ");
        
        match readline {
            Ok(line) => {
                // Add non-empty entries to history
                if !line.trim().is_empty() {
                    rl.add_history_entry(&line);
                }
                
                // Split input by semicolons to handle multiple commands
                let commands: Vec<&str> = line.trim().split(';').map(|s| s.trim()).collect();
                
                for cmd in commands {
                    if cmd.is_empty() {
                        continue;
                    }
                    
                    let parts: Vec<&str> = cmd.split_whitespace().collect();
                    
                    if parts.is_empty() {
                        continue;
                    }
                    
                    // Check if it's a wait command
                    if parts[0] == "wait" && parts.len() > 1 {
                        if let Ok(seconds) = parts[1].parse::<f64>() {
                            let millis = (seconds * 1000.0) as u64;
                            println!("Waiting for {} seconds...", seconds);
                            thread::sleep(Duration::from_millis(millis));
                            println!("Wait completed");
                            continue;
                        } else {
                            println!("Invalid wait time: {}. Please specify a number of seconds.", parts[1]);
                            continue;
                        }
                    }
                    
                    // Execute the command
                    if let Err(e) = execute_command(&mut drone, &parts) {
                        if let Some(message) = e.get_ref() {
                            if message.to_string() == "Exit requested" {
                                // Save command history before exiting
                                if let Err(history_err) = rl.save_history(&history_path) {
                                    eprintln!("Warning: Failed to save command history: {}", history_err);
                                }
                                return Err(e);
                            }
                        }
                        eprintln!("Error executing command: {}", e);
                    }
                    
                    // Add a delay between commands based on the command type
                    let delay = command_delays.get_delay(parts[0]);
                    
                    if delay > 0 {
                        println!("Waiting for command completion ({} ms)...", delay);
                        thread::sleep(Duration::from_millis(delay));
                    }
                }
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C pressed, exiting...");
                break;
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D pressed, exiting...");
                break;
            },
            Err(err) => {
                eprintln!("Error reading line: {:?}", err);
                break;
            }
        }
    }
    
    // Save history when exiting normally
    if let Err(err) = rl.save_history(&history_path) {
        eprintln!("Error saving command history: {}", err);
    }
    
    Ok(())
}

/// Get the path to the history file
fn get_history_file_path() -> PathBuf {
    let mut home_dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
    home_dir.push(".tello_history");
    home_dir
}

/// Print available commands
fn print_available_commands() {
    println!("  takeoff [height] - Take off (optional height in meters, default 1m, max 8m)");
    println!("  land           - Land the drone");
    println!("  state          - Get current drone state/telemetry");
    
    // Movement commands
    println!("  forward <distance> - Move forward by specified distance in cm (1-500)");
    println!("  back <distance>    - Move backward by specified distance in cm (1-500)");
    println!("  left <distance>    - Move left by specified distance in cm (1-500)");
    println!("  right <distance>   - Move right by specified distance in cm (1-500)");
    println!("  up <distance>      - Move up by specified distance in cm (1-500)");
    println!("  down <distance>    - Move down by specified distance in cm (1-500)");
    
    // Wait command
    println!("  wait <seconds>     - Wait specified number of seconds between commands");
    
    // Camera commands
    println!("  photo          - Take a photo");
    println!("  video start    - Start recording video");
    println!("  video stop     - Stop recording video");
    println!("  media list     - List media files on the drone");
    println!("  media download <filename> - Download media file from drone");
    println!("  media direct <filename>   - Download media using direct TCP connection");
    println!("  media delete <filename>   - Delete media file from drone");
    println!("  media deleteall - Delete all media files from drone");
    println!("  media path <path> - Set download path for media files");
    
    // Position commands
    println!("  position <x> <y> <z> - Set current drone position for camera positioning");
    println!("  get_position         - Display current drone position");
    
    // Rotation commands
    println!("  rotate_cw <degrees> - Rotate clockwise by specified degrees");
    println!("  rotate_ccw <degrees> - Rotate counter-clockwise by specified degrees");
    println!("  camera_to_center <x> <y> - Point camera towards the specified center point");
    println!("  camera_from_center <x> <y> - Point camera away from the specified center point");
    
    println!("  exit           - Exit the application");
}

/// Execute a single command
fn execute_command(drone: &mut Tello, parts: &[&str]) -> io::Result<()> {
    match parts[0] {
        "takeoff" => {
            let height = if parts.len() > 1 {
                match parts[1].parse::<f32>() {
                    Ok(h) => Some(h),
                    Err(_) => {
                        println!("Warning: Invalid height value. Using default height (1m).");
                        None
                    }
                }
            } else {
                None
            };
            
            if let Err(e) = drone.takeoff(height) {
                eprintln!("Takeoff failed: {}", e);
            } else {
                println!("Takeoff command executed successfully");
            }
        },
        "land" => {
            if let Err(e) = drone.land() {
                eprintln!("Landing failed: {}", e);
            } else {
                println!("Landing command executed successfully");
            }
        },
        "forward" => {
            if parts.len() < 2 {
                println!("Please specify distance for forward movement");
                return Ok(());
            }
            
            match parts[1].parse::<i32>() {
                Ok(distance) => {
                    match drone.forward(distance) {
                        Ok(_) => println!("Moved forward by {} cm", distance),
                        Err(e) => eprintln!("Failed to move forward: {}", e),
                    }
                },
                Err(_) => {
                    eprintln!("Invalid distance value: {}", parts[1]);
                }
            }
        },
        "back" => {
            if parts.len() < 2 {
                println!("Please specify distance for backward movement");
                return Ok(());
            }
            
            match parts[1].parse::<i32>() {
                Ok(distance) => {
                    match drone.back(distance) {
                        Ok(_) => println!("Moved backward by {} cm", distance),
                        Err(e) => eprintln!("Failed to move backward: {}", e),
                    }
                },
                Err(_) => {
                    eprintln!("Invalid distance value: {}", parts[1]);
                }
            }
        },
        "left" => {
            if parts.len() < 2 {
                println!("Please specify distance for left movement");
                return Ok(());
            }
            
            match parts[1].parse::<i32>() {
                Ok(distance) => {
                    match drone.left(distance) {
                        Ok(_) => println!("Moved left by {} cm", distance),
                        Err(e) => eprintln!("Failed to move left: {}", e),
                    }
                },
                Err(_) => {
                    eprintln!("Invalid distance value: {}", parts[1]);
                }
            }
        },
        "right" => {
            if parts.len() < 2 {
                println!("Please specify distance for right movement");
                return Ok(());
            }
            
            match parts[1].parse::<i32>() {
                Ok(distance) => {
                    match drone.right(distance) {
                        Ok(_) => println!("Moved right by {} cm", distance),
                        Err(e) => eprintln!("Failed to move right: {}", e),
                    }
                },
                Err(_) => {
                    eprintln!("Invalid distance value: {}", parts[1]);
                }
            }
        },
        "up" => {
            if parts.len() < 2 {
                println!("Please specify distance for upward movement");
                return Ok(());
            }
            
            match parts[1].parse::<i32>() {
                Ok(distance) => {
                    match drone.up(distance) {
                        Ok(_) => println!("Moved up by {} cm", distance),
                        Err(e) => eprintln!("Failed to move up: {}", e),
                    }
                },
                Err(_) => {
                    eprintln!("Invalid distance value: {}", parts[1]);
                }
            }
        },
        "down" => {
            if parts.len() < 2 {
                println!("Please specify distance for downward movement");
                return Ok(());
            }
            
            match parts[1].parse::<i32>() {
                Ok(distance) => {
                    match drone.down(distance) {
                        Ok(_) => println!("Moved down by {} cm", distance),
                        Err(e) => eprintln!("Failed to move down: {}", e),
                    }
                },
                Err(_) => {
                    eprintln!("Invalid distance value: {}", parts[1]);
                }
            }
        },
        "state" => {
            if let Some(state) = drone.get_state() {
                println!("Drone state: {}", state);
                
                // Parse and display the state in a more readable format
                // State format is typically: pitch:%d;roll:%d;yaw:%d;vgx:%d;vgy:%d;vgz:%d;templ:%d;temph:%d;tof:%d;h:%d;bat:%d;baro:%.2f;time:%d;agx:%.2f;agy:%.2f;agz:%.2f;
                let state_pairs: Vec<&str> = state.split(';').collect();
                println!("Parsed state:");
                for pair in state_pairs {
                    if !pair.is_empty() {
                        println!("  {}", pair);
                    }
                }
            } else {
                println!("No state information available. Make sure the drone is connected.");
            }
        },
        "photo" => {
            match drone.take_photo() {
                Ok(_) => println!("Photo taken successfully"),
                Err(e) => eprintln!("Failed to take photo: {}", e),
            }
        },
        "video" => {
            if parts.len() < 2 {
                println!("Please specify 'start' or 'stop' after 'video'");
                return Ok(());
            }
            
            match parts[1] {
                "start" => {
                    match drone.start_video() {
                        Ok(_) => println!("Video recording started"),
                        Err(e) => eprintln!("Failed to start video: {}", e),
                    }
                },
                "stop" => {
                    match drone.stop_video() {
                        Ok(_) => println!("Video recording stopped"),
                        Err(e) => eprintln!("Failed to stop video: {}", e),
                    }
                },
                _ => println!("Unknown video command: {}", parts[1]),
            }
        },
        "media" => {
            if parts.len() < 2 {
                println!("Please specify a media command: list, download, delete, deleteall, path");
                return Ok(());
            }
            
            match parts[1] {
                "list" => {
                    match drone.list_media() {
                        Ok(files) => {
                            println!("Media files on drone:");
                            for file in files {
                                println!("  {}", file);
                            }
                        },
                        Err(e) => eprintln!("Failed to list media: {}", e),
                    }
                },
                "download" => {
                    if parts.len() < 3 {
                        println!("Please specify a filename to download");
                        return Ok(());
                    }
                    
                    let filename = parts[2];
                    match drone.download_media(filename) {
                        Ok(result) => println!("{}", result),
                        Err(e) => eprintln!("Failed to download media: {}", e),
                    }
                },
                "direct" => {
                    if parts.len() < 3 {
                        println!("Please specify a filename for direct transfer");
                        return Ok(());
                    }
                    
                    let filename = parts[2];
                    match drone.transfer_file_via_direct_connection(filename) {
                        Ok(result) => println!("{}", result),
                        Err(e) => eprintln!("Failed to transfer file: {}", e),
                    }
                },
                "delete" => {
                    if parts.len() < 3 {
                        println!("Please specify a filename to delete");
                        return Ok(());
                    }
                    
                    let filename = parts[2];
                    match drone.delete_media(filename) {
                        Ok(result) => println!("{}", result),
                        Err(e) => eprintln!("Failed to delete media: {}", e),
                    }
                },
                "deleteall" => {
                    match drone.delete_all_media() {
                        Ok(result) => println!("{}", result),
                        Err(e) => eprintln!("Failed to delete all media: {}", e),
                    }
                },
                "path" => {
                    if parts.len() < 3 {
                        println!("Please specify a path for media downloads");
                        return Ok(());
                    }
                    
                    let path = parts[2];
                    match drone.set_download_path(path) {
                        Ok(_) => println!("Download path set to: {}", path),
                        Err(e) => eprintln!("Failed to set download path: {}", e),
                    }
                },
                _ => println!("Unknown media command: {}", parts[1]),
            }
        },
        "rotate_cw" => {
            if parts.len() < 2 {
                println!("Please specify degrees for clockwise rotation");
                return Ok(());
            }
            
            match parts[1].parse::<i32>() {
                Ok(degrees) => {
                    match drone.rotate_cw(degrees) {
                        Ok(_) => println!("Rotated clockwise by {} degrees", degrees),
                        Err(e) => eprintln!("Failed to rotate: {}", e),
                    }
                },
                Err(_) => {
                    eprintln!("Invalid degrees value: {}", parts[1]);
                }
            }
        },
        "rotate_ccw" => {
            if parts.len() < 2 {
                println!("Please specify degrees for counter-clockwise rotation");
                return Ok(());
            }
            
            match parts[1].parse::<i32>() {
                Ok(degrees) => {
                    match drone.rotate_ccw(degrees) {
                        Ok(_) => println!("Rotated counter-clockwise by {} degrees", degrees),
                        Err(e) => eprintln!("Failed to rotate: {}", e),
                    }
                },
                Err(_) => {
                    eprintln!("Invalid degrees value: {}", parts[1]);
                }
            }
        },
        "camera_to_center" => {
            if parts.len() < 3 {
                println!("Please specify center coordinates: camera_to_center <x> <y>");
                return Ok(());
            }
            
            let x = match parts[1].parse::<f32>() {
                Ok(val) => val,
                Err(_) => {
                    eprintln!("Invalid x-coordinate: {}", parts[1]);
                    return Ok(());
                }
            };
            
            let y = match parts[2].parse::<f32>() {
                Ok(val) => val,
                Err(_) => {
                    eprintln!("Invalid y-coordinate: {}", parts[2]);
                    return Ok(());
                }
            };
            
            match drone.point_camera_to_center(x, y) {
                Ok(_) => println!("Camera pointed towards center point ({}, {})", x, y),
                Err(e) => eprintln!("Failed to point camera: {}", e),
            }
        },
        "camera_from_center" => {
            if parts.len() < 3 {
                println!("Please specify center coordinates: camera_from_center <x> <y>");
                return Ok(());
            }
            
            let x = match parts[1].parse::<f32>() {
                Ok(val) => val,
                Err(_) => {
                    eprintln!("Invalid x-coordinate: {}", parts[1]);
                    return Ok(());
                }
            };
            
            let y = match parts[2].parse::<f32>() {
                Ok(val) => val,
                Err(_) => {
                    eprintln!("Invalid y-coordinate: {}", parts[2]);
                    return Ok(());
                }
            };
            
            match drone.point_camera_from_center(x, y) {
                Ok(_) => println!("Camera pointed away from center point ({}, {})", x, y),
                Err(e) => eprintln!("Failed to point camera: {}", e),
            }
        },
        "position" => {
            if parts.len() < 4 {
                println!("Please specify all coordinates: position <x> <y> <z>");
                return Ok(());
            }
            
            let x = match parts[1].parse::<f32>() {
                Ok(val) => val,
                Err(_) => {
                    eprintln!("Invalid x-coordinate: {}", parts[1]);
                    return Ok(());
                }
            };
            
            let y = match parts[2].parse::<f32>() {
                Ok(val) => val,
                Err(_) => {
                    eprintln!("Invalid y-coordinate: {}", parts[2]);
                    return Ok(());
                }
            };
            
            let z = match parts[3].parse::<f32>() {
                Ok(val) => val,
                Err(_) => {
                    eprintln!("Invalid z-coordinate: {}", parts[3]);
                    return Ok(());
                }
            };
            
            drone.set_position(x, y, z);
            println!("Drone position set to ({}, {}, {})", x, y, z);
        },
        "get_position" => {
            let pos = drone.get_position();
            println!("Current drone position: ({:.2}, {:.2}, {:.2})", pos.x, pos.y, pos.z);
        },
        "exit" => {
            println!("Exiting Tello Control...");
            return Err(io::Error::new(io::ErrorKind::Other, "Exit requested"));
        },
        _ => {
            println!("Unknown command: {}", parts[0]);
        }
    }
    
    Ok(())
}