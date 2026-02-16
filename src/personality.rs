use rand::seq::SliceRandom;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Professional,
    Friendly,
    Snarky,
}

impl Default for Mode {
    fn default() -> Self {
        Mode::Professional
    }
}

pub struct Personality {
    mode: Mode,
}

impl Personality {
    pub fn new(mode: Mode) -> Self {
        Self { mode }
    }
    pub fn is_professional(&self) -> bool {
        self.mode == Mode::Professional
    }
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
                let s = [
                    format!("✓ {}. Even a broken clock is right twice a day.", msg),
                    format!("✓ {}. Wow! You're fast!", msg),
                    format!("✓ {}. Miracles are now part of the roadmap.", msg),
                    format!("✓ {}. I expected chaos, but this works.", msg),
                ];
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
    pub fn typo_suggestion(&self, t: &str, e: &str) -> String {
        match self.mode {
            Mode::Professional => format!("Unknown: {}. Did you mean {}?", t, e),
            Mode::Friendly => format!("I couldn't find `{}`. Did you mean `{}`?", t, e),
            Mode::Snarky => format!("`{}` isn't a command. `{}` probably is. Try that.", t, e),
        }
    }
    pub fn empty_input(&self, c: u32) -> String {
        match self.mode {
            Mode::Professional => String::new(),
            Mode::Friendly => {
                if c == 0 {
                    "".to_string()
                } else {
                    "No command entered. Type HELP if you need examples.".to_string()
                }
            }
            Mode::Snarky => {
                if c <= 1 {
                    "".to_string()
                } else {
                    "Silence detected. The keyboard still works, right?".to_string()
                }
            }
        }
    }
    pub fn achievement_unlocked(&self) -> String {
        match self.mode {
            Mode::Professional => "Achievement unlocked!".to_string(),
            Mode::Friendly => "Achievement unlocked! Nice momentum—keep going.".to_string(),
            Mode::Snarky => "Achievement unlocked: Barely supervised competence.".to_string(),
        }
    }
    pub fn why_mean(&self) -> String {
        "I'm not mean. I'm precise.".to_string()
    }
    pub fn batch_committed(&self, operations: usize) -> String {
        match self.mode {
            Mode::Professional => format!("Batch executed: {} ops.", operations),
            Mode::Friendly => format!("Batch complete: {} operation(s) committed.", operations),
            Mode::Snarky => format!(
                "Batch done: {} operation(s). Somehow, no explosions.",
                operations
            ),
        }
    }
}
