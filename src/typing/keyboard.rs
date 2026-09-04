// Physical QWERTY keyboard geometry model and Euclidean distance calculator

use rand::Rng;
use std::collections::HashMap;

/// Represents a single physical key on a standard QWERTY keyboard
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Key {
    pub character: char,
    pub x: f64,
    pub y: f64,
    pub row: u8,
}

/// Physical keyboard layout and geometric distance model
#[derive(Debug, Clone)]
pub struct KeyboardModel {
    keys: HashMap<char, Key>,
    /// Directional weight for horizontal distance (X axis)
    pub weight_x: f64,
    /// Directional weight for vertical distance (Y axis)
    pub weight_y: f64,
}

impl Default for KeyboardModel {
    fn default() -> Self {
        Self::new_qwerty(1.0, 1.1)
    }
}

impl KeyboardModel {
    /// Creates a new QWERTY keyboard model with physical coordinates and row staggering
    pub fn new_qwerty(weight_x: f64, weight_y: f64) -> Self {
        let mut keys = HashMap::new();

        // Row 0: Number Row (y = 0.0, step = 1.0)
        let row0 = [
            ('`', 0.0),
            ('1', 1.0),
            ('2', 2.0),
            ('3', 3.0),
            ('4', 4.0),
            ('5', 5.0),
            ('6', 6.0),
            ('7', 7.0),
            ('8', 8.0),
            ('9', 9.0),
            ('0', 10.0),
            ('-', 11.0),
            ('=', 12.0),
        ];
        for (c, x) in row0 {
            keys.insert(
                c,
                Key {
                    character: c,
                    x,
                    y: 0.0,
                    row: 0,
                },
            );
        }

        // Row 1: Top Row (y = 1.0, offset = 0.5)
        let row1 = [
            ('q', 0.5),
            ('w', 1.5),
            ('e', 2.5),
            ('r', 3.5),
            ('t', 4.5),
            ('y', 5.5),
            ('u', 6.5),
            ('i', 7.5),
            ('o', 8.5),
            ('p', 9.5),
            ('[', 10.5),
            (']', 11.5),
            ('\\', 12.5),
        ];
        for (c, x) in row1 {
            keys.insert(
                c,
                Key {
                    character: c,
                    x,
                    y: 1.0,
                    row: 1,
                },
            );
        }

        // Row 2: Home Row (y = 2.0, offset = 0.75)
        let row2 = [
            ('a', 0.75),
            ('s', 1.75),
            ('d', 2.75),
            ('f', 3.75),
            ('g', 4.75),
            ('h', 5.75),
            ('j', 6.75),
            ('k', 7.75),
            ('l', 8.75),
            (';', 9.75),
            ('\'', 10.75),
        ];
        for (c, x) in row2 {
            keys.insert(
                c,
                Key {
                    character: c,
                    x,
                    y: 2.0,
                    row: 2,
                },
            );
        }

        // Row 3: Bottom Row (y = 3.0, offset = 1.25)
        let row3 = [
            ('z', 1.25),
            ('x', 2.25),
            ('c', 3.25),
            ('v', 4.25),
            ('b', 5.25),
            ('n', 6.25),
            ('m', 7.25),
            (',', 8.25),
            ('.', 9.25),
            ('/', 10.25),
        ];
        for (c, x) in row3 {
            keys.insert(
                c,
                Key {
                    character: c,
                    x,
                    y: 3.0,
                    row: 3,
                },
            );
        }

        // Spacebar
        keys.insert(
            ' ',
            Key {
                character: ' ',
                x: 5.5,
                y: 4.0,
                row: 4,
            },
        );

        Self {
            keys,
            weight_x,
            weight_y,
        }
    }

    /// Looks up key representation for a character (case-insensitive mapping to key physical position)
    pub fn get_key(&self, c: char) -> Option<&Key> {
        let lower = c.to_ascii_lowercase();
        self.keys.get(&lower)
    }

    /// Computes the weighted physical Euclidean distance between two characters
    pub fn distance(&self, c1: char, c2: char) -> Option<f64> {
        let k1 = self.get_key(c1)?;
        let k2 = self.get_key(c2)?;

        let dx = (k1.x - k2.x).abs();
        let dy = (k1.y - k2.y).abs();

        Some((self.weight_x * dx * dx + self.weight_y * dy * dy).sqrt())
    }

    /// Returns all keyboard characters sorted by physical proximity to the target character
    pub fn neighbors(&self, target: char, max_distance: f64) -> Vec<(char, f64)> {
        let mut candidates = Vec::new();
        let target_lower = target.to_ascii_lowercase();

        for &c in self.keys.keys() {
            if c == target_lower || !c.is_alphabetic() {
                continue;
            }
            if let Some(dist) = self.distance(target_lower, c) {
                if dist <= max_distance {
                    candidates.push((c, dist));
                }
            }
        }

        candidates.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
        candidates
    }

    /// Samples a plausible typo substitute key based on keyboard proximity distribution
    /// P(c_cand | c_target) proportional to exp(-alpha * distance)
    pub fn sample_neighbor<R: Rng>(&self, target: char, alpha: f64, rng: &mut R) -> Option<char> {
        let neighbors = self.neighbors(target, 2.5);
        if neighbors.is_empty() {
            return None;
        }

        // Compute unnormalized exponential proximity weights
        let weights: Vec<f64> = neighbors
            .iter()
            .map(|(_, dist)| (-alpha * dist).exp())
            .collect();

        let total_weight: f64 = weights.iter().sum();
        if total_weight <= 0.0 {
            return None;
        }

        let mut r = rng.gen_range(0.0..total_weight);
        for (i, (c, _)) in neighbors.iter().enumerate() {
            r -= weights[i];
            if r <= 0.0 {
                // Preserve original casing
                return if target.is_uppercase() {
                    Some(c.to_ascii_uppercase())
                } else {
                    Some(*c)
                };
            }
        }

        let last_c = neighbors.last().unwrap().0;
        if target.is_uppercase() {
            Some(last_c.to_ascii_uppercase())
        } else {
            Some(last_c)
        }
    }
}
