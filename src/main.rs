use std::{env::args, process::{exit, Command}};

const VERSION:&str = "0.1.0";

fn main() {
    let args:Vec<String> = args().collect();
    
    if args.len() != 1 {
        exit(1);
    }

    println!("a2v {VERSION}");
    println!("A simple tool (that uses ffmpeg under the hood)\nto convert mp3 audio to mp4 video.");
    loop {
        println!("Enter mp3/m4a file name:");
        let mp3_file = input();
        if mp3_file.is_empty() {
            continue;
        }
        println!("Enter output mp4 file name:");
        let mp4_file = input();
        if mp4_file.is_empty() {
            continue;
        }
        println!("Enter image file name(optional):");
        let image = input();

        a2v(&mp3_file, &mp4_file, &image);

    }
}

fn input() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("internal error!");
    input.trim().to_string()
}

fn a2v(mp3_file_name:&str, mp4_file_name:&str, image_file_name:&str){
    if !mp3_file_name.ends_with(".mp3") && !mp3_file_name.ends_with(".m4a") {
        println!("Error: audio file name must end with .mp3/m4a");
        return;
        
    }
    if !mp4_file_name.ends_with(".mp4") {
        println!("Error: mp4 file name must end with .mp4");
        return;
    }



    if image_file_name.is_empty()  {
        let mut ffmpeg = Command::new("sh");
        
            match ffmpeg.arg("-c")
            .arg(format!("ffmpeg -i {mp3_file_name} -vn -c:a copy {mp4_file_name}"))
            .output(){
                Ok(output) => {
                    if output.status.success() {
                        println!("Successfully converted:\n {} -> {}", mp3_file_name, mp4_file_name);
                        exit(0);
                    } else {
                        println!("error while converting!");
                        exit(1);
                    }
                }
                Err(_) => {
                    println!("internal error!");
                    exit(1);
                }
            }
    }else {
        let mut ffmpeg = Command::new("sh");

        match ffmpeg.arg("-c")
          .arg(format!("ffmpeg -loop 1 -i {image_file_name} -i {mp3_file_name} -c:v libx264 -tune stillimage -c:a aac -b:a 192k -pix_fmt yuv420p -shortest {mp4_file_name}")).output() {
            Ok(output) => {
                if output.status.success() {
                    println!("Successfully converted:\n {} -> {}", mp3_file_name, mp4_file_name);
                    exit(0);
                } else {
                    dbg!(&output);
                    println!("error while converting!");
                    exit(1);
                }
            }
            Err(_) => {
                println!("internal error!");
                exit(1);
            }    
        }

        
    }
}