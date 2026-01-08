use once_cell::sync::Lazy;
use std::collections::{HashMap, HashSet};

use crate::common::{
    is_vowel, lower_char, KeyTransformAction, WTransformKind,
    TONED_TO_BASE, VOWEL_TO_TONED,
};
use crate::VitypeEngine;

// ==================== Telex Helper Functions ====================

fn is_tone_key(ch: char) -> bool {
    TONE_KEYS.contains(&lower_char(ch))
}

pub(super) fn is_telex_word_boundary(ch: char) -> bool {
    WORD_BOUNDARY_CHARS.contains(&ch)
}

// ==================== Telex Static Data ====================

static TONE_KEYS: Lazy<HashSet<char>> = Lazy::new(|| {
    let chars = ['s', 'f', 'r', 'x', 'j', 'z'];
    chars.iter().cloned().collect()
});

static WORD_BOUNDARY_CHARS: Lazy<HashSet<char>> = Lazy::new(|| {
    let chars = [
        ' ', '\n', '\r', '\t', ',', '.', ';', ':', '!', '?',
        '(', ')', '[', ']', '{', '}', '"', '\'', '/', '\\',
        '-', '_', '@', '#', '$', '%', '^', '&', '*', '=', '+',
        '<', '>', '`', '~', '|',
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
    ];
    chars.iter().cloned().collect()
});

static VOWEL_TRANSFORMS: Lazy<HashMap<char, Vec<(char, char)>>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert('a', vec![('a', 'â'), ('A', 'Â')]);
    map.insert('A', vec![('a', 'Â'), ('A', 'Â')]);
    map.insert('e', vec![('e', 'ê'), ('E', 'Ê')]);
    map.insert('E', vec![('e', 'Ê'), ('E', 'Ê')]);
    map.insert('o', vec![('o', 'ô'), ('O', 'Ô')]);
    map.insert('O', vec![('o', 'Ô'), ('O', 'Ô')]);
    map.insert(
        'w',
        vec![
            ('a', 'ă'), ('A', 'Ă'),
            ('ă', 'ă'), ('Ă', 'Ă'),
            ('o', 'ơ'), ('O', 'Ơ'),
            ('ô', 'ơ'), ('Ô', 'Ơ'),
            ('u', 'ư'), ('U', 'Ư'),
            ('ư', 'ư'), ('Ư', 'Ư'),
            ('á', 'ắ'), ('à', 'ằ'), ('ả', 'ẳ'), ('ã', 'ẵ'), ('ạ', 'ặ'),
            ('Á', 'Ắ'), ('À', 'Ằ'), ('Ả', 'Ẳ'), ('Ã', 'Ẵ'), ('Ạ', 'Ặ'),
            ('ó', 'ớ'), ('ò', 'ờ'), ('ỏ', 'ở'), ('õ', 'ỡ'), ('ọ', 'ợ'),
            ('Ó', 'Ớ'), ('Ò', 'Ờ'), ('Ỏ', 'Ở'), ('Õ', 'Ỡ'), ('Ọ', 'Ợ'),
            ('ố', 'ớ'), ('ồ', 'ờ'), ('ổ', 'ở'), ('ỗ', 'ỡ'), ('ộ', 'ợ'),
            ('Ố', 'Ớ'), ('Ồ', 'Ờ'), ('Ổ', 'Ở'), ('Ỗ', 'Ỡ'), ('Ộ', 'Ợ'),
            ('ú', 'ứ'), ('ù', 'ừ'), ('ủ', 'ử'), ('ũ', 'ữ'), ('ụ', 'ự'),
            ('Ú', 'Ứ'), ('Ù', 'Ừ'), ('Ủ', 'Ử'), ('Ũ', 'Ữ'), ('Ụ', 'Ự'),
        ],
    );
    map.insert(
        'W',
        vec![
            ('a', 'Ă'), ('A', 'Ă'),
            ('ă', 'Ă'), ('Ă', 'Ă'),
            ('o', 'Ơ'), ('O', 'Ơ'),
            ('ô', 'Ơ'), ('Ô', 'Ơ'),
            ('u', 'Ư'), ('U', 'Ư'),
            ('ư', 'Ư'), ('Ư', 'Ư'),
            ('á', 'Ắ'), ('à', 'Ằ'), ('ả', 'Ẳ'), ('ã', 'Ẵ'), ('ạ', 'Ặ'),
            ('Á', 'Ắ'), ('À', 'Ằ'), ('Ả', 'Ẳ'), ('Ã', 'Ẵ'), ('Ạ', 'Ặ'),
            ('ó', 'Ớ'), ('ò', 'Ờ'), ('ỏ', 'Ở'), ('õ', 'Ỡ'), ('ọ', 'Ợ'),
            ('Ó', 'Ớ'), ('Ò', 'Ờ'), ('Ỏ', 'Ở'), ('Õ', 'Ỡ'), ('Ọ', 'Ợ'),
            ('ố', 'Ớ'), ('ồ', 'Ờ'), ('ổ', 'Ở'), ('ỗ', 'Ỡ'), ('ộ', 'Ợ'),
            ('Ố', 'Ớ'), ('Ồ', 'Ờ'), ('Ổ', 'Ở'), ('Ỗ', 'Ỡ'), ('Ộ', 'Ợ'),
            ('ú', 'Ứ'), ('ù', 'Ừ'), ('ủ', 'Ử'), ('ũ', 'Ữ'), ('ụ', 'Ự'),
            ('Ú', 'Ứ'), ('Ù', 'Ừ'), ('Ủ', 'Ử'), ('Ũ', 'Ữ'), ('Ụ', 'Ự'),
        ],
    );
    map
});

static VOWEL_UNTRANSFORMS: Lazy<HashMap<char, (char, char)>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert('â', ('a', 'a')); map.insert('Â', ('a', 'A'));
    map.insert('ê', ('e', 'e')); map.insert('Ê', ('e', 'E'));
    map.insert('ô', ('o', 'o')); map.insert('Ô', ('o', 'O'));
    map.insert('ă', ('w', 'a')); map.insert('Ă', ('w', 'A'));
    map.insert('ơ', ('w', 'o')); map.insert('Ơ', ('w', 'O'));
    map.insert('ư', ('w', 'u')); map.insert('Ư', ('w', 'U'));
    map.insert('ắ', ('w', 'á')); map.insert('ằ', ('w', 'à')); map.insert('ẳ', ('w', 'ả')); map.insert('ẵ', ('w', 'ã')); map.insert('ặ', ('w', 'ạ'));
    map.insert('Ắ', ('w', 'Á')); map.insert('Ằ', ('w', 'À')); map.insert('Ẳ', ('w', 'Ả')); map.insert('Ẵ', ('w', 'Ã')); map.insert('Ặ', ('w', 'Ạ'));
    map.insert('ớ', ('w', 'ó')); map.insert('ờ', ('w', 'ò')); map.insert('ở', ('w', 'ỏ')); map.insert('ỡ', ('w', 'õ')); map.insert('ợ', ('w', 'ọ'));
    map.insert('Ớ', ('w', 'Ó')); map.insert('Ờ', ('w', 'Ò')); map.insert('Ở', ('w', 'Ỏ')); map.insert('Ỡ', ('w', 'Õ')); map.insert('Ợ', ('w', 'Ọ'));
    map.insert('ứ', ('w', 'ú')); map.insert('ừ', ('w', 'ù')); map.insert('ử', ('w', 'ủ')); map.insert('ữ', ('w', 'ũ')); map.insert('ự', ('w', 'ụ'));
    map.insert('Ứ', ('w', 'Ú')); map.insert('Ừ', ('w', 'Ù')); map.insert('Ử', ('w', 'Ủ')); map.insert('Ữ', ('w', 'Ũ')); map.insert('Ự', ('w', 'Ụ'));
    map.insert('ấ', ('a', 'á')); map.insert('ầ', ('a', 'à')); map.insert('ẩ', ('a', 'ả')); map.insert('ẫ', ('a', 'ã')); map.insert('ậ', ('a', 'ạ'));
    map.insert('Ấ', ('a', 'Á')); map.insert('Ầ', ('a', 'À')); map.insert('Ẩ', ('a', 'Ả')); map.insert('Ẫ', ('a', 'Ã')); map.insert('Ậ', ('a', 'Ạ'));
    map.insert('ế', ('e', 'é')); map.insert('ề', ('e', 'è')); map.insert('ể', ('e', 'ẻ')); map.insert('ễ', ('e', 'ẽ')); map.insert('ệ', ('e', 'ẹ'));
    map.insert('Ế', ('e', 'É')); map.insert('Ề', ('e', 'È')); map.insert('Ể', ('e', 'Ẻ')); map.insert('Ễ', ('e', 'Ẽ')); map.insert('Ệ', ('e', 'Ẹ'));
    map.insert('ố', ('o', 'ó')); map.insert('ồ', ('o', 'ò')); map.insert('ổ', ('o', 'ỏ')); map.insert('ỗ', ('o', 'õ')); map.insert('ộ', ('o', 'ọ'));
    map.insert('Ố', ('o', 'Ó')); map.insert('Ồ', ('o', 'Ò')); map.insert('Ổ', ('o', 'Ỏ')); map.insert('Ỗ', ('o', 'Õ')); map.insert('Ộ', ('o', 'Ọ'));
    map
});

// ==================== Telex Methods on VitypeEngine ====================

impl VitypeEngine {
    pub(super) fn try_telex_escape_sequence(&mut self, ch: char) -> Option<KeyTransformAction> {
        let last_key = self.last_transform_key?;
        let ch_lower = lower_char(ch);
        let last_key_lower = lower_char(last_key);
        if ch_lower != last_key_lower {
            return None;
        }

        if ch_lower == 'w' {
            match self.last_w_transform_kind {
                WTransformKind::Standalone => {
                    if let Some(&last_char) = self.buffer.last() {
                        if last_char == 'ư' || last_char == 'Ư' {
                            let replacement = if last_char.is_uppercase() { 'W' } else { 'w' };
                            self.buffer.pop();
                            self.buffer.push(replacement);
                            self.last_transform_key = None;
                            self.last_w_transform_kind = WTransformKind::None;
                            self.suppressed_transform_key = Some(ch_lower);
                            return Some(KeyTransformAction {
                                delete_count: 1,
                                text: replacement.to_string(),
                            });
                        }
                    }
                }
                WTransformKind::CompoundUow => {
                    let end_index = self.buffer.len();
                    if self.buffer.len() >= 2 {
                        let o_index = end_index - 1;
                        let u_index = end_index - 2;
                        let u_horn = self.buffer[u_index];
                        let o_horn = self.buffer[o_index];
                        if (u_horn == 'ư' || u_horn == 'Ư') && (o_horn == 'ơ' || o_horn == 'Ơ') {
                            let original_u = if u_horn.is_uppercase() { 'U' } else { 'u' };
                            let original_o = if o_horn.is_uppercase() { 'O' } else { 'o' };
                            self.buffer.drain(u_index..);
                            self.buffer.push(original_u);
                            self.buffer.push(original_o);
                            self.buffer.push(ch);
                            self.last_transform_key = None;
                            self.last_w_transform_kind = WTransformKind::None;
                            self.suppressed_transform_key = Some(ch_lower);
                            return Some(KeyTransformAction {
                                delete_count: 2,
                                text: format!("{}{}{}", original_u, original_o, ch),
                            });
                        }
                    }
                }
                WTransformKind::CompoundUoFinalConsonantW => {
                    if self.buffer.len() >= 3 {
                        let mut o_index = self.buffer.len();
                        while o_index > 0 {
                            o_index -= 1;
                            if is_vowel(self.buffer[o_index]) {
                                break;
                            }
                        }
                        if o_index < self.buffer.len() && is_vowel(self.buffer[o_index]) {
                            if o_index == 0 {
                                return None;
                            }
                            let u_index = o_index - 1;
                            let u_horn = self.buffer[u_index];
                            let o_horn = self.buffer[o_index];
                            if (u_horn == 'ư' || u_horn == 'Ư') && (o_horn == 'ơ' || o_horn == 'Ơ') {
                                let delete_count = self.buffer.len() - u_index;
                                let original_u = if u_horn.is_uppercase() { 'U' } else { 'u' };
                                let original_o = if o_horn.is_uppercase() { 'O' } else { 'o' };
                                self.buffer[u_index] = original_u;
                                self.buffer[o_index] = original_o;
                                self.buffer.push(ch);
                                self.last_transform_key = None;
                                self.last_w_transform_kind = WTransformKind::None;
                                self.suppressed_transform_key = Some(ch_lower);
                                let output_text = self.buffer_string_from(u_index);
                                return Some(KeyTransformAction {
                                    delete_count,
                                    text: output_text,
                                });
                            }
                        }
                    }
                }
                WTransformKind::CompoundUaw => {
                    let end_index = self.buffer.len();
                    if self.buffer.len() >= 2 {
                        let a_index = end_index - 1;
                        let u_index = end_index - 2;
                        let u_horn = self.buffer[u_index];
                        let a_char = self.buffer[a_index];
                        if (u_horn == 'ư' || u_horn == 'Ư') && (a_char == 'a' || a_char == 'A') {
                            let original_u = if u_horn.is_uppercase() { 'U' } else { 'u' };
                            self.buffer.drain(u_index..);
                            self.buffer.push(original_u);
                            self.buffer.push(a_char);
                            self.buffer.push(ch);
                            self.last_transform_key = None;
                            self.last_w_transform_kind = WTransformKind::None;
                            self.suppressed_transform_key = Some(ch_lower);
                            return Some(KeyTransformAction {
                                delete_count: 2,
                                text: format!("{}{}{}", original_u, a_char, ch),
                            });
                        }
                    }
                }
                WTransformKind::None => {
                    if let Some((index, original)) =
                        self.find_last_untransformable_vowel(ch_lower, self.buffer.len())
                    {
                        let delete_count = self.buffer.len() - index;
                        self.buffer[index] = original;
                        self.buffer.push(ch);
                        self.last_transform_key = None;
                        self.last_w_transform_kind = WTransformKind::None;
                        self.suppressed_transform_key = Some(ch_lower);
                        let output_text = self.buffer_string_from(index);
                        return Some(KeyTransformAction {
                            delete_count,
                            text: output_text,
                        });
                    }
                }
            }
        }

        if ch_lower == 'd' {
            if let Some(&last_char) = self.buffer.last() {
                if last_char == 'đ' || last_char == 'Đ' {
                    let is_upper = last_char == 'Đ';
                    let replacement = if is_upper {
                        if ch.is_uppercase() {
                            "DD".to_string()
                        } else {
                            "Dd".to_string()
                        }
                    } else if ch.is_uppercase() {
                        "dD".to_string()
                    } else {
                        "dd".to_string()
                    };
                    self.buffer.pop();
                    self.buffer.extend(replacement.chars());
                    self.last_transform_key = None;
                    self.last_w_transform_kind = WTransformKind::None;
                    self.suppressed_transform_key = Some(ch_lower);
                    return Some(KeyTransformAction {
                        delete_count: 1,
                        text: replacement,
                    });
                }
            }
        }

        if let Some(&last_char) = self.buffer.last() {
            if let Some((key, original)) = VOWEL_UNTRANSFORMS.get(&last_char) {
                if lower_char(*key) == ch_lower {
                    self.buffer.pop();
                    self.buffer.push(*original);
                    self.buffer.push(ch);
                    self.last_transform_key = None;
                    self.last_w_transform_kind = WTransformKind::None;
                    self.suppressed_transform_key = Some(ch_lower);
                    return Some(KeyTransformAction {
                        delete_count: 1,
                        text: format!("{}{}", original, ch),
                    });
                }
            }
        }

        if ch_lower == 'a' || ch_lower == 'e' || ch_lower == 'o' {
            if let Some((index, original)) =
                self.find_last_untransformable_vowel(ch_lower, self.buffer.len())
            {
                let delete_count = self.buffer.len() - index;
                self.buffer[index] = original;
                self.buffer.push(ch);
                self.last_transform_key = None;
                self.last_w_transform_kind = WTransformKind::None;
                self.suppressed_transform_key = Some(ch_lower);
                let output_text = self.buffer_string_from(index);
                return Some(KeyTransformAction {
                    delete_count,
                    text: output_text,
                });
            }
        }

        if is_tone_key(ch) {
            if let Some(toned_index) = self.find_last_toned_vowel_index() {
                if let Some((base_vowel, last_tone)) = TONED_TO_BASE.get(&self.buffer[toned_index])
                {
                    if lower_char(*last_tone) == ch_lower {
                        let delete_count = self.buffer.len() - toned_index;
                        self.buffer[toned_index] = *base_vowel;
                        self.buffer.push(ch);
                        self.last_transform_key = None;
                        self.last_w_transform_kind = WTransformKind::None;
                        self.suppressed_transform_key = Some(ch_lower);
                        let output_text = self.buffer_string_from(toned_index);
                        return Some(KeyTransformAction {
                            delete_count,
                            text: output_text,
                        });
                    }
                }
            }
        }

        None
    }

    pub(super) fn try_telex_consonant_transform(&mut self, ch: char) -> Option<KeyTransformAction> {
        let ch_lower = lower_char(ch);
        if ch_lower != 'd' {
            return None;
        }

        if self.buffer.len() < 2 {
            return None;
        }

        let trigger_index = self.buffer.len() - 1;
        let first_d_index =
            self.find_last_matching_d_index(trigger_index, 4)?;

        let first_d = self.buffer[first_d_index];
        let result = if first_d.is_uppercase() { 'Đ' } else { 'đ' };
        let delete_count = trigger_index - first_d_index;

        self.buffer[first_d_index] = result;
        self.buffer.pop();
        self.last_transform_key = Some('d');
        self.last_w_transform_kind = WTransformKind::None;

        let output_text = self.buffer_string_from(first_d_index);
        Some(KeyTransformAction {
            delete_count,
            text: output_text,
        })
    }

    pub(super) fn try_telex_vowel_transform(&mut self, ch: char) -> Option<KeyTransformAction> {
        let ch_lower = lower_char(ch);

        if ch_lower == 'w' {
            if self.buffer.is_empty() {
                return None;
            }

            if let Some(action) = self.try_compound_uaw_escape() {
                return Some(action);
            }

            if let Some(action) = self.try_compound_uo_final_consonant_w_transform() {
                return Some(action);
            }

            if let Some(action) = self.try_compound_uuw_transform() {
                return Some(action);
            }

            if let Some(action) = self.try_compound_uou_w_transform() {
                return Some(action);
            }

            if let Some(action) = self.try_compound_ouw_transform() {
                return Some(action);
            }

            if let Some(action) = self.try_compound_uow_transform() {
                return Some(action);
            }

            if let Some(action) = self.try_compound_uaw_transform() {
                return Some(action);
            }

            if let Some((index, transform)) =
                self.find_last_transformable_vowel(ch, self.buffer.len() - 1)
            {
                let result = transform.1;
                self.buffer[index] = result;
                self.buffer.pop();
                self.last_transform_key = Some('w');
                self.last_w_transform_kind = WTransformKind::None;
                let delete_count = self.buffer.len() - index;
                let output_text = self.buffer_string_from(index);
                return Some(KeyTransformAction {
                    delete_count,
                    text: output_text,
                });
            }

            let replacement = if ch.is_uppercase() { 'Ư' } else { 'ư' };
            self.buffer.pop();
            self.buffer.push(replacement);
            self.last_transform_key = Some('w');
            self.last_w_transform_kind = WTransformKind::Standalone;
            return Some(KeyTransformAction {
                delete_count: 0,
                text: replacement.to_string(),
            });
        }

        if self.buffer.len() < 2 {
            return None;
        }

        if ch_lower == 'a' || ch_lower == 'e' || ch_lower == 'o' {
            if let Some(transforms) = VOWEL_TRANSFORMS.get(&ch) {
                if let Some(vowel_index) = self.find_last_matching_vowel_index(ch, self.buffer.len() - 1, 4) {
                    let vowel = self.buffer[vowel_index];
                    let vowel_base = self.get_base_vowel(vowel);
                    if let Some(transform) = transforms
                        .iter()
                        .find(|(from, _)| *from == vowel_base || *from == vowel)
                    {
                        let mut result = transform.1;
                        if let Some((_, tone)) = TONED_TO_BASE.get(&vowel) {
                            if let Some(tone_map) = VOWEL_TO_TONED.get(&result) {
                                if let Some(toned_result) = tone_map.get(tone) {
                                    result = *toned_result;
                                }
                            }
                        }

                        let vowel_offset = vowel_index;
                        let trigger_index = self.buffer.len() - 1;
                        let delete_count = trigger_index - vowel_index;

                        self.buffer[vowel_index] = result;
                        self.buffer.pop();
                        self.last_transform_key = Some(ch);
                        self.last_w_transform_kind = WTransformKind::None;

                        if self.auto_fix_tone {
                            if let Some(action) = self.reposition_tone_if_needed(false, Some(vowel_offset)) {
                                return Some(action);
                            }
                        }

                        let output_text = self.buffer_string_from(vowel_offset);
                        return Some(KeyTransformAction {
                            delete_count,
                            text: output_text,
                        });
                    }
                }
            }
        }

        None
    }

    pub(super) fn try_telex_tone_mark(&mut self, ch: char) -> Option<KeyTransformAction> {
        let ch_lower = lower_char(ch);
        if !is_tone_key(ch_lower) {
            return None;
        }

        if self.buffer.is_empty() {
            return None;
        }

        let trigger_index = self.buffer.len() - 1;
        let vowel_index = self.find_target_vowel_index(trigger_index)?;
        let vowel = self.buffer[vowel_index];
        let mut start_index = vowel_index;
        if let Some(earliest) = self.clear_other_tones(vowel_index, trigger_index) {
            if earliest < start_index {
                start_index = earliest;
            }
        }

        if ch_lower == 'z' {
            let base_vowel = self.get_base_vowel(vowel);
            let mut changed = false;
            if base_vowel != vowel {
                self.buffer[vowel_index] = base_vowel;
                changed = true;
            }
            if start_index != vowel_index {
                changed = true;
            }
            if !changed {
                return None;
            }
            self.buffer.pop();
            self.last_transform_key = Some('z');
            self.last_w_transform_kind = WTransformKind::None;
            let delete_count = trigger_index - start_index;
            let output_text = self.buffer_string_from(start_index);
            return Some(KeyTransformAction {
                delete_count,
                text: output_text,
            });
        }

        let base_vowel = self.get_base_vowel(vowel);
        let tone_map = VOWEL_TO_TONED.get(&base_vowel)?;
        let toned_vowel = tone_map.get(&ch_lower)?;

        self.buffer[vowel_index] = *toned_vowel;
        self.buffer.pop();
        self.last_transform_key = Some(ch);
        self.last_w_transform_kind = WTransformKind::None;

        let delete_count = trigger_index - start_index;
        let output_text = self.buffer_string_from(start_index);
        Some(KeyTransformAction {
            delete_count,
            text: output_text,
        })
    }

    // ==================== Telex Compound Transforms ====================

    fn try_compound_uo_final_consonant_w_transform(&mut self) -> Option<KeyTransformAction> {
        if self.buffer.len() < 4 {
            return None;
        }

        let trigger_index = self.buffer.len() - 1;
        if trigger_index == 0 {
            return None;
        }
        let mut scan_index = trigger_index - 1;

        if is_vowel(self.buffer[scan_index]) {
            return None;
        }

        while scan_index > 0 && !is_vowel(self.buffer[scan_index]) {
            scan_index -= 1;
        }

        if !is_vowel(self.buffer[scan_index]) {
            return None;
        }

        let o_index = scan_index;
        let o = self.buffer[o_index];
        if o != 'o' && o != 'O' {
            return None;
        }
        if TONED_TO_BASE.contains_key(&o) {
            return None;
        }
        if o_index == 0 {
            return None;
        }

        let u_index = o_index - 1;
        let u = self.buffer[u_index];
        if u != 'u' && u != 'U' {
            return None;
        }
        if TONED_TO_BASE.contains_key(&u) {
            return None;
        }

        if u_index > 0 {
            let prev_char = self.buffer[u_index - 1];
            if prev_char == 'q' || prev_char == 'Q' {
                return None;
            }
        }

        let u_horn = if u.is_uppercase() { 'Ư' } else { 'ư' };
        let o_horn = if o.is_uppercase() { 'Ơ' } else { 'ơ' };

        self.buffer[u_index] = u_horn;
        self.buffer[o_index] = o_horn;
        self.buffer.pop();
        self.last_transform_key = Some('w');
        self.last_w_transform_kind = WTransformKind::CompoundUoFinalConsonantW;

        let delete_count = self.buffer.len() - u_index;
        let output_text = self.buffer_string_from(u_index);
        Some(KeyTransformAction {
            delete_count,
            text: output_text,
        })
    }

    fn try_compound_uow_transform(&mut self) -> Option<KeyTransformAction> {
        if self.buffer.len() < 3 {
            return None;
        }

        let trigger_index = self.buffer.len() - 1;
        let o_index = trigger_index - 1;
        let u_index = o_index - 1;

        let u = self.buffer[u_index];
        let o = self.buffer[o_index];

        if (u != 'u' && u != 'U') || (o != 'o' && o != 'O') {
            return None;
        }

        if TONED_TO_BASE.contains_key(&u) || TONED_TO_BASE.contains_key(&o) {
            return None;
        }

        if u_index > 0 {
            let prev_char = self.buffer[u_index - 1];
            if prev_char == 'q' || prev_char == 'Q' {
                return None;
            }
        }

        let u_horn = if u.is_uppercase() { 'Ư' } else { 'ư' };
        let o_horn = if o.is_uppercase() { 'Ơ' } else { 'ơ' };

        self.buffer[u_index] = u_horn;
        self.buffer[o_index] = o_horn;
        self.buffer.pop();
        self.last_transform_key = Some('w');
        self.last_w_transform_kind = WTransformKind::CompoundUow;

        let delete_count = self.buffer.len() - u_index;
        let output_text = self.buffer_string_from(u_index);
        Some(KeyTransformAction {
            delete_count,
            text: output_text,
        })
    }

    fn try_compound_uaw_transform(&mut self) -> Option<KeyTransformAction> {
        if self.buffer.len() < 3 {
            return None;
        }

        let trigger_index = self.buffer.len() - 1;
        let a_index = trigger_index - 1;
        let u_index = a_index - 1;

        let u = self.buffer[u_index];
        let a = self.buffer[a_index];

        if (u != 'u' && u != 'U') || (a != 'a' && a != 'A') {
            return None;
        }

        if TONED_TO_BASE.contains_key(&u) || TONED_TO_BASE.contains_key(&a) {
            return None;
        }

        if u_index > 0 {
            let prev_char = self.buffer[u_index - 1];
            if prev_char == 'q' || prev_char == 'Q' {
                return None;
            }
        }

        let u_horn = if u.is_uppercase() { 'Ư' } else { 'ư' };
        self.buffer[u_index] = u_horn;
        self.buffer.pop();
        self.last_transform_key = Some('w');
        self.last_w_transform_kind = WTransformKind::CompoundUaw;

        let delete_count = self.buffer.len() - u_index;
        let output_text = self.buffer_string_from(u_index);
        Some(KeyTransformAction {
            delete_count,
            text: output_text,
        })
    }

    fn try_compound_uaw_escape(&mut self) -> Option<KeyTransformAction> {
        if self.buffer.len() < 3 {
            return None;
        }

        let trigger_index = self.buffer.len() - 1;
        let a_index = trigger_index - 1;
        let u_index = a_index - 1;

        let u_horn = self.buffer[u_index];
        let a = self.buffer[a_index];

        if (u_horn != 'ư' && u_horn != 'Ư') || (a != 'a' && a != 'A') {
            return None;
        }

        let delete_count = trigger_index - u_index;
        let original_u = if u_horn.is_uppercase() { 'U' } else { 'u' };

        self.buffer[u_index] = original_u;
        self.last_transform_key = None;
        self.last_w_transform_kind = WTransformKind::None;
        self.suppressed_transform_key = Some('w');

        let output_text = self.buffer_string_from(u_index);
        Some(KeyTransformAction {
            delete_count,
            text: output_text,
        })
    }

    fn try_compound_uuw_transform(&mut self) -> Option<KeyTransformAction> {
        if self.buffer.len() < 3 {
            return None;
        }

        let trigger_index = self.buffer.len() - 1;
        let last_u_index = trigger_index - 1;
        let first_u_index = last_u_index - 1;

        let first_u = self.buffer[first_u_index];
        let last_u = self.buffer[last_u_index];

        if (first_u != 'u' && first_u != 'U') || (last_u != 'u' && last_u != 'U') {
            return None;
        }

        if TONED_TO_BASE.contains_key(&first_u) || TONED_TO_BASE.contains_key(&last_u) {
            return None;
        }

        if first_u_index > 0 {
            let prev_char = self.buffer[first_u_index - 1];
            if prev_char == 'q' || prev_char == 'Q' {
                return None;
            }
        }

        let u_horn = if first_u.is_uppercase() { 'Ư' } else { 'ư' };
        self.buffer[first_u_index] = u_horn;
        self.buffer.pop();
        self.last_transform_key = Some('w');
        self.last_w_transform_kind = WTransformKind::None;

        let delete_count = self.buffer.len() - first_u_index;
        let output_text = self.buffer_string_from(first_u_index);
        Some(KeyTransformAction {
            delete_count,
            text: output_text,
        })
    }

    fn try_compound_ouw_transform(&mut self) -> Option<KeyTransformAction> {
        if self.buffer.len() < 3 {
            return None;
        }

        let trigger_index = self.buffer.len() - 1;
        let u_index = trigger_index - 1;
        let o_index = u_index - 1;

        let o = self.buffer[o_index];
        let u = self.buffer[u_index];

        if (o != 'o' && o != 'O') || (u != 'u' && u != 'U') {
            return None;
        }

        if TONED_TO_BASE.contains_key(&o) || TONED_TO_BASE.contains_key(&u) {
            return None;
        }

        let u_horn = if u.is_uppercase() { 'Ư' } else { 'ư' };
        let o_horn = if o.is_uppercase() { 'Ơ' } else { 'ơ' };

        self.buffer[o_index] = u_horn;
        self.buffer[u_index] = o_horn;
        self.buffer.pop();
        self.last_transform_key = Some('w');
        self.last_w_transform_kind = WTransformKind::CompoundUow;

        let delete_count = self.buffer.len() - o_index;
        let output_text = self.buffer_string_from(o_index);
        Some(KeyTransformAction {
            delete_count,
            text: output_text,
        })
    }

    fn try_compound_uou_w_transform(&mut self) -> Option<KeyTransformAction> {
        if self.buffer.len() < 4 {
            return None;
        }

        let trigger_index = self.buffer.len() - 1;
        let last_u_index = trigger_index - 1;
        let o_index = last_u_index - 1;
        let first_u_index = o_index - 1;

        let first_u = self.buffer[first_u_index];
        let o = self.buffer[o_index];
        let last_u = self.buffer[last_u_index];

        if (first_u != 'u' && first_u != 'U')
            || (o != 'o' && o != 'O')
            || (last_u != 'u' && last_u != 'U')
        {
            return None;
        }

        if TONED_TO_BASE.contains_key(&first_u)
            || TONED_TO_BASE.contains_key(&o)
            || TONED_TO_BASE.contains_key(&last_u)
        {
            return None;
        }

        if first_u_index > 0 {
            let prev_char = self.buffer[first_u_index - 1];
            if prev_char == 'q' || prev_char == 'Q' {
                return None;
            }
        }

        let u_horn = if first_u.is_uppercase() { 'Ư' } else { 'ư' };
        let o_horn = if o.is_uppercase() { 'Ơ' } else { 'ơ' };

        self.buffer[first_u_index] = u_horn;
        self.buffer[o_index] = o_horn;
        self.buffer.pop();
        self.last_transform_key = Some('w');
        self.last_w_transform_kind = WTransformKind::CompoundUow;

        let delete_count = self.buffer.len() - first_u_index;
        let output_text = self.buffer_string_from(first_u_index);
        Some(KeyTransformAction {
            delete_count,
            text: output_text,
        })
    }

    // ==================== Telex Helper Methods ====================

    fn find_last_matching_d_index(&self, end_index: usize, max_distance: usize) -> Option<usize> {
        let mut index = end_index;
        let mut distance = 0;
        while index > 0 && distance < max_distance {
            index -= 1;
            distance += 1;
            let ch = self.buffer[index];
            if lower_char(ch) == 'd' {
                return Some(index);
            }
        }
        None
    }

    fn find_last_untransformable_vowel(
        &self,
        key_lower: char,
        before: usize,
    ) -> Option<(usize, char)> {
        let mut index = before;
        while index > 0 {
            index -= 1;
            if let Some((key, original)) = VOWEL_UNTRANSFORMS.get(&self.buffer[index]) {
                if lower_char(*key) == key_lower {
                    return Some((index, *original));
                }
            }
        }
        None
    }

    fn find_last_transformable_vowel(
        &self,
        key: char,
        before: usize,
    ) -> Option<(usize, (char, char))> {
        let transforms = VOWEL_TRANSFORMS.get(&key)?;
        let mut index = before;
        while index > 0 {
            index -= 1;
            let ch = self.buffer[index];
            if let Some(transform) = transforms.iter().find(|(from, _)| *from == ch) {
                return Some((index, *transform));
            }
        }
        None
    }
}
