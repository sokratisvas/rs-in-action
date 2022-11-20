use std::os::linux::fs::MetadataExt;

const S_IFDIR: u32 = 0o040000;
const S_MASK: u32 = 0o170000;

fn S_IFMT(st_mode: u32) -> u32{
    st_mode & S_MASK 
}

fn filemode(st_mode: u32) {
    let mut unix_name: [&str; 4] = [""; 4];
    if S_IFMT(st_mode) == S_IFDIR {
       unix_name[0] = "d";
    } else {
       unix_name[0] = "-";
    }

    // Iterate the last 3 digits of st_mode in octal
    let mut digit: usize = 1;
    let mut mode = st_mode;
    while digit <= 3 {
        println!("{}", mode);    
        match mode % 8 {
            7 => unix_name[4 - digit] = "rwx",
            6 => unix_name[4 - digit] = "rw-",
            5 => unix_name[4 - digit] = "r-x",
            4 => unix_name[4 - digit] = "r--",
            0 => unix_name[4 - digit] = "---",
            _ => println!("Invalid octal repr."),
        }
        mode /= 8;
        digit += 1;
    }
    println!("{:?}", unix_name);    
}

fn main() -> std::io::Result<()> {
    let meta = std::fs::metadata("file")?;
    //println!("{:o}", meta.st_mode() & 07777);
    println!("{:o}", meta.st_mode());
    //println!("{}", meta.st_mode());
    filemode(meta.st_mode());
    Ok(())
}
