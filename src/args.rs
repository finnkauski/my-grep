use std::convert::{TryFrom, TryInto};
use std::env;

pub struct Args {
    #[allow(dead_code)]
    filename: String,
    pub pattern: String,
    pub files: std::env::Args,
}

impl TryFrom<env::Args> for Args {
    type Error = &'static str;

    fn try_from(mut args: env::Args) -> Result<Self, Self::Error> {
        let filename: String = args.next().unwrap(); // SAFETY: Always available
        let pattern: String = args.next().ok_or("No pattern provided")?;
        let files = args;
        if files.len() == 0 {
            return Err("No files provided");
        }
        Ok(Args {
            filename,
            pattern,
            files,
        })
    }
}

pub fn get_args() -> Option<Args> {
    env::args()
        .try_into()
        .map_err(|e| {
            println!("Could not acquire arguments: {}", e);
        })
        .ok()
}
