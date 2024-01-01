enum FileSize {
    Bytes(u64),
    Kilobytes(u64),
    Megabytes(u64),
    Gigabytes(u64),
    Terabytes(u64),
}

fn format_largest_unit(size: u64) -> String {
    let filesize = match size {
        0..=999 => FileSize::Bytes(size),
        1000..=999_999 => FileSize::Kilobytes(size / 1000),
        1_000_000..=999_999_999 => FileSize::Megabytes(size / 1_000_000),
        1_000_000_000..=999_999_999_999 => FileSize::Gigabytes(size / 1_000_000_000),
        _ => FileSize::Terabytes(size / 1_000_000_000_000),
    };

    match filesize {
        FileSize::Bytes(bytes) => format!("{} bytes", bytes),
        FileSize::Kilobytes(kb) => format!("{:.2} KB", kb as f64),
        FileSize::Megabytes(mb) => format!("{:.2} MB", mb as f64),
        FileSize::Gigabytes(gb) => format!("{:.2} GB", gb as f64),
        FileSize::Terabytes(tb) => format!("{:.2} TB", tb as f64),
    }
}

fn main() {
    let file_sizes = vec![2500, 1_500_000, 2_000_000_000, 5_000_000_000_000];
    
    for size in file_sizes {
        let result = format_largest_unit(size);
        println!("File size: {} => {}", size, result);
    }
}
