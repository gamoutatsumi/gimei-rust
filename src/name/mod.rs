extern crate rand;

mod gender;

use toml::Value;
use self::gender::Gender;

#[derive(Debug)]
pub struct Name {
    pub first: First,
    pub last: Last
}

impl Name {
    pub fn new() -> Name {
        Name::new_with_gender(Gender::sample())
    }

    pub fn new_with_gender(gender: Gender) -> Name {
        Name {
            first: First::new(gender),
            last: Last::new(),
        }
    }

    pub fn kanji(&self) -> String {
        let first = self.first.kanji();
        let last = self.last.kanji();
        vec![last, first].connect(" ")
    }

    pub fn hiragana(&self) -> String {
        let first = self.first.hiragana();
        let last = self.last.hiragana();
        vec![last, first].connect(" ")
    }

    pub fn katakana(&self) -> String {
        let first = self.first.katakana();
        let last = self.last.katakana();
        vec![last, first].connect(" ")
    }

    pub fn is_female(&self) -> bool {
        self.first.is_female()
    }

    pub fn is_male(&self) -> bool {
        self.first.is_male()
    }
}

#[derive(Debug)]
pub struct First {
    pub gender: Gender,
    name: Vec<String>
}

impl First {
    pub fn new(gender: Gender) -> First {
        let name = super::names()
            .get("first_name")
            .and_then(|n| n.lookup(gender.type_str()))
            .and_then(|n| n.sample().as_slice())
            .unwrap()
            .iter()
            .map(|n| n.as_str().unwrap().to_string())
            .collect::<Vec<String>>();
        First {
            gender: gender,
            name: name
        }
    }

    pub fn is_female(&self) -> bool {
        self.gender.is_female()
    }

    pub fn is_male(&self) -> bool {
        self.gender.is_male()
    }

    pub fn kanji(&self) -> String {
        self.name.get(0).unwrap().to_string()
    }

    pub fn hiragana(&self) -> String {
        self.name.get(1).unwrap().to_string()
    }

    pub fn katakana(&self) -> String {
        self.name.get(2).unwrap().to_string()
    }
}

#[derive(Debug)]
pub struct Last {
    name: Vec<String>
}

impl Last {
    pub fn new() -> Last {
        let name = super::names()
            .get("last_name")
            .and_then(|n| n.sample().as_slice())
            .unwrap()
            .iter()
            .map(|n| n.as_str().unwrap().to_string())
            .collect::<Vec<String>>();
        Last {
            name: name
        }
    }

    pub fn kanji(&self) -> String {
        self.name.get(0).unwrap().to_string()
    }

    pub fn hiragana(&self) -> String {
        self.name.get(1).unwrap().to_string()
    }

    pub fn katakana(&self) -> String {
        self.name.get(2).unwrap().to_string()
    }
}

trait Samplable {
    fn sample(&self) -> & Value;
}

impl Samplable for Value {
    fn sample(&self) -> & Value {
        match *self {
            Value::Array(..) => {
                let vec = &self.as_slice().unwrap();
                let index = rand::random::<usize>() % vec.len();
                &vec[index]
            },
            _ => self
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::gender::Gender;

    #[test]
    fn kanji() {
        let name = Name::new();
        assert!(!name.kanji().is_empty());
    }

    #[test]
    fn hiragana() {
        let name = Name::new();
        assert!(!name.hiragana().is_empty());
    }

    #[test]
    fn katakana() {
        let name = Name::new();
        assert!(!name.katakana().is_empty());
    }

    #[test]
    fn gender() {
        {
            let name = Name::new_with_gender(Gender::Female);
            assert!(name.is_female());
        }
        {
            let name = Name::new_with_gender(Gender::Male);
            assert!(name.is_male());
        }
    }

    #[test]
    fn first_name() {
        let first = Name::new().first;
        assert!(!first.kanji().is_empty());
        assert!(!first.hiragana().is_empty());
        assert!(!first.katakana().is_empty());
    }

    #[test]
    fn last_name() {
        let last = Name::new().last;
        assert!(!last.kanji().is_empty());
        assert!(!last.hiragana().is_empty());
        assert!(!last.katakana().is_empty());
    }
}
