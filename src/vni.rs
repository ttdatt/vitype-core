use once_cell::sync::Lazy;
use std::collections::{HashMap, HashSet};

use crate::common::{
    is_vowel, KeyTransformAction, WTransformKind,
    TONED_TO_BASE, VOWEL_TO_TONED,
};
use crate::VitypeEngine;

// ==================== VNI Helper Functions ====================

fn is_vni_tone_key(ch: char) -> bool {
    VNI_TONE_KEYS.contains(&ch)
}

fn is_vni_vowel_transform_key(ch: char) -> bool {
    ch == '6' || ch == '7' || ch == '8'
}

pub(super) fn is_vni_word_boundary(ch: char) -> bool {
    VNI_WORD_BOUNDARY_CHARS.contains(&ch)
}

// ==================== VNI Static Data ====================

static VNI_TONE_KEYS: Lazy<HashSet<char>> = Lazy::new(|| {
    HashSet::from(['0', '1', '2', '3', '4', '5'])
});

static VNI_TONE_MAP: Lazy<HashMap<char, char>> = Lazy::new(|| {
    HashMap::from([
        ('1', 's'), // sắc
        ('2', 'f'), // huyền
        ('3', 'r'), // hỏi
        ('4', 'x'), // ngã
        ('5', 'j'), // nặng
        ('0', 'z'), // remove tone
    ])
});

static VNI_WORD_BOUNDARY_CHARS: Lazy<HashSet<char>> = Lazy::new(|| {
    let chars = [
        ' ', '\n', '\r', '\t', ',', '.', ';', ':', '!', '?',
        '(', ')', '[', ']', '{', '}', '"', '\'', '/', '\\',
        '-', '_', '@', '#', '$', '%', '^', '&', '*', '=', '+',
        '<', '>', '`', '~', '|',
        // Note: NO digits here for VNI - they are transform keys
    ];
    chars.iter().cloned().collect()
});

static VNI_VOWEL_TRANSFORMS: Lazy<HashMap<char, Vec<(char, char)>>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert('6', vec![
        // Base transforms (circumflex)
        ('a', 'â'), ('A', 'Â'), ('e', 'ê'), ('E', 'Ê'), ('o', 'ô'), ('O', 'Ô'),
        // Override from breve (ă→â)
        ('ă', 'â'), ('Ă', 'Â'),
        // Override from horn (ơ→ô)
        ('ơ', 'ô'), ('Ơ', 'Ô'),
        // Toned variants: breve→circumflex
        ('ắ', 'ấ'), ('ằ', 'ầ'), ('ẳ', 'ẩ'), ('ẵ', 'ẫ'), ('ặ', 'ậ'),
        ('Ắ', 'Ấ'), ('Ằ', 'Ầ'), ('Ẳ', 'Ẩ'), ('Ẵ', 'Ẫ'), ('Ặ', 'Ậ'),
        // Toned variants: horn→circumflex (o only)
        ('ớ', 'ố'), ('ờ', 'ồ'), ('ở', 'ổ'), ('ỡ', 'ỗ'), ('ợ', 'ộ'),
        ('Ớ', 'Ố'), ('Ờ', 'Ồ'), ('Ở', 'Ổ'), ('Ỡ', 'Ỗ'), ('Ợ', 'Ộ'),
        // Toned base vowels
        ('á', 'ấ'), ('à', 'ầ'), ('ả', 'ẩ'), ('ã', 'ẫ'), ('ạ', 'ậ'),
        ('Á', 'Ấ'), ('À', 'Ầ'), ('Ả', 'Ẩ'), ('Ã', 'Ẫ'), ('Ạ', 'Ậ'),
        ('é', 'ế'), ('è', 'ề'), ('ẻ', 'ể'), ('ẽ', 'ễ'), ('ẹ', 'ệ'),
        ('É', 'Ế'), ('È', 'Ề'), ('Ẻ', 'Ể'), ('Ẽ', 'Ễ'), ('Ẹ', 'Ệ'),
        ('ó', 'ố'), ('ò', 'ồ'), ('ỏ', 'ổ'), ('õ', 'ỗ'), ('ọ', 'ộ'),
        ('Ó', 'Ố'), ('Ò', 'Ồ'), ('Ỏ', 'Ổ'), ('Õ', 'Ỗ'), ('Ọ', 'Ộ'),
    ]);
    map.insert('7', vec![
        // Base transforms (horn)
        ('o', 'ơ'), ('O', 'Ơ'), ('u', 'ư'), ('U', 'Ư'),
        // Override from circumflex (ô→ơ)
        ('ô', 'ơ'), ('Ô', 'Ơ'),
        // Toned variants: circumflex→horn (o only)
        ('ố', 'ớ'), ('ồ', 'ờ'), ('ổ', 'ở'), ('ỗ', 'ỡ'), ('ộ', 'ợ'),
        ('Ố', 'Ớ'), ('Ồ', 'Ờ'), ('Ổ', 'Ở'), ('Ỗ', 'Ỡ'), ('Ộ', 'Ợ'),
        // Toned base vowels
        ('ó', 'ớ'), ('ò', 'ờ'), ('ỏ', 'ở'), ('õ', 'ỡ'), ('ọ', 'ợ'),
        ('Ó', 'Ớ'), ('Ò', 'Ờ'), ('Ỏ', 'Ở'), ('Õ', 'Ỡ'), ('Ọ', 'Ợ'),
        ('ú', 'ứ'), ('ù', 'ừ'), ('ủ', 'ử'), ('ũ', 'ữ'), ('ụ', 'ự'),
        ('Ú', 'Ứ'), ('Ù', 'Ừ'), ('Ủ', 'Ử'), ('Ũ', 'Ữ'), ('Ụ', 'Ự'),
    ]);
    map.insert('8', vec![
        // Base transform (breve)
        ('a', 'ă'), ('A', 'Ă'),
        // Override from circumflex (â→ă)
        ('â', 'ă'), ('Â', 'Ă'),
        // Toned variants: circumflex→breve
        ('ấ', 'ắ'), ('ầ', 'ằ'), ('ẩ', 'ẳ'), ('ẫ', 'ẵ'), ('ậ', 'ặ'),
        ('Ấ', 'Ắ'), ('Ầ', 'Ằ'), ('Ẩ', 'Ẳ'), ('Ẫ', 'Ẵ'), ('Ậ', 'Ặ'),
        // Toned base vowels
        ('á', 'ắ'), ('à', 'ằ'), ('ả', 'ẳ'), ('ã', 'ẵ'), ('ạ', 'ặ'),
        ('Á', 'Ắ'), ('À', 'Ằ'), ('Ả', 'Ẳ'), ('Ã', 'Ẵ'), ('Ạ', 'Ặ'),
    ]);
    map
});

static VNI_VOWEL_UNTRANSFORMS: Lazy<HashMap<char, (char, char)>> = Lazy::new(|| {
    let mut map = HashMap::new();
    // '6' escapes (circumflex → base)
    map.insert('â', ('6', 'a')); map.insert('Â', ('6', 'A'));
    map.insert('ê', ('6', 'e')); map.insert('Ê', ('6', 'E'));
    map.insert('ô', ('6', 'o')); map.insert('Ô', ('6', 'O'));
    // Toned circumflex escapes
    map.insert('ấ', ('6', 'á')); map.insert('ầ', ('6', 'à')); map.insert('ẩ', ('6', 'ả')); map.insert('ẫ', ('6', 'ã')); map.insert('ậ', ('6', 'ạ'));
    map.insert('Ấ', ('6', 'Á')); map.insert('Ầ', ('6', 'À')); map.insert('Ẩ', ('6', 'Ả')); map.insert('Ẫ', ('6', 'Ã')); map.insert('Ậ', ('6', 'Ạ'));
    map.insert('ế', ('6', 'é')); map.insert('ề', ('6', 'è')); map.insert('ể', ('6', 'ẻ')); map.insert('ễ', ('6', 'ẽ')); map.insert('ệ', ('6', 'ẹ'));
    map.insert('Ế', ('6', 'É')); map.insert('Ề', ('6', 'È')); map.insert('Ể', ('6', 'Ẻ')); map.insert('Ễ', ('6', 'Ẽ')); map.insert('Ệ', ('6', 'Ẹ'));
    map.insert('ố', ('6', 'ó')); map.insert('ồ', ('6', 'ò')); map.insert('ổ', ('6', 'ỏ')); map.insert('ỗ', ('6', 'õ')); map.insert('ộ', ('6', 'ọ'));
    map.insert('Ố', ('6', 'Ó')); map.insert('Ồ', ('6', 'Ò')); map.insert('Ổ', ('6', 'Ỏ')); map.insert('Ỗ', ('6', 'Õ')); map.insert('Ộ', ('6', 'Ọ'));
    // '7' escapes (horn → base)
    map.insert('ơ', ('7', 'o')); map.insert('Ơ', ('7', 'O'));
    map.insert('ư', ('7', 'u')); map.insert('Ư', ('7', 'U'));
    // Toned horn escapes
    map.insert('ớ', ('7', 'ó')); map.insert('ờ', ('7', 'ò')); map.insert('ở', ('7', 'ỏ')); map.insert('ỡ', ('7', 'õ')); map.insert('ợ', ('7', 'ọ'));
    map.insert('Ớ', ('7', 'Ó')); map.insert('Ờ', ('7', 'Ò')); map.insert('Ở', ('7', 'Ỏ')); map.insert('Ỡ', ('7', 'Õ')); map.insert('Ợ', ('7', 'Ọ'));
    map.insert('ứ', ('7', 'ú')); map.insert('ừ', ('7', 'ù')); map.insert('ử', ('7', 'ủ')); map.insert('ữ', ('7', 'ũ')); map.insert('ự', ('7', 'ụ'));
    map.insert('Ứ', ('7', 'Ú')); map.insert('Ừ', ('7', 'Ù')); map.insert('Ử', ('7', 'Ủ')); map.insert('Ữ', ('7', 'Ũ')); map.insert('Ự', ('7', 'Ụ'));
    // '8' escapes (breve → base)
    map.insert('ă', ('8', 'a')); map.insert('Ă', ('8', 'A'));
    // Toned breve escapes
    map.insert('ắ', ('8', 'á')); map.insert('ằ', ('8', 'à')); map.insert('ẳ', ('8', 'ả')); map.insert('ẵ', ('8', 'ã')); map.insert('ặ', ('8', 'ạ'));
    map.insert('Ắ', ('8', 'Á')); map.insert('Ằ', ('8', 'À')); map.insert('Ẳ', ('8', 'Ả')); map.insert('Ẵ', ('8', 'Ã')); map.insert('Ặ', ('8', 'Ạ'));
    map
});

// ==================== VNI Methods on VitypeEngine ====================

impl VitypeEngine {
    pub(super) fn try_vni_escape_sequence(&mut self, ch: char) -> Option<KeyTransformAction> {
        let last_key = self.last_transform_key?;
        if ch != last_key {
            return None;
        }

        // Handle '7' escape for compound transforms (similar to 'w' in Telex)
        if ch == '7' {
            match self.last_w_transform_kind {
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
                            self.suppressed_transform_key = Some(ch);
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
                                self.suppressed_transform_key = Some(ch);
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
                            self.suppressed_transform_key = Some(ch);
                            return Some(KeyTransformAction {
                                delete_count: 2,
                                text: format!("{}{}{}", original_u, a_char, ch),
                            });
                        }
                    }
                }
                _ => {}
            }
        }

        // Handle '9' escape (đ → d9)
        if ch == '9' {
            if let Some(&last_char) = self.buffer.last() {
                if last_char == 'đ' || last_char == 'Đ' {
                    let d_char = if last_char == 'Đ' { 'D' } else { 'd' };
                    self.buffer.pop();
                    self.buffer.push(d_char);
                    self.buffer.push('9');
                    self.last_transform_key = None;
                    self.last_w_transform_kind = WTransformKind::None;
                    self.suppressed_transform_key = Some('9');
                    return Some(KeyTransformAction {
                        delete_count: 1,
                        text: format!("{}9", d_char),
                    });
                }
            }
        }

        // Handle '6', '7', '8' escapes (vowel transforms)
        if ch == '6' || ch == '7' || ch == '8' {
            // Check if last char in buffer is a transformed vowel
            if let Some(&last_char) = self.buffer.last() {
                if let Some((key, original)) = VNI_VOWEL_UNTRANSFORMS.get(&last_char) {
                    if *key == ch {
                        self.buffer.pop();
                        self.buffer.push(*original);
                        self.buffer.push(ch);
                        self.last_transform_key = None;
                        self.last_w_transform_kind = WTransformKind::None;
                        self.suppressed_transform_key = Some(ch);
                        return Some(KeyTransformAction {
                            delete_count: 1,
                            text: format!("{}{}", original, ch),
                        });
                    }
                }
            }

            // Check for non-adjacent transformed vowel (free transform escape)
            if let Some((index, original)) = self.find_last_vni_untransformable_vowel(ch, self.buffer.len()) {
                let delete_count = self.buffer.len() - index;
                self.buffer[index] = original;
                self.buffer.push(ch);
                self.last_transform_key = None;
                self.last_w_transform_kind = WTransformKind::None;
                self.suppressed_transform_key = Some(ch);
                let output_text = self.buffer_string_from(index);
                return Some(KeyTransformAction {
                    delete_count,
                    text: output_text,
                });
            }
        }

        // Handle tone escapes ('1'-'5', '0')
        if is_vni_tone_key(ch) {
            if let Some(toned_index) = self.find_last_toned_vowel_index() {
                if let Some((base_vowel, last_internal_tone)) = TONED_TO_BASE.get(&self.buffer[toned_index]) {
                    // Map internal tone back to VNI key
                    let last_vni_tone = VNI_TONE_MAP.iter()
                        .find(|(_, &v)| v == *last_internal_tone)
                        .map(|(&k, _)| k);

                    if last_vni_tone == Some(ch) {
                        let delete_count = self.buffer.len() - toned_index;
                        self.buffer[toned_index] = *base_vowel;
                        self.buffer.push(ch);
                        self.last_transform_key = None;
                        self.last_w_transform_kind = WTransformKind::None;
                        self.suppressed_transform_key = Some(ch);
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

    pub(super) fn try_vni_consonant_transform(&mut self, ch: char) -> Option<KeyTransformAction> {
        if ch != '9' {
            return None;
        }

        if self.buffer.is_empty() {
            return None;
        }

        let trigger_index = self.buffer.len() - 1;
        let d_index = self.find_last_d_index_for_vni(trigger_index, 4)?;

        let d_char = self.buffer[d_index];
        let result = if d_char.is_uppercase() { 'Đ' } else { 'đ' };
        let delete_count = trigger_index - d_index;

        self.buffer[d_index] = result;
        self.buffer.pop(); // Remove the '9'
        self.last_transform_key = Some('9');
        self.last_w_transform_kind = WTransformKind::None;

        let output_text = self.buffer_string_from(d_index);
        Some(KeyTransformAction {
            delete_count,
            text: output_text,
        })
    }

    pub(super) fn try_vni_vowel_transform(&mut self, ch: char) -> Option<KeyTransformAction> {
        if !is_vni_vowel_transform_key(ch) {
            return None;
        }

        if self.buffer.is_empty() {
            return None;
        }

        // Handle compound transforms for '7' key (similar to 'w' in Telex)
        if ch == '7' {
            if let Some(action) = self.try_vni_compound_ua7_escape() {
                return Some(action);
            }

            if let Some(action) = self.try_vni_compound_uo_final_consonant_7_transform() {
                return Some(action);
            }

            if let Some(action) = self.try_vni_compound_uu7_transform() {
                return Some(action);
            }

            if let Some(action) = self.try_vni_compound_uou7_transform() {
                return Some(action);
            }

            if let Some(action) = self.try_vni_compound_ou7_transform() {
                return Some(action);
            }

            if let Some(action) = self.try_vni_compound_uo7_transform() {
                return Some(action);
            }

            if let Some(action) = self.try_vni_compound_ua7_transform() {
                return Some(action);
            }
        }

        // Find transformable vowel and apply transform
        let transforms = VNI_VOWEL_TRANSFORMS.get(&ch)?;
        let trigger_index = self.buffer.len() - 1;

        // Search backward for a transformable vowel (free transform, up to 4 chars)
        let vowel_index = self.find_last_vni_transformable_vowel(ch, trigger_index, 4)?;
        let vowel = self.buffer[vowel_index];

        let transform = transforms.iter().find(|(from, _)| *from == vowel)?;
        let result = transform.1;

        let delete_count = trigger_index - vowel_index;
        self.buffer[vowel_index] = result;
        self.buffer.pop(); // Remove the transform key
        self.last_transform_key = Some(ch);
        self.last_w_transform_kind = WTransformKind::None;

        if self.auto_fix_tone {
            if let Some(action) = self.reposition_tone_if_needed(false, Some(vowel_index)) {
                return Some(action);
            }
        }

        let output_text = self.buffer_string_from(vowel_index);
        Some(KeyTransformAction {
            delete_count,
            text: output_text,
        })
    }

    pub(super) fn try_vni_tone_mark(&mut self, ch: char) -> Option<KeyTransformAction> {
        if !is_vni_tone_key(ch) {
            return None;
        }

        // Map VNI number key to internal (Telex) tone key
        let internal_tone_key = VNI_TONE_MAP.get(&ch)?;

        if self.buffer.is_empty() {
            return None;
        }

        let trigger_index = self.buffer.len() - 1;
        let vowel_index = self.find_target_vowel_index(trigger_index)?;
        let vowel = self.buffer[vowel_index];
        let delete_count = trigger_index - vowel_index;

        // Handle tone removal ('0' maps to 'z')
        if *internal_tone_key == 'z' {
            let base_vowel = self.get_base_vowel(vowel);
            if base_vowel == vowel {
                return None;
            }
            self.buffer[vowel_index] = base_vowel;
            self.buffer.pop();
            self.last_transform_key = Some(ch); // Store original VNI key
            self.last_w_transform_kind = WTransformKind::None;
            let output_text = self.buffer_string_from(vowel_index);
            return Some(KeyTransformAction {
                delete_count,
                text: output_text,
            });
        }

        let base_vowel = self.get_base_vowel(vowel);
        let tone_map = VOWEL_TO_TONED.get(&base_vowel)?;
        let toned_vowel = tone_map.get(internal_tone_key)?;

        self.buffer[vowel_index] = *toned_vowel;
        self.buffer.pop();
        self.last_transform_key = Some(ch); // Store original VNI key
        self.last_w_transform_kind = WTransformKind::None;

        let output_text = self.buffer_string_from(vowel_index);
        Some(KeyTransformAction {
            delete_count,
            text: output_text,
        })
    }

    // ==================== VNI Compound Transforms ====================

    fn try_vni_compound_uo7_transform(&mut self) -> Option<KeyTransformAction> {
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

        // Check for 'qu' cluster
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
        self.buffer.pop(); // Remove '7'
        self.last_transform_key = Some('7');
        self.last_w_transform_kind = WTransformKind::CompoundUow; // Reuse for escape handling

        let delete_count = self.buffer.len() - u_index;
        let output_text = self.buffer_string_from(u_index);
        Some(KeyTransformAction {
            delete_count,
            text: output_text,
        })
    }

    fn try_vni_compound_uo_final_consonant_7_transform(&mut self) -> Option<KeyTransformAction> {
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
        self.last_transform_key = Some('7');
        self.last_w_transform_kind = WTransformKind::CompoundUoFinalConsonantW;

        let delete_count = self.buffer.len() - u_index;
        let output_text = self.buffer_string_from(u_index);
        Some(KeyTransformAction {
            delete_count,
            text: output_text,
        })
    }

    fn try_vni_compound_ua7_transform(&mut self) -> Option<KeyTransformAction> {
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
        self.last_transform_key = Some('7');
        self.last_w_transform_kind = WTransformKind::CompoundUaw;

        let delete_count = self.buffer.len() - u_index;
        let output_text = self.buffer_string_from(u_index);
        Some(KeyTransformAction {
            delete_count,
            text: output_text,
        })
    }

    fn try_vni_compound_ua7_escape(&mut self) -> Option<KeyTransformAction> {
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
        self.suppressed_transform_key = Some('7');

        let output_text = self.buffer_string_from(u_index);
        Some(KeyTransformAction {
            delete_count,
            text: output_text,
        })
    }

    fn try_vni_compound_uu7_transform(&mut self) -> Option<KeyTransformAction> {
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
        self.last_transform_key = Some('7');
        self.last_w_transform_kind = WTransformKind::None;

        let delete_count = self.buffer.len() - first_u_index;
        let output_text = self.buffer_string_from(first_u_index);
        Some(KeyTransformAction {
            delete_count,
            text: output_text,
        })
    }

    fn try_vni_compound_ou7_transform(&mut self) -> Option<KeyTransformAction> {
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
        self.last_transform_key = Some('7');
        self.last_w_transform_kind = WTransformKind::CompoundUow;

        let delete_count = self.buffer.len() - o_index;
        let output_text = self.buffer_string_from(o_index);
        Some(KeyTransformAction {
            delete_count,
            text: output_text,
        })
    }

    fn try_vni_compound_uou7_transform(&mut self) -> Option<KeyTransformAction> {
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
        self.last_transform_key = Some('7');
        self.last_w_transform_kind = WTransformKind::CompoundUow;

        let delete_count = self.buffer.len() - first_u_index;
        let output_text = self.buffer_string_from(first_u_index);
        Some(KeyTransformAction {
            delete_count,
            text: output_text,
        })
    }

    // ==================== VNI Helper Methods ====================

    fn find_last_d_index_for_vni(&self, before: usize, max_distance: usize) -> Option<usize> {
        let mut index = before;
        let mut distance = 0;
        while index > 0 && distance < max_distance {
            index -= 1;
            distance += 1;
            let ch = self.buffer[index];
            if ch == 'd' || ch == 'D' {
                return Some(index);
            }
        }
        None
    }

    fn find_last_vni_transformable_vowel(&self, key: char, before: usize, max_distance: usize) -> Option<usize> {
        let transforms = VNI_VOWEL_TRANSFORMS.get(&key)?;
        let mut index = before;
        let mut distance = 0;

        while index > 0 && distance < max_distance {
            index -= 1;
            distance += 1;
            let ch = self.buffer[index];

            if transforms.iter().any(|(from, _)| *from == ch) {
                return Some(index);
            }

            // For free transform: if we hit a non-transformable vowel, stop
            // (except for trailing glides i/y/u immediately before the trigger)
            if is_vowel(ch) && distance > 1 {
                return None;
            }
        }
        None
    }

    fn find_last_vni_untransformable_vowel(&self, key: char, before: usize) -> Option<(usize, char)> {
        let mut index = before;
        while index > 0 {
            index -= 1;
            if let Some((k, original)) = VNI_VOWEL_UNTRANSFORMS.get(&self.buffer[index]) {
                if *k == key {
                    return Some((index, *original));
                }
            }
        }
        None
    }
}
