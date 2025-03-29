mod tello;

use std::io::{self, Write};
use tello::Tello;

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
    
    // Camera commands
    println!("  photo          - Take a photo");
    println!("  video start    - Start recording video");
    println!("  video stop     - Stop recording video");
    println!("  media list     - List media files on the drone");
    println!("  media download <filename> - Download media file from drone");
    println!("  media delete <filename>   - Delete media file from drone");
    println!("  media deleteall - Delete all media files from drone");
    println!("  media path <path> - Set download path for media files");
    
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
                        continue;
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
                        continue;
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
                                continue;
                            }
                            
                            let filename = parts[2];
                            match drone.download_media(filename) {
                                Ok(result) => println!("{}", result),
                                Err(e) => eprintln!("Failed to download media: {}", e),
                            }
                        },
                        "delete" => {
                            if parts.len() < 3 {
                                println!("Please specify a filename to delete");
                                continue;
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
                                continue;
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
                "exit" => {
                    println!("Exiting Tello Control...");
                    return Ok(());
                },
                _ => {
                    println!("Unknown command: {}", parts[0]);
                }
            }
        }
    }
}
