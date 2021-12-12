//! The trait that implements character counting for the &str type.
//!
//! # Quick Start
//! ```
//! use chars_counter::{ICharsCounter, ICharCounterExt};
//!
//! let str = "Hello world!";
//! let result = str.count_chars();
//! // result = [CharsCounter { character: 'l', count: 3 }, CharsCounter { character: 'o', count: 2 }, CharsCounter { character: ' ', count: 1 }, CharsCounter { character: '!', count: 1 }, CharsCounter { character: 'H', count: 1 }, CharsCounter { character: 'd', count: 1 }, CharsCounter { character: 'e', count: 1 }, CharsCounter { character: 'r', count: 1 }, CharsCounter { character: 'w', count: 1 }]
//!
//! // You can also use like this:
//! let result = str.count_chars_numeric();
//! let result = str.count_chars_alphabetic();
//! let result = str.count_chars_chinese();
//! // ...... Others you can try by yourself.
//! // if those can't meet your needs, you can custom your own rules by
//! let result = str.count_chars_filter(|x| *x != ' '); // ignore whitespaces.
//!
//! // More features:
//! let result = str.count_chars().most_chars();
//! // result = [CharsCounter { character: 'l', count: 3 }]
//! let result = str.count_chars().least_chars();
//! // result = [CharsCounter { character: ' ', count: 1 }, CharsCounter { character: '!', count: 1 }, CharsCounter { character: 'H', count: 1 }, CharsCounter { character: 'd', count: 1 }, CharsCounter { character: 'e', count: 1 }, CharsCounter { character: 'r', count: 1 }, CharsCounter { character: 'w', count: 1 }]
//! let result = str.count_chars().find_by_char('l');
//! // result = Some(CharsCounter { character: 'l', count: 3 })
//! let result = str.count_chars().find_by_num(2);
//! // result = [CharsCounter { character: 'o', count: 2 }]
//! let result = str.count_chars().least_chars().find_by_char('H');
//! // result = Some(CharsCounter { character: 'H', count: 1 })
//! ```

use itertools::Itertools;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct CharsCounter {
    pub character: char,
    pub count: usize,
}

pub trait ICharsCounter {
    fn count_chars(&self) -> Vec<CharsCounter>;
    fn count_chars_ascii(&self) -> Vec<CharsCounter>;
    fn count_chars_numeric(&self) -> Vec<CharsCounter>;
    fn count_chars_alphabetic(&self) -> Vec<CharsCounter>;
    fn count_chars_alphanumeric(&self) -> Vec<CharsCounter>;
    fn count_chars_whitespace(&self) -> Vec<CharsCounter>;
    fn count_chars_no_whitespace(&self) -> Vec<CharsCounter>;
    fn count_chars_chinese(&self) -> Vec<CharsCounter>;

    fn count_chars_filter<P>(&self, predicate: P) -> Vec<CharsCounter>
    where
        P: FnMut(&char) -> bool;
}

pub trait ICharCounterExt {
    fn most_chars(&self) -> Vec<CharsCounter>;
    fn least_chars(&self) -> Vec<CharsCounter>;
    fn find_by_num(&self, n: usize) -> Vec<CharsCounter>;
    fn find_by_char(&self, c: char) -> Option<CharsCounter>;
    fn counter_filter<P>(&self, predicate: P) -> Vec<CharsCounter>
    where
        P: FnMut(&&CharsCounter) -> bool;
}

impl ICharsCounter for &str {
    fn count_chars(&self) -> Vec<CharsCounter> {
        self.count_chars_filter(|_| true)
    }

    fn count_chars_ascii(&self) -> Vec<CharsCounter> {
        self.count_chars_filter(|x| x.is_ascii())
    }

    fn count_chars_numeric(&self) -> Vec<CharsCounter> {
        self.count_chars_filter(|x| x.is_numeric())
    }

    fn count_chars_alphabetic(&self) -> Vec<CharsCounter> {
        self.count_chars_filter(|x| x.is_alphabetic())
    }

    fn count_chars_alphanumeric(&self) -> Vec<CharsCounter> {
        self.count_chars_filter(|x| x.is_alphanumeric())
    }

    fn count_chars_whitespace(&self) -> Vec<CharsCounter> {
        self.count_chars_filter(|x| x.is_whitespace())
    }

    fn count_chars_no_whitespace(&self) -> Vec<CharsCounter> {
        self.count_chars_filter(|x| *x != ' ')
    }

    fn count_chars_chinese(&self) -> Vec<CharsCounter> {
        self.count_chars_filter(|x| *x as u32 >= 19968 && *x as u32 <= 40959)
    }

    fn count_chars_filter<P>(&self, predicate: P) -> Vec<CharsCounter>
    where
        P: FnMut(&char) -> bool,
    {
        self.chars()
            .filter(predicate)
            .into_group_map_by(|&x| x)
            .into_iter()
            .map(|x| CharsCounter {
                character: x.0,
                count: x.1.len(),
            })
            .sorted_by(|x, y| y.count.cmp(&x.count).then(x.character.cmp(&y.character)))
            .collect::<Vec<_>>()
    }
}

impl ICharCounterExt for Vec<CharsCounter> {
    fn most_chars(&self) -> Vec<CharsCounter> {
        self.counter_filter(|x| x.count == self[0].count)
    }

    fn least_chars(&self) -> Vec<CharsCounter> {
        self.counter_filter(|x| x.count == self[self.len() - 1].count)
    }

    fn find_by_num(&self, n: usize) -> Vec<CharsCounter> {
        self.counter_filter(|x| x.count == n)
    }

    fn find_by_char(&self, c: char) -> Option<CharsCounter> {
        self.iter().find(|x| x.character == c).map(|&x| x)
    }

    fn counter_filter<P>(&self, predicate: P) -> Vec<CharsCounter>
    where
        P: FnMut(&&CharsCounter) -> bool,
    {
        self.iter()
            .filter(predicate)
            .map(|&x| x)
            .collect::<Vec<_>>()
    }
}

#[cfg(test)]
mod tests {
    use crate::{CharsCounter, ICharCounterExt, ICharsCounter};

    #[test]
    fn most_chars_test() {
        let str = "Hello world!";
        let result = str.count_chars().most_chars();
        assert_eq!(
            result[0],
            CharsCounter {
                character: 'l',
                count: 3
            }
        );
    }

    #[test]
    fn least_chars_test() {
        let str = "Hello world!";
        let result = str.count_chars().least_chars();
        assert_eq!(
            result[0],
            CharsCounter {
                character: ' ',
                count: 1
            }
        );
    }

    #[test]
    fn find_by_num_test() {
        let str = "Hello world!";
        let result = str.count_chars().find_by_num(2);
        assert_eq!(
            result[0],
            CharsCounter {
                character: 'o',
                count: 2
            }
        );
    }

    #[test]
    fn find_by_char_test() {
        let str = "Hello world!";
        let result = str.count_chars().find_by_char('H').unwrap();
        assert_eq!(
            result,
            CharsCounter {
                character: 'H',
                count: 1
            }
        );
    }
}
