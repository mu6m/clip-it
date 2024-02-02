use std::process::Command;
use std::time::SystemTime;
use std::fs;
use std::{thread, time};

fn main() {
    let ffmpeg_location = "/path/to/ffmpeg.exe";
    let seconds = 1 * 5;
    let sleep_time = time::Duration::from_millis(1000 * seconds);
    let folder_name = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
    let directory = format!("./records/{}/",folder_name);
    fs::create_dir_all(&directory);
    loop {
        let current_unix = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();

        println!("Started recording {}",current_unix);
        let mut child = Command::new(ffmpeg_location)
            .args(&[
                "-y", 
                "-video_size",
                "1920x1080",
                "-framerate",
                "240",
                "-f",
                "gdigrab",
                "-i",
                "desktop",
                "-f",
                "dshow",
                "-i",
                r#"audio=Stereo Mix (Realtek(R) Audio)"#,
                "-t",
                &format!("{}",seconds),
                "-preset",
                "ultrafast",
                "-c:v",
                "libx264",
                "-crf",
                "0",
                "-c:a",
                "libopus",
                &format!("{}/{}.mkv",directory,current_unix)

            ])
            .spawn()
            .unwrap();
        let _result = child.wait().unwrap();
        println!("{}\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n", _result)

    }
}
