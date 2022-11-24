use std::os::linux::fs::MetadataExt;

const S_IFDIR: u32 = 0o040000;
const S_MASK: u32 = 0o170000;

fn S_IFMT(st_mode: u32) -> u32 {
    st_mode & S_MASK 
}

fn beautify_rwx(perm: &str) -> String {
    let full_word = |permission: char| -> &str {
        match permission {
            'r' => "read",
            'w' => "write",
            'x' => "execute",
            '-' => "not_set",
            _ => "that's bad"
        }
    };
    //let format: Vec<&str> = perm.chars().map(|x| full_word(x)).collect::<Vec<&str>>().join(", ");
    let format = perm.chars().map(|x| full_word(x))
        .fold("".to_string(), |mut res, permission| {
            if permission != "not_set" {
                res += permission;
                res += ", ";
            }
            res
        })
        .to_string();
    let trimmed = format.trim_end_matches(", ");
    trimmed.to_string()
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
    for owner in 1..=3 {
        let curr = match owner {
            1 => "User",
            2 => "Groups",
            3 => "Other",
            _ => "no way"
        };
        let beaut = beautify_rwx(unix_name[owner]);
        println!("{}: {}", curr, beaut);
    }

}

fn main() -> std::io::Result<()> {
    let meta = std::fs::metadata("file")?;
    filemode(meta.st_mode());
    Ok(())
}
