use base64::engine::general_purpose;
use base64::Engine;
use std::fs::File;
use std::io::{self, Read, Write};

fn main() -> Result<(), io::Error> {
    println!(
        r"
    
░█████╗░░██████╗██╗░░██╗░█████╗░███╗░░██╗██████╗░███████╗██████╗░
██╔══██╗██╔════╝██║░██╔╝██╔══██╗████╗░██║██╔══██╗██╔════╝██╔══██╗
███████║╚█████╗░█████═╝░███████║██╔██╗██║██║░░██║█████╗░░██████╔╝
██╔══██║░╚═══██╗██╔═██╗░██╔══██║██║╚████║██║░░██║██╔══╝░░██╔══██╗
██║░░██║██████╔╝██║░╚██╗██║░░██║██║░╚███║██████╔╝███████╗██║░░██║
╚═╝░░╚═╝╚═════╝░╚═╝░░╚═╝╚═╝░░╚═╝╚═╝░░╚══╝╚═════╝░╚══════╝╚═╝░░╚═╝
    "
    );
    println!("Github =>   https://github.com/hamdymohamedak ");
    println!("Linkedin => https://www.linkedin.com/in/hamdy-askander-b67b32246/ ");
    println!("Facebook =>   https://www.facebook.com/hamdy.elgokar.5 ");
    println!(r"
    ⠀⠀⠀⠀⠀⠀⠀⠀⣀⣀⣀⣀⣀⣠⣀⣀⣀⣠⡀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡇⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡇⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡇⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡇⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡇⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡇⠀⠀⠀⠀⠀⠀⠀
⣦⣤⣤⣤⣤⣤⣤⣤⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣧⣤⣤⣤⣤⣤⣤⣤
⠈⠻⢿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠟⠁
⠀⠀⠈⠙⢿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠟⠁⠀⠀
⠀⠀⠀⠀⠀⠙⢿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠟⠁⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠙⢿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠟⠁⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠙⢿⣿⣿⣿⣿⣿⠟⠁⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠙⢿⣿⠟⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
    ");
    print!("");
    println!("From Base64 To video [1]");
    println!("From video To Base64 [2]");
    println!("Enter your choice:");

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let input = input.trim();

    if input == "1" {
        println!("Enter the path to the Base64 file:");
        let mut base64_path = String::new();
        io::stdin().read_line(&mut base64_path)?;
        let base64_path = base64_path.trim();

        println!("Enter the path for the resulting video file:");
        let mut video_path = String::new();
        io::stdin().read_line(&mut video_path)?;
        let video_path = video_path.trim();

        from_base64_to_video(base64_path, video_path)?;
    } else if input == "2" {
        println!("Enter the path to the video file:");
        let mut video_path = String::new();
        io::stdin().read_line(&mut video_path)?;
        let video_path = video_path.trim();

        println!("Enter the path for the resulting Base64 encoded text file:");
        let mut base64_path = String::new();
        io::stdin().read_line(&mut base64_path)?;
        let base64_path = base64_path.trim();

        from_video_to_base64(video_path, base64_path)?;
    } else {
        println!("Invalid input. Please enter either '1' or '2'.");
    }

    Ok(())
}

fn from_base64_to_video(base64_path: &str, video_path: &str) -> Result<(), io::Error> {
    // Open the Base64 encoded file
    let mut base64_file = File::open(base64_path)?;
    let mut base64_data = String::new();
    base64_file.read_to_string(&mut base64_data)?;

    // Decode Base64 data to video
    let decoded_data = match general_purpose::STANDARD.decode(&base64_data) {
        Ok(data) => data,
        Err(err) => return Err(io::Error::new(io::ErrorKind::InvalidData, err)),
    };

    // Create a new video file and write the decoded data to it
    let mut video_file = File::create(video_path)?;
    video_file.write_all(&decoded_data)?;

    println!("Video has been successfully decoded from Base64.");

    Ok(())
}

fn from_video_to_base64(video_path: &str, base64_path: &str) -> Result<(), io::Error> {
    // Open the video file
    let mut video_file = File::open(video_path)?;
    let mut video_data = Vec::new();
    video_file.read_to_end(&mut video_data)?;

    // Encode video data to Base64
    let encoded_data = general_purpose::STANDARD.encode(&video_data);

    // Create a new text file and write the Base64 encoded data to it
    let mut base64_file = File::create(base64_path)?;
    base64_file.write_all(encoded_data.as_bytes())?;

    println!("Video has been successfully encoded to Base64.");

    Ok(())
}
