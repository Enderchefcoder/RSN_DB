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
            Mode::Snarky => {
                "Oh, it's you again. Let's try not to break causality today.".to_string()
            }
        }
    }

    pub fn success(&self, msg: &str) -> String {
        match self.mode {
            Mode::Professional => format!("✓ {}", msg),
            Mode::Friendly => format!("✓ Done! {}.", msg),
            Mode::Snarky => {
                let options = [
                    format!("✓ {}. Statistically improbable, but welcome.", msg),
                    format!("✓ {}. You and accuracy shook hands for once.", msg),
                    format!("✓ {}. No alarms, no smoke, no existential dread.", msg),
                    format!("✓ {}. I had prepared a lecture and now it's wasted.", msg),
                ];
                options
                    .choose(&mut rand::thread_rng())
                    .map(ToString::to_string)
                    .unwrap_or_else(|| format!("✓ {}", msg))
            }
        }
    }

    pub fn error(&self, err: &str) -> String {
        match self.mode {
            Mode::Professional => format!("✗ {}", err),
            Mode::Friendly => format!("✗ Oops! {}.", err),
            Mode::Snarky => {
                let thing = self.reference_for(err);
                format!(
                    "✗ {}. This went down like a {} on a steep staircase.",
                    err, thing
                )
            }
        }
    }

    pub fn typo_suggestion(&self, typo: &str, expected: &str) -> String {
        match self.mode {
            Mode::Professional => format!("Unknown: {}. Did you mean {}?", typo, expected),
            Mode::Friendly => format!("I couldn't find `{}`. Did you mean `{}`?", typo, expected),
            Mode::Snarky => {
                let thing = self.reference_for(typo);
                let options = [
                    format!("`{}` isn't a command. `{}` probably is. Even a {} could've guessed that.", typo, expected, thing),
                    format!("Unknown command `{}`. Try `{}` before I replace your keyboard with a {}.", typo, expected, thing),
                    format!("`{}` is creative, not valid. Use `{}`. We're running a database, not a {} museum.", typo, expected, thing),
                ];
                options
                    .choose(&mut rand::thread_rng())
                    .map(ToString::to_string)
                    .unwrap_or_else(|| format!("Unknown: {}. Did you mean {}?", typo, expected))
            }
        }
    }

    pub fn empty_input(&self, count: u32) -> String {
        match self.mode {
            Mode::Professional => String::new(),
            Mode::Friendly => {
                if count <= 1 {
                    String::new()
                } else {
                    "No command entered. Type HELP if you need examples.".to_string()
                }
            }
            Mode::Snarky => match count {
                0 | 1 => String::new(),
                2 => "  ...hello?".to_string(),
                3 => "  Okay, I get it. You're having a moment.
  Take your time. I'm just sitting here, managing your
  data, keeping your indexes fresh, protecting your
  referential integrity from your questionable decisions.
  
  No rush. Really."
                    .to_string(),
                _ => {
                    let thing = self.reference_for(&format!("empty-input-{}", count));
                    format!(
                        "  Still no command. At this point I could have indexed a {} and a {}.",
                        thing,
                        self.reference_for("still-empty")
                    )
                }
            },
        }
    }

    pub fn achievement_unlocked(&self) -> String {
        match self.mode {
            Mode::Professional => "Achievement unlocked!".to_string(),
            Mode::Friendly => "Achievement unlocked! Nice momentum—keep going.".to_string(),
            Mode::Snarky => "Achievement unlocked: Barely supervised competence.".to_string(),
        }
    }

    pub fn competent_streak(&self) -> String {
        match self.mode {
            Mode::Professional => "[SYSTEM]: Achievement unlocked: Actually Competent".to_string(),
            Mode::Friendly => "[SYSTEM]: Achievement unlocked: Actually Competent
[SYSTEM]: That was a clean streak. Nicely done."
                .to_string(),
            Mode::Snarky => "  Wait. Hold on.
  
  Did you just... complete 50 commands in a row without
  a single typo? No syntax errors? Not even a misplaced
  comma?
  
  I... I don't know how to process this.
  Are you feeling okay? Should I call someone?
  
  [SYSTEM]: Achievement unlocked: \"Actually Competent\"
  [SYSTEM]: Updating user classification from
            \"Hopeless\" to \"Occasionally Capable\""
                .to_string(),
        }
    }

    pub fn why_mean(&self) -> String {
        match self.mode {
            Mode::Professional => "I'm not mean. I'm precise.".to_string(),
            Mode::Friendly => {
                "I'm not trying to be mean—just direct so your queries stay correct.".to_string()
            }
            Mode::Snarky => "  Oh, we're doing feelings now? Fine.
  
  I'm not mean. I'm *precise*. There's a difference.
  You make mistakes. I point them out. That's literally
  my job. Don't blame me for your fragile ego.
  
  Besides, if I didn't roast you, who would? Your code
  review? Please. I've seen your commit messages.
  
  Now, did you have an actual DATABASE query, or are we
  done with therapy hour?"
                .to_string(),
        }
    }

    pub fn help_optimize(&self) -> String {
        let thing = self.reference_for("optimize-help");
        match self.mode {
            Mode::Professional => "OPTIMIZE <table>
Rebuild indexes and compact storage for better read performance."
                .to_string(),
            Mode::Friendly => "OPTIMIZE <table>
Rebuilds indexes and compacts storage for faster queries.

Example:
  OPTIMIZE users"
                .to_string(),
            Mode::Snarky => format!(
                "╭──────────────────────────────────────────────────────────╮
│  OPTIMIZE <table>                                        │
│                                                          │
│  Rebuilds indexes and compacts storage for faster       │
│  queries. You know, the thing you should've done        │
│  before complaining about performance.                  │
│                                                          │
│  Example:                                               │
│    OPTIMIZE users                                       │
│                                                          │
│  Note: This takes time. Maybe use it to reflect on      │
│  your life choices. Or polish your {}.                │
╰──────────────────────────────────────────────────────────╯",
                thing
            ),
        }
    }

    pub fn explain_typo(&self) -> String {
        let thing = self.reference_for("explin");
        match self.mode {
            Mode::Professional => "Unknown command: 'EXPLIN'. Did you mean: EXPLAIN ?".to_string(),
            Mode::Friendly => "Unknown command: 'EXPLIN'
Did you mean: EXPLAIN ?"
                .to_string(),
            Mode::Snarky => format!(
                "✗ Unknown command: 'EXPLIN'

  Did you mean: EXPLAIN ?
  
  You know what? I'm not even mad. I'm impressed.
  You managed to misspell a word that's literally on the
  screen in the HELP menu. That takes talent.
  
  Go ahead. Try again. I'll wait. I have all day.
  It's not like I have anything better to do than watch
  you struggle with a six-letter word while holding a {}.",
                thing
            ),
        }
    }

    pub fn type_mismatch(&self, field: &str, expected: &str, got: &str) -> String {
        let thing = self.reference_for(field);
        match self.mode {
            Mode::Professional => format!(
                "Type mismatch on field '{}': expected {} but got {}.",
                field, expected, got
            ),
            Mode::Friendly => format!(
                "Type mismatch on field '{}': expected {} but got {}. Please adjust and retry.",
                field, expected, got
            ),
            Mode::Snarky => format!(
                "✗ Type mismatch on field '{}':
  Expected: {}
  Got:      {}

  This is database typing, not interpretive dance.
  A {} won't parse itself.",
                field, expected, got, thing
            ),
        }
    }

    pub fn destructive_confirmation_failed(&self, expected: &str, got: &str) -> String {
        let thing = self.reference_for(expected);
        match self.mode {
            Mode::Professional => format!(
                "Confirmation failed. You typed '{}' but expected '{}'.",
                got, expected
            ),
            Mode::Friendly => format!(
                "Confirmation failed. You typed '{}' but expected '{}'.",
                got, expected
            ),
            Mode::Snarky => format!(
                "✗ Confirmation failed. You typed '{}' but expected '{}'.
  
  You know what? I changed my mind. You clearly need this
  table more than you realize. Request denied.
  
  Come back when you can spell your own table names.
  In the meantime, guard this {} with your life.",
                got, expected, thing
            ),
        }
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

    fn reference_for(&self, seed: &str) -> &'static str {
        let mut hash: usize = 0;
        for byte in seed.as_bytes() {
            hash = hash.wrapping_mul(31).wrapping_add(*byte as usize);
        }
        REAL_THINGS[hash % REAL_THINGS.len()]
    }
}

pub const STARTUP_BANNER: &str = r#"
╔══════════════════════════════════════════════════════════════╗
║                                                              ║
║     ██████╗ ███████╗███╗   ██╗    ██████╗ ██████╗           ║
║     ██╔══██╗██╔════╝████╗  ██║    ██╔══██╗██╔══██╗          ║
║     ██████╔╝███████╗██╔██╗ ██║    ██║  ██║██████╔╝          ║
║     ██╔══██╗╚════██║██║╚██╗██║    ██║  ██║██╔══██╗          ║
║     ██║  ██║███████║██║ ╚████║    ██████╔╝██████╔╝          ║
║     ╚═╝  ╚═╝╚══════╝╚═╝  ╚═══╝    ╚═════╝ ╚═════╝           ║
║                                                              ║
╚══════════════════════════════════════════════════════════════╝
"#;

pub const REAL_THINGS: [&str; 1000] = [
    "red apple",
    "red banana",
    "red carrot",
    "red tomato",
    "red potato",
    "red onion",
    "red garlic",
    "red pepper",
    "red cucumber",
    "red pumpkin",
    "red hammer",
    "red screwdriver",
    "red wrench",
    "red drill",
    "red saw",
    "red ladder",
    "red shovel",
    "red rake",
    "red bucket",
    "red wheelbarrow",
    "blue apple",
    "blue banana",
    "blue carrot",
    "blue tomato",
    "blue potato",
    "blue onion",
    "blue garlic",
    "blue pepper",
    "blue cucumber",
    "blue pumpkin",
    "blue hammer",
    "blue screwdriver",
    "blue wrench",
    "blue drill",
    "blue saw",
    "blue ladder",
    "blue shovel",
    "blue rake",
    "blue bucket",
    "blue wheelbarrow",
    "green apple",
    "green banana",
    "green carrot",
    "green tomato",
    "green potato",
    "green onion",
    "green garlic",
    "green pepper",
    "green cucumber",
    "green pumpkin",
    "green hammer",
    "green screwdriver",
    "green wrench",
    "green drill",
    "green saw",
    "green ladder",
    "green shovel",
    "green rake",
    "green bucket",
    "green wheelbarrow",
    "yellow apple",
    "yellow banana",
    "yellow carrot",
    "yellow tomato",
    "yellow potato",
    "yellow onion",
    "yellow garlic",
    "yellow pepper",
    "yellow cucumber",
    "yellow pumpkin",
    "yellow hammer",
    "yellow screwdriver",
    "yellow wrench",
    "yellow drill",
    "yellow saw",
    "yellow ladder",
    "yellow shovel",
    "yellow rake",
    "yellow bucket",
    "yellow wheelbarrow",
    "purple apple",
    "purple banana",
    "purple carrot",
    "purple tomato",
    "purple potato",
    "purple onion",
    "purple garlic",
    "purple pepper",
    "purple cucumber",
    "purple pumpkin",
    "purple hammer",
    "purple screwdriver",
    "purple wrench",
    "purple drill",
    "purple saw",
    "purple ladder",
    "purple shovel",
    "purple rake",
    "purple bucket",
    "purple wheelbarrow",
    "orange apple",
    "orange banana",
    "orange carrot",
    "orange tomato",
    "orange potato",
    "orange onion",
    "orange garlic",
    "orange pepper",
    "orange cucumber",
    "orange pumpkin",
    "orange hammer",
    "orange screwdriver",
    "orange wrench",
    "orange drill",
    "orange saw",
    "orange ladder",
    "orange shovel",
    "orange rake",
    "orange bucket",
    "orange wheelbarrow",
    "black apple",
    "black banana",
    "black carrot",
    "black tomato",
    "black potato",
    "black onion",
    "black garlic",
    "black pepper",
    "black cucumber",
    "black pumpkin",
    "black hammer",
    "black screwdriver",
    "black wrench",
    "black drill",
    "black saw",
    "black ladder",
    "black shovel",
    "black rake",
    "black bucket",
    "black wheelbarrow",
    "white apple",
    "white banana",
    "white carrot",
    "white tomato",
    "white potato",
    "white onion",
    "white garlic",
    "white pepper",
    "white cucumber",
    "white pumpkin",
    "white hammer",
    "white screwdriver",
    "white wrench",
    "white drill",
    "white saw",
    "white ladder",
    "white shovel",
    "white rake",
    "white bucket",
    "white wheelbarrow",
    "silver apple",
    "silver banana",
    "silver carrot",
    "silver tomato",
    "silver potato",
    "silver onion",
    "silver garlic",
    "silver pepper",
    "silver cucumber",
    "silver pumpkin",
    "silver hammer",
    "silver screwdriver",
    "silver wrench",
    "silver drill",
    "silver saw",
    "silver ladder",
    "silver shovel",
    "silver rake",
    "silver bucket",
    "silver wheelbarrow",
    "golden apple",
    "golden banana",
    "golden carrot",
    "golden tomato",
    "golden potato",
    "golden onion",
    "golden garlic",
    "golden pepper",
    "golden cucumber",
    "golden pumpkin",
    "golden hammer",
    "golden screwdriver",
    "golden wrench",
    "golden drill",
    "golden saw",
    "golden ladder",
    "golden shovel",
    "golden rake",
    "golden bucket",
    "golden wheelbarrow",
    "wooden apple",
    "wooden banana",
    "wooden carrot",
    "wooden tomato",
    "wooden potato",
    "wooden onion",
    "wooden garlic",
    "wooden pepper",
    "wooden cucumber",
    "wooden pumpkin",
    "wooden hammer",
    "wooden screwdriver",
    "wooden wrench",
    "wooden drill",
    "wooden saw",
    "wooden ladder",
    "wooden shovel",
    "wooden rake",
    "wooden bucket",
    "wooden wheelbarrow",
    "metal apple",
    "metal banana",
    "metal carrot",
    "metal tomato",
    "metal potato",
    "metal onion",
    "metal garlic",
    "metal pepper",
    "metal cucumber",
    "metal pumpkin",
    "metal hammer",
    "metal screwdriver",
    "metal wrench",
    "metal drill",
    "metal saw",
    "metal ladder",
    "metal shovel",
    "metal rake",
    "metal bucket",
    "metal wheelbarrow",
    "ceramic apple",
    "ceramic banana",
    "ceramic carrot",
    "ceramic tomato",
    "ceramic potato",
    "ceramic onion",
    "ceramic garlic",
    "ceramic pepper",
    "ceramic cucumber",
    "ceramic pumpkin",
    "ceramic hammer",
    "ceramic screwdriver",
    "ceramic wrench",
    "ceramic drill",
    "ceramic saw",
    "ceramic ladder",
    "ceramic shovel",
    "ceramic rake",
    "ceramic bucket",
    "ceramic wheelbarrow",
    "glass apple",
    "glass banana",
    "glass carrot",
    "glass tomato",
    "glass potato",
    "glass onion",
    "glass garlic",
    "glass pepper",
    "glass cucumber",
    "glass pumpkin",
    "glass hammer",
    "glass screwdriver",
    "glass wrench",
    "glass drill",
    "glass saw",
    "glass ladder",
    "glass shovel",
    "glass rake",
    "glass bucket",
    "glass wheelbarrow",
    "plastic apple",
    "plastic banana",
    "plastic carrot",
    "plastic tomato",
    "plastic potato",
    "plastic onion",
    "plastic garlic",
    "plastic pepper",
    "plastic cucumber",
    "plastic pumpkin",
    "plastic hammer",
    "plastic screwdriver",
    "plastic wrench",
    "plastic drill",
    "plastic saw",
    "plastic ladder",
    "plastic shovel",
    "plastic rake",
    "plastic bucket",
    "plastic wheelbarrow",
    "cotton apple",
    "cotton banana",
    "cotton carrot",
    "cotton tomato",
    "cotton potato",
    "cotton onion",
    "cotton garlic",
    "cotton pepper",
    "cotton cucumber",
    "cotton pumpkin",
    "cotton hammer",
    "cotton screwdriver",
    "cotton wrench",
    "cotton drill",
    "cotton saw",
    "cotton ladder",
    "cotton shovel",
    "cotton rake",
    "cotton bucket",
    "cotton wheelbarrow",
    "wool apple",
    "wool banana",
    "wool carrot",
    "wool tomato",
    "wool potato",
    "wool onion",
    "wool garlic",
    "wool pepper",
    "wool cucumber",
    "wool pumpkin",
    "wool hammer",
    "wool screwdriver",
    "wool wrench",
    "wool drill",
    "wool saw",
    "wool ladder",
    "wool shovel",
    "wool rake",
    "wool bucket",
    "wool wheelbarrow",
    "leather apple",
    "leather banana",
    "leather carrot",
    "leather tomato",
    "leather potato",
    "leather onion",
    "leather garlic",
    "leather pepper",
    "leather cucumber",
    "leather pumpkin",
    "leather hammer",
    "leather screwdriver",
    "leather wrench",
    "leather drill",
    "leather saw",
    "leather ladder",
    "leather shovel",
    "leather rake",
    "leather bucket",
    "leather wheelbarrow",
    "paper apple",
    "paper banana",
    "paper carrot",
    "paper tomato",
    "paper potato",
    "paper onion",
    "paper garlic",
    "paper pepper",
    "paper cucumber",
    "paper pumpkin",
    "paper hammer",
    "paper screwdriver",
    "paper wrench",
    "paper drill",
    "paper saw",
    "paper ladder",
    "paper shovel",
    "paper rake",
    "paper bucket",
    "paper wheelbarrow",
    "stone apple",
    "stone banana",
    "stone carrot",
    "stone tomato",
    "stone potato",
    "stone onion",
    "stone garlic",
    "stone pepper",
    "stone cucumber",
    "stone pumpkin",
    "stone hammer",
    "stone screwdriver",
    "stone wrench",
    "stone drill",
    "stone saw",
    "stone ladder",
    "stone shovel",
    "stone rake",
    "stone bucket",
    "stone wheelbarrow",
    "fresh apple",
    "fresh banana",
    "fresh carrot",
    "fresh tomato",
    "fresh potato",
    "fresh onion",
    "fresh garlic",
    "fresh pepper",
    "fresh cucumber",
    "fresh pumpkin",
    "fresh hammer",
    "fresh screwdriver",
    "fresh wrench",
    "fresh drill",
    "fresh saw",
    "fresh ladder",
    "fresh shovel",
    "fresh rake",
    "fresh bucket",
    "fresh wheelbarrow",
    "ripe apple",
    "ripe banana",
    "ripe carrot",
    "ripe tomato",
    "ripe potato",
    "ripe onion",
    "ripe garlic",
    "ripe pepper",
    "ripe cucumber",
    "ripe pumpkin",
    "ripe hammer",
    "ripe screwdriver",
    "ripe wrench",
    "ripe drill",
    "ripe saw",
    "ripe ladder",
    "ripe shovel",
    "ripe rake",
    "ripe bucket",
    "ripe wheelbarrow",
    "dry apple",
    "dry banana",
    "dry carrot",
    "dry tomato",
    "dry potato",
    "dry onion",
    "dry garlic",
    "dry pepper",
    "dry cucumber",
    "dry pumpkin",
    "dry hammer",
    "dry screwdriver",
    "dry wrench",
    "dry drill",
    "dry saw",
    "dry ladder",
    "dry shovel",
    "dry rake",
    "dry bucket",
    "dry wheelbarrow",
    "wet apple",
    "wet banana",
    "wet carrot",
    "wet tomato",
    "wet potato",
    "wet onion",
    "wet garlic",
    "wet pepper",
    "wet cucumber",
    "wet pumpkin",
    "wet hammer",
    "wet screwdriver",
    "wet wrench",
    "wet drill",
    "wet saw",
    "wet ladder",
    "wet shovel",
    "wet rake",
    "wet bucket",
    "wet wheelbarrow",
    "warm apple",
    "warm banana",
    "warm carrot",
    "warm tomato",
    "warm potato",
    "warm onion",
    "warm garlic",
    "warm pepper",
    "warm cucumber",
    "warm pumpkin",
    "warm hammer",
    "warm screwdriver",
    "warm wrench",
    "warm drill",
    "warm saw",
    "warm ladder",
    "warm shovel",
    "warm rake",
    "warm bucket",
    "warm wheelbarrow",
    "cold apple",
    "cold banana",
    "cold carrot",
    "cold tomato",
    "cold potato",
    "cold onion",
    "cold garlic",
    "cold pepper",
    "cold cucumber",
    "cold pumpkin",
    "cold hammer",
    "cold screwdriver",
    "cold wrench",
    "cold drill",
    "cold saw",
    "cold ladder",
    "cold shovel",
    "cold rake",
    "cold bucket",
    "cold wheelbarrow",
    "tiny apple",
    "tiny banana",
    "tiny carrot",
    "tiny tomato",
    "tiny potato",
    "tiny onion",
    "tiny garlic",
    "tiny pepper",
    "tiny cucumber",
    "tiny pumpkin",
    "tiny hammer",
    "tiny screwdriver",
    "tiny wrench",
    "tiny drill",
    "tiny saw",
    "tiny ladder",
    "tiny shovel",
    "tiny rake",
    "tiny bucket",
    "tiny wheelbarrow",
    "small apple",
    "small banana",
    "small carrot",
    "small tomato",
    "small potato",
    "small onion",
    "small garlic",
    "small pepper",
    "small cucumber",
    "small pumpkin",
    "small hammer",
    "small screwdriver",
    "small wrench",
    "small drill",
    "small saw",
    "small ladder",
    "small shovel",
    "small rake",
    "small bucket",
    "small wheelbarrow",
    "medium apple",
    "medium banana",
    "medium carrot",
    "medium tomato",
    "medium potato",
    "medium onion",
    "medium garlic",
    "medium pepper",
    "medium cucumber",
    "medium pumpkin",
    "medium hammer",
    "medium screwdriver",
    "medium wrench",
    "medium drill",
    "medium saw",
    "medium ladder",
    "medium shovel",
    "medium rake",
    "medium bucket",
    "medium wheelbarrow",
    "large apple",
    "large banana",
    "large carrot",
    "large tomato",
    "large potato",
    "large onion",
    "large garlic",
    "large pepper",
    "large cucumber",
    "large pumpkin",
    "large hammer",
    "large screwdriver",
    "large wrench",
    "large drill",
    "large saw",
    "large ladder",
    "large shovel",
    "large rake",
    "large bucket",
    "large wheelbarrow",
    "giant apple",
    "giant banana",
    "giant carrot",
    "giant tomato",
    "giant potato",
    "giant onion",
    "giant garlic",
    "giant pepper",
    "giant cucumber",
    "giant pumpkin",
    "giant hammer",
    "giant screwdriver",
    "giant wrench",
    "giant drill",
    "giant saw",
    "giant ladder",
    "giant shovel",
    "giant rake",
    "giant bucket",
    "giant wheelbarrow",
    "narrow apple",
    "narrow banana",
    "narrow carrot",
    "narrow tomato",
    "narrow potato",
    "narrow onion",
    "narrow garlic",
    "narrow pepper",
    "narrow cucumber",
    "narrow pumpkin",
    "narrow hammer",
    "narrow screwdriver",
    "narrow wrench",
    "narrow drill",
    "narrow saw",
    "narrow ladder",
    "narrow shovel",
    "narrow rake",
    "narrow bucket",
    "narrow wheelbarrow",
    "wide apple",
    "wide banana",
    "wide carrot",
    "wide tomato",
    "wide potato",
    "wide onion",
    "wide garlic",
    "wide pepper",
    "wide cucumber",
    "wide pumpkin",
    "wide hammer",
    "wide screwdriver",
    "wide wrench",
    "wide drill",
    "wide saw",
    "wide ladder",
    "wide shovel",
    "wide rake",
    "wide bucket",
    "wide wheelbarrow",
    "round apple",
    "round banana",
    "round carrot",
    "round tomato",
    "round potato",
    "round onion",
    "round garlic",
    "round pepper",
    "round cucumber",
    "round pumpkin",
    "round hammer",
    "round screwdriver",
    "round wrench",
    "round drill",
    "round saw",
    "round ladder",
    "round shovel",
    "round rake",
    "round bucket",
    "round wheelbarrow",
    "square apple",
    "square banana",
    "square carrot",
    "square tomato",
    "square potato",
    "square onion",
    "square garlic",
    "square pepper",
    "square cucumber",
    "square pumpkin",
    "square hammer",
    "square screwdriver",
    "square wrench",
    "square drill",
    "square saw",
    "square ladder",
    "square shovel",
    "square rake",
    "square bucket",
    "square wheelbarrow",
    "triangular apple",
    "triangular banana",
    "triangular carrot",
    "triangular tomato",
    "triangular potato",
    "triangular onion",
    "triangular garlic",
    "triangular pepper",
    "triangular cucumber",
    "triangular pumpkin",
    "triangular hammer",
    "triangular screwdriver",
    "triangular wrench",
    "triangular drill",
    "triangular saw",
    "triangular ladder",
    "triangular shovel",
    "triangular rake",
    "triangular bucket",
    "triangular wheelbarrow",
    "smooth apple",
    "smooth banana",
    "smooth carrot",
    "smooth tomato",
    "smooth potato",
    "smooth onion",
    "smooth garlic",
    "smooth pepper",
    "smooth cucumber",
    "smooth pumpkin",
    "smooth hammer",
    "smooth screwdriver",
    "smooth wrench",
    "smooth drill",
    "smooth saw",
    "smooth ladder",
    "smooth shovel",
    "smooth rake",
    "smooth bucket",
    "smooth wheelbarrow",
    "rough apple",
    "rough banana",
    "rough carrot",
    "rough tomato",
    "rough potato",
    "rough onion",
    "rough garlic",
    "rough pepper",
    "rough cucumber",
    "rough pumpkin",
    "rough hammer",
    "rough screwdriver",
    "rough wrench",
    "rough drill",
    "rough saw",
    "rough ladder",
    "rough shovel",
    "rough rake",
    "rough bucket",
    "rough wheelbarrow",
    "soft apple",
    "soft banana",
    "soft carrot",
    "soft tomato",
    "soft potato",
    "soft onion",
    "soft garlic",
    "soft pepper",
    "soft cucumber",
    "soft pumpkin",
    "soft hammer",
    "soft screwdriver",
    "soft wrench",
    "soft drill",
    "soft saw",
    "soft ladder",
    "soft shovel",
    "soft rake",
    "soft bucket",
    "soft wheelbarrow",
    "hard apple",
    "hard banana",
    "hard carrot",
    "hard tomato",
    "hard potato",
    "hard onion",
    "hard garlic",
    "hard pepper",
    "hard cucumber",
    "hard pumpkin",
    "hard hammer",
    "hard screwdriver",
    "hard wrench",
    "hard drill",
    "hard saw",
    "hard ladder",
    "hard shovel",
    "hard rake",
    "hard bucket",
    "hard wheelbarrow",
    "bright apple",
    "bright banana",
    "bright carrot",
    "bright tomato",
    "bright potato",
    "bright onion",
    "bright garlic",
    "bright pepper",
    "bright cucumber",
    "bright pumpkin",
    "bright hammer",
    "bright screwdriver",
    "bright wrench",
    "bright drill",
    "bright saw",
    "bright ladder",
    "bright shovel",
    "bright rake",
    "bright bucket",
    "bright wheelbarrow",
    "dim apple",
    "dim banana",
    "dim carrot",
    "dim tomato",
    "dim potato",
    "dim onion",
    "dim garlic",
    "dim pepper",
    "dim cucumber",
    "dim pumpkin",
    "dim hammer",
    "dim screwdriver",
    "dim wrench",
    "dim drill",
    "dim saw",
    "dim ladder",
    "dim shovel",
    "dim rake",
    "dim bucket",
    "dim wheelbarrow",
    "ancient apple",
    "ancient banana",
    "ancient carrot",
    "ancient tomato",
    "ancient potato",
    "ancient onion",
    "ancient garlic",
    "ancient pepper",
    "ancient cucumber",
    "ancient pumpkin",
    "ancient hammer",
    "ancient screwdriver",
    "ancient wrench",
    "ancient drill",
    "ancient saw",
    "ancient ladder",
    "ancient shovel",
    "ancient rake",
    "ancient bucket",
    "ancient wheelbarrow",
    "modern apple",
    "modern banana",
    "modern carrot",
    "modern tomato",
    "modern potato",
    "modern onion",
    "modern garlic",
    "modern pepper",
    "modern cucumber",
    "modern pumpkin",
    "modern hammer",
    "modern screwdriver",
    "modern wrench",
    "modern drill",
    "modern saw",
    "modern ladder",
    "modern shovel",
    "modern rake",
    "modern bucket",
    "modern wheelbarrow",
    "vintage apple",
    "vintage banana",
    "vintage carrot",
    "vintage tomato",
    "vintage potato",
    "vintage onion",
    "vintage garlic",
    "vintage pepper",
    "vintage cucumber",
    "vintage pumpkin",
    "vintage hammer",
    "vintage screwdriver",
    "vintage wrench",
    "vintage drill",
    "vintage saw",
    "vintage ladder",
    "vintage shovel",
    "vintage rake",
    "vintage bucket",
    "vintage wheelbarrow",
    "portable apple",
    "portable banana",
    "portable carrot",
    "portable tomato",
    "portable potato",
    "portable onion",
    "portable garlic",
    "portable pepper",
    "portable cucumber",
    "portable pumpkin",
    "portable hammer",
    "portable screwdriver",
    "portable wrench",
    "portable drill",
    "portable saw",
    "portable ladder",
    "portable shovel",
    "portable rake",
    "portable bucket",
    "portable wheelbarrow",
    "folding apple",
    "folding banana",
    "folding carrot",
    "folding tomato",
    "folding potato",
    "folding onion",
    "folding garlic",
    "folding pepper",
    "folding cucumber",
    "folding pumpkin",
    "folding hammer",
    "folding screwdriver",
    "folding wrench",
    "folding drill",
    "folding saw",
    "folding ladder",
    "folding shovel",
    "folding rake",
    "folding bucket",
    "folding wheelbarrow",
    "electric apple",
    "electric banana",
    "electric carrot",
    "electric tomato",
    "electric potato",
    "electric onion",
    "electric garlic",
    "electric pepper",
    "electric cucumber",
    "electric pumpkin",
    "electric hammer",
    "electric screwdriver",
    "electric wrench",
    "electric drill",
    "electric saw",
    "electric ladder",
    "electric shovel",
    "electric rake",
    "electric bucket",
    "electric wheelbarrow",
    "manual apple",
    "manual banana",
    "manual carrot",
    "manual tomato",
    "manual potato",
    "manual onion",
    "manual garlic",
    "manual pepper",
    "manual cucumber",
    "manual pumpkin",
    "manual hammer",
    "manual screwdriver",
    "manual wrench",
    "manual drill",
    "manual saw",
    "manual ladder",
    "manual shovel",
    "manual rake",
    "manual bucket",
    "manual wheelbarrow",
    "solar apple",
    "solar banana",
    "solar carrot",
    "solar tomato",
    "solar potato",
    "solar onion",
    "solar garlic",
    "solar pepper",
    "solar cucumber",
    "solar pumpkin",
    "solar hammer",
    "solar screwdriver",
    "solar wrench",
    "solar drill",
    "solar saw",
    "solar ladder",
    "solar shovel",
    "solar rake",
    "solar bucket",
    "solar wheelbarrow",
];

#[cfg(test)]
mod tests {
    use super::REAL_THINGS;

    #[test]
    fn real_things_collection_has_expected_volume() {
        assert_eq!(REAL_THINGS.len(), 1000);
    }

    #[test]
    fn real_things_are_non_empty() {
        assert!(REAL_THINGS.iter().all(|item| !item.trim().is_empty()));
    }
}
