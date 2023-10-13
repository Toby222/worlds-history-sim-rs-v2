pub trait Separator {
    fn separated<const SPACING: usize, const SEPARATOR: char>(&self) -> String;
}

impl<T: ToString> Separator for T {
    fn separated<const SPACING: usize, const SEPARATOR: char>(&self) -> String {
        self.to_string()
            .as_bytes()
            .rchunks(SPACING)
            .rev()
            .map(std::str::from_utf8)
            .collect::<Result<Vec<&str>, _>>()
            .unwrap()
            .join(&SEPARATOR.to_string())
    }
}
