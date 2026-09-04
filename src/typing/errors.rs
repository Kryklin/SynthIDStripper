// Independent error classes and character mutation generators

use crate::types::TypingErrorClass;
use crate::typing::keyboard::KeyboardModel;
use rand::Rng;

/// Result of applying a single typing error to a word
#[derive(Debug, Clone, PartialEq)]
pub struct ErrorMutationResult {
    pub mutated_word: String,
    pub error_class: TypingErrorClass,
    pub char_position: usize,
    pub original_char: String,
    pub mutated_char: String,
    pub keyboard_distance: f64,
}

/// Applies a substitution error: replaces intended character with a nearby key
pub fn apply_substitution<R: Rng>(
    word: &str,
    keyboard: &KeyboardModel,
    rng: &mut R,
) -> Option<ErrorMutationResult> {
    let chars: Vec<char> = word.chars().collect();
    if chars.is_empty() {
        return None;
    }

    // Pick an alphabetic character position
    let alpha_indices: Vec<usize> = chars
        .iter()
        .enumerate()
        .filter(|(_, c)| c.is_alphabetic())
        .map(|(i, _)| i)
        .collect();

    if alpha_indices.is_empty() {
        return None;
    }

    let pos = alpha_indices[rng.gen_range(0..alpha_indices.len())];
    let original_c = chars[pos];

    let sub_c = keyboard.sample_neighbor(original_c, 1.8, rng)?;
    let dist = keyboard.distance(original_c, sub_c).unwrap_or(1.0);

    let mut new_chars = chars.clone();
    new_chars[pos] = sub_c;

    Some(ErrorMutationResult {
        mutated_word: new_chars.into_iter().collect(),
        error_class: TypingErrorClass::Substitution,
        char_position: pos,
        original_char: original_c.to_string(),
        mutated_char: sub_c.to_string(),
        keyboard_distance: dist,
    })
}

/// Applies a transposition error: swaps two adjacent characters
pub fn apply_transposition<R: Rng>(
    word: &str,
    keyboard: &KeyboardModel,
    rng: &mut R,
) -> Option<ErrorMutationResult> {
    let chars: Vec<char> = word.chars().collect();
    if chars.len() < 2 {
        return None;
    }

    // Find candidate adjacent pairs of alphabetic characters
    let mut candidate_pairs = Vec::new();
    for i in 0..chars.len() - 1 {
        if chars[i].is_alphabetic() && chars[i + 1].is_alphabetic() && chars[i] != chars[i + 1] {
            let dist = keyboard.distance(chars[i], chars[i + 1]).unwrap_or(2.0);
            candidate_pairs.push((i, dist));
        }
    }

    if candidate_pairs.is_empty() {
        return None;
    }

    // Weight transposition probability higher for closer adjacent keys (same hand / nearby fingers)
    let weights: Vec<f64> = candidate_pairs
        .iter()
        .map(|(_, d)| (-0.5 * d).exp())
        .collect();
    let total_w: f64 = weights.iter().sum();

    let mut r = rng.gen_range(0.0..total_w);
    let mut selected_idx = candidate_pairs[0].0;
    let mut selected_dist = candidate_pairs[0].1;

    for (i, &(pos, d)) in candidate_pairs.iter().enumerate() {
        r -= weights[i];
        if r <= 0.0 {
            selected_idx = pos;
            selected_dist = d;
            break;
        }
    }

    let mut new_chars = chars.clone();
    new_chars.swap(selected_idx, selected_idx + 1);

    Some(ErrorMutationResult {
        mutated_word: new_chars.into_iter().collect(),
        error_class: TypingErrorClass::Transposition,
        char_position: selected_idx,
        original_char: format!("{}{}", chars[selected_idx], chars[selected_idx + 1]),
        mutated_char: format!("{}{}", chars[selected_idx + 1], chars[selected_idx]),
        keyboard_distance: selected_dist,
    })
}

/// Applies an omission error: drops one character
pub fn apply_omission<R: Rng>(word: &str, rng: &mut R) -> Option<ErrorMutationResult> {
    let chars: Vec<char> = word.chars().collect();
    // Do not omit from very short words (<= 3 chars) to avoid unreadable tokens
    if chars.len() <= 3 {
        return None;
    }

    let alpha_indices: Vec<usize> = chars
        .iter()
        .enumerate()
        .filter(|(_, c)| c.is_alphabetic())
        .map(|(i, _)| i)
        .collect();

    if alpha_indices.is_empty() {
        return None;
    }

    let pos = alpha_indices[rng.gen_range(0..alpha_indices.len())];
    let original_c = chars[pos];

    let mut new_chars = chars.clone();
    new_chars.remove(pos);

    Some(ErrorMutationResult {
        mutated_word: new_chars.into_iter().collect(),
        error_class: TypingErrorClass::Omission,
        char_position: pos,
        original_char: original_c.to_string(),
        mutated_char: String::new(),
        keyboard_distance: 0.0,
    })
}

/// Applies an insertion error: inserts a nearby keyboard key adjacent to a character
pub fn apply_insertion<R: Rng>(
    word: &str,
    keyboard: &KeyboardModel,
    rng: &mut R,
) -> Option<ErrorMutationResult> {
    let chars: Vec<char> = word.chars().collect();
    if chars.is_empty() {
        return None;
    }

    let alpha_indices: Vec<usize> = chars
        .iter()
        .enumerate()
        .filter(|(_, c)| c.is_alphabetic())
        .map(|(i, _)| i)
        .collect();

    if alpha_indices.is_empty() {
        return None;
    }

    let pos = alpha_indices[rng.gen_range(0..alpha_indices.len())];
    let original_c = chars[pos];

    let insert_c = keyboard.sample_neighbor(original_c, 2.0, rng)?;
    let dist = keyboard.distance(original_c, insert_c).unwrap_or(1.0);

    let mut new_chars = chars.clone();
    // Randomly insert immediately before or after the target position
    let insert_pos = if rng.gen_bool(0.5) { pos } else { pos + 1 };
    new_chars.insert(insert_pos, insert_c.to_ascii_lowercase());

    Some(ErrorMutationResult {
        mutated_word: new_chars.into_iter().collect(),
        error_class: TypingErrorClass::Insertion,
        char_position: insert_pos,
        original_char: original_c.to_string(),
        mutated_char: format!("+{}", insert_c),
        keyboard_distance: dist,
    })
}

/// Applies a duplication error: repeats a character stroke
pub fn apply_duplication<R: Rng>(word: &str, rng: &mut R) -> Option<ErrorMutationResult> {
    let chars: Vec<char> = word.chars().collect();
    if chars.is_empty() {
        return None;
    }

    let alpha_indices: Vec<usize> = chars
        .iter()
        .enumerate()
        .filter(|(_, c)| c.is_alphabetic())
        .map(|(i, _)| i)
        .collect();

    if alpha_indices.is_empty() {
        return None;
    }

    let pos = alpha_indices[rng.gen_range(0..alpha_indices.len())];
    let original_c = chars[pos];

    let mut new_chars = chars.clone();
    new_chars.insert(pos + 1, original_c.to_ascii_lowercase());

    Some(ErrorMutationResult {
        mutated_word: new_chars.into_iter().collect(),
        error_class: TypingErrorClass::Duplication,
        char_position: pos,
        original_char: original_c.to_string(),
        mutated_char: format!("{}{}", original_c, original_c),
        keyboard_distance: 0.0,
    })
}

/// Applies a temporal / key-transition error:
/// Models sequential finger-reach trajectory errors where hand movement between
/// previous_key and next_key leads to an intermediate neighbor substitution.
/// P(observed_key | intended_key, prev_key, next_key)
pub fn apply_temporal_transition<R: Rng>(
    word: &str,
    keyboard: &KeyboardModel,
    rng: &mut R,
) -> Option<ErrorMutationResult> {
    let chars: Vec<char> = word.chars().collect();
    if chars.len() < 3 {
        return None;
    }

    // Pick an interior character position with both a predecessor and successor
    let valid_positions: Vec<usize> = (1..chars.len() - 1)
        .filter(|&i| {
            chars[i - 1].is_alphabetic() && chars[i].is_alphabetic() && chars[i + 1].is_alphabetic()
        })
        .collect();

    if valid_positions.is_empty() {
        return None;
    }

    let pos = valid_positions[rng.gen_range(0..valid_positions.len())];
    let prev_c = chars[pos - 1];
    let curr_c = chars[pos];
    let next_c = chars[pos + 1];

    let neighbors = keyboard.neighbors(curr_c, 2.2);
    if neighbors.is_empty() {
        return None;
    }

    // Score candidates based on geometric interpolation along the trajectory between prev and next key
    let mut candidate_scores = Vec::new();
    for (cand_c, dist_to_curr) in neighbors {
        let dist_to_prev = keyboard.distance(prev_c, cand_c).unwrap_or(3.0);
        let dist_to_next = keyboard.distance(next_c, cand_c).unwrap_or(3.0);

        // A key along the physical travel path between prev and next has lower total path overhead
        let trajectory_cost = dist_to_curr * 1.5 + (dist_to_prev + dist_to_next) * 0.5;
        let score = (-1.2 * trajectory_cost).exp();
        candidate_scores.push((cand_c, dist_to_curr, score));
    }

    let total_score: f64 = candidate_scores.iter().map(|(_, _, s)| s).sum();
    if total_score <= 0.0 {
        return None;
    }

    let mut r = rng.gen_range(0.0..total_score);
    let mut chosen_c = candidate_scores[0].0;
    let mut chosen_dist = candidate_scores[0].1;

    for &(c, d, s) in &candidate_scores {
        r -= s;
        if r <= 0.0 {
            chosen_c = c;
            chosen_dist = d;
            break;
        }
    }

    let mut new_chars = chars.clone();
    new_chars[pos] = if curr_c.is_uppercase() {
        chosen_c.to_ascii_uppercase()
    } else {
        chosen_c
    };

    Some(ErrorMutationResult {
        mutated_word: new_chars.into_iter().collect(),
        error_class: TypingErrorClass::Temporal,
        char_position: pos,
        original_char: curr_c.to_string(),
        mutated_char: chosen_c.to_string(),
        keyboard_distance: chosen_dist,
    })
}
