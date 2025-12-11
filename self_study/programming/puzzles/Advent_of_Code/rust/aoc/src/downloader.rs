use std::{process::Command, path::Path};

pub fn download(session: &str, year: &str, day: &str) {
    let filename = format!("y{}_d{}.in", year, day);
    if !Path::new(&filename).exists() {
        
        println!("Downloading file: {}", filename);

        // Dirty hack to remove leading zero (will maybe improve it later..)
        let day = day.parse::<u8>().unwrap().to_string();
        
        let output = Command::new("curl")
                .arg(format!("https://adventofcode.com/{year}/day/{day}/input"))
                .arg("--cookie")
                .arg(session)
                .output()
                .expect("failed to execute process");
            
        if output.status.success() {
            std::fs::write(filename, &output.stdout).unwrap();
        } else {
            panic!("curl failed: {}", String::from_utf8_lossy(&output.stderr));
        }
    }
}