// Advanced formatter features

use crate::formatter::config::KeySortStyle;
use mon_core::ast::Member;

/// Sort object members according to configured style
pub fn sort_members(members: &[Member], style: KeySortStyle) -> Vec<Member> {
    if style == KeySortStyle::None {
        return members.to_vec();
    }

    let sorted = members.to_vec();

    // Separate type definitions (keep at top), anchors, and regular pairs
    let mut type_defs = Vec::new();
    let mut anchored = Vec::new();
    let mut regular = Vec::new();
    let mut other = Vec::new();

    for member in sorted.iter() {
        match member {
            Member::TypeDefinition(_) => type_defs.push(member.clone()),
            Member::Pair(pair) => {
                if pair.value.anchor.is_some() && pair.value.anchor.as_ref() == Some(&pair.key) {
                    anchored.push(member.clone());
                } else {
                    regular.push(member.clone());
                }
            }
            _ => other.push(member.clone()),
        }
    }

    // Sort anchored and regular pairs
    match style {
        KeySortStyle::None => {}
        KeySortStyle::Alpha => {
            anchored.sort_by(|a, b| {
                let key_a = get_member_key(a);
                let key_b = get_member_key(b);
                key_a.cmp(&key_b)
            });
            regular.sort_by(|a, b| {
                let key_a = get_member_key(a);
                let key_b = get_member_key(b);
                key_a.cmp(&key_b)
            });
        }
        KeySortStyle::Length => {
            anchored.sort_by(|a, b| {
                let key_a = get_member_key(a);
                let key_b = get_member_key(b);
                key_a.len().cmp(&key_b.len())
            });
            regular.sort_by(|a, b| {
                let key_a = get_member_key(a);
                let key_b = get_member_key(b);
                key_a.len().cmp(&key_b.len())
            });
        }
    }

    // Reassemble: type definitions, then anchored, then regular, then other
    let mut result = Vec::new();
    result.extend(type_defs);
    result.extend(anchored);
    result.extend(regular);
    result.extend(other);

    result
}

fn get_member_key(member: &Member) -> String {
    match member {
        Member::Pair(pair) => pair.key.clone(),
        Member::TypeDefinition(typedef) => typedef.name.clone(),
        Member::Spread(name) => name.clone(),
        Member::Import(_) => String::new(),
    }
}

/// Calculate optimal comment alignment column for a set of lines
pub fn calculate_comment_alignment(lines: &[(String, Option<String>)]) -> usize {
    // lines = vec![(content, optional_comment), ...]

    let mut max_content_len = 0;
    for (content, comment) in lines {
        if comment.is_some() {
            max_content_len = max_content_len.max(content.len());
        }
    }

    // Round up to nearest multiple of 4, add some padding
    let aligned = max_content_len.div_ceil(4) * 4 + 4;

    // Cap at reasonable maximum
    aligned.min(80)
}

/// Align a comment at the specified column
pub fn align_comment_at(content: &str, comment: &str, column: usize) -> String {
    let content_len = content.len();

    if content_len >= column {
        // Content too long, add minimal spacing
        format!("{}  {}", content, comment)
    } else {
        // Pad to alignment column
        let padding = column - content_len;
        format!("{}  {}", content, " ".repeat(padding - 2) + comment)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_comment_alignment() {
        let lines = vec![
            ("short: 1".to_string(), Some("// Comment".to_string())),
            ("much_longer_key: 2".to_string(), Some("// Another".to_string())),
        ];

        let col = calculate_comment_alignment(&lines);
        assert!(col >= "much_longer_key: 2".len());
    }

    #[test]
    fn test_align_comment() {
        let aligned = align_comment_at("key: value", "// Comment", 40);
        assert_eq!(
            aligned.len(),
            "key: value".len() + 2 + "// Comment".len() + (40 - "key: value".len() - 2)
        );
    }
}
