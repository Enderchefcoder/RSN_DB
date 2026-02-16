use rand::seq::SliceRandom;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Professional, Friendly, Snarky,
}

impl Default for Mode { fn default() -> Self { Mode::Professional } }

pub struct Personality { mode: Mode }

impl Personality {
    pub fn new(mode: Mode) -> Self { Self { mode } }
    pub fn is_professional(&self) -> bool { self.mode == Mode::Professional }
    pub fn welcome(&self) -> String {
        match self.mode {
            Mode::Professional => "RSN DB Ready.".to_string(),
            Mode::Friendly => "Welcome back!".to_string(),
            Mode::Snarky => "Oh, it's you again.".to_string(),
        }
    }
    pub fn success(&self, msg: &str) -> String {
        match self.mode {
            Mode::Professional => format!("✓ {}", msg),
            Mode::Friendly => format!("✓ Done! {}.", msg),
            Mode::Snarky => {
                let s = [format!("✓ {}. Even a broken clock is right twice a day.", msg), format!("✓ {}. Wow! You're fast!", msg)];
                s.choose(&mut rand::thread_rng()).unwrap().clone()
            }
        }
    }
    pub fn error(&self, err: &str) -> String {
        match self.mode {
            Mode::Professional => format!("✗ {}", err),
            Mode::Friendly => format!("✗ Oops! {}.", err),
            Mode::Snarky => format!("✗ {}. Spellcheck is free.", err),
        }
    }
    pub fn typo_suggestion(&self, t: &str, e: &str) -> String { format!("Unknown: {}. Did you mean {}?", t, e) }
    pub fn empty_input(&self, _c: u32) -> String { "".to_string() }
    pub fn achievement_unlocked(&self) -> String { "Achievement unlocked!".to_string() }
    pub fn why_mean(&self) -> String { "I'm not mean. I'm precise.".to_string() }
}
