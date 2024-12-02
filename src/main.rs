#[cfg(feature = "cli")]
use chip_8::io::terminal_io::run;

fn main() {
    #[cfg(feature = "cli")]
    run()
}
