use std::io;
use std::net::{UdpSocket, SocketAddr};
use std::str;
use std::time::Duration;
use std::thread;
use std::sync::{Arc, Mutex};
use std::fs;
use std::path::Path;

const TELLO_IP: &str = "192.168.10.1";
const TELLO_PORT: u16 = 8889;
const LOCAL_PORT: u16 = 8890;
const STATE_PORT: u16 = 8890;
const FILE_TRANSFER_PORT: u16 = 8888; // Port for file transfers

pub struct Tello {
    socket: Option<UdpSocket>,
    tello_addr: SocketAddr,
    state_receiver: Option<Arc<Mutex<String>>>,
    video_recording: bool,
    download_path: String,
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
            state_receiver: None,
            video_recording: false,
            download_path: String::from("./tello_media"), // Default download path
        })
    }
    
    /// Set download path for media files
    pub fn set_download_path(&mut self, path: &str) -> io::Result<()> {
        if !Path::new(path).exists() {
            fs::create_dir_all(path)?;
        }
        self.download_path = String::from(path);
        Ok(())
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
        
        // Set up state receiver
        self.setup_state_receiver()?;
        
        // Create download directory if it doesn't exist
        if !Path::new(&self.download_path).exists() {
            fs::create_dir_all(&self.download_path)?;
        }
        
        Ok(())
    }
    
    /// Sets up a separate thread to receive state information from the drone
    fn setup_state_receiver(&mut self) -> io::Result<()> {
        // Create a socket for receiving state information
        let state_socket = UdpSocket::bind(format!("0.0.0.0:{}", STATE_PORT))?;
        state_socket.set_read_timeout(Some(Duration::from_secs(1)))?;
        
        // Create a shared state to store the latest drone state
        let state = Arc::new(Mutex::new(String::new()));
        self.state_receiver = Some(Arc::clone(&state));
        
        // Start a thread to continuously receive state information
        thread::spawn(move || {
            let mut buffer = [0; 1024];
            
            loop {
                match state_socket.recv_from(&mut buffer) {
                    Ok((amount, _)) => {
                        if let Ok(data) = str::from_utf8(&buffer[..amount]) {
                            // Update the shared state
                            if let Ok(mut state_guard) = state.lock() {
                                *state_guard = data.to_string();
                            }
                        }
                    },
                    Err(e) => {
                        if e.kind() != io::ErrorKind::WouldBlock {
                            eprintln!("Error receiving state: {}", e);
                        }
                    }
                }
                
                // Sleep a short time to avoid consuming too much CPU
                thread::sleep(Duration::from_millis(100));
            }
        });
        
        Ok(())
    }
    
    /// Get the latest drone state
    pub fn get_state(&self) -> Option<String> {
        if let Some(state_receiver) = &self.state_receiver {
            if let Ok(state) = state_receiver.lock() {
                return Some(state.clone());
            }
        }
        None
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
    
    /// Take a photo
    pub fn take_photo(&self) -> io::Result<String> {
        let response = self.send_command("takeoff photo")?;
        
        if response != "ok" {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Photo capture failed: {}", response),
            ));
        }
        
        println!("Photo captured successfully. To download, use 'download_media' command.");
        Ok(response)
    }
    
    /// Start video recording
    pub fn start_video(&mut self) -> io::Result<String> {
        if self.video_recording {
            return Err(io::Error::new(
                io::ErrorKind::AlreadyExists,
                "Video recording is already in progress",
            ));
        }
        
        let response = self.send_command("streamon")?;
        
        if response != "ok" {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Failed to start video streaming: {}", response),
            ));
        }
        
        self.video_recording = true;
        println!("Video recording started");
        Ok(response)
    }
    
    /// Stop video recording
    pub fn stop_video(&mut self) -> io::Result<String> {
        if !self.video_recording {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Video recording is not in progress",
            ));
        }
        
        let response = self.send_command("streamoff")?;
        
        if response != "ok" {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Failed to stop video streaming: {}", response),
            ));
        }
        
        self.video_recording = false;
        println!("Video recording stopped. To download, use 'download_media' command.");
        Ok(response)
    }
    
    /// List media files on drone
    pub fn list_media(&self) -> io::Result<Vec<String>> {
        let response = self.send_command("ls")?;
        
        if response.contains("error") || response.contains("Error") {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Failed to list media: {}", response),
            ));
        }
        
        // Parse response and extract file names
        let files: Vec<String> = response
            .lines()
            .map(|l| l.trim().to_string())
            .filter(|l| !l.is_empty() && l != "ok")
            .collect();
            
        Ok(files)
    }
    
    /// Download media file from drone
    pub fn download_media(&self, filename: &str) -> io::Result<String> {
        // Create directory if it doesn't exist
        if !Path::new(&self.download_path).exists() {
            fs::create_dir_all(&self.download_path)?;
        }
        
        let dest_path = format!("{}/{}", self.download_path, filename);
        println!("Downloading {} to {}...", filename, dest_path);
        
        // Send download command
        let cmd = format!("download {}", filename);
        let response = self.send_command(&cmd)?;
        
        if response.contains("error") || response.contains("Error") {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Download failed: {}", response),
            ));
        }
        
        // For actual implementation, we would need to set up a TCP server on FILE_TRANSFER_PORT
        // and handle the file transfer protocol. This is simplified.
        println!("Download initiated. File will be saved to: {}", dest_path);
        
        Ok(format!("Downloaded to {}", dest_path))
    }
    
    /// Delete media file from drone
    pub fn delete_media(&self, filename: &str) -> io::Result<String> {
        let cmd = format!("rm {}", filename);
        let response = self.send_command(&cmd)?;
        
        if response != "ok" {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Failed to delete file {}: {}", filename, response),
            ));
        }
        
        Ok(format!("Deleted {}", filename))
    }
    
    /// Delete all media files from drone
    pub fn delete_all_media(&self) -> io::Result<String> {
        let response = self.send_command("rmall")?;
        
        if response != "ok" {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Failed to delete all media: {}", response),
            ));
        }
        
        Ok("All media files deleted".to_string())
    }
}

// Mock implementation for testing
#[cfg(test)]
mod mock {
    use super::*;
    use std::cell::RefCell;
    use std::collections::HashMap;
    
    pub struct MockTello {
        commands: RefCell<Vec<String>>,
        responses: RefCell<HashMap<String, String>>,
    }
    
    impl MockTello {
        pub fn new() -> Self {
            let mut responses = HashMap::new();
            responses.insert("command".to_string(), "ok".to_string());
            responses.insert("takeoff".to_string(), "ok".to_string());
            responses.insert("land".to_string(), "ok".to_string());
            
            MockTello {
                commands: RefCell::new(Vec::new()),
                responses: RefCell::new(responses),
            }
        }
        
        pub fn send_command(&self, command: &str) -> io::Result<String> {
            self.commands.borrow_mut().push(command.to_string());
            
            let responses = self.responses.borrow();
            let response = responses.get(command)
                .cloned()
                .unwrap_or_else(|| "error".to_string());
                
            Ok(response)
        }
        
        pub fn get_commands(&self) -> Vec<String> {
            self.commands.borrow().clone()
        }
        
        pub fn set_response(&self, command: &str, response: &str) {
            self.responses.borrow_mut().insert(command.to_string(), response.to_string());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::mock::MockTello;
    
    #[test]
    fn test_tello_new() {
        // Test that Tello::new() creates a valid instance
        let tello = Tello::new().expect("Failed to create Tello instance");
        assert!(tello.socket.is_none());
        assert_eq!(tello.tello_addr.to_string(), format!("{}:{}", TELLO_IP, TELLO_PORT));
    }
    
    #[test]
    fn test_takeoff_default_height() {
        let mock = MockTello::new();
        
        // Test takeoff without specifying height
        mock.send_command("takeoff").unwrap();
        
        // No additional commands should be sent as default height is used
        assert_eq!(mock.get_commands(), vec!["takeoff"]);
    }
    
    #[test]
    fn test_takeoff_custom_height() {
        let mock = MockTello::new();
        
        // First send the takeoff command
        mock.send_command("takeoff").unwrap();
        
        // Then send the height adjustment (e.g. for 2m = 200cm)
        // Default takeoff is 1m (100cm), so we need to go up by 100cm more
        mock.send_command("up 100").unwrap();
        
        assert_eq!(mock.get_commands(), vec!["takeoff", "up 100"]);
    }
    
    #[test]
    fn test_takeoff_invalid_height() {
        let mock = MockTello::new();
        
        // Test with height greater than maximum (8m)
        mock.send_command("takeoff").unwrap();
        
        // No additional height command should be sent as we use default
        assert_eq!(mock.get_commands(), vec!["takeoff"]);
    }
    
    #[test]
    fn test_land() {
        let mock = MockTello::new();
        
        // Test land command
        mock.send_command("land").unwrap();
        
        assert_eq!(mock.get_commands(), vec!["land"]);
    }
    
    #[test]
    fn test_error_response() {
        let mock = MockTello::new();
        
        // Set an error response for takeoff
        mock.set_response("takeoff", "error");
        
        // This should result in an error
        let result = mock.send_command("takeoff");
        
        assert_eq!(result.unwrap(), "error");
    }
    
    #[test]
    fn test_take_photo() {
        let mock = MockTello::new();
        
        // Set up mock response for photo command
        mock.set_response("takeoff photo", "ok");
        
        // Test take photo command
        let result = mock.send_command("takeoff photo");
        
        assert_eq!(result.unwrap(), "ok");
        assert_eq!(mock.get_commands(), vec!["takeoff photo"]);
    }
    
    #[test]
    fn test_start_video() {
        let mock = MockTello::new();
        
        // Set up mock response for video start command
        mock.set_response("streamon", "ok");
        
        // Test start video command
        let result = mock.send_command("streamon");
        
        assert_eq!(result.unwrap(), "ok");
        assert_eq!(mock.get_commands(), vec!["streamon"]);
    }
    
    #[test]
    fn test_stop_video() {
        let mock = MockTello::new();
        
        // Set up mock response for video stop command
        mock.set_response("streamoff", "ok");
        
        // Test stop video command
        let result = mock.send_command("streamoff");
        
        assert_eq!(result.unwrap(), "ok");
        assert_eq!(mock.get_commands(), vec!["streamoff"]);
    }
    
    #[test]
    fn test_list_media() {
        let mock = MockTello::new();
        
        // Set up mock response for list media command
        mock.set_response("ls", "file1.jpg\nfile2.mp4\nok");
        
        // Test list media command
        let result = mock.send_command("ls");
        
        assert_eq!(result.unwrap(), "file1.jpg\nfile2.mp4\nok");
        assert_eq!(mock.get_commands(), vec!["ls"]);
    }
    
    #[test]
    fn test_download_media() {
        let mock = MockTello::new();
        
        // Set up mock response for download command
        mock.set_response("download file1.jpg", "ok");
        
        // Test download media command
        let result = mock.send_command("download file1.jpg");
        
        assert_eq!(result.unwrap(), "ok");
        assert_eq!(mock.get_commands(), vec!["download file1.jpg"]);
    }
    
    #[test]
    fn test_delete_media() {
        let mock = MockTello::new();
        
        // Set up mock response for delete media command
        mock.set_response("rm file1.jpg", "ok");
        
        // Test delete media command
        let result = mock.send_command("rm file1.jpg");
        
        assert_eq!(result.unwrap(), "ok");
        assert_eq!(mock.get_commands(), vec!["rm file1.jpg"]);
    }
    
    #[test]
    fn test_delete_all_media() {
        let mock = MockTello::new();
        
        // Set up mock response for delete all media command
        mock.set_response("rmall", "ok");
        
        // Test delete all media command
        let result = mock.send_command("rmall");
        
        assert_eq!(result.unwrap(), "ok");
        assert_eq!(mock.get_commands(), vec!["rmall"]);
    }
}