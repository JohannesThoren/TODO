pub mod item;
pub mod prio;
pub mod file;
pub mod dir;

#[cfg(test)]
mod tests {
    use dir::read_dir;
    use crate::dir;

    #[test]
    fn test_read_dir() -> Result<(), std::io::Error> {
        let dir_content = read_dir(".".to_string(), &vec![], &false)?;
        println!("{dir_content:?}");
        Ok(())
    }
}
