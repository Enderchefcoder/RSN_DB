use rand::seq::SliceRandom;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

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
                let thing = pick_thing("welcome", "session");
                format!("Oh good, you're back. I was just benchmarking a {}.", thing)
            }
        }
    }

    pub fn success(&self, message: &str) -> String {
        match self.mode {
            Mode::Professional => format!("✓ {}", message),
            Mode::Friendly => format!("✓ Done! {}.", message),
            Mode::Snarky => {
                let variants = [
                    format!(
                        "✓ {}. Surprising, but in a pleasant {} kind of way.",
                        message,
                        pick_thing(message, "success")
                    ),
                    format!(
                        "✓ {}. That landed cleaner than a calibrated {}.",
                        message,
                        pick_thing(message, "precision")
                    ),
                    format!(
                        "✓ {}. Nice. Even your {} didn't get in the way this time.",
                        message,
                        pick_thing(message, "tool")
                    ),
                    format!(
                        "✓ {}. Minimal chaos. Maximum {}.",
                        message,
                        pick_thing(message, "stability")
                    ),
                ];
                choose_variant(&variants)
            }
        }
    }

    pub fn error(&self, error: &str) -> String {
        match self.mode {
            Mode::Professional => format!("✗ {}", error),
            Mode::Friendly => format!("✗ Oops! {}.", error),
            Mode::Snarky => {
                let thing = pick_thing(error, "error");
                format!(
                    "✗ {}. This failed with the confidence of a cracked {}.",
                    error, thing
                )
            }
        }
    }

    pub fn typo_suggestion(&self, typo: &str, expected: &str) -> String {
        match self.mode {
            Mode::Professional => format!("Unknown: {}. Did you mean {}?", typo, expected),
            Mode::Friendly => format!("I couldn't find `{}`. Did you mean `{}`?", typo, expected),
            Mode::Snarky => {
                let thing = pick_thing(typo, expected);
                let variants = [
                    format!("`{}` isn't a command. `{}` probably is. Try that before I repurpose your {}.", typo, expected, thing),
                    format!("You typed `{}`. The command is `{}`. That's not a typo, that's performance art with a {}.", typo, expected, thing),
                    format!("`{}`? Bold choice. `{}` exists. Keep going and I'll log this next to a {}.", typo, expected, thing),
                ];
                choose_variant(&variants)
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
            Mode::Snarky => {
                let thing = pick_thing(&count.to_string(), "empty");
                match count {
                    0 | 1 => String::new(),
                    2 => format!("  ...hello? Did your {} eat the rest of the command?", thing),
                    3 => format!(
                        "  Okay, I get it. You're having a moment.
  I'm over here maintaining indexes like a {}
  while you submit interpretive silence.
  
  No rush. Really.",
                        thing
                    ),
                    _ => format!(
                        "  Still nothing. Incredible. I'm filing this under 'advanced {} operations'.",
                        thing
                    ),
                }
            }
        }
    }

    pub fn achievement_unlocked(&self) -> String {
        match self.mode {
            Mode::Professional => "Achievement unlocked!".to_string(),
            Mode::Friendly => "Achievement unlocked! Nice momentum—keep going.".to_string(),
            Mode::Snarky => {
                let thing = pick_thing("achievement", "streak");
                format!("Achievement unlocked: Barely supervised competence. Reward: one ceremonial {}.", thing)
            }
        }
    }

    pub fn competent_streak(&self, streak_size: usize) -> String {
        match self.mode {
            Mode::Professional => format!(
                "[SYSTEM]: Achievement unlocked: {} clean commands",
                streak_size
            ),
            Mode::Friendly => format!(
                "[SYSTEM]: Achievement unlocked: {} clean commands
[SYSTEM]: That was excellent consistency.",
                streak_size
            ),
            Mode::Snarky => {
                let thing = pick_thing(&streak_size.to_string(), "streak");
                format!(
                    "  Wait. Hold on.
  
  {} commands in a row with no typo?
  No syntax errors? Not even one chaotic comma?
  
  I'm documenting this next to a {}.
  [SYSTEM]: User status upgraded to 'Occasionally Capable'.",
                    streak_size, thing
                )
            }
        }
    }

    pub fn why_mean(&self) -> String {
        match self.mode {
            Mode::Professional => "I'm not mean. I'm precise.".to_string(),
            Mode::Friendly => {
                "I'm not trying to be mean—just direct so your queries stay correct.".to_string()
            }
            Mode::Snarky => {
                let thing = pick_thing("feelings", "why_mean");
                format!(
                    "  I'm not mean. I'm precise.
  You make mistakes; I annotate them.
  That's the job description, not a personality flaw.
  Also, this conversation has the emotional structure of a {}.",
                    thing
                )
            }
        }
    }

    pub fn help_optimize(&self, table: &str) -> String {
        match self.mode {
            Mode::Professional => format!(
                "OPTIMIZE <table>
Rebuild indexes and compact storage for better read performance.
Example: OPTIMIZE {}",
                table
            ),
            Mode::Friendly => format!(
                "OPTIMIZE <table>
Rebuilds indexes and compacts storage for faster queries.

Example:
  OPTIMIZE {}",
                table
            ),
            Mode::Snarky => {
                let thing = pick_thing(table, "optimize");
                let jab = pick_snark_phrase(table);
                format!(
                    "╭──────────────────────────────────────────────────────────╮
│  OPTIMIZE <table>                                        │
│                                                          │
│  Rebuilds indexes and compacts storage for faster       │
│  queries. {}
│                                                          │
│  Example:                                               │
│    OPTIMIZE {}{}
│                                                          │
│  Note: This takes time. Bring patience and maybe a {}.  │
╰──────────────────────────────────────────────────────────╯",
                    pad_to_width(jab, 56),
                    table,
                    pad_to_width("", 46usize.saturating_sub(table.len())),
                    thing
                )
            }
        }
    }

    pub fn explain_typo(&self, typo: &str, expected: &str) -> String {
        match self.mode {
            Mode::Professional => {
                format!("Unknown command: '{}'. Did you mean: {} ?", typo, expected)
            }
            Mode::Friendly => format!(
                "Unknown command: '{}'
Did you mean: {} ?",
                typo, expected
            ),
            Mode::Snarky => {
                let thing = pick_thing(typo, expected);
                let variants = [
                    format!(
                        "✗ Unknown command: '{}'

  Did you mean: {} ?
  
  You misspelled a six-letter word that's in HELP.
  That's elite confidence paired with a {}.",
                        typo, expected, thing
                    ),
                    format!(
                        "✗ Unknown command: '{}'

  Did you mean: {} ?
  
  I admire the improvisation, but this parser
  is not a jazz club. Bring a {} and try again.",
                        typo, expected, thing
                    ),
                    format!(
                        "✗ Unknown command: '{}'

  Did you mean: {} ?
  
  You were one vowel away from success.
  Please reattempt without launching a {}.",
                        typo, expected, thing
                    ),
                ];
                choose_variant(&variants)
            }
        }
    }

    pub fn type_mismatch(&self, field: &str, expected: &str, got: &str) -> String {
        match self.mode {
            Mode::Professional => format!(
                "✗ Type mismatch on field '{}': expected {}, got {}",
                field, expected, got
            ),
            Mode::Friendly => format!(
                "✗ Type mismatch on field '{}': expected {}, got {}.",
                field, expected, got
            ),
            Mode::Snarky => {
                let thing = pick_thing(field, got);
                let coach = pick_type_hint(expected);
                format!(
                    "✗ Type mismatch on field '{}':
  Expected: {}
  Got:      {}

  You're feeding me shapes that don't fit.
  This is like installing a {} where a number should be.

  Try: {}",
                    field, expected, got, thing, coach
                )
            }
        }
    }

    pub fn destructive_confirmation_failed(&self, expected: &str, got: &str) -> String {
        match self.mode {
            Mode::Professional => format!(
                "Confirmation failed. You typed '{}' but expected '{}'.",
                got, expected
            ),
            Mode::Friendly => format!(
                "Confirmation failed. You typed '{}' but the table is '{}'.",
                got, expected
            ),
            Mode::Snarky => {
                let thing = pick_thing(expected, got);
                format!(
                    "✗ Confirmation failed. You typed '{}' but the table is '{}'.
  
  That's close in the same way a {} is close to a data center.
  Request denied. Come back with the exact name.",
                    got, expected, thing
                )
            }
        }
    }

    pub fn batch_committed(&self, operations: usize) -> String {
        match self.mode {
            Mode::Professional => format!("Batch executed: {} ops.", operations),
            Mode::Friendly => format!("Batch complete: {} operation(s) committed.", operations),
            Mode::Snarky => {
                let thing = pick_thing(&operations.to_string(), "batch");
                format!(
                    "Batch done: {} operation(s). Fewer explosions than a {}.",
                    operations, thing
                )
            }
        }
    }
}

fn choose_variant(options: &[String]) -> String {
    options
        .choose(&mut rand::thread_rng())
        .cloned()
        .unwrap_or_else(String::new)
}

fn seed_index(seed_left: &str, seed_right: &str, len: usize) -> usize {
    if len == 0 {
        return 0;
    }
    let mut hasher = DefaultHasher::new();
    seed_left.hash(&mut hasher);
    seed_right.hash(&mut hasher);
    (hasher.finish() as usize) % len
}

fn pick_thing(seed_left: &str, seed_right: &str) -> &'static str {
    REAL_THINGS[seed_index(seed_left, seed_right, REAL_THINGS.len())]
}

fn pick_snark_phrase(seed: &str) -> &'static str {
    const PHRASES: [&str; 12] = [
        "Because waiting for fragmented indexes to heal themselves is adorable.",
        "You know, the maintenance step that mysteriously gets skipped.",
        "The thing people rediscover right after saying 'this query is slow'.",
        "Like rotating tires, except the tires are your query plans.",
        "Yes, this is the boring command that makes everything faster.",
        "This is where performance debt pays interest.",
        "Your future self will thank you, quietly and with less latency.",
        "Do it now, or debug sluggish scans later.",
        "It is preventative care for your storage engine.",
        "Neglect this long enough and every read becomes cardio.",
        "Think of it as spring-cleaning for your B-trees.",
        "If speed matters, this isn't optional theater.",
    ];
    PHRASES[seed_index(seed, "optimize-phrase", PHRASES.len())]
}

fn pick_type_hint(expected: &str) -> &'static str {
    match expected.to_ascii_lowercase().as_str() {
        "float" => "12.00",
        "integer" | "int" => "12",
        "boolean" | "bool" => "true",
        "string" => "\"example\"",
        _ => "a correctly typed value",
    }
}

fn pad_to_width(input: &str, width: usize) -> String {
    let mut text = input.to_string();
    if text.len() < width {
        text.push_str(&" ".repeat(width - text.len()));
    }
    text
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
║     v0.1.0-alpha   |   engine: ember   |   mode: persistent ║
║     storage: ./rsn_data/              |   cache: 256MB      ║
║                                                              ║
║     Type HELP for commands. Type EXIT to quit.              ║
║     "Handle your data, or I will." — RSN_DB                ║
╚══════════════════════════════════════════════════════════════╝
"#;

pub const REAL_THINGS: [&str; 1000] = [
    "basalt bridge",
    "basalt teapot",
    "basalt lantern",
    "basalt compass",
    "basalt typewriter",
    "basalt hourglass",
    "basalt violin",
    "basalt backpack",
    "basalt notebook",
    "basalt watch",
    "basalt hammer",
    "basalt screwdriver",
    "basalt wrench",
    "basalt bicycle",
    "basalt helmet",
    "basalt kettle",
    "basalt microscope",
    "basalt telescope",
    "basalt camera",
    "basalt lighthouse",
    "basalt windmill",
    "basalt greenhouse",
    "basalt workbench",
    "basalt bookshelf",
    "basalt door",
    "granite bridge",
    "granite teapot",
    "granite lantern",
    "granite compass",
    "granite typewriter",
    "granite hourglass",
    "granite violin",
    "granite backpack",
    "granite notebook",
    "granite watch",
    "granite hammer",
    "granite screwdriver",
    "granite wrench",
    "granite bicycle",
    "granite helmet",
    "granite kettle",
    "granite microscope",
    "granite telescope",
    "granite camera",
    "granite lighthouse",
    "granite windmill",
    "granite greenhouse",
    "granite workbench",
    "granite bookshelf",
    "granite door",
    "limestone bridge",
    "limestone teapot",
    "limestone lantern",
    "limestone compass",
    "limestone typewriter",
    "limestone hourglass",
    "limestone violin",
    "limestone backpack",
    "limestone notebook",
    "limestone watch",
    "limestone hammer",
    "limestone screwdriver",
    "limestone wrench",
    "limestone bicycle",
    "limestone helmet",
    "limestone kettle",
    "limestone microscope",
    "limestone telescope",
    "limestone camera",
    "limestone lighthouse",
    "limestone windmill",
    "limestone greenhouse",
    "limestone workbench",
    "limestone bookshelf",
    "limestone door",
    "sandstone bridge",
    "sandstone teapot",
    "sandstone lantern",
    "sandstone compass",
    "sandstone typewriter",
    "sandstone hourglass",
    "sandstone violin",
    "sandstone backpack",
    "sandstone notebook",
    "sandstone watch",
    "sandstone hammer",
    "sandstone screwdriver",
    "sandstone wrench",
    "sandstone bicycle",
    "sandstone helmet",
    "sandstone kettle",
    "sandstone microscope",
    "sandstone telescope",
    "sandstone camera",
    "sandstone lighthouse",
    "sandstone windmill",
    "sandstone greenhouse",
    "sandstone workbench",
    "sandstone bookshelf",
    "sandstone door",
    "marble bridge",
    "marble teapot",
    "marble lantern",
    "marble compass",
    "marble typewriter",
    "marble hourglass",
    "marble violin",
    "marble backpack",
    "marble notebook",
    "marble watch",
    "marble hammer",
    "marble screwdriver",
    "marble wrench",
    "marble bicycle",
    "marble helmet",
    "marble kettle",
    "marble microscope",
    "marble telescope",
    "marble camera",
    "marble lighthouse",
    "marble windmill",
    "marble greenhouse",
    "marble workbench",
    "marble bookshelf",
    "marble door",
    "slate bridge",
    "slate teapot",
    "slate lantern",
    "slate compass",
    "slate typewriter",
    "slate hourglass",
    "slate violin",
    "slate backpack",
    "slate notebook",
    "slate watch",
    "slate hammer",
    "slate screwdriver",
    "slate wrench",
    "slate bicycle",
    "slate helmet",
    "slate kettle",
    "slate microscope",
    "slate telescope",
    "slate camera",
    "slate lighthouse",
    "slate windmill",
    "slate greenhouse",
    "slate workbench",
    "slate bookshelf",
    "slate door",
    "obsidian bridge",
    "obsidian teapot",
    "obsidian lantern",
    "obsidian compass",
    "obsidian typewriter",
    "obsidian hourglass",
    "obsidian violin",
    "obsidian backpack",
    "obsidian notebook",
    "obsidian watch",
    "obsidian hammer",
    "obsidian screwdriver",
    "obsidian wrench",
    "obsidian bicycle",
    "obsidian helmet",
    "obsidian kettle",
    "obsidian microscope",
    "obsidian telescope",
    "obsidian camera",
    "obsidian lighthouse",
    "obsidian windmill",
    "obsidian greenhouse",
    "obsidian workbench",
    "obsidian bookshelf",
    "obsidian door",
    "quartz bridge",
    "quartz teapot",
    "quartz lantern",
    "quartz compass",
    "quartz typewriter",
    "quartz hourglass",
    "quartz violin",
    "quartz backpack",
    "quartz notebook",
    "quartz watch",
    "quartz hammer",
    "quartz screwdriver",
    "quartz wrench",
    "quartz bicycle",
    "quartz helmet",
    "quartz kettle",
    "quartz microscope",
    "quartz telescope",
    "quartz camera",
    "quartz lighthouse",
    "quartz windmill",
    "quartz greenhouse",
    "quartz workbench",
    "quartz bookshelf",
    "quartz door",
    "jade bridge",
    "jade teapot",
    "jade lantern",
    "jade compass",
    "jade typewriter",
    "jade hourglass",
    "jade violin",
    "jade backpack",
    "jade notebook",
    "jade watch",
    "jade hammer",
    "jade screwdriver",
    "jade wrench",
    "jade bicycle",
    "jade helmet",
    "jade kettle",
    "jade microscope",
    "jade telescope",
    "jade camera",
    "jade lighthouse",
    "jade windmill",
    "jade greenhouse",
    "jade workbench",
    "jade bookshelf",
    "jade door",
    "amber bridge",
    "amber teapot",
    "amber lantern",
    "amber compass",
    "amber typewriter",
    "amber hourglass",
    "amber violin",
    "amber backpack",
    "amber notebook",
    "amber watch",
    "amber hammer",
    "amber screwdriver",
    "amber wrench",
    "amber bicycle",
    "amber helmet",
    "amber kettle",
    "amber microscope",
    "amber telescope",
    "amber camera",
    "amber lighthouse",
    "amber windmill",
    "amber greenhouse",
    "amber workbench",
    "amber bookshelf",
    "amber door",
    "copper bridge",
    "copper teapot",
    "copper lantern",
    "copper compass",
    "copper typewriter",
    "copper hourglass",
    "copper violin",
    "copper backpack",
    "copper notebook",
    "copper watch",
    "copper hammer",
    "copper screwdriver",
    "copper wrench",
    "copper bicycle",
    "copper helmet",
    "copper kettle",
    "copper microscope",
    "copper telescope",
    "copper camera",
    "copper lighthouse",
    "copper windmill",
    "copper greenhouse",
    "copper workbench",
    "copper bookshelf",
    "copper door",
    "bronze bridge",
    "bronze teapot",
    "bronze lantern",
    "bronze compass",
    "bronze typewriter",
    "bronze hourglass",
    "bronze violin",
    "bronze backpack",
    "bronze notebook",
    "bronze watch",
    "bronze hammer",
    "bronze screwdriver",
    "bronze wrench",
    "bronze bicycle",
    "bronze helmet",
    "bronze kettle",
    "bronze microscope",
    "bronze telescope",
    "bronze camera",
    "bronze lighthouse",
    "bronze windmill",
    "bronze greenhouse",
    "bronze workbench",
    "bronze bookshelf",
    "bronze door",
    "brass bridge",
    "brass teapot",
    "brass lantern",
    "brass compass",
    "brass typewriter",
    "brass hourglass",
    "brass violin",
    "brass backpack",
    "brass notebook",
    "brass watch",
    "brass hammer",
    "brass screwdriver",
    "brass wrench",
    "brass bicycle",
    "brass helmet",
    "brass kettle",
    "brass microscope",
    "brass telescope",
    "brass camera",
    "brass lighthouse",
    "brass windmill",
    "brass greenhouse",
    "brass workbench",
    "brass bookshelf",
    "brass door",
    "steel bridge",
    "steel teapot",
    "steel lantern",
    "steel compass",
    "steel typewriter",
    "steel hourglass",
    "steel violin",
    "steel backpack",
    "steel notebook",
    "steel watch",
    "steel hammer",
    "steel screwdriver",
    "steel wrench",
    "steel bicycle",
    "steel helmet",
    "steel kettle",
    "steel microscope",
    "steel telescope",
    "steel camera",
    "steel lighthouse",
    "steel windmill",
    "steel greenhouse",
    "steel workbench",
    "steel bookshelf",
    "steel door",
    "iron bridge",
    "iron teapot",
    "iron lantern",
    "iron compass",
    "iron typewriter",
    "iron hourglass",
    "iron violin",
    "iron backpack",
    "iron notebook",
    "iron watch",
    "iron hammer",
    "iron screwdriver",
    "iron wrench",
    "iron bicycle",
    "iron helmet",
    "iron kettle",
    "iron microscope",
    "iron telescope",
    "iron camera",
    "iron lighthouse",
    "iron windmill",
    "iron greenhouse",
    "iron workbench",
    "iron bookshelf",
    "iron door",
    "silver bridge",
    "silver teapot",
    "silver lantern",
    "silver compass",
    "silver typewriter",
    "silver hourglass",
    "silver violin",
    "silver backpack",
    "silver notebook",
    "silver watch",
    "silver hammer",
    "silver screwdriver",
    "silver wrench",
    "silver bicycle",
    "silver helmet",
    "silver kettle",
    "silver microscope",
    "silver telescope",
    "silver camera",
    "silver lighthouse",
    "silver windmill",
    "silver greenhouse",
    "silver workbench",
    "silver bookshelf",
    "silver door",
    "gold bridge",
    "gold teapot",
    "gold lantern",
    "gold compass",
    "gold typewriter",
    "gold hourglass",
    "gold violin",
    "gold backpack",
    "gold notebook",
    "gold watch",
    "gold hammer",
    "gold screwdriver",
    "gold wrench",
    "gold bicycle",
    "gold helmet",
    "gold kettle",
    "gold microscope",
    "gold telescope",
    "gold camera",
    "gold lighthouse",
    "gold windmill",
    "gold greenhouse",
    "gold workbench",
    "gold bookshelf",
    "gold door",
    "tin bridge",
    "tin teapot",
    "tin lantern",
    "tin compass",
    "tin typewriter",
    "tin hourglass",
    "tin violin",
    "tin backpack",
    "tin notebook",
    "tin watch",
    "tin hammer",
    "tin screwdriver",
    "tin wrench",
    "tin bicycle",
    "tin helmet",
    "tin kettle",
    "tin microscope",
    "tin telescope",
    "tin camera",
    "tin lighthouse",
    "tin windmill",
    "tin greenhouse",
    "tin workbench",
    "tin bookshelf",
    "tin door",
    "nickel bridge",
    "nickel teapot",
    "nickel lantern",
    "nickel compass",
    "nickel typewriter",
    "nickel hourglass",
    "nickel violin",
    "nickel backpack",
    "nickel notebook",
    "nickel watch",
    "nickel hammer",
    "nickel screwdriver",
    "nickel wrench",
    "nickel bicycle",
    "nickel helmet",
    "nickel kettle",
    "nickel microscope",
    "nickel telescope",
    "nickel camera",
    "nickel lighthouse",
    "nickel windmill",
    "nickel greenhouse",
    "nickel workbench",
    "nickel bookshelf",
    "nickel door",
    "aluminum bridge",
    "aluminum teapot",
    "aluminum lantern",
    "aluminum compass",
    "aluminum typewriter",
    "aluminum hourglass",
    "aluminum violin",
    "aluminum backpack",
    "aluminum notebook",
    "aluminum watch",
    "aluminum hammer",
    "aluminum screwdriver",
    "aluminum wrench",
    "aluminum bicycle",
    "aluminum helmet",
    "aluminum kettle",
    "aluminum microscope",
    "aluminum telescope",
    "aluminum camera",
    "aluminum lighthouse",
    "aluminum windmill",
    "aluminum greenhouse",
    "aluminum workbench",
    "aluminum bookshelf",
    "aluminum door",
    "oak bridge",
    "oak teapot",
    "oak lantern",
    "oak compass",
    "oak typewriter",
    "oak hourglass",
    "oak violin",
    "oak backpack",
    "oak notebook",
    "oak watch",
    "oak hammer",
    "oak screwdriver",
    "oak wrench",
    "oak bicycle",
    "oak helmet",
    "oak kettle",
    "oak microscope",
    "oak telescope",
    "oak camera",
    "oak lighthouse",
    "oak windmill",
    "oak greenhouse",
    "oak workbench",
    "oak bookshelf",
    "oak door",
    "maple bridge",
    "maple teapot",
    "maple lantern",
    "maple compass",
    "maple typewriter",
    "maple hourglass",
    "maple violin",
    "maple backpack",
    "maple notebook",
    "maple watch",
    "maple hammer",
    "maple screwdriver",
    "maple wrench",
    "maple bicycle",
    "maple helmet",
    "maple kettle",
    "maple microscope",
    "maple telescope",
    "maple camera",
    "maple lighthouse",
    "maple windmill",
    "maple greenhouse",
    "maple workbench",
    "maple bookshelf",
    "maple door",
    "cedar bridge",
    "cedar teapot",
    "cedar lantern",
    "cedar compass",
    "cedar typewriter",
    "cedar hourglass",
    "cedar violin",
    "cedar backpack",
    "cedar notebook",
    "cedar watch",
    "cedar hammer",
    "cedar screwdriver",
    "cedar wrench",
    "cedar bicycle",
    "cedar helmet",
    "cedar kettle",
    "cedar microscope",
    "cedar telescope",
    "cedar camera",
    "cedar lighthouse",
    "cedar windmill",
    "cedar greenhouse",
    "cedar workbench",
    "cedar bookshelf",
    "cedar door",
    "walnut bridge",
    "walnut teapot",
    "walnut lantern",
    "walnut compass",
    "walnut typewriter",
    "walnut hourglass",
    "walnut violin",
    "walnut backpack",
    "walnut notebook",
    "walnut watch",
    "walnut hammer",
    "walnut screwdriver",
    "walnut wrench",
    "walnut bicycle",
    "walnut helmet",
    "walnut kettle",
    "walnut microscope",
    "walnut telescope",
    "walnut camera",
    "walnut lighthouse",
    "walnut windmill",
    "walnut greenhouse",
    "walnut workbench",
    "walnut bookshelf",
    "walnut door",
    "birch bridge",
    "birch teapot",
    "birch lantern",
    "birch compass",
    "birch typewriter",
    "birch hourglass",
    "birch violin",
    "birch backpack",
    "birch notebook",
    "birch watch",
    "birch hammer",
    "birch screwdriver",
    "birch wrench",
    "birch bicycle",
    "birch helmet",
    "birch kettle",
    "birch microscope",
    "birch telescope",
    "birch camera",
    "birch lighthouse",
    "birch windmill",
    "birch greenhouse",
    "birch workbench",
    "birch bookshelf",
    "birch door",
    "pine bridge",
    "pine teapot",
    "pine lantern",
    "pine compass",
    "pine typewriter",
    "pine hourglass",
    "pine violin",
    "pine backpack",
    "pine notebook",
    "pine watch",
    "pine hammer",
    "pine screwdriver",
    "pine wrench",
    "pine bicycle",
    "pine helmet",
    "pine kettle",
    "pine microscope",
    "pine telescope",
    "pine camera",
    "pine lighthouse",
    "pine windmill",
    "pine greenhouse",
    "pine workbench",
    "pine bookshelf",
    "pine door",
    "bamboo bridge",
    "bamboo teapot",
    "bamboo lantern",
    "bamboo compass",
    "bamboo typewriter",
    "bamboo hourglass",
    "bamboo violin",
    "bamboo backpack",
    "bamboo notebook",
    "bamboo watch",
    "bamboo hammer",
    "bamboo screwdriver",
    "bamboo wrench",
    "bamboo bicycle",
    "bamboo helmet",
    "bamboo kettle",
    "bamboo microscope",
    "bamboo telescope",
    "bamboo camera",
    "bamboo lighthouse",
    "bamboo windmill",
    "bamboo greenhouse",
    "bamboo workbench",
    "bamboo bookshelf",
    "bamboo door",
    "cotton bridge",
    "cotton teapot",
    "cotton lantern",
    "cotton compass",
    "cotton typewriter",
    "cotton hourglass",
    "cotton violin",
    "cotton backpack",
    "cotton notebook",
    "cotton watch",
    "cotton hammer",
    "cotton screwdriver",
    "cotton wrench",
    "cotton bicycle",
    "cotton helmet",
    "cotton kettle",
    "cotton microscope",
    "cotton telescope",
    "cotton camera",
    "cotton lighthouse",
    "cotton windmill",
    "cotton greenhouse",
    "cotton workbench",
    "cotton bookshelf",
    "cotton door",
    "wool bridge",
    "wool teapot",
    "wool lantern",
    "wool compass",
    "wool typewriter",
    "wool hourglass",
    "wool violin",
    "wool backpack",
    "wool notebook",
    "wool watch",
    "wool hammer",
    "wool screwdriver",
    "wool wrench",
    "wool bicycle",
    "wool helmet",
    "wool kettle",
    "wool microscope",
    "wool telescope",
    "wool camera",
    "wool lighthouse",
    "wool windmill",
    "wool greenhouse",
    "wool workbench",
    "wool bookshelf",
    "wool door",
    "leather bridge",
    "leather teapot",
    "leather lantern",
    "leather compass",
    "leather typewriter",
    "leather hourglass",
    "leather violin",
    "leather backpack",
    "leather notebook",
    "leather watch",
    "leather hammer",
    "leather screwdriver",
    "leather wrench",
    "leather bicycle",
    "leather helmet",
    "leather kettle",
    "leather microscope",
    "leather telescope",
    "leather camera",
    "leather lighthouse",
    "leather windmill",
    "leather greenhouse",
    "leather workbench",
    "leather bookshelf",
    "leather door",
    "glass bridge",
    "glass teapot",
    "glass lantern",
    "glass compass",
    "glass typewriter",
    "glass hourglass",
    "glass violin",
    "glass backpack",
    "glass notebook",
    "glass watch",
    "glass hammer",
    "glass screwdriver",
    "glass wrench",
    "glass bicycle",
    "glass helmet",
    "glass kettle",
    "glass microscope",
    "glass telescope",
    "glass camera",
    "glass lighthouse",
    "glass windmill",
    "glass greenhouse",
    "glass workbench",
    "glass bookshelf",
    "glass door",
    "ceramic bridge",
    "ceramic teapot",
    "ceramic lantern",
    "ceramic compass",
    "ceramic typewriter",
    "ceramic hourglass",
    "ceramic violin",
    "ceramic backpack",
    "ceramic notebook",
    "ceramic watch",
    "ceramic hammer",
    "ceramic screwdriver",
    "ceramic wrench",
    "ceramic bicycle",
    "ceramic helmet",
    "ceramic kettle",
    "ceramic microscope",
    "ceramic telescope",
    "ceramic camera",
    "ceramic lighthouse",
    "ceramic windmill",
    "ceramic greenhouse",
    "ceramic workbench",
    "ceramic bookshelf",
    "ceramic door",
    "porcelain bridge",
    "porcelain teapot",
    "porcelain lantern",
    "porcelain compass",
    "porcelain typewriter",
    "porcelain hourglass",
    "porcelain violin",
    "porcelain backpack",
    "porcelain notebook",
    "porcelain watch",
    "porcelain hammer",
    "porcelain screwdriver",
    "porcelain wrench",
    "porcelain bicycle",
    "porcelain helmet",
    "porcelain kettle",
    "porcelain microscope",
    "porcelain telescope",
    "porcelain camera",
    "porcelain lighthouse",
    "porcelain windmill",
    "porcelain greenhouse",
    "porcelain workbench",
    "porcelain bookshelf",
    "porcelain door",
    "rubber bridge",
    "rubber teapot",
    "rubber lantern",
    "rubber compass",
    "rubber typewriter",
    "rubber hourglass",
    "rubber violin",
    "rubber backpack",
    "rubber notebook",
    "rubber watch",
    "rubber hammer",
    "rubber screwdriver",
    "rubber wrench",
    "rubber bicycle",
    "rubber helmet",
    "rubber kettle",
    "rubber microscope",
    "rubber telescope",
    "rubber camera",
    "rubber lighthouse",
    "rubber windmill",
    "rubber greenhouse",
    "rubber workbench",
    "rubber bookshelf",
    "rubber door",
    "silk bridge",
    "silk teapot",
    "silk lantern",
    "silk compass",
    "silk typewriter",
    "silk hourglass",
    "silk violin",
    "silk backpack",
    "silk notebook",
    "silk watch",
    "silk hammer",
    "silk screwdriver",
    "silk wrench",
    "silk bicycle",
    "silk helmet",
    "silk kettle",
    "silk microscope",
    "silk telescope",
    "silk camera",
    "silk lighthouse",
    "silk windmill",
    "silk greenhouse",
    "silk workbench",
    "silk bookshelf",
    "silk door",
    "linen bridge",
    "linen teapot",
    "linen lantern",
    "linen compass",
    "linen typewriter",
    "linen hourglass",
    "linen violin",
    "linen backpack",
    "linen notebook",
    "linen watch",
    "linen hammer",
    "linen screwdriver",
    "linen wrench",
    "linen bicycle",
    "linen helmet",
    "linen kettle",
    "linen microscope",
    "linen telescope",
    "linen camera",
    "linen lighthouse",
    "linen windmill",
    "linen greenhouse",
    "linen workbench",
    "linen bookshelf",
    "linen door",
    "paper bridge",
    "paper teapot",
    "paper lantern",
    "paper compass",
    "paper typewriter",
    "paper hourglass",
    "paper violin",
    "paper backpack",
    "paper notebook",
    "paper watch",
    "paper hammer",
    "paper screwdriver",
    "paper wrench",
    "paper bicycle",
    "paper helmet",
    "paper kettle",
    "paper microscope",
    "paper telescope",
    "paper camera",
    "paper lighthouse",
    "paper windmill",
    "paper greenhouse",
    "paper workbench",
    "paper bookshelf",
    "paper door",
    "clay bridge",
    "clay teapot",
    "clay lantern",
    "clay compass",
    "clay typewriter",
    "clay hourglass",
    "clay violin",
    "clay backpack",
    "clay notebook",
    "clay watch",
    "clay hammer",
    "clay screwdriver",
    "clay wrench",
    "clay bicycle",
    "clay helmet",
    "clay kettle",
    "clay microscope",
    "clay telescope",
    "clay camera",
    "clay lighthouse",
    "clay windmill",
    "clay greenhouse",
    "clay workbench",
    "clay bookshelf",
    "clay door",
    "terracotta bridge",
    "terracotta teapot",
    "terracotta lantern",
    "terracotta compass",
    "terracotta typewriter",
    "terracotta hourglass",
    "terracotta violin",
    "terracotta backpack",
    "terracotta notebook",
    "terracotta watch",
    "terracotta hammer",
    "terracotta screwdriver",
    "terracotta wrench",
    "terracotta bicycle",
    "terracotta helmet",
    "terracotta kettle",
    "terracotta microscope",
    "terracotta telescope",
    "terracotta camera",
    "terracotta lighthouse",
    "terracotta windmill",
    "terracotta greenhouse",
    "terracotta workbench",
    "terracotta bookshelf",
    "terracotta door",
    "carbon bridge",
    "carbon teapot",
    "carbon lantern",
    "carbon compass",
    "carbon typewriter",
    "carbon hourglass",
    "carbon violin",
    "carbon backpack",
    "carbon notebook",
    "carbon watch",
    "carbon hammer",
    "carbon screwdriver",
    "carbon wrench",
    "carbon bicycle",
    "carbon helmet",
    "carbon kettle",
    "carbon microscope",
    "carbon telescope",
    "carbon camera",
    "carbon lighthouse",
    "carbon windmill",
    "carbon greenhouse",
    "carbon workbench",
    "carbon bookshelf",
    "carbon door",
];
