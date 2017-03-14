use encoding::Encoding;
use encoding::all::{ISO_8859_15, WINDOWS_1252};
use encoding::EncoderTrap;

pub fn decode(name: String) -> String {
    let latin9 = ISO_8859_15;
    if let Ok(Ok(res)) = latin9.encode(&name, EncoderTrap::Strict).map(String::from_utf8) {
        return res;
    }
    let latin1 = WINDOWS_1252;
    if let Ok(Ok(res)) = latin1.encode(&name, EncoderTrap::Strict).map(String::from_utf8) {
        return res;
    }
    name
}


// split into words (based on non-alphanumeric chars)
pub fn get_words(name: &String) -> Vec<&str> {
    let mut words = Vec::<&str>::new();
    let mut index_start_word = 0;
    let mut is_current_alpha = name.chars()
        .next()
        .map(char::is_alphanumeric)
        .unwrap_or(true);
    for c in name.char_indices() {
        if c.1.is_alphanumeric() != is_current_alpha {
            words.push(&name[index_start_word..c.0]);
            is_current_alpha = c.1.is_alphanumeric();
            index_start_word = c.0;
        }
    }
    words.push(&name[index_start_word..]);
    words
}


// Force the first char uppercase
pub fn first_upper(name: String) -> String {
    let mut chars = name.chars();
    let mut new_name = String::new();
    new_name.extend(chars.next().map(|c| c.to_uppercase().collect::<String>()));
    new_name.extend(chars);
    new_name
}


/// MUSEE dE La GARE sncf > Musee de la gare de lyon
pub fn first_upper_all_lower(name: String) -> String {
    let mut chars = name.chars();
    let mut new_name = String::new();
    new_name.extend(chars.next().map(|c| c.to_uppercase().collect::<String>()));
    new_name.extend(chars.flat_map(char::to_lowercase));
    new_name
}


/// every word becomes Mmmmmm
pub fn snake_case(name: String) -> String {
    let mut new_name = String::new();
    for word in get_words(&name) {
        new_name.push_str(&first_upper_all_lower(word.to_string()));
    }
    new_name
}
