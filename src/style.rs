use std::fmt;

#[cfg(not(test))]
pub fn bold<D: fmt::Display>(d: D) -> String {
    format!("\x1B[1m{}\x1B[0m", d)
}

// In tests replace ANSI codes with HTML tags for easier testing
#[cfg(test)]
pub fn bold<D: fmt::Display>(d: D) -> String {
    format!("<b>{}</b>", d)
}

#[cfg(not(test))]
pub fn underline<D: fmt::Display>(d: D) -> String {
    format!("\x1B[4m{}\x1B[0m", d)
}

// In tests replace ANSI codes with HTML tags for easier testing
#[cfg(test)]
pub fn underline<D: fmt::Display>(d: D) -> String {
    format!("<u>{}</u>", d)
}
