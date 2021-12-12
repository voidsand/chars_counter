# Chars Counter
The trait that implements character counting for the &str type.

## Quick Start
```rust
use chars_counter::{ICharsCounter, ICharCounterExt};

let str = "Hello world!";
let result = str.count_chars();
// result = [CharsCounter { character: 'l', count: 3 }, CharsCounter { character: 'o', count: 2 }, CharsCounter { character: ' ', count: 1 }, CharsCounter { character: '!', count: 1 }, CharsCounter { character: 'H', count: 1 }, CharsCounter { character: 'd', count: 1 }, CharsCounter { character: 'e', count: 1 }, CharsCounter { character: 'r', count: 1 }, CharsCounter { character: 'w', count: 1 }]

// You can also use like this:
let result = str.count_chars_numeric();
let result = str.count_chars_alphabetic();
let result = str.count_chars_chinese();
// ...... Others you can try by yourself.
// if those can't meet your needs, you can custom your own rules by
let result = str.count_chars_filter(|x| *x != ' '); // ignore whitespaces.

// More features:
let result = str.count_chars().most_chars();
// result = [CharsCounter { character: 'l', count: 3 }]
let result = str.count_chars().least_chars();
// result = [CharsCounter { character: ' ', count: 1 }, CharsCounter { character: '!', count: 1 }, CharsCounter { character: 'H', count: 1 }, CharsCounter { character: 'd', count: 1 }, CharsCounter { character: 'e', count: 1 }, CharsCounter { character: 'r', count: 1 }, CharsCounter { character: 'w', count: 1 }]
let result = str.count_chars().find_by_char('l');
// result = Some(CharsCounter { character: 'l', count: 3 })
let result = str.count_chars().find_by_num(2);
// result = [CharsCounter { character: 'o', count: 2 }]
let result = str.count_chars().least_chars().find_by_char('H');
// result = Some(CharsCounter { character: 'H', count: 1 })
```