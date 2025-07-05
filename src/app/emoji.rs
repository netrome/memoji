#[derive(Clone, Debug)]
pub struct Emoji {
    pub symbol: &'static str,
    pub name: &'static str,
    pub keywords: &'static [&'static str],
}

const EMOJIS: &[Emoji] = &[
    Emoji {
        symbol: "😀",
        name: "grinning face",
        keywords: &["happy", "smile", "grin"],
    },
    Emoji {
        symbol: "😃",
        name: "grinning face with big eyes",
        keywords: &["happy", "smile", "joy"],
    },
    Emoji {
        symbol: "😄",
        name: "grinning face with smiling eyes",
        keywords: &["happy", "smile", "joy"],
    },
    Emoji {
        symbol: "😁",
        name: "beaming face with smiling eyes",
        keywords: &["happy", "smile", "grin"],
    },
    Emoji {
        symbol: "😆",
        name: "grinning squinting face",
        keywords: &["happy", "laugh", "funny"],
    },
    Emoji {
        symbol: "😅",
        name: "grinning face with sweat",
        keywords: &["happy", "sweat", "relief"],
    },
    Emoji {
        symbol: "😂",
        name: "face with tears of joy",
        keywords: &["laugh", "funny", "joy"],
    },
    Emoji {
        symbol: "🤣",
        name: "rolling on the floor laughing",
        keywords: &["laugh", "funny", "rofl"],
    },
    Emoji {
        symbol: "😊",
        name: "smiling face with smiling eyes",
        keywords: &["happy", "smile", "blush"],
    },
    Emoji {
        symbol: "😇",
        name: "smiling face with halo",
        keywords: &["angel", "innocent", "halo"],
    },
    Emoji {
        symbol: "😋",
        name: "face savoring food",
        keywords: &["yum", "delicious", "food"],
    },
    Emoji {
        symbol: "😎",
        name: "smiling face with sunglasses",
        keywords: &["cool", "sunglasses", "awesome"],
    },
    Emoji {
        symbol: "😍",
        name: "smiling face with heart-eyes",
        keywords: &["love", "heart", "crush"],
    },
    Emoji {
        symbol: "😘",
        name: "face blowing a kiss",
        keywords: &["kiss", "love", "flirt"],
    },
    Emoji {
        symbol: "😗",
        name: "kissing face",
        keywords: &["kiss", "love", "smooch"],
    },
    Emoji {
        symbol: "😙",
        name: "kissing face with smiling eyes",
        keywords: &["kiss", "love", "happy"],
    },
    Emoji {
        symbol: "😚",
        name: "kissing face with closed eyes",
        keywords: &["kiss", "love", "peace"],
    },
    Emoji {
        symbol: "🙂",
        name: "slightly smiling face",
        keywords: &["smile", "happy", "subtle"],
    },
    Emoji {
        symbol: "🤗",
        name: "hugging face",
        keywords: &["hug", "embrace", "comfort"],
    },
    Emoji {
        symbol: "🤔",
        name: "thinking face",
        keywords: &["think", "hmm", "wonder"],
    },
];

pub fn search(query: &str) -> Vec<Emoji> {
    if query.is_empty() {
        return EMOJIS.iter().cloned().collect();
    }

    let query_lower = query.to_lowercase();
    let mut results: Vec<Emoji> = EMOJIS
        .iter()
        .filter(|emoji| {
            emoji.name.contains(&query_lower)
                || emoji
                    .keywords
                    .iter()
                    .any(|keyword| keyword.contains(&query_lower))
        })
        .cloned()
        .collect();

    // Limit results to prevent UI overflow
    results.truncate(10);
    results
}
