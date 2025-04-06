#[derive(Debug)]
pub enum WrcMode {
    Bytes,
    Words,
    Chars,
    Lines,
}

pub struct Wrc {
    mode: WrcMode,
    pub filename: String,
}

fn read_file(path: &str) -> Result<String, std::io::Error> {
    std::fs::read_to_string(path)
}

impl Wrc {
    pub fn new(filename: String, mode: WrcMode) -> Self {
        Self { mode, filename }
    }

    pub fn count(&self) -> Result<usize, std::io::Error> {
        let contents = read_file(&self.filename)?;

        match self.mode {
            WrcMode::Bytes => Ok(Self::count_bytes(&contents)),
            WrcMode::Words => Ok(Self::count_words(&contents)),
            WrcMode::Chars => Ok(Self::count_chars(&contents)),
            WrcMode::Lines => Ok(Self::count_lines(&contents)),
        }
    }

    fn count_chars(contents: &str) -> usize {
        contents.char_indices().count()
    }

    fn count_bytes(contents: &str) -> usize {
        contents.as_bytes().iter().count()
    }

    fn count_words(contents: &str) -> usize {
        contents.split_ascii_whitespace().count()
    }

    fn count_lines(contents: &str) -> usize {
        contents.lines().count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_STR: &str = r#"Project Gutenberg™ eBooks are often created from several printed
editions, all of which are confirmed as not protected by copyright in
the U.S. unless a copyright notice is included. Thus, we do not
necessarily keep eBooks in compliance with any particular paper
edition."#;

    #[test]
    fn test_count_chars() {
        let count = Wrc::count_chars(TEST_STR);
        assert_eq!(271, count)
    }

    #[test]
    fn test_count_lines() {
        let count = Wrc::count_lines(TEST_STR);
        assert_eq!(5, count)
    }

    #[test]
    fn test_count_words() {
        let count = Wrc::count_words(TEST_STR);
        assert_eq!(43, count)
    }

    #[test]
    fn test_count_bytes() {
        let count = Wrc::count_bytes(TEST_STR);
        assert_eq!(273, count)
    }
}
