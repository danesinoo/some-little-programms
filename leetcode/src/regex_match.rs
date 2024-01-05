#![allow(dead_code, unused_variables)]
use std::collections::VecDeque;

pub struct Solution;

// * means zero or more of any char element
// . means any single char element
impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        if s == p || p == ".*".to_string() {
            return true;
        }

        Self::eq(s, p)
    }

    // preconditions:
    // - i, j >= 0
    // s[0..i] == p[0..j]
    fn eq(s: String, p: String) -> bool {
        let mut d = VecDeque::new();
        d.push_back((0, 0));
        let mut solutions = Vec::new();

        while d.len() > 0 {
            let (mut i, j) = d.pop_front().unwrap(); // there is always at least one element in the queue
            if solutions.contains(&(i, j)) {
                continue;
            } else {
                solutions.push((i, j));
            }

            if i == s.len() && j == p.len() {
                return true;
            }

            let mut s_char = s.chars().nth(i).unwrap_or(' ');
            let p_char = p.chars().nth(j).unwrap_or(' ');
            let p_char_next_is_asteriks = p.chars().nth(j + 1).unwrap_or(' ') == '*';

            // if i > s.len(), j > p.len() -> s_char == ' ', p_char == ' '

            if p_char_next_is_asteriks {
                // there need to be the asteriks and the char before it
                // so p_char is some char
                while i < s.len() + 1 {
                    d.push_back((i, j + 2)); // skip the asteriks
                    if s_char == p_char || p_char == '.' {
                        i += 1;
                        s_char = s.chars().nth(i).unwrap_or(' ');
                    } else {
                        break;
                    }
                }
            }

            if (p_char == '.' && s_char != ' ') || (p_char == s_char) {
                d.push_back((i + 1, j + 1));
            }
        }
        false
    }
}

#[cfg(test)]
pub mod test {
    use super::Solution;

    pub fn abbreviazione(s: &str, p: &str, expected: bool) {
        assert_eq!(
            Solution::is_match(s.to_string(), p.to_string()),
            expected,
            "FAILED: s:{} p:{}",
            s,
            p
        );
    }

    #[test]
    fn is_match() {
        abbreviazione("aa", "a*", true);
        abbreviazione("aa", "a", false);
        abbreviazione("ab", ".*", true);
        abbreviazione("aab", "c*a*b", true);
        abbreviazione("mississippi", "mis*s*ip*.", false);
        abbreviazione("aa", "...", false);
        abbreviazione("a", "ab*", true);
        abbreviazione("ab", ".*c", false);
        abbreviazione("a", "ab*a", false);
        abbreviazione("abcaaaaaaabaabcabac", ".*ab.a.*a*a*.*b*b*", true);
        abbreviazione("aaaaaaaaaaaaaaaaaaab", "a*a*a*a*a*a*a*a*a*a*", false);
    }
}
