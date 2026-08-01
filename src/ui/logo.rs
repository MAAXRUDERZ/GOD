use crate::ui::colors::RESET;

const R1: &str = "\x1b[38;2;255;75;75m";
const R2: &str = "\x1b[38;2;220;40;40m";
const R3: &str = "\x1b[38;2;170;20;20m";
const R4: &str = "\x1b[38;2;110;10;10m";

const PAD: &str = "                    ";

pub fn print_logo() {
    // Top spacing
    println!();
    println!();

    println!("{PAD}{R1} ██████╗  ██████╗ ██████╗ {RESET}");
    println!("{PAD}{R1}██╔════╝ ██╔═══██╗██╔══██╗{RESET}");
    println!("{PAD}{R2}██║  ███╗██║   ██║██║  ██║{RESET}");
    println!("{PAD}{R3}██║   ██║██║   ██║██║  ██║{RESET}");
    println!("{PAD}{R4}╚██████╔╝╚██████╔╝██████╔╝{RESET}");
    println!("{PAD}{R4} ╚═════╝  ╚═════╝ ╚═════╝ {RESET}");

    println!();
    println!("{PAD}              {R2}v0.1.0{RESET}");
    println!();
}