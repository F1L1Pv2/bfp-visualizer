use ::core::slice::{self};
use raylib::prelude::*;
use std::io::prelude::*;
use std::process::{Command, Stdio};

fn main() {

    let window_size = (640, 480);
    let framerate = 60;

    let (mut rl, thread) = raylib::init()
        .size(window_size.0, window_size.1)
        .title("bfp-visualizer")
        .build();

    rl.set_target_fps(framerate);

    let background_color = Color::from_hex("000000").unwrap();
    let foreground_color = Color::from_hex("FFFFFF").unwrap();

    let mut process = match Command::new("ffmpeg").args([
        "-loglevel",
        "verbose",
        "-y",

        "-f", "rawvideo",
        "-pix_fmt", "rgba",
        "-s", format!("{}x{}",window_size.0,window_size.1).as_str(),
        "-r", format!("{}",framerate).as_str(),
        "-i", "-",

        "-c:v", "libx264",
        "-vb", "2500k",
        "-c:a", "aac",
        "-ab", "200k",
        "-pix_fmt", "yuv420p",
        "output.mp4",

    ]).stdin(Stdio::piped()).spawn(){
        Err(why) => panic!("Couldn't execute ffmpeg: {}", why),
        Ok(child) => child
    };

    let child_stdin = process.stdin.as_mut().unwrap();

    let mut t = 0.0;

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        let (s_width, s_height) = (d.get_screen_width(), d.get_screen_height());

        d.clear_background(background_color);

        d.draw_rectangle(t  as i32, t as i32, s_width/2, s_height/2, foreground_color);

        t+=1.0;

        drop(d);

        let img = rl.get_screen_data(&thread);

        let buf: &[u8] =
            unsafe { slice::from_raw_parts(img.data as *const u8, s_width as usize * s_height as usize * 4) };

        child_stdin.write_all(buf).unwrap();

        child_stdin.flush().unwrap();

        
    }

}
