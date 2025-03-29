use std::io;
use std::net::{UdpSocket, SocketAddr};
use std::str;
use std::time::Duration;

const TELLO_IP: &str = "192.168.10.1";
const TELLO_PORT: u16 = 8889;
const LOCAL_PORT: u16 = 8890;
const STATE_PORT: u16 = 8890;

pub struct Tello {
    socket: Option<UdpSocket>,
    tello_addr: SocketAddr,
}

impl Tello {
    /// Create a new Tello instance
    pub fn new() -> io::Result<Self> {
        let tello_addr = format!("{}:{}", TELLO_IP, TELLO_PORT)
            .parse()
            .expect("Failed to parse Tello address");
            
        Ok(Tello {
            socket: None,
            tello_addr,
        })
    }
    
    /// Connect to the Tello drone
    pub fn connect(&mut self) -> io::Result<()> {
        let socket = UdpSocket::bind(format!("0.0.0.0:{}", LOCAL_PORT))?;
        socket.set_read_timeout(Some(Duration::from_secs(5)))?;
        socket.set_write_timeout(Some(Duration::from_secs(5)))?;
        
        // Store the socket in the struct
        self.socket = Some(socket);
        
        // Initialize the SDK mode
        self.send_command("command")?;
        
        Ok(())
    }
    
    /// Send a command to the drone
    fn send_command(&self, command: &str) -> io::Result<String> {
        if let Some(socket) = &self.socket {
            println!("Sending command: {}", command);
            
            socket.send_to(command.as_bytes(), self.tello_addr)?;
            
            // Get response
            let mut buffer = [0; 1024];
            let (amount, _) = socket.recv_from(&mut buffer)?;
            
            let response = str::from_utf8(&buffer[..amount])
                .unwrap_or("Invalid UTF-8 response")
                .to_string();
                
            println!("Response: {}", response);
            
            Ok(response)
        } else {
            Err(io::Error::new(io::ErrorKind::NotConnected, "Drone not connected"))
        }
    }
    
    /// Take off
    /// 
    /// Optional height parameter in meters (default: 1m, max: 8m)
    pub fn takeoff(&self, height: Option<f32>) -> io::Result<()> {
        // First issue standard takeoff command
        let response = self.send_command("takeoff")?;
        
        if response != "ok" {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Takeoff command failed: {}", response),
            ));
        }
        
        // If a specific height was requested
        if let Some(h) = height {
            if h > 8.0 {
                println!("Warning: Requested height {}m exceeds maximum. Using default height (1m).", h);
                return Ok(());
            }
            
            if h <= 0.0 {
                println!("Warning: Invalid height value ({}m). Using default height (1m).", h);
                return Ok(());
            }
            
            // Convert height to centimeters for the command
            let height_cm = (h * 100.0) as i32;
            
            // The "go" command uses x y z speed format
            // We'll move up to desired height
            if height_cm > 100 {
                let _ = self.send_command(&format!("up {}", height_cm - 100))?;
            } else if height_cm < 100 {
                let _ = self.send_command(&format!("down {}", 100 - height_cm))?;
            }
        }
        
        Ok(())
    }
    
    /// Land the drone
    pub fn land(&self) -> io::Result<()> {
        let response = self.send_command("land")?;
        
        if response != "ok" {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Landing command failed: {}", response),
            ));
        }
        
        Ok(())
    }
}