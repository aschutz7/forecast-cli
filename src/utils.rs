use colored::Colorize;

pub fn display(level: u8, msg: String) -> () {
    if level == 1 {
        // error
        eprintln!("{} {}", "[ERROR]".red().bold(), msg);
    } else if level == 2 {
        // warn (unlikely to be used)
        eprintln!("{} {}", "[WARN]".yellow().bold(), msg);
    } else if level == 3 {
        // success
        println!("{} {}", "[SUCCESS]".green().bold(), msg);
    } else {
        // default (info)
        println!("{} {}", "[INFO]".cyan().bold(), msg);
    }
}
