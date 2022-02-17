use super::Key;

pub struct KeyList<'a> {
    input: &'a str,
}

impl<'a> KeyList<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input }
    }

    fn fetch_next_key(&mut self) -> Option<(Key<'a>, bool)> {
        let mut key = None;

        if let Some(i) = self.input.find('<') {
            self.input = &self.input[i..];

            let mut omit = false;

            let rest = self
                .input
                .char_indices()
                .take_while(|(idx, c)| {
                    if *idx == 0 && *c == '<' {
                        return true;
                    }

                    if *c == '<' {
                        omit = true;
                        return false;
                    }

                    *c != '>'
                })
                .last()
                .map(|(idx, c)| idx + c.len_utf8())
                .unwrap_or_default();

            // +1 to get the last '>' that's excluded only if the key
            // isn't a 'fake' key with a false opening
            let adder = if omit {
                0
            } else if self.input[..rest].len() == self.input.len() {
                // Don't add anything else if we're at the end of the string
                0
            } else {
                1
            };

            println!("Dying {}", &self.input[..(rest + adder)]);

            key = Some((Key::new(&self.input[..(rest + adder)]), omit));
            self.input = &self.input[(rest + adder)..];
        }

        key
    }
}

impl<'a> Iterator for KeyList<'a> {
    type Item = Key<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let key = self.fetch_next_key();

            if key.is_none() {
                break;
            }

            let (key, omit) = key.unwrap();

            // If provided key was a 'fake' with a false opening
            // like this "<---------------" we don't want it in the key list.
            if omit {
                continue;
            }

            return Some(key);
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_keys() {
        let input = "<black> <red> one two <three>";
        let key_count = KeyList::new(&input).count();

        assert_eq!(key_count, 3);
    }

    #[test]
    fn ignore_fake_keys() {
        let input = "<black><-------------------- some text <some random opening here, <and another here </>";
        let key_count = KeyList::new(&input).count();

        assert_eq!(key_count, 2);
    }

    #[test]
    fn mess_around() {
        let input = "<< powering on 'TV' (0)";
        let _keys = KeyList::new(&input).count();

        let input = "<< something that doesn't end after weird patterns < alksdfa < ngi2oueng <<ikdoqlksmads <black></>";
        let keys = KeyList::new(&input).count();

        assert_eq!(keys, 2);
    }
}
