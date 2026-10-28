/*!
Modules for generating completions for various shells.
*/

static ENCODINGS: &str = include_str!("encodings.sh");

pub(super) mod bash;
pub(super) mod fish;
pub(super) mod powershell;
pub(super) mod zsh;
