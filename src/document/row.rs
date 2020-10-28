use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug, Default)]
pub struct Row {
    string: String,
    len: usize,
}

impl Row {
    pub fn to_string(&self, start: usize, end: usize) -> String {
        use std::cmp;

        let end = cmp::min(end, self.string.len());
        let start = cmp::min(start, end);
        let mut result = String::new();

        for grapheme in self.string[..]
            .graphemes(true)
            .skip(start)
            .take(end - start)
        {
            if grapheme == "\t" {
                result.push_str(" ");
            } else {
                result.push_str(grapheme);
            }
        }

        result
    }

    pub fn append(&mut self, new: &Self) {
        self.string = format!("{}{}", self.string, new.string);
        self.update_len();
    }

    pub fn delete(&mut self, at: usize) {
        if at >= self.len() {
            self.update_len();
            return;
        }

        let mut result: String = self.string[..].graphemes(true).take(at).collect();
        let remainder: String = self.string[..].graphemes(true).skip(at + 1).collect();
        result.push_str(&remainder);
        self.string = result;

        self.update_len();
    }

    pub fn insert(&mut self, at: usize, ch: char) {
        if at >= self.len() {
            self.string.push(ch);
            self.update_len();
            return;
        }

        let mut result: String = self.string[..].graphemes(true).take(at).collect();
        let remainder: String = self.string[..].graphemes(true).skip(at).collect();

        result.push(ch);
        result.push_str(&remainder);
        self.string = result;

        self.update_len();
    }

    pub fn split(&mut self, at: usize) -> Self {
        let beginning: String = self.string[..].graphemes(true).take(at).collect();
        let remainder: String = self.string[..].graphemes(true).skip(at).collect();
        self.string = beginning;
        self.update_len();
        Self::from(&remainder[..])
    }

    pub fn len(&self) -> usize {
        self.len
    }

    fn update_len(&mut self) {
        self.len = self.string[..].graphemes(true).count()
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.string.as_bytes()
    }
}

impl From<&str> for Row {
    fn from(slice: &str) -> Self {
        let mut row = Self {
            string: String::from(slice),
            len: 0,
        };

        row.update_len();
        row
    }
}