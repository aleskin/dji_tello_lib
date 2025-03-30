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

mod tello;
mod tello_movement;

use std::io::{self, Write};
use tello::Tello;
use std::thread;
use std::time::Duration;

fn main() -> io::Result<()> {
    // Initialize the drone connection
    println!("Connecting to Tello drone...");
    let mut drone = match Tello::new() {
        Ok(d) => d,
        Err(e) => {
            eprintln!("Failed to initialize drone connection: {}", e);
            return Err(e);
        }
    };
    
    if let Err(e) = drone.connect() {
        eprintln!("Failed to connect to drone: {}", e);
        return Err(e);
    }
    
    println!("Tello Control - Interactive Mode");
    println!("Type commands to control the drone. Separate multiple commands with semicolons (;)");
    println!("Available commands:");
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
    
    // Main command loop
    let mut input = String::new();
    
    loop {
        print!("> ");
        io::stdout().flush()?;
        
        input.clear();
        io::stdin().read_line(&mut input)?;
        
        // Split input by semicolons to handle multiple commands
        let commands: Vec<&str> = input.trim().split(';').map(|s| s.trim()).collect();
        
        for cmd in commands {
            if cmd.is_empty() {
                continue;
            }
            
            let parts: Vec<&str> = cmd.split_whitespace().collect();
            
            if parts.is_empty() {
                continue;
            }
            
            // Execute the command
            execute_command(&mut drone, &parts)?;
            
            // Add a delay between commands to give the drone time to stabilize
            // This is especially important for movement commands
            thread::sleep(Duration::from_millis(500));
        }
    }
}

// Extracted command execution to a separate function for better readability
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
