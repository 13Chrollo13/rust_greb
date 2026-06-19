use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let user_input = "was mögen sie gerne";
    if let Ok(lines) = read_lines("/home/shu/französich_mündlich.md") {
        for line in lines.map_while(Result::ok) {
            if line.contains(user_input) {
                println!("{}", line);
                let a = ":3";

            }
        }
    }
    /*
    read line by line
    search the line after the context
    if input == true {
        println(line)
    }
    
     */
}
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}