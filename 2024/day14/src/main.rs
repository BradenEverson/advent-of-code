use image::{Rgb, RgbImage};
use std::{
    fs::{self, File},
    io::Read,
    path::Path,
    process::Command,
};

fn main() {
    let mut file = File::open("data/input").expect("Unable to open input file");
    let mut buf = String::new();
    file.read_to_string(&mut buf)
        .expect("Unable to read input file");

    let mut robots = parse_input(&buf);

    let width = 101;
    let height = 103;
    let total_steps = 15_000;

    let pattern = image::ImageReader::open("tree.png")
        .expect("Failed to open pattern image")
        .decode()
        .expect("Failed to decode pattern image")
        .into_rgb8();

    let output_dir = "imgs";
    if !Path::new(output_dir).exists() {
        fs::create_dir(output_dir).expect("Failed to create 'imgs' directory");
    }

    for step in 0..total_steps {
        let current_image = save_grid_as_image(&robots, width, height, step);

        if contains_subimage(&current_image, &pattern) {
            println!("Christmas tree pattern detected at step: {}", step);
            break;
        }

        for robot in robots.iter_mut() {
            robot.update_position(width, height);
        }
    }
    generate_video_from_frames("trees.mp4", 60);
}

struct Robot {
    position: (i32, i32),
    velocity: (i32, i32),
}

impl Robot {
    fn update_position(&mut self, width: i32, height: i32) {
        self.position.0 = (self.position.0 + self.velocity.0).rem_euclid(width);
        self.position.1 = (self.position.1 + self.velocity.1).rem_euclid(height);
    }
}

fn generate_video_from_frames(output_video: &str, frame_rate: u32) {
    let status = Command::new("ffmpeg")
        .arg("-y")
        .arg("-framerate")
        .arg(frame_rate.to_string())
        .arg("-i")
        .arg("imgs/%05d.png")
        .arg("-vf")
        .arg("pad=width=102:height=104:x=0:y=0:color=black")
        .arg("-c:v")
        .arg("libx264")
        .arg("-pix_fmt")
        .arg("yuv420p")
        .arg(output_video)
        .status()
        .expect("Failed to execute ffmpeg");

    if status.success() {
        println!("Video successfully created: {}", output_video);
    } else {
        eprintln!("Error occurred while creating video.");
    }
}

fn parse_input(input: &str) -> Vec<Robot> {
    input
        .lines()
        .filter_map(|line| {
            let parts: Vec<_> = line.split_whitespace().collect();
            if parts.len() != 2 {
                return None;
            }
            let pos = parts[0]
                .trim_start_matches("p=")
                .trim_end_matches(',')
                .split(',');
            let vel = parts[1].trim_start_matches("v=").split(',');

            let position = pos
                .filter_map(|x| x.parse::<i32>().ok())
                .collect::<Vec<_>>();
            let velocity = vel
                .filter_map(|x| x.parse::<i32>().ok())
                .collect::<Vec<_>>();

            if position.len() == 2 && velocity.len() == 2 {
                Some(Robot {
                    position: (position[0], position[1]),
                    velocity: (velocity[0], velocity[1]),
                })
            } else {
                None
            }
        })
        .collect()
}

fn save_grid_as_image(robots: &[Robot], width: i32, height: i32, step: usize) -> RgbImage {
    let mut img = RgbImage::new(width as u32, height as u32);

    for pixel in img.pixels_mut() {
        *pixel = Rgb([0, 0, 0]);
    }

    for robot in robots {
        let x = robot.position.0 as u32;
        let y = robot.position.1 as u32;
        if x < width as u32 && y < height as u32 {
            img.put_pixel(x, y, Rgb([0, 255, 0]));
        }
    }

    let file_name = format!("imgs/{:05}.png", step);
    img.save(&file_name).expect("Failed to save image");

    img
}

fn contains_subimage(image: &RgbImage, subimage: &RgbImage) -> bool {
    let (image_width, image_height) = image.dimensions();
    let (sub_width, sub_height) = subimage.dimensions();

    for y in 0..=image_height.saturating_sub(sub_height) {
        for x in 0..=image_width.saturating_sub(sub_width) {
            let mut is_match = true;
            for sy in 0..sub_height {
                for sx in 0..sub_width {
                    if image.get_pixel(x + sx, y + sy) != subimage.get_pixel(sx, sy) {
                        is_match = false;
                        break;
                    }
                }
                if !is_match {
                    break;
                }
            }
            if is_match {
                return true;
            }
        }
    }

    false
}
