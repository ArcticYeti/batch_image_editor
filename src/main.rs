use std::{fs, path::Path};

fn main() {
    let src_dir = Path::new(r"C:\Users\FIX_ME\Desktop\neon_sprites\source");
    let out_dir = Path::new(r"C:\Users\FIX_ME\Desktop\neon_sprites\output");
    let blur_amount = 5.0;

    fs::remove_dir_all(out_dir).unwrap();
    fs::create_dir(out_dir).unwrap();

    let dir_entry: Vec<_> = fs::read_dir(src_dir)
        .expect("Source directory not found.")
        .map(move |x| x.expect("I/O error reading directory entry."))
        .collect();

    for file in dir_entry {
        if !file.metadata().unwrap().is_file() {
            continue;
        }

        let src_file_name = file.file_name().into_string().unwrap();
        let src_file_path = src_dir.join(&src_file_name);

        let img = image::open(&src_file_path).unwrap();
        let blurred_image = img.blur(blur_amount);

        let blur_suffix = "__blur_".to_owned() + &blur_amount.to_string().replace(".", "_");
        let (name, ext) = src_file_name.split_once(".").unwrap();
        let out_file_name = name.to_owned() + &blur_suffix + "." + ext;

        let out_file_path = out_dir.join(&out_file_name);
        let _ = blurred_image.save(&out_file_path);

        // let blur_suffix = blur_amount.to_string().replace(".", "_");
        // let foo = out_file_path.join(blur_suffix);

        println!("Test: {}", out_file_path.display());
    }

    println!("Finished! Output: {}", out_dir.display());
}
