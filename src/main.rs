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
