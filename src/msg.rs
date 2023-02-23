///! Arch Linux makepkg like logging

#[macro_export]
macro_rules! msg {
  ($($arg:tt)*) => {
    println!("{} {}", "==>".bold().bright_green(), format!($($arg)*).bold())
  };
}

#[macro_export]
macro_rules! msg2 {
  ($($arg:tt)*) => {
    println!("{} {}", "  ->".bold().bright_cyan(), format!($($arg)*).bold())
  };
}

#[macro_export]
macro_rules! error {
  ($($arg:tt)*) => {
    println!("{} {}", "ERROR:".bold().red(), format!($($arg)*).bold())
  };
}
