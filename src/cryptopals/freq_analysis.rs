use std::collections::HashMap;
use std::ascii::AsciiExt;

fn letter_freq(c: char) -> Option<f64> {
    match c {
        ' ' => Some(0.18288462654132653),
        'E' | 'e' => Some(0.10266650371711405),
        'T' | 't' => Some(0.07516998273511516),
        'A' | 'a' => Some(0.06532167023346977),
        'O' | 'o' => Some(0.06159577254159049),
        'N' | 'n' => Some(0.05712011128985469),
        'I' | 'i' => Some(0.05668443260048856),
        'S' | 's' => Some(0.05317005343812784),
        'R' | 'r' => Some(0.04987908553231180),
        'H' | 'h' => Some(0.04978563962655234),
        'L' | 'l' => Some(0.03317547959533063),
        'D' | 'd' => Some(0.03282923097335889),
        'U' | 'u' => Some(0.02275795359120720),
        'C' | 'c' => Some(0.02233675963832357),
        'M' | 'm' => Some(0.02026567834113036),
        'F' | 'f' => Some(0.01983067155219636),
        'W' | 'w' => Some(0.01703893766467868),
        'G' | 'g' => Some(0.01624904409178952),
        'P' | 'p' => Some(0.01504324284647170),
        'Y' | 'y' => Some(0.01427666624127353),
        'B' | 'b' => Some(0.01258880743014620),
        'V' | 'v' => Some(0.00796116438442061),
        'K' | 'k' => Some(0.00560962722644426),
        'X' | 'x' => Some(0.00140920161949961),
        'J' | 'j' => Some(0.00097521808184139),
        'Q' | 'q' => Some(0.00083675498119895),
        'Z' | 'z' => Some(0.00051284690692656),
        // TODO: account for other punctuation characters + their actual frequency.
        // The frequencies below are fake.
        '.' | ',' | ';' | ':' | '!' | '?' | '\"' | '-' | '\'' |
        '(' | ')' | '{' | '}' | '/' | '\\' | '|' | '<' | '>' |
        '[' | ']' | '+' | '=' | '_' | '*' | '&' | '%' | '$' |
        '@' | '#' | '\n' => Some(0.001),
        _ => None
    }
}

/// Generate a xi^2 score for the given piece of English text, including spaces.
/// If the given text contains non-English characters, None is returned, otherwise
/// the xi^2 score is returned in a Some.
pub fn chi_square(text: &str) -> Option<f64> {
    let length: u32 = text.len() as u32;
    let mut letter_to_count: HashMap<char, u32> = HashMap::new();
    for c in text.chars() {
        match letter_freq(c) {
            Some(_) => {
                *(letter_to_count.entry(c).or_insert(0)) += 1;
            },
            None => {
                // println!("no match for {}", c);
                return None
            }
        }
    }

    let res: f64 = letter_to_count.iter()
        .map(|(&c, &count)| -> f64 {
            // E[number of occurrences of c]
            let expectation_c: f64 = match letter_freq(c) {
                Some(f) => (length as f64) * f,
                None => panic!("Impossible state.")
            };
            (count as f64 - expectation_c).powf(2.0) / expectation_c
        }).sum();

    Some(res)
}
