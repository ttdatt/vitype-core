use once_cell::sync::Lazy;
use std::collections::{HashMap, HashSet};

// ==================== Enums ====================

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Default)]
pub(crate) enum OutputEncoding {
    #[default]
    Unicode = 0,
    CompositeUnicode = 1,
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Default)]
pub(crate) enum InputMethod {
    #[default]
    Telex = 0,
    Vni = 1,
}

#[derive(Clone, Copy, PartialEq)]
pub(crate) enum WTransformKind {
    None,
    Standalone,
    CompoundUow,
    CompoundUoFinalConsonantW,
    CompoundUaw,
}

// ==================== Action Struct ====================

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct KeyTransformAction {
    pub(crate) delete_count: usize,
    pub(crate) text: String,
}

// ==================== Helper Functions ====================

pub(crate) fn lower_char(ch: char) -> char {
    ch.to_lowercase().next().unwrap_or(ch)
}

pub(crate) fn is_vowel(ch: char) -> bool {
    VOWELS.contains(&ch)
}

// ==================== Shared Static Data ====================

static VOWELS: Lazy<HashSet<char>> = Lazy::new(|| {
    let chars = [
        'a', 'ă', 'â', 'e', 'ê', 'i', 'o', 'ô', 'ơ', 'u', 'ư', 'y',
        'A', 'Ă', 'Â', 'E', 'Ê', 'I', 'O', 'Ô', 'Ơ', 'U', 'Ư', 'Y',
        'á', 'à', 'ả', 'ã', 'ạ',
        'ắ', 'ằ', 'ẳ', 'ẵ', 'ặ',
        'ấ', 'ầ', 'ẩ', 'ẫ', 'ậ',
        'é', 'è', 'ẻ', 'ẽ', 'ẹ',
        'ế', 'ề', 'ể', 'ễ', 'ệ',
        'í', 'ì', 'ỉ', 'ĩ', 'ị',
        'ó', 'ò', 'ỏ', 'õ', 'ọ',
        'ố', 'ồ', 'ổ', 'ỗ', 'ộ',
        'ớ', 'ờ', 'ở', 'ỡ', 'ợ',
        'ú', 'ù', 'ủ', 'ũ', 'ụ',
        'ứ', 'ừ', 'ử', 'ữ', 'ự',
        'ý', 'ỳ', 'ỷ', 'ỹ', 'ỵ',
        'Á', 'À', 'Ả', 'Ã', 'Ạ',
        'Ắ', 'Ằ', 'Ẳ', 'Ẵ', 'Ặ',
        'Ấ', 'Ầ', 'Ẩ', 'Ẫ', 'Ậ',
        'É', 'È', 'Ẻ', 'Ẽ', 'Ẹ',
        'Ế', 'Ề', 'Ể', 'Ễ', 'Ệ',
        'Í', 'Ì', 'Ỉ', 'Ĩ', 'Ị',
        'Ó', 'Ò', 'Ỏ', 'Õ', 'Ọ',
        'Ố', 'Ồ', 'Ổ', 'Ỗ', 'Ộ',
        'Ớ', 'Ờ', 'Ở', 'Ỡ', 'Ợ',
        'Ú', 'Ù', 'Ủ', 'Ũ', 'Ụ',
        'Ứ', 'Ừ', 'Ử', 'Ữ', 'Ự',
        'Ý', 'Ỳ', 'Ỷ', 'Ỹ', 'Ỵ',
    ];
    chars.iter().cloned().collect()
});

pub(crate) static VOWEL_TO_TONED: Lazy<HashMap<char, HashMap<char, char>>> = Lazy::new(|| {
    fn tone_map(entries: &[(char, char)]) -> HashMap<char, char> {
        entries.iter().cloned().collect()
    }

    let mut map = HashMap::new();
    map.insert('a', tone_map(&[('s', 'á'), ('f', 'à'), ('r', 'ả'), ('x', 'ã'), ('j', 'ạ')]));
    map.insert('ă', tone_map(&[('s', 'ắ'), ('f', 'ằ'), ('r', 'ẳ'), ('x', 'ẵ'), ('j', 'ặ')]));
    map.insert('â', tone_map(&[('s', 'ấ'), ('f', 'ầ'), ('r', 'ẩ'), ('x', 'ẫ'), ('j', 'ậ')]));
    map.insert('e', tone_map(&[('s', 'é'), ('f', 'è'), ('r', 'ẻ'), ('x', 'ẽ'), ('j', 'ẹ')]));
    map.insert('ê', tone_map(&[('s', 'ế'), ('f', 'ề'), ('r', 'ể'), ('x', 'ễ'), ('j', 'ệ')]));
    map.insert('i', tone_map(&[('s', 'í'), ('f', 'ì'), ('r', 'ỉ'), ('x', 'ĩ'), ('j', 'ị')]));
    map.insert('o', tone_map(&[('s', 'ó'), ('f', 'ò'), ('r', 'ỏ'), ('x', 'õ'), ('j', 'ọ')]));
    map.insert('ô', tone_map(&[('s', 'ố'), ('f', 'ồ'), ('r', 'ổ'), ('x', 'ỗ'), ('j', 'ộ')]));
    map.insert('ơ', tone_map(&[('s', 'ớ'), ('f', 'ờ'), ('r', 'ở'), ('x', 'ỡ'), ('j', 'ợ')]));
    map.insert('u', tone_map(&[('s', 'ú'), ('f', 'ù'), ('r', 'ủ'), ('x', 'ũ'), ('j', 'ụ')]));
    map.insert('ư', tone_map(&[('s', 'ứ'), ('f', 'ừ'), ('r', 'ử'), ('x', 'ữ'), ('j', 'ự')]));
    map.insert('y', tone_map(&[('s', 'ý'), ('f', 'ỳ'), ('r', 'ỷ'), ('x', 'ỹ'), ('j', 'ỵ')]));
    map.insert('A', tone_map(&[('s', 'Á'), ('f', 'À'), ('r', 'Ả'), ('x', 'Ã'), ('j', 'Ạ')]));
    map.insert('Ă', tone_map(&[('s', 'Ắ'), ('f', 'Ằ'), ('r', 'Ẳ'), ('x', 'Ẵ'), ('j', 'Ặ')]));
    map.insert('Â', tone_map(&[('s', 'Ấ'), ('f', 'Ầ'), ('r', 'Ẩ'), ('x', 'Ẫ'), ('j', 'Ậ')]));
    map.insert('E', tone_map(&[('s', 'É'), ('f', 'È'), ('r', 'Ẻ'), ('x', 'Ẽ'), ('j', 'Ẹ')]));
    map.insert('Ê', tone_map(&[('s', 'Ế'), ('f', 'Ề'), ('r', 'Ể'), ('x', 'Ễ'), ('j', 'Ệ')]));
    map.insert('I', tone_map(&[('s', 'Í'), ('f', 'Ì'), ('r', 'Ỉ'), ('x', 'Ĩ'), ('j', 'Ị')]));
    map.insert('O', tone_map(&[('s', 'Ó'), ('f', 'Ò'), ('r', 'Ỏ'), ('x', 'Õ'), ('j', 'Ọ')]));
    map.insert('Ô', tone_map(&[('s', 'Ố'), ('f', 'Ồ'), ('r', 'Ổ'), ('x', 'Ỗ'), ('j', 'Ộ')]));
    map.insert('Ơ', tone_map(&[('s', 'Ớ'), ('f', 'Ờ'), ('r', 'Ở'), ('x', 'Ỡ'), ('j', 'Ợ')]));
    map.insert('U', tone_map(&[('s', 'Ú'), ('f', 'Ù'), ('r', 'Ủ'), ('x', 'Ũ'), ('j', 'Ụ')]));
    map.insert('Ư', tone_map(&[('s', 'Ứ'), ('f', 'Ừ'), ('r', 'Ử'), ('x', 'Ữ'), ('j', 'Ự')]));
    map.insert('Y', tone_map(&[('s', 'Ý'), ('f', 'Ỳ'), ('r', 'Ỷ'), ('x', 'Ỹ'), ('j', 'Ỵ')]));
    map
});

pub(crate) static TONED_TO_BASE: Lazy<HashMap<char, (char, char)>> = Lazy::new(|| {
    let mut map = HashMap::new();
    for (base, tones) in VOWEL_TO_TONED.iter() {
        for (tone, accented) in tones.iter() {
            map.insert(*accented, (*base, *tone));
        }
    }
    map
});

pub(crate) static NUCLEUS_ONLY_VOWELS: Lazy<HashSet<char>> = Lazy::new(|| {
    let chars = [
        'ă', 'ắ', 'ằ', 'ẳ', 'ẵ', 'ặ',
        'Ă', 'Ắ', 'Ằ', 'Ẳ', 'Ẵ', 'Ặ',
        'â', 'ấ', 'ầ', 'ẩ', 'ẫ', 'ậ',
        'Â', 'Ấ', 'Ầ', 'Ẩ', 'Ẫ', 'Ậ',
        'ê', 'ế', 'ề', 'ể', 'ễ', 'ệ',
        'Ê', 'Ế', 'Ề', 'Ể', 'Ễ', 'Ệ',
        'ô', 'ố', 'ồ', 'ổ', 'ỗ', 'ộ',
        'Ô', 'Ố', 'Ồ', 'Ổ', 'Ỗ', 'Ộ',
        'ơ', 'ớ', 'ờ', 'ở', 'ỡ', 'ợ',
        'Ơ', 'Ớ', 'Ờ', 'Ở', 'Ỡ', 'Ợ',
        'ư', 'ứ', 'ừ', 'ử', 'ữ', 'ự',
        'Ư', 'Ứ', 'Ừ', 'Ử', 'Ữ', 'Ự',
    ];
    chars.iter().cloned().collect()
});

pub(crate) static BASE_VOWELS: Lazy<HashSet<char>> = Lazy::new(|| {
    let chars = [
        'a', 'ă', 'â', 'e', 'ê', 'i', 'o', 'ô', 'ơ', 'u', 'ư', 'y',
        'A', 'Ă', 'Â', 'E', 'Ê', 'I', 'O', 'Ô', 'Ơ', 'U', 'Ư', 'Y',
    ];
    chars.iter().cloned().collect()
});
