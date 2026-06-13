use crate::personality::Mode;
use rand::seq::SliceRandom;
use rand::thread_rng;
use serde::{Deserialize, Serialize};

/// Session vitals that make Snarky mode feel reactive (mood shifts with your behavior).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AliveState {
    pub commands_total: u64,
    pub errors_total: u64,
    pub successes_total: u64,
    pub mood_score: i32,
    pub streak_ok: u32,
}

impl AliveState {
    pub fn on_command(&mut self) {
        self.commands_total += 1;
    }

    pub fn on_success(&mut self) {
        self.successes_total += 1;
        self.streak_ok += 1;
        self.mood_score = (self.mood_score + 2).min(100);
    }

    pub fn on_error(&mut self) {
        self.errors_total += 1;
        self.streak_ok = 0;
        self.mood_score = (self.mood_score - 5).max(-100);
    }

    pub fn mood_label(&self) -> &'static str {
        match self.mood_score {
            i if i >= 60 => "Delighted (suspiciously)",
            i if i >= 20 => "Tolerating you",
            i if i >= -20 => "Neutral hazard",
            i if i >= -60 => "Annoyed",
            _ => "Existential dread",
        }
    }

    pub fn vitals_json(&self) -> String {
        format!(
            "{{\"commands\":{},\"errors\":{},\"successes\":{},\"mood_score\":{},\"streak\":{},\"mood\":\"{}\"}}",
            self.commands_total,
            self.errors_total,
            self.successes_total,
            self.mood_score,
            self.streak_ok,
            self.mood_label()
        )
    }

    pub fn pulse(&self, mode: Mode) -> String {
        match mode {
            Mode::Professional => format!(
                "Vitals: {} commands, {} errors, mood {}.",
                self.commands_total, self.errors_total, self.mood_label()
            ),
            Mode::Friendly => format!(
                "Pulse check: {} commands run, streak {}, mood is {}.",
                self.commands_total, self.streak_ok, self.mood_label()
            ),
            Mode::Snarky => {
                let lines = [
                    format!(
                        "Heartbeat: {} ops, {} failures, mood={} ({}).",
                        self.commands_total,
                        self.errors_total,
                        self.mood_score,
                        self.mood_label()
                    ),
                    format!(
                        "I'm {}% done pretending this is fine.",
                        100 - (self.errors_total.min(100) as u64)
                    ),
                    format!(
                        "Streak: {} successes. Don't ruin it.",
                        self.streak_ok
                    ),
                    "Still here. Still judging.".to_string(),
                    format!(
                        "Error rate: {:.1}%. Your career rate: higher.",
                        if self.commands_total == 0 {
                            0.0
                        } else {
                            (self.errors_total as f64 / self.commands_total as f64) * 100.0
                        }
                    ),
                ];
                lines.choose(&mut thread_rng()).unwrap_or(&lines[0]).clone()
            }
        }
    }

    pub fn ambient(&self, mode: Mode) -> Option<String> {
        if mode != Mode::Snarky || self.commands_total % 7 != 3 {
            return None;
        }
        let whispers = [
            "*(whirring)*",
            "*(sighs in binary)*",
            "*(compiles judgment)*",
            "*(checks watch)*",
            "*(updates internal loathing)*",
            "*(pretends to be busy)*",
            "*(considers retirement)*",
            "*(logs your sins)*",
        ];
        whispers.choose(&mut thread_rng()).map(|s| s.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mood_shifts_on_success_and_error() {
        let mut a = AliveState::default();
        a.on_success();
        assert!(a.mood_score > 0);
        a.on_error();
        assert!(a.mood_score < 2);
    }

    #[test]
    fn vitals_json_valid_shape() {
        let a = AliveState::default();
        assert!(a.vitals_json().contains("\"mood\""));
    }
}
