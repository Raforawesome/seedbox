//! Currently the scope of this module only covers modifying the PropertyGroup part
//! of every `.csproj` file for a SMAPI project.

use std::path::Path;

use itertools::Itertools;

#[derive(Debug, Clone)]
pub enum XmlToken<'a> {
    TagStart(bool), // bool represents if the tag is a closing tag
    TagEnd,
    SelfClose,
    Close,
    TagContent(&'a str),
}

impl<'a> XmlToken<'a> {
    pub fn is_tag_start(&self) -> bool {
        matches!(self, XmlToken::TagStart(_))
    }

    pub fn flip_tag_start(&mut self) {
        if let XmlToken::TagStart(b) = self {
            *self = Self::TagStart(!*b)
        } else {
            unreachable!("Used flip tag on a token that isn't a tag.")
        }
    }
}

pub fn insert_game_path_buf(buffer: &mut String, path: &Path) {
    let insert_pos: usize = buffer
        .lines()
        .take_while_inclusive(|s| !s.contains("PropertyGroup"))
        .map(|s| s.len() + 1) // + 1 to account for \n
        .sum::<usize>();
    let prop_string: String = format!("\t<GamePath>{}</GamePath>\n", path.display());

    let bytes: &mut Vec<u8> = unsafe { buffer.as_mut_vec() }; // unsafe to avoid copying string
    bytes.splice(insert_pos..insert_pos, prop_string.bytes());
}

#[cfg(target_os = "none")]
pub fn tokenize_csproj(raw: &str) -> Vec<XmlToken> {
    let mut tokens: Vec<XmlToken> = Vec::new();
    let mut content_start: usize = 0;
    let mut read_mode = 0;

    for (i, c) in raw.chars().enumerate() {
        match c {
            ' ' | '\n' | '\t' => continue,
            '<' => tokens.push(XmlToken::TagStart(false)),
            '/' => {
                if let Some(XmlToken::TagStart(false)) = tokens.last() {
                    let tag_token = tokens.last_mut().unwrap();
                    tag_token.flip_tag_start();
                } else {
                    tokens.push(XmlToken::Close);
                }
            }
            '>' => {
                if let Some(XmlToken::Close) = tokens.last() {
                    let _ = tokens.pop();
                    tokens.push(XmlToken::SelfClose);
                } else {
                    tokens.push(XmlToken::TagEnd);
                }
                content_start = i + 1;
            }
            _ => {
                warn!(token = %c, "invalid xml character");
                continue;
            }
        };
    }

    tokens
}
