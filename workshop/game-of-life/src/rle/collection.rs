use super::{ParseError, Pattern};
use include_dir::{include_dir, Dir};

static PATTERNS_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/patterns");

pub struct PatternCollection {
    patterns: Vec<Pattern>,
}

impl PatternCollection {
    pub fn load() -> Result<Self, ParseError> {
        let mut patterns: Vec<Pattern> = Vec::new();

        for file in PATTERNS_DIR.files() {
            if file.path().extension().map_or(false, |ext| ext == "rle") {
                let content: &str = file
                    .contents_utf8()
                    .ok_or_else(|| ParseError::InvalidPattern("Invalid UTF-8".to_string()))?;
                let pattern: Pattern = Pattern::parse_rle(content)?;
                patterns.push(pattern);
            }
        }

        Ok(Self { patterns })
    }
}
