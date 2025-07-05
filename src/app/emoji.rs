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
    Emoji {
        symbol: "😐",
        name: "neutral face",
        keywords: &["neutral", "meh", "whatever"],
    },
    Emoji {
        symbol: "😑",
        name: "expressionless face",
        keywords: &["blank", "meh", "deadpan"],
    },
    Emoji {
        symbol: "😔",
        name: "pensive face",
        keywords: &["sad", "depressed", "upset"],
    },
    Emoji {
        symbol: "😴",
        name: "sleeping face",
        keywords: &["sleep", "tired", "zzz"],
    },
    Emoji {
        symbol: "😵",
        name: "dizzy face",
        keywords: &["dizzy", "confused", "dead"],
    },
    Emoji {
        symbol: "😤",
        name: "face with steam from nose",
        keywords: &["angry", "mad", "frustrated"],
    },
    Emoji {
        symbol: "😠",
        name: "angry face",
        keywords: &["angry", "mad", "upset"],
    },
    Emoji {
        symbol: "😡",
        name: "pouting face",
        keywords: &["angry", "mad", "rage"],
    },
    Emoji {
        symbol: "🤬",
        name: "face with symbols on mouth",
        keywords: &["swear", "curse", "angry"],
    },
    Emoji {
        symbol: "😭",
        name: "loudly crying face",
        keywords: &["cry", "sad", "tears"],
    },
    Emoji {
        symbol: "😢",
        name: "crying face",
        keywords: &["cry", "sad", "tear"],
    },
    Emoji {
        symbol: "😨",
        name: "fearful face",
        keywords: &["scared", "afraid", "fear"],
    },
    Emoji {
        symbol: "😱",
        name: "face screaming in fear",
        keywords: &["scared", "shock", "surprised"],
    },
    Emoji {
        symbol: "🤯",
        name: "exploding head",
        keywords: &["mind blown", "shocked", "amazed"],
    },
    Emoji {
        symbol: "😳",
        name: "flushed face",
        keywords: &["embarrassed", "shy", "awkward"],
    },
    Emoji {
        symbol: "🥵",
        name: "hot face",
        keywords: &["hot", "sweat", "heat"],
    },
    Emoji {
        symbol: "🥶",
        name: "cold face",
        keywords: &["cold", "freezing", "blue"],
    },
    Emoji {
        symbol: "😬",
        name: "grimacing face",
        keywords: &["awkward", "oops", "cringe"],
    },
    Emoji {
        symbol: "🙄",
        name: "face with rolling eyes",
        keywords: &["eye roll", "annoyed", "whatever"],
    },
    Emoji {
        symbol: "😒",
        name: "unamused face",
        keywords: &["annoyed", "meh", "unimpressed"],
    },
    Emoji {
        symbol: "🤐",
        name: "zipper-mouth face",
        keywords: &["quiet", "secret", "zip"],
    },
    Emoji {
        symbol: "🤫",
        name: "shushing face",
        keywords: &["quiet", "shush", "secret"],
    },
    Emoji {
        symbol: "🤭",
        name: "face with hand over mouth",
        keywords: &["oops", "giggle", "secret"],
    },
    Emoji {
        symbol: "🥱",
        name: "yawning face",
        keywords: &["tired", "sleepy", "bored"],
    },
    Emoji {
        symbol: "😪",
        name: "sleepy face",
        keywords: &["tired", "sleepy", "drowsy"],
    },
    Emoji {
        symbol: "😌",
        name: "relieved face",
        keywords: &["calm", "peaceful", "relief"],
    },
    Emoji {
        symbol: "😞",
        name: "disappointed face",
        keywords: &["sad", "disappointed", "upset"],
    },
    Emoji {
        symbol: "😓",
        name: "downcast face with sweat",
        keywords: &["sad", "sweat", "tired"],
    },
    Emoji {
        symbol: "😥",
        name: "sad but relieved face",
        keywords: &["sad", "relief", "disappointed"],
    },
    Emoji {
        symbol: "😰",
        name: "anxious face with sweat",
        keywords: &["nervous", "anxious", "worried"],
    },
    Emoji {
        symbol: "😏",
        name: "smirking face",
        keywords: &["smirk", "sly", "mischievous"],
    },
    Emoji {
        symbol: "😉",
        name: "winking face",
        keywords: &["wink", "flirt", "playful"],
    },
    Emoji {
        symbol: "🤤",
        name: "drooling face",
        keywords: &["drool", "hungry", "want"],
    },
    Emoji {
        symbol: "🤢",
        name: "nauseated face",
        keywords: &["sick", "nauseous", "gross"],
    },
    Emoji {
        symbol: "🤮",
        name: "face vomiting",
        keywords: &["sick", "vomit", "puke"],
    },
    Emoji {
        symbol: "🤧",
        name: "sneezing face",
        keywords: &["sick", "sneeze", "tissue"],
    },
    Emoji {
        symbol: "😷",
        name: "face with medical mask",
        keywords: &["sick", "mask", "doctor"],
    },
    Emoji {
        symbol: "🤒",
        name: "face with thermometer",
        keywords: &["sick", "fever", "ill"],
    },
    Emoji {
        symbol: "🤕",
        name: "face with head-bandage",
        keywords: &["hurt", "injured", "bandage"],
    },
    Emoji {
        symbol: "🥳",
        name: "partying face",
        keywords: &["party", "celebration", "happy"],
    },
    Emoji {
        symbol: "🥺",
        name: "pleading face",
        keywords: &["puppy eyes", "please", "cute"],
    },
    Emoji {
        symbol: "😖",
        name: "confounded face",
        keywords: &["confused", "frustrated", "upset"],
    },
    Emoji {
        symbol: "😣",
        name: "persevering face",
        keywords: &["struggle", "effort", "determined"],
    },
    Emoji {
        symbol: "😫",
        name: "tired face",
        keywords: &["exhausted", "tired", "weary"],
    },
    Emoji {
        symbol: "😩",
        name: "weary face",
        keywords: &["tired", "exhausted", "fed up"],
    },
    Emoji {
        symbol: "🙃",
        name: "upside-down face",
        keywords: &["silly", "sarcastic", "ironic"],
    },
    Emoji {
        symbol: "👍",
        name: "thumbs up",
        keywords: &["good", "ok", "approve"],
    },
    Emoji {
        symbol: "👎",
        name: "thumbs down",
        keywords: &["bad", "no", "disapprove"],
    },
    Emoji {
        symbol: "👌",
        name: "OK hand",
        keywords: &["ok", "good", "perfect"],
    },
    Emoji {
        symbol: "✌️",
        name: "victory hand",
        keywords: &["peace", "victory", "two"],
    },
    Emoji {
        symbol: "🤞",
        name: "crossed fingers",
        keywords: &["luck", "hope", "wish"],
    },
    Emoji {
        symbol: "🤟",
        name: "love-you gesture",
        keywords: &["love", "rock", "sign"],
    },
    Emoji {
        symbol: "🤘",
        name: "sign of the horns",
        keywords: &["rock", "metal", "horns"],
    },
    Emoji {
        symbol: "👏",
        name: "clapping hands",
        keywords: &["clap", "applause", "good"],
    },
    Emoji {
        symbol: "🙌",
        name: "raising hands",
        keywords: &["celebrate", "hooray", "praise"],
    },
    Emoji {
        symbol: "👐",
        name: "open hands",
        keywords: &["open", "hug", "jazz hands"],
    },
    Emoji {
        symbol: "🤲",
        name: "palms up together",
        keywords: &["pray", "please", "open"],
    },
    Emoji {
        symbol: "🙏",
        name: "folded hands",
        keywords: &["pray", "thanks", "please"],
    },
    Emoji {
        symbol: "🤝",
        name: "handshake",
        keywords: &["deal", "agreement", "shake"],
    },
    Emoji {
        symbol: "💪",
        name: "flexed biceps",
        keywords: &["strong", "muscle", "flex"],
    },
    Emoji {
        symbol: "🦾",
        name: "mechanical arm",
        keywords: &["robot", "cyborg", "strong"],
    },
    Emoji {
        symbol: "🖤",
        name: "black heart",
        keywords: &["love", "dark", "black"],
    },
    Emoji {
        symbol: "🤍",
        name: "white heart",
        keywords: &["love", "pure", "white"],
    },
    Emoji {
        symbol: "❤️",
        name: "red heart",
        keywords: &["love", "heart", "romance"],
    },
    Emoji {
        symbol: "🧡",
        name: "orange heart",
        keywords: &["love", "heart", "orange"],
    },
    Emoji {
        symbol: "💛",
        name: "yellow heart",
        keywords: &["love", "heart", "yellow"],
    },
    Emoji {
        symbol: "💚",
        name: "green heart",
        keywords: &["love", "heart", "green"],
    },
    Emoji {
        symbol: "💙",
        name: "blue heart",
        keywords: &["love", "heart", "blue"],
    },
    Emoji {
        symbol: "💜",
        name: "purple heart",
        keywords: &["love", "heart", "purple"],
    },
    Emoji {
        symbol: "🤎",
        name: "brown heart",
        keywords: &["love", "heart", "brown"],
    },
    Emoji {
        symbol: "💔",
        name: "broken heart",
        keywords: &["heartbreak", "sad", "broken"],
    },
    Emoji {
        symbol: "💕",
        name: "two hearts",
        keywords: &["love", "hearts", "romance"],
    },
    Emoji {
        symbol: "💖",
        name: "sparkling heart",
        keywords: &["love", "sparkle", "special"],
    },
    Emoji {
        symbol: "💗",
        name: "growing heart",
        keywords: &["love", "growing", "heartbeat"],
    },
    Emoji {
        symbol: "💘",
        name: "heart with arrow",
        keywords: &["love", "cupid", "arrow"],
    },
    Emoji {
        symbol: "💝",
        name: "heart with ribbon",
        keywords: &["love", "gift", "present"],
    },
    Emoji {
        symbol: "💟",
        name: "heart decoration",
        keywords: &["love", "heart", "decoration"],
    },
    Emoji {
        symbol: "💯",
        name: "hundred points",
        keywords: &["100", "perfect", "score"],
    },
    Emoji {
        symbol: "🔥",
        name: "fire",
        keywords: &["fire", "hot", "lit"],
    },
    Emoji {
        symbol: "✨",
        name: "sparkles",
        keywords: &["sparkle", "magic", "shine"],
    },
    Emoji {
        symbol: "⭐",
        name: "star",
        keywords: &["star", "favorite", "good"],
    },
    Emoji {
        symbol: "🌟",
        name: "glowing star",
        keywords: &["star", "shine", "bright"],
    },
    Emoji {
        symbol: "💫",
        name: "dizzy",
        keywords: &["dizzy", "star", "sparkle"],
    },
    Emoji {
        symbol: "⚡",
        name: "high voltage",
        keywords: &["lightning", "electric", "fast"],
    },
    Emoji {
        symbol: "🎉",
        name: "party popper",
        keywords: &["party", "celebration", "confetti"],
    },
    Emoji {
        symbol: "🎊",
        name: "confetti ball",
        keywords: &["party", "celebration", "confetti"],
    },
    Emoji {
        symbol: "🎈",
        name: "balloon",
        keywords: &["party", "celebration", "balloon"],
    },
    Emoji {
        symbol: "🎂",
        name: "birthday cake",
        keywords: &["cake", "birthday", "celebration"],
    },
    Emoji {
        symbol: "🍰",
        name: "shortcake",
        keywords: &["cake", "dessert", "sweet"],
    },
    Emoji {
        symbol: "🎁",
        name: "gift",
        keywords: &["present", "gift", "surprise"],
    },
    Emoji {
        symbol: "🎀",
        name: "ribbon",
        keywords: &["ribbon", "bow", "gift"],
    },
    Emoji {
        symbol: "🎄",
        name: "Christmas tree",
        keywords: &["christmas", "tree", "holiday"],
    },
    Emoji {
        symbol: "🎃",
        name: "jack-o-lantern",
        keywords: &["halloween", "pumpkin", "scary"],
    },
    Emoji {
        symbol: "👻",
        name: "ghost",
        keywords: &["ghost", "spooky", "boo"],
    },
    Emoji {
        symbol: "🎭",
        name: "performing arts",
        keywords: &["theater", "drama", "mask"],
    },
    Emoji {
        symbol: "🎨",
        name: "artist palette",
        keywords: &["art", "paint", "creative"],
    },
    Emoji {
        symbol: "🎵",
        name: "musical note",
        keywords: &["music", "note", "sound"],
    },
    Emoji {
        symbol: "🎶",
        name: "musical notes",
        keywords: &["music", "notes", "song"],
    },
    Emoji {
        symbol: "🎤",
        name: "microphone",
        keywords: &["sing", "karaoke", "mic"],
    },
    Emoji {
        symbol: "🎧",
        name: "headphone",
        keywords: &["music", "headphones", "listen"],
    },
    Emoji {
        symbol: "🎮",
        name: "video game",
        keywords: &["game", "controller", "gaming"],
    },
    Emoji {
        symbol: "🎯",
        name: "direct hit",
        keywords: &["target", "bullseye", "goal"],
    },
    Emoji {
        symbol: "🏆",
        name: "trophy",
        keywords: &["win", "award", "champion"],
    },
    Emoji {
        symbol: "🥇",
        name: "1st place medal",
        keywords: &["gold", "first", "winner"],
    },
    Emoji {
        symbol: "🥈",
        name: "2nd place medal",
        keywords: &["silver", "second", "medal"],
    },
    Emoji {
        symbol: "🥉",
        name: "3rd place medal",
        keywords: &["bronze", "third", "medal"],
    },
    Emoji {
        symbol: "🎖️",
        name: "military medal",
        keywords: &["medal", "military", "honor"],
    },
    Emoji {
        symbol: "🏅",
        name: "sports medal",
        keywords: &["medal", "sports", "achievement"],
    },
    Emoji {
        symbol: "⚽",
        name: "soccer ball",
        keywords: &["soccer", "football", "ball"],
    },
    Emoji {
        symbol: "🏀",
        name: "basketball",
        keywords: &["basketball", "ball", "sport"],
    },
    Emoji {
        symbol: "🏈",
        name: "american football",
        keywords: &["football", "american", "sport"],
    },
    Emoji {
        symbol: "⚾",
        name: "baseball",
        keywords: &["baseball", "ball", "sport"],
    },
    Emoji {
        symbol: "🎾",
        name: "tennis",
        keywords: &["tennis", "ball", "sport"],
    },
    Emoji {
        symbol: "🏓",
        name: "ping pong",
        keywords: &["ping pong", "table tennis", "paddle"],
    },
    Emoji {
        symbol: "🏐",
        name: "volleyball",
        keywords: &["volleyball", "ball", "sport"],
    },
    Emoji {
        symbol: "🏑",
        name: "field hockey",
        keywords: &["hockey", "field", "stick"],
    },
    Emoji {
        symbol: "🏒",
        name: "ice hockey",
        keywords: &["hockey", "ice", "puck"],
    },
    Emoji {
        symbol: "🥍",
        name: "lacrosse",
        keywords: &["lacrosse", "stick", "sport"],
    },
    Emoji {
        symbol: "🏸",
        name: "badminton",
        keywords: &["badminton", "shuttlecock", "racquet"],
    },
    Emoji {
        symbol: "🏹",
        name: "bow and arrow",
        keywords: &["archery", "bow", "arrow"],
    },
    Emoji {
        symbol: "🛹",
        name: "skateboard",
        keywords: &["skateboard", "skate", "board"],
    },
    Emoji {
        symbol: "🛼",
        name: "roller skate",
        keywords: &["roller", "skate", "wheels"],
    },
    Emoji {
        symbol: "🥊",
        name: "boxing glove",
        keywords: &["boxing", "glove", "fight"],
    },
    Emoji {
        symbol: "🥋",
        name: "martial arts uniform",
        keywords: &["martial arts", "karate", "uniform"],
    },
    Emoji {
        symbol: "🚗",
        name: "car",
        keywords: &["car", "automobile", "vehicle"],
    },
    Emoji {
        symbol: "🚕",
        name: "taxi",
        keywords: &["taxi", "cab", "car"],
    },
    Emoji {
        symbol: "🚙",
        name: "sport utility vehicle",
        keywords: &["suv", "car", "vehicle"],
    },
    Emoji {
        symbol: "🚌",
        name: "bus",
        keywords: &["bus", "vehicle", "transport"],
    },
    Emoji {
        symbol: "🚓",
        name: "police car",
        keywords: &["police", "cop", "car"],
    },
    Emoji {
        symbol: "🚑",
        name: "ambulance",
        keywords: &["ambulance", "emergency", "medical"],
    },
    Emoji {
        symbol: "🚒",
        name: "fire engine",
        keywords: &["fire truck", "emergency", "fire"],
    },
    Emoji {
        symbol: "🚐",
        name: "minibus",
        keywords: &["van", "minibus", "vehicle"],
    },
    Emoji {
        symbol: "🚚",
        name: "delivery truck",
        keywords: &["truck", "delivery", "vehicle"],
    },
    Emoji {
        symbol: "🚛",
        name: "articulated lorry",
        keywords: &["truck", "semi", "lorry"],
    },
    Emoji {
        symbol: "🚜",
        name: "tractor",
        keywords: &["tractor", "farm", "agriculture"],
    },
    Emoji {
        symbol: "🏎️",
        name: "racing car",
        keywords: &["race car", "fast", "speed"],
    },
    Emoji {
        symbol: "🏍️",
        name: "motorcycle",
        keywords: &["motorcycle", "bike", "motor"],
    },
    Emoji {
        symbol: "🚲",
        name: "bicycle",
        keywords: &["bike", "bicycle", "cycle"],
    },
    Emoji {
        symbol: "🛴",
        name: "kick scooter",
        keywords: &["scooter", "kick", "ride"],
    },
    Emoji {
        symbol: "✈️",
        name: "airplane",
        keywords: &["plane", "airplane", "flight"],
    },
    Emoji {
        symbol: "🚁",
        name: "helicopter",
        keywords: &["helicopter", "chopper", "aircraft"],
    },
    Emoji {
        symbol: "🚀",
        name: "rocket",
        keywords: &["rocket", "space", "launch"],
    },
    Emoji {
        symbol: "🛸",
        name: "flying saucer",
        keywords: &["ufo", "alien", "saucer"],
    },
    Emoji {
        symbol: "🚂",
        name: "locomotive",
        keywords: &["train", "locomotive", "steam"],
    },
    Emoji {
        symbol: "🚃",
        name: "railway car",
        keywords: &["train", "car", "railway"],
    },
    Emoji {
        symbol: "🚄",
        name: "high-speed train",
        keywords: &["train", "fast", "bullet"],
    },
    Emoji {
        symbol: "🚅",
        name: "bullet train",
        keywords: &["train", "bullet", "fast"],
    },
    Emoji {
        symbol: "🚆",
        name: "train",
        keywords: &["train", "railway", "transport"],
    },
    Emoji {
        symbol: "🚇",
        name: "metro",
        keywords: &["subway", "metro", "underground"],
    },
    Emoji {
        symbol: "🚈",
        name: "light rail",
        keywords: &["light rail", "tram", "rail"],
    },
    Emoji {
        symbol: "🚉",
        name: "station",
        keywords: &["station", "train", "stop"],
    },
    Emoji {
        symbol: "🚊",
        name: "tram",
        keywords: &["tram", "trolley", "streetcar"],
    },
    Emoji {
        symbol: "🚝",
        name: "monorail",
        keywords: &["monorail", "train", "rail"],
    },
    Emoji {
        symbol: "🚞",
        name: "mountain railway",
        keywords: &["mountain", "railway", "train"],
    },
    Emoji {
        symbol: "🚋",
        name: "tram car",
        keywords: &["tram", "streetcar", "trolley"],
    },
    Emoji {
        symbol: "🚌",
        name: "bus",
        keywords: &["bus", "public", "transport"],
    },
    Emoji {
        symbol: "🚍",
        name: "oncoming bus",
        keywords: &["bus", "oncoming", "front"],
    },
    Emoji {
        symbol: "🚎",
        name: "trolleybus",
        keywords: &["trolley", "bus", "electric"],
    },
    Emoji {
        symbol: "🚐",
        name: "minibus",
        keywords: &["minibus", "van", "small"],
    },
    Emoji {
        symbol: "🚑",
        name: "ambulance",
        keywords: &["ambulance", "medical", "emergency"],
    },
    Emoji {
        symbol: "🚒",
        name: "fire engine",
        keywords: &["fire", "engine", "truck"],
    },
    Emoji {
        symbol: "🚓",
        name: "police car",
        keywords: &["police", "car", "cop"],
    },
    Emoji {
        symbol: "🚔",
        name: "oncoming police car",
        keywords: &["police", "car", "oncoming"],
    },
    Emoji {
        symbol: "🚕",
        name: "taxi",
        keywords: &["taxi", "cab", "yellow"],
    },
    Emoji {
        symbol: "🚖",
        name: "oncoming taxi",
        keywords: &["taxi", "oncoming", "cab"],
    },
    Emoji {
        symbol: "🚗",
        name: "car",
        keywords: &["car", "red", "automobile"],
    },
    Emoji {
        symbol: "🚘",
        name: "oncoming automobile",
        keywords: &["car", "oncoming", "blue"],
    },
    Emoji {
        symbol: "🚙",
        name: "sport utility vehicle",
        keywords: &["suv", "blue", "recreational"],
    },
    Emoji {
        symbol: "🚚",
        name: "delivery truck",
        keywords: &["truck", "delivery", "green"],
    },
    Emoji {
        symbol: "🚛",
        name: "articulated lorry",
        keywords: &["truck", "semi", "big"],
    },
    Emoji {
        symbol: "🚜",
        name: "tractor",
        keywords: &["tractor", "farm", "green"],
    },
    Emoji {
        symbol: "🛻",
        name: "pickup truck",
        keywords: &["pickup", "truck", "vehicle"],
    },
    Emoji {
        symbol: "🏎️",
        name: "racing car",
        keywords: &["race", "car", "fast"],
    },
    Emoji {
        symbol: "🏍️",
        name: "motorcycle",
        keywords: &["motorcycle", "racing", "bike"],
    },
    Emoji {
        symbol: "🛵",
        name: "motor scooter",
        keywords: &["scooter", "vespa", "motor"],
    },
    Emoji {
        symbol: "🚲",
        name: "bicycle",
        keywords: &["bicycle", "bike", "cycle"],
    },
    Emoji {
        symbol: "🛴",
        name: "kick scooter",
        keywords: &["scooter", "kick", "child"],
    },
    Emoji {
        symbol: "⛽",
        name: "fuel pump",
        keywords: &["gas", "fuel", "pump"],
    },
    Emoji {
        symbol: "🚨",
        name: "police car light",
        keywords: &["siren", "emergency", "police"],
    },
    Emoji {
        symbol: "🚥",
        name: "horizontal traffic light",
        keywords: &["traffic", "light", "horizontal"],
    },
    Emoji {
        symbol: "🚦",
        name: "vertical traffic light",
        keywords: &["traffic", "light", "vertical"],
    },
    Emoji {
        symbol: "🚧",
        name: "construction",
        keywords: &["construction", "work", "warning"],
    },
    Emoji {
        symbol: "⚓",
        name: "anchor",
        keywords: &["anchor", "ship", "boat"],
    },
    Emoji {
        symbol: "⛵",
        name: "sailboat",
        keywords: &["sailboat", "boat", "sailing"],
    },
    Emoji {
        symbol: "🚤",
        name: "speedboat",
        keywords: &["speedboat", "boat", "fast"],
    },
    Emoji {
        symbol: "🛥️",
        name: "motor boat",
        keywords: &["boat", "motor", "yacht"],
    },
    Emoji {
        symbol: "🛳️",
        name: "passenger ship",
        keywords: &["ship", "cruise", "passenger"],
    },
    Emoji {
        symbol: "⛴️",
        name: "ferry",
        keywords: &["ferry", "boat", "transport"],
    },
    Emoji {
        symbol: "🚢",
        name: "ship",
        keywords: &["ship", "boat", "cruise"],
    },
    Emoji {
        symbol: "🍎",
        name: "red apple",
        keywords: &["apple", "fruit", "red"],
    },
    Emoji {
        symbol: "🍊",
        name: "tangerine",
        keywords: &["orange", "fruit", "citrus"],
    },
    Emoji {
        symbol: "🍋",
        name: "lemon",
        keywords: &["lemon", "fruit", "sour"],
    },
    Emoji {
        symbol: "🍌",
        name: "banana",
        keywords: &["banana", "fruit", "yellow"],
    },
    Emoji {
        symbol: "🍉",
        name: "watermelon",
        keywords: &["watermelon", "fruit", "summer"],
    },
    Emoji {
        symbol: "🍇",
        name: "grapes",
        keywords: &["grapes", "fruit", "wine"],
    },
    Emoji {
        symbol: "🍓",
        name: "strawberry",
        keywords: &["strawberry", "fruit", "berry"],
    },
    Emoji {
        symbol: "🫐",
        name: "blueberries",
        keywords: &["blueberry", "fruit", "berry"],
    },
    Emoji {
        symbol: "🍈",
        name: "melon",
        keywords: &["melon", "fruit", "cantaloupe"],
    },
    Emoji {
        symbol: "🍒",
        name: "cherries",
        keywords: &["cherry", "fruit", "red"],
    },
    Emoji {
        symbol: "🍑",
        name: "peach",
        keywords: &["peach", "fruit", "butt"],
    },
    Emoji {
        symbol: "🥭",
        name: "mango",
        keywords: &["mango", "fruit", "tropical"],
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
