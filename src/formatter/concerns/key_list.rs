pub struct KeyList<'a> {
    input: &'a str,
}


impl<'a> KeyList<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input }
    }
}


impl<'a> Iterator for KeyList<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        let mut key = None;

        if let Some(i) = self.input.find('<') {
            let rest = self.input.char_indices()
                .skip(i)
                .take_while(|(_, c)| *c != '>')
                .last()
                .map(|(idx, c)| idx + c.len_utf8())
                .unwrap_or_default();

            // +1 to get the last '>' that's excluded
            key = Some(&self.input[i..(rest + 1)]);
            self.input = &self.input[(rest + 1)..];
        }

        key
    }
}