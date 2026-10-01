//! Lightweight Markdown helpers: front matter, wiki links, tags and titles.
//!
//! These scanners deliberately work on plain text. Code spans and fenced code
//! blocks are masked first (see [`mask_code`]) so that `[[links]]` and
//! `#tags` inside code are ignored.

use std::ops::Range;

use serde::Serialize;
use serde_json::Value;

/// Splits YAML front matter from the note body.
///
/// Returns `(Some(yaml), body)` when the note starts with a `---` line that is
/// closed by a `---` or `...` line, otherwise `(None, whole_text)`.
pub fn split_front_matter(src: &str) -> (Option<&str>, &str) {
    let s = src.strip_prefix('\u{feff}').unwrap_or(src);
    let rest = if let Some(r) = s.strip_prefix("---\n") {
        r
    } else if let Some(r) = s.strip_prefix("---\r\n") {
        r
    } else {
        return (None, s);
    };
    let mut offset = 0;
    for line in rest.split_inclusive('\n') {
        let trimmed = line.trim_end_matches(['\r', '\n']);
        if trimmed == "---" || trimmed == "..." {
            return (Some(&rest[..offset]), &rest[offset + line.len()..]);
        }
        offset += line.len();
    }
    (None, s)
}

/// Parses front matter YAML into a JSON value. Empty front matter becomes an
/// empty object.
pub fn parse_front_matter(yaml: &str) -> Result<Value, String> {
    if yaml.trim().is_empty() {
        return Ok(Value::Object(Default::default()));
    }
    serde_yaml::from_str::<Value>(yaml).map_err(|e| e.to_string())
}

/// Replaces the contents of fenced code blocks and inline code spans with
/// spaces. The result has the same byte length and line structure as the
/// input, so byte offsets and line numbers apply to the original text.
pub fn mask_code(src: &str) -> String {
    let mut out = String::with_capacity(src.len());
    let mut fence: Option<(char, usize)> = None;
    for line in src.split_inclusive('\n') {
        let trimmed = line.trim_start_matches(' ');
        let marker = if line.len() - trimmed.len() <= 3 {
            fence_marker(trimmed)
        } else {
            None
        };
        match fence {
            Some((ch, len)) => {
                if let Some((c, l)) = marker {
                    let tail = trimmed.trim_end_matches(['\r', '\n']).trim_start_matches(c);
                    if c == ch && l >= len && tail.trim().is_empty() {
                        fence = None;
                    }
                }
                out.push_str(&blank(line));
            }
            None => {
                if marker.is_some() {
                    fence = marker;
                    out.push_str(&blank(line));
                } else {
                    out.push_str(&mask_inline_code(line));
                }
            }
        }
    }
    out
}

fn fence_marker(s: &str) -> Option<(char, usize)> {
    let c = s.chars().next()?;
    if c != '`' && c != '~' {
        return None;
    }
    let n = s.chars().take_while(|&x| x == c).count();
    (n >= 3).then_some((c, n))
}

fn blank(line: &str) -> String {
    let mut out = String::with_capacity(line.len());
    for c in line.chars() {
        if c == '\n' || c == '\r' {
            out.push(c);
        } else {
            push_spaces(&mut out, c.len_utf8());
        }
    }
    out
}

fn push_spaces(out: &mut String, n: usize) {
    out.extend(std::iter::repeat_n(' ', n));
}

fn mask_inline_code(line: &str) -> String {
    let chars: Vec<char> = line.chars().collect();
    let mut out = String::with_capacity(line.len());
    let mut i = 0;
    while i < chars.len() {
        if chars[i] != '`' {
            out.push(chars[i]);
            i += 1;
            continue;
        }
        let run = backtick_run(&chars, i);
        match closing_run(&chars, i + run, run) {
            Some(end) => {
                let width = chars[i..end + run].iter().map(|c| c.len_utf8()).sum();
                push_spaces(&mut out, width);
                i = end + run;
            }
            None => {
                out.extend(std::iter::repeat_n('`', run));
                i += run;
            }
        }
    }
    out
}

fn backtick_run(chars: &[char], start: usize) -> usize {
    chars[start..].iter().take_while(|&&c| c == '`').count()
}

fn closing_run(chars: &[char], from: usize, run: usize) -> Option<usize> {
    let mut j = from;
    while j < chars.len() {
        if chars[j] == '`' {
            let r = backtick_run(chars, j);
            if r == run {
                return Some(j);
            }
            j += r;
        } else {
            j += 1;
        }
    }
    None
}

/// A `[[wiki link]]` found in a note.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WikiLink {
    /// Linked note name or path, without heading and alias.
    pub target: String,
    /// `Heading` in `[[Note#Heading]]`.
    pub heading: Option<String>,
    /// `Alias` in `[[Note|Alias]]`.
    pub alias: Option<String>,
    /// `true` for embeds: `![[file]]`.
    pub embed: bool,
}

/// Extracts unique wiki links from masked text (see [`mask_code`]).
pub fn extract_wikilinks(masked: &str) -> Vec<WikiLink> {
    let mut links: Vec<WikiLink> = Vec::new();
    for (_, link) in wikilink_spans(masked) {
        if !links.contains(&link) {
            links.push(link);
        }
    }
    links
}

/// Every wiki link in masked text with the byte range of its inner text
/// (between `[[` and `]]`).
pub fn wikilink_spans(masked: &str) -> Vec<(Range<usize>, WikiLink)> {
    let mut spans = Vec::new();
    let mut pos = 0;
    while let Some(start) = masked[pos..].find("[[") {
        let open = pos + start;
        let inner_start = open + 2;
        let Some(close) = masked[inner_start..].find("]]") else {
            break;
        };
        let inner = &masked[inner_start..inner_start + close];
        if inner.contains('\n') || inner.contains("[[") {
            pos = inner_start;
            continue;
        }
        pos = inner_start + close + 2;

        let embed = open > 0 && masked.as_bytes()[open - 1] == b'!';
        if let Some(link) = parse_wikilink(inner, embed) {
            spans.push((inner_start..inner_start + close, link));
        }
    }
    spans
}

fn parse_wikilink(inner: &str, embed: bool) -> Option<WikiLink> {
    let (link, alias) = match inner.split_once('|') {
        Some((l, a)) => (l, non_empty(a)),
        None => (inner, None),
    };
    // `[[Note\|alias]]` is how a link is written inside a Markdown table.
    let link = link.strip_suffix('\\').unwrap_or(link);
    let (target, heading) = match link.split_once('#') {
        Some((t, h)) => (t.trim(), non_empty(h)),
        None => (link.trim(), None),
    };
    if target.is_empty() && heading.is_none() {
        return None;
    }
    Some(WikiLink {
        target: target.to_string(),
        heading,
        alias,
        embed,
    })
}

/// Replaces link targets in `src`, keeping headings, aliases and code
/// untouched. `rewrite` returns the new target for a link, or `None` to keep
/// it as is.
pub fn rewrite_wikilinks(
    src: &str,
    mut rewrite: impl FnMut(&WikiLink) -> Option<String>,
) -> String {
    let masked = mask_code(src);
    let mut out = String::with_capacity(src.len());
    let mut last = 0;
    for (range, link) in wikilink_spans(&masked) {
        let Some(new_target) = rewrite(&link) else {
            continue;
        };
        let inner = &src[range.clone()];
        let mut target_end = inner.find(['#', '|']).unwrap_or(inner.len());
        if inner[..target_end].ends_with('\\') {
            target_end -= 1;
        }
        let leading = inner.len() - inner.trim_start().len();
        out.push_str(&src[last..range.start + leading]);
        out.push_str(&new_target);
        out.push_str(&inner[target_end..]);
        last = range.end;
    }
    out.push_str(&src[last..]);
    out
}

/// Like [`extract_wikilinks`], but pairs every link with the trimmed source
/// line it appears on (used for backlink previews). Links are not
/// deduplicated across lines.
pub fn extract_wikilinks_with_context(body: &str, masked: &str) -> Vec<(WikiLink, String)> {
    let mut out = Vec::new();
    for (original, masked_line) in body.lines().zip(masked.lines()) {
        if !masked_line.contains("[[") {
            continue;
        }
        let context: String = original.trim().chars().take(CONTEXT_CHARS).collect();
        for link in extract_wikilinks(masked_line) {
            out.push((link, context.clone()));
        }
    }
    out
}

const CONTEXT_CHARS: usize = 240;

fn non_empty(s: &str) -> Option<String> {
    let s = s.trim();
    (!s.is_empty()).then(|| s.to_string())
}

/// Extracts unique inline `#tags` from masked text (see [`mask_code`]).
///
/// A tag starts with `#` at the beginning of a line or after whitespace and
/// may contain letters, digits, `_`, `-` and `/` (nested tags). Pure numbers
/// such as `#123` are not tags, and `# Heading` is not a tag either.
pub fn extract_inline_tags(masked: &str) -> Vec<String> {
    let chars: Vec<char> = masked.chars().collect();
    let mut tags: Vec<String> = Vec::new();
    let mut i = 0;
    while i < chars.len() {
        let starts_tag = chars[i] == '#' && (i == 0 || chars[i - 1].is_whitespace());
        if !starts_tag {
            i += 1;
            continue;
        }
        let mut j = i + 1;
        while j < chars.len() && is_tag_char(chars[j]) {
            j += 1;
        }
        let tag: String = chars[i + 1..j].iter().collect();
        let tag = tag.trim_end_matches(['/', '-']).to_string();
        if tag.chars().any(|c| !c.is_ascii_digit()) && !tags.contains(&tag) {
            tags.push(tag);
        }
        i = j.max(i + 1);
    }
    tags
}

fn is_tag_char(c: char) -> bool {
    c.is_alphanumeric() || matches!(c, '_' | '-' | '/')
}

/// Returns the text of the first level-1 ATX heading (`# Title`).
pub fn extract_title(body: &str, masked: &str) -> Option<String> {
    for (original, masked_line) in body.lines().zip(masked.lines()) {
        if masked_line.starts_with("# ") {
            let mut title = original[2..].trim().trim_end_matches('#').trim();
            // Trailing `#tags` belong to the note, not to its title.
            while let Some((head, last)) = title.rsplit_once(char::is_whitespace) {
                if !is_tag_word(last) {
                    break;
                }
                title = head.trim_end();
            }
            if !title.is_empty() {
                return Some(title.to_string());
            }
        }
    }
    None
}

fn is_tag_word(word: &str) -> bool {
    match word.strip_prefix('#') {
        Some(tag) => {
            !tag.is_empty()
                && tag.chars().all(is_tag_char)
                && tag.chars().any(|c| !c.is_ascii_digit())
        }
        None => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn splits_front_matter() {
        let (fm, body) = split_front_matter("---\ntitle: A\n---\n# Body\n");
        assert_eq!(fm, Some("title: A\n"));
        assert_eq!(body, "# Body\n");
    }

    #[test]
    fn splits_crlf_and_bom_front_matter() {
        let (fm, body) = split_front_matter("\u{feff}---\r\nid: 1\r\n---\r\ntext");
        assert_eq!(fm, Some("id: 1\r\n"));
        assert_eq!(body, "text");
    }

    #[test]
    fn unclosed_front_matter_is_body() {
        let src = "---\ntitle: A\n# no end";
        assert_eq!(split_front_matter(src), (None, src));
        assert_eq!(
            split_front_matter("no front matter"),
            (None, "no front matter")
        );
    }

    #[test]
    fn parses_front_matter_yaml() {
        let v = parse_front_matter("title: Тест\ntags: [a, b]\ndue: 2026-10-01\n").unwrap();
        assert_eq!(v["title"], "Тест");
        assert_eq!(v["tags"][1], "b");
        assert_eq!(v["due"], "2026-10-01");
        assert!(parse_front_matter("").unwrap().is_object());
        assert!(parse_front_matter("a: [unclosed").is_err());
    }

    #[test]
    fn masks_code() {
        let src = "see `[[a]]` and [[b]]\n```\n[[c]] #tag\n```\n#real\n";
        let masked = mask_code(src);
        assert_eq!(masked.lines().count(), src.lines().count());
        let links = extract_wikilinks(&masked);
        assert_eq!(links.len(), 1);
        assert_eq!(links[0].target, "b");
        assert_eq!(extract_inline_tags(&masked), vec!["real"]);
    }

    #[test]
    fn extracts_wikilinks() {
        let src = "[[Note]] [[Note#Part|Alias]] ![[img.png]] [[Note]] [[]] [[#Local]]";
        let links = extract_wikilinks(src);
        assert_eq!(links.len(), 4);
        assert_eq!(links[1].heading.as_deref(), Some("Part"));
        assert_eq!(links[1].alias.as_deref(), Some("Alias"));
        assert!(links[2].embed);
        assert_eq!(links[3].target, "");
        assert_eq!(links[3].heading.as_deref(), Some("Local"));
    }

    #[test]
    fn masking_keeps_byte_offsets() {
        let src = "текст `код [[x]]` ще\n```\nблок\n```\n[[Ціль]]";
        let masked = mask_code(src);
        assert_eq!(masked.len(), src.len());
        let spans = wikilink_spans(&masked);
        assert_eq!(spans.len(), 1);
        assert_eq!(&src[spans[0].0.clone()], "Ціль");
    }

    #[test]
    fn rewrites_wikilinks() {
        let src = "[[Old]] [[ Old#H|alias]] `[[Old]]` [[Other]] ![[Old]] | [[Old\\|t]] |";
        let out = rewrite_wikilinks(src, |l| (l.target == "Old").then(|| "Нова".to_string()));
        assert_eq!(
            out,
            "[[Нова]] [[ Нова#H|alias]] `[[Old]]` [[Other]] ![[Нова]] | [[Нова\\|t]] |"
        );
        assert_eq!(rewrite_wikilinks(src, |_| None), src);
    }

    #[test]
    fn parses_table_escaped_pipe() {
        let links = extract_wikilinks("| [[Note\\|text]] |");
        assert_eq!(links[0].target, "Note");
        assert_eq!(links[0].alias.as_deref(), Some("text"));
    }

    #[test]
    fn ignores_broken_wikilinks() {
        assert!(extract_wikilinks("[[open\nclose]]").is_empty());
        let links = extract_wikilinks("[[a [[b]]");
        assert_eq!(links.len(), 1);
        assert_eq!(links[0].target, "b");
    }

    #[test]
    fn extracts_tags() {
        let src = "# Heading\n#проєкт text #area/work, #123 url#frag #a-\n## Sub";
        let tags = extract_inline_tags(src);
        assert_eq!(tags, vec!["проєкт", "area/work", "a"]);
    }

    #[test]
    fn extracts_title() {
        let body = "intro\n```\n# not title\n```\n# Real `code` title #\n";
        assert_eq!(
            extract_title(body, &mask_code(body)).as_deref(),
            Some("Real `code` title")
        );
        assert_eq!(extract_title("no heading", "no heading"), None);
        let body = "# Alpha #work #area/sub\n";
        assert_eq!(extract_title(body, body).as_deref(), Some("Alpha"));
    }
}
