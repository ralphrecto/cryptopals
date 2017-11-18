use std::collections::HashMap;
use std::ascii::AsciiExt;

fn letter_freq(c: char) -> Option<f64> {
    let c_prime: char = c.to_ascii_uppercase();
    match c_prime {
        ' ' => Some(0.18288462654132653),
        'E' => Some(0.10266650371711405),
        'T' => Some(0.07516998273511516),
        'A' => Some(0.06532167023346977),
        'O' => Some(0.06159577254159049),
        'N' => Some(0.05712011128985469),
        'I' => Some(0.05668443260048856),
        'S' => Some(0.05317005343812784),
        'R' => Some(0.04987908553231180),
        'H' => Some(0.04978563962655234),
        'L' => Some(0.03317547959533063),
        'D' => Some(0.03282923097335889),
        'U' => Some(0.02275795359120720),
        'C' => Some(0.02233675963832357),
        'M' => Some(0.02026567834113036),
        'F' => Some(0.01983067155219636),
        'W' => Some(0.01703893766467868),
        'G' => Some(0.01624904409178952),
        'P' => Some(0.01504324284647170),
        'Y' => Some(0.01427666624127353),
        'B' => Some(0.01258880743014620),
        'V' => Some(0.00796116438442061),
        'K' => Some(0.00560962722644426),
        'X' => Some(0.00140920161949961),
        'J' => Some(0.00097521808184139),
        'Q' => Some(0.00083675498119895),
        'Z' => Some(0.00051284690692656),
        // TODO: account for other punctuation characters + their actual frequency.
        // Frequencies below is fake.
        '\'' | ',' => Some(0.001),
        _ => None
    }
}

/// Generate a xi^2 score for the given piece of English text, including spaces.
/// If the given text contains non-English characters, None is returned, otherwise
/// the xi^2 score is returned in a Some.
pub fn xi_square(text: &str) -> Option<f64> {
    let length: u32 = text.len() as u32;
    let mut letter_to_count: HashMap<char, u32> = HashMap::new();
    for c in text.chars() {
        match letter_freq(c) {
            Some(_) => {
                *(letter_to_count.entry(c).or_insert(0)) += 1;
            },
            None => return None
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
