use std::{fs::File, io::Read};

struct FileInfo {
    id: u32,
    start: usize,
    length: usize,
}

fn main() {
    let mut file = File::open("data/input").expect("Unable to open input file");
    let mut buf = String::new();
    file.read_to_string(&mut buf)
        .expect("Unable to read input file");
    let input = buf.trim();

    let mut disk: Vec<Option<u32>> = Vec::new();
    let mut file_id = 0;
    let mut is_file_run = true;

    for c in input.chars() {
        let length = c.to_digit(10).expect("Invalid digit in input");
        if is_file_run {
            for _ in 0..length {
                disk.push(Some(file_id));
            }
            file_id += 1;
        } else {
            for _ in 0..length {
                disk.push(None);
            }
        }
        is_file_run = !is_file_run;
    }

    let max_file_id = file_id - 1;
    let mut files: Vec<FileInfo> = Vec::new();
    let mut current_file = 0;
    let mut pos = 0;
    while current_file <= max_file_id {
        while pos < disk.len() && (disk[pos].is_none() || disk[pos].unwrap() != current_file) {
            pos += 1;
        }
        if pos == disk.len() {
            current_file += 1;
            continue;
        }
        let start = pos;
        while pos < disk.len() && disk[pos].map_or(false, |fid| fid == current_file) {
            pos += 1;
        }
        let length = pos - start;
        files.push(FileInfo {
            id: current_file,
            start,
            length,
        });
        current_file += 1;
    }

    files.sort_by(|a, b| b.id.cmp(&a.id));

    for file_info in &mut files {
        let file_start = file_info.start;
        let file_length = file_info.length;
        if file_start == 0 {
            continue;
        }

        let mut candidate_start: Option<usize> = None;
        let mut candidate_length = 0;
        let mut best_segment: Option<(usize, usize)> = None; // (start, length)

        for i in 0..file_start {
            if disk[i].is_none() {
                if candidate_start.is_none() {
                    candidate_start = Some(i);
                    candidate_length = 1;
                } else {
                    candidate_length += 1;
                }
            } else {
                if let Some(s) = candidate_start {
                    if candidate_length >= file_length {
                        best_segment = Some((s, candidate_length));
                        break;
                    }
                }
                candidate_start = None;
                candidate_length = 0;
            }
        }

        if best_segment.is_none() {
            if let Some(s) = candidate_start {
                if candidate_length >= file_length {
                    best_segment = Some((s, candidate_length));
                }
            }
        }

        if let Some((segment_start, _segment_len)) = best_segment {
            let mut blocks = Vec::with_capacity(file_length);
            for i in file_info.start..file_info.start + file_length {
                blocks.push(disk[i].take());
            }

            for i in 0..file_length {
                disk[segment_start + i] = blocks[i];
            }

            file_info.start = segment_start;
        }
    }

    let mut checksum: u64 = 0;
    for (i, block) in disk.iter().enumerate() {
        if let Some(fid) = block {
            checksum += (i as u64) * (*fid as u64);
        }
    }

    println!("{}", checksum);
}
