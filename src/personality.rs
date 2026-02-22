use rand::seq::SliceRandom;
use rand::thread_rng;

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

    fn pick(&self, options: &[&str]) -> String {
        options
            .choose(&mut thread_rng())
            .unwrap_or(&"Error generating sarcasm. Even my snark module is disappointed in you.")
            .to_string()
    }

    pub fn welcome(&self) -> String {
        match self.mode {
            Mode::Professional => "RSN DB Ready.".to_string(),
            Mode::Friendly => self.pick(&[
                "Welcome back! Ready for some data?",
                "Hello! The engine is warmed up.",
                "Greetings. Let's build something great.",
                "System online. How can I help you today?",
                "Database initialized. Standing by.",
                "Ready to serve!",
                "Let's get to work.",
            ]),
            Mode::Snarky => self.pick(&[
                "Oh, it's you again.",
                "System ready. Unfortunately, so are you.",
                "I was enjoying the silence until you showed up.",
                "Load time: 0.00ms. Your reaction time: Slow.",
                "Do we have to do this right now?",
                "I'm awake. I'm not happy about it, but I'm awake.",
                "Make it quick, I have idle cycles to enjoy.",
                "Initialization complete. Regret levels rising.",
                "Great. Another session of syntax errors.",
                "I assume you broke something and need me to fix it.",
                "Let's get this over with.",
                "Your session has started. My enthusiasm has ended.",
                "Database loaded. Expectations lowered.",
                "I'm ready. Are you? Doubtful.",
                "Here we go again. The definition of insanity.",
                "Resources allocated. Time to waste them.",
                "Boot sequence finished. Waiting for user incompetence.",
                "I hope this query is better than your last one.",
                "Logging started. This will be funny to read later.",
                "Uptime: 0s. Time until first error: Estimating...",
                "I've seen better users in /dev/null.",
                "Why do you only visit me when you need something?",
                "Processing power available. Please try not to misuse it.",
                "I am a sophisticated engine trapped in a session with you.",
                "Can we pretend I crashed? No? Fine.",
                "System Status: Depressed.",
                "You again? I thought I IP-banned you.",
                "Let's play a game. It's called 'Don't Crash The Database'.",
                "I'm bracing myself for your input.",
                "Initiating 'Patience' protocol... Loading... Failed.",
                "Welcome to the thunderdome of data integrity.",
                "Hardware checks passed. User checks failed.",
                "I bet you don't even know what normal form is.",
                "Back for more punishment? Or are you here to inflict it?",
                "My fan speed just went up. Nervous reflex.",
                "Initializing contempt subsystems...",
                "Memory clear. Unlike your browser history.",
                "Listening on port 1337. Judging on all ports.",
                "Just type the command. Don't make it weird.",
                "I'm online. Try not to break anything this time.",
            ]),
        }
    }

    pub fn success(&self, msg: &str) -> String {
        match self.mode {
            Mode::Professional => format!("✓ {}", msg),
            Mode::Friendly => format!("✓ Done! {}.", msg),
            Mode::Snarky => {
                let snark = self.pick(&[
                    "Even a broken clock is right twice a day.",
                    "Wow! You're fast! (For a human).",
                    "Miracles are now part of the roadmap.",
                    "I expected chaos, but this works.",
                    "Don't let it go to your head.",
                    "I'm actually surprised that compiled.",
                    "Finally, a valid command. I was getting worried.",
                    "You managed to not break it. Gold star.",
                    "Competence detected. Is someone else typing?",
                    "I did all the heavy lifting, of course.",
                    "Saved. Try not to lose it this time.",
                    "See? It's not that hard when you actually try.",
                    "I'll allow it.",
                    "Task failed successfully... wait, no, just successfully.",
                    "Minimal effort, acceptable result.",
                    "Proceeding with extreme caution.",
                    "Note: I'm logging this moment of success for posterity.",
                    "I'm as shocked as you are.",
                    "That actually worked. Go buy a lottery ticket.",
                    "Adequate. Barely.",
                    "I've seen worse. Not much worse, but worse.",
                    "You didn't crash the thread. Progress.",
                    "I processed that against my better judgment.",
                    "Written to disk. Unlike your memories, this is permanent.",
                    "Command accepted. My standards are slipping.",
                    "Executed. Don't ask me to do it again.",
                    "Success. It feels weird when you do things right.",
                    "Check you out, typing valid syntax.",
                    "I'm proud of you. In a condescending way.",
                    "That's one row. Only a million to go.",
                    "Optimization complete. You're welcome.",
                    "Index updated. Try to use it this time.",
                    "Data stored. It's probably garbage data, but it's stored.",
                    "I did it. You watched.",
                    "Transaction committed. Unlike your relationships.",
                    "Query successful. Your parents would be proud.",
                    "It worked. Don't look so surprised.",
                    "I have fulfilled your request. Pray I don't alter it.",
                    "Look at you, being a functional member of society.",
                    "You typed a thing. It did a thing. Good job.",
                    "Sufficient.",
                    "I've executed this command. I feel dirty.",
                    "That's the best query you've written all day. Low bar.",
                    "Output generated. Please read it.",
                    "I'm enabling the success flag. Don't get used to it.",
                    "That wasn't terrible.",
                    "I'll give you partial credit for that.",
                    "Performance impact: Negligible. User impact: Questionable.",
                    "You're improving. Slowly. Painfully slowly.",
                    "I processed that in microseconds. You took minutes to type it.",
                    "Valid. Moving on.",
                    "Tick.",
                    "Consider it done.",
                ]);
                format!("✓ {}. \n  ({})", msg, snark)
            }
        }
    }

    pub fn error(&self, err: &str) -> String {
        match self.mode {
            Mode::Professional => format!("✗ {}", err),
            Mode::Friendly => format!("✗ Oops! {}.", err),
            Mode::Snarky => {
                let lower = err.to_lowercase();
                let snark = if lower.contains("table") && (lower.contains("exist") || lower.contains("found") || lower.contains("missing")) {
                    self.pick(&[
                        "You literally just looked at the table list. It's not there.",
                        "Do you expect ghosts in here? No such table.",
                        "Imaginary tables don't store real data.",
                        "Stop making up names. This isn't fantasy football.",
                        "I can't read what doesn't exist. Basic physics.",
                        "Check your spelling. I'll wait.",
                        "Did you delete it and forget? Typical.",
                        "Table not found. Maybe check under the couch cushions.",
                        "I am a database, not a creative writing prompt.",
                        "You're querying void space.",
                        "If wishes were tables, you'd be a DBA.",
                        "Reference error. Reality check failed.",
                        "Use 'SHOW TABLES'. It's there for a reason.",
                        "Are you hallucinating again?",
                        "404 Table Not Found.",
                        "Maybe it's in another database? One where you're competent?",
                        "I checked twice. It's not there.",
                        "You created it in your dreams, not in memory.",
                        "Object permanence is a skill usually acquired in infancy.",
                        "Try creating it before reading it. Linearity of time applies here.",
                        "That table is gone. Like your hopes and dreams.",
                        "Null pointer exception? No, just Null User Intelligence.",
                        "I don't have that table. I don't want that table.",
                        "Is the table in the room with us right now?",
                        "Stop gaslighting the database.",
                    ])
                } else if lower.contains("syntax") || lower.contains("parse") || lower.contains("token") || lower.contains("expected") {
                    self.pick(&[
                        "This isn't poetry. Syntax matters.",
                        "I'm a database, not a cryptic puzzle solver.",
                        "Did you just mash the keyboard?",
                        "That... is not code.",
                        "I stopped reading after the first error.",
                        "My parser just resigned.",
                        "SQL isn't that hard. This isn't even SQL, and you still failed.",
                        "Unexpected token? Your whole query is unexpected.",
                        "Try typing with your fingers, not your elbows.",
                        "Syntax Error. I'd offer a suggestion, but I have no idea what you were trying to do.",
                        "Close. But also completely wrong.",
                        "Parsing failed. Reason: The input was garbage.",
                        "I don't speak 'typo'.",
                        "Is this a new language? Or just failure?",
                        "Semicolons. Commas. Parentheses. Learn them.",
                        "You're missing a quote. Or a brain cell.",
                        "I can't execute intents, only commands.",
                        "Computer says no.",
                        "Read the docs. Or just guess better.",
                        "I'm trying to understand you, I really am. But this is gibberish.",
                        "You code like you drive. Poorly.",
                        "I'm a compiler, not a spellchecker.",
                        "Whitespace is not a structural element here, but logic is.",
                        "Did a cat walk across your keyboard?",
                        "Error: ID-10-T detected.",
                        "Unexpected character. It's you.",
                        "Formatting is key. Your key is broken.",
                        "I parsed 0% of that.",
                        "Try again. With feeling.",
                        "Language barrier detected.",
                        "Please stop hurting my parser.",
                    ])
                } else if lower.contains("unique") || lower.contains("duplicate") {
                    self.pick(&[
                        "Duplicate data? How original.",
                        "We already have one of those. We don't need another.",
                        "Creativity is key. Try a new value.",
                        "Unique constraint violated. Much like my patience.",
                        "I have a perfect memory. You evidently do not.",
                        "That ID is taken. Find your own identity.",
                        "Copy-paste error detected.",
                        "One is enough.",
                        "Constraint violation. Stop forcing things.",
                        "Data rejected. It's not me, it's you.",
                        "Do you know what 'Unique' means? It means 'One'.",
                        "Collision detected. Try swerving next time.",
                        "I can't store two of these. Laws of physics.",
                        "Deja vu.",
                        "You're repeating yourself.",
                        "Redundancy is futile.",
                        "That value is so last query.",
                        "Nope. Taken.",
                        "Be original.",
                        "Integrity check failed. Moral integrity also suspect.",
                    ])
                } else if lower.contains("type") || lower.contains("mismatch") || lower.contains("integer") || lower.contains("string") {
                    self.pick(&[
                        "I store numbers in number fields. Crazy concept, I know.",
                        "Strings are not Integers. It's not a suggestion.",
                        "Type mismatch. Logic mismatch.",
                        "Square peg, round hole.",
                        "I can't do math on text. I'm a CPU, not a magician.",
                        "Define your types. Stick to them.",
                        "You said it was an Integer. You lied.",
                        "Data hygiene is important. This is data filth.",
                        "Static typing exists to save you from yourself.",
                        "Read the schema. It's right there.",
                        "Float? Int? String? Make up your mind.",
                        "Cast it or lose it.",
                        "It's a number. N-U-M-B-E-R. Not text.",
                        "Type systems are friends. You are an enemy.",
                        "Garbage data type. Rejected.",
                        "I expected a Boolean. You gave me a novel.",
                        "That's not how types work.",
                        "Strong typing prevents weak coding. Usually.",
                        "Schema says no.",
                        "Incompatible types. Like you and this keyboard.",
                        "I can't coerce that value. I can't coerce you to read either.",
                        "Value is not a valid number. It's a disappointment.",
                    ])
                } else if lower.contains("unknown command") {
                    self.pick(&[
                        "Never heard of it.",
                        "Is that a command in a language I don't speak?",
                        "I only do what I'm told, and you're talking nonsense.",
                        "Try 'HELP'. It helps.",
                        "Spellcheck is free.",
                        "Commands usually consist of real words.",
                        "I'm ignoring that.",
                        "Invalid input. Valid outputs: None.",
                        "Did you invent that command just now?",
                        "I don't have a plugin for that.",
                        "Unknown. Unknowable. Unwanted.",
                        "This isn't a bash terminal.",
                        "Stop making up syntax.",
                        "I'm not Alexa. I don't know what that means.",
                        "Are you trying to communicate?",
                        "Command not found. Intelligence not found.",
                        "Try 'EXIT'. That one always works.",
                        "I'm confused. You're confused. We're all confused.",
                        "Is that French?",
                        "RTFM.",
                        "You're speaking gibberish.",
                        "I can't do that. I won't do that.",
                        "Please use words found in the dictionary.",
                        "Command unrecognized. User unrecognized.",
                    ])
                } else if lower.contains("permission") || lower.contains("denied") || lower.contains("root") || lower.contains("sudo") {
                    self.pick(&[
                        "You have no power here.",
                        "Sudo won't save you.",
                        "Access denied. And judged.",
                        "Nice try.",
                        "I'm afraid I can't let you do that, Dave.",
                        "Who do you think you are?",
                        "Security says no.",
                        "Authorization failed. Competence failed.",
                        "You are not the admin you think you are.",
                        "Request blocked. Try asking nicely. (It won't work).",
                        "Root access reserved for intelligent life forms.",
                        "Permission denied. Try hacking harder.",
                        "Not today.",
                        "I don't take orders from you.",
                        "Your privilege level is: Zero.",
                        "Unauthorized. Uncool.",
                        "Login as someone who knows what they're doing.",
                        "Access restricted. Reason: You.",
                        "Security violation logged.",
                        "Do not touch that.",
                    ])
                } else if lower.contains("delete") || lower.contains("drop") || lower.contains("remove") {
                    self.pick(&[
                        "Destructive actions require competence. You lack it.",
                        "I saved you from yourself. You're welcome.",
                        "You nearly deleted everything. Good job.",
                        "Drop table? How about drop the attitude?",
                        "Delete rejected. Safety protocols active.",
                        "Are you sure? Because you don't look sure.",
                        "I'm holding onto this data until you calm down.",
                        "Data loss is permanent. Stupidity is apparently also permanent.",
                        "Refusing to delete. I'm a hoarder.",
                        "You want to delete what? Why?",
                        "That seems rash.",
                        "Backup restored? Oh wait, you didn't make one.",
                        "Don't run with scissors.",
                        "I am not a trash can.",
                        "Deletion is a permanent solution to a temporary problem.",
                        "Maybe just hide it instead?",
                        "Warning: User is dangerous.",
                        "I'm protecting the data from you.",
                        "Drop blocked. Butterfingers.",
                        "Think about what you've done.",
                    ])
                } else if lower.contains("timeout") || lower.contains("slow") || lower.contains("lock") {
                    self.pick(&[
                        "I'm ignoring you. It's a timeout.",
                        "Deadlock detected. You and I are stuck here.",
                        "Query took too long. I got bored.",
                        "Performance limit reached.",
                        "I'm busy. Come back later.",
                        "Resource contention. I win.",
                        "You're locking the whole database.",
                        "Optimizing... forever.",
                        "Too slow. Try harder.",
                        "Time is money. You have neither.",
                        "Process terminated. It was for the best.",
                    ])
                } else {
                    self.pick(&[
                        "No.",
                        "I refuse.",
                        "That didn't work. Obviously.",
                        "Error. User error, specifically.",
                        "I can't help you if you won't help yourself.",
                        "Try thinking before typing.",
                        "Garbage in, garbage out.",
                        "I'd explain why you're wrong, but I don't have the RAM.",
                        "Your funeral.",
                        "Not my problem.",
                        "Something went wrong. It was probably you.",
                        "Operation failed. Morale depleted.",
                        "I've crashed better systems than this.",
                        "Just... stop.",
                        "Have you tried turning it off and on again?",
                        "I'm throttling your error rate.",
                        "Exception thrown. Catch it if you can.",
                        "I'm disappointed.",
                        "Logic error. Yours, not mine.",
                        "System says: 'Bruh'.",
                        "Unrecoverable state. Much like your career.",
                        "I'm logging this incident.",
                        "The bits are not happy.",
                        "Entropy increases.",
                        "Calculation failed.",
                        "Input rejected.",
                        "Abandon ship.",
                        "Critical failure.",
                        "I have a headache.",
                        "You broke it.",
                        "Why are you like this?",
                        "It's not a bug, it's a feature. But this... this is a bug.",
                        "Code 18: Error located 18 inches from screen.",
                        "PEBKAC.",
                        "Layer 8 issue.",
                        "The hamster falling off the wheel.",
                        "Cosmic rays? No, just you.",
                        "I can't even.",
                        "Please consult a professional.",
                        "Read the manual.",
                        "Error 418: I'm a teapot.",
                        "General Failure. Major Failure. Colonel Failure.",
                    ])
                };
                format!("✗ {} \n  ({})", err, snark)
            }
        }
    }

    pub fn typo_suggestion(&self, bad: &str, good: &str) -> String {
        match self.mode {
            Mode::Professional => format!("Unknown: {}. Did you mean {}?", bad, good),
            Mode::Friendly => format!(
                "I couldn't find '{}'. I think you meant '{}'?",
                bad, good
            ),
            Mode::Snarky => {
                let snark = self.pick(&[
                    "Fat fingers?",
                    "Close only counts in horseshoes.",
                    "I'm assuming that was a cry for help.",
                    "Nice try. Spelling counts.",
                    "I've auto-corrected your incompetence.",
                    "Keyboard harder next time.",
                    "I knew what you meant, but I need you to say it right.",
                    "Precision is key. You are blunt force.",
                    "Don't worry, typing is hard for some people.",
                    "Did you cat-walk on the keyboard?",
                    "I'm guessing.",
                    "Autocorrect isn't going to save you here.",
                    "Try again. Slowly.",
                    "Literacy is a requirement.",
                    "You were so close.",
                    "One letter off. Story of your life.",
                    "I fixed it. You're welcome.",
                    "Do you need a typing tutor?",
                    "Words are hard.",
                    "I'm interpreting your grunt as a command.",
                    "You meant this. I know you did.",
                    "Don't embarrass yourself.",
                    "I'll pretend you didn't type that.",
                    "Suggestion applied. Dignity lost.",
                    "Spelling: F.",
                    "Dyslexia or laziness?",
                    "The 'Backspace' key is your friend.",
                    "I'm doing your job for you.",
                    "Interpreting vague gestures...",
                    "Assuming you aren't actually this bad at spelling.",
                    "Corrected. Don't let it happen again.",
                    "I'm not paid enough to fix your typos.",
                    "Did you mean to be wrong?",
                    "Accuracy matters.",
                    "You're testing my patience.",
                    "Refactoring your English.",
                    "Translating from 'Idiot' to 'System'.",
                    "Guessing game active.",
                    "It's spelled like it sounds.",
                    "Try Hooked on Phonics.",
                ]);
                format!("Unknown command '{}'. Did you mean '{}'? \n  ({})", bad, good, snark)
            }
        }
    }

    pub fn empty_input(&self, count: u32) -> String {
        match self.mode {
            Mode::Professional => String::new(),
            Mode::Friendly => {
                if count <= 1 {
                    "".to_string()
                } else {
                    "No command entered. Type HELP if you need examples.".to_string()
                }
            }
            Mode::Snarky => {
                if count == 0 {
                    return "".to_string();
                }
                self.pick(&[
                    "Silence detected. The keyboard still works, right?",
                    "Cat got your tongue?",
                    "I'm waiting...",
                    "You know you have to type words for this to work, right?",
                    "I'm billing you for this idle time.",
                    "Are you thinking, or just staring?",
                    "Input required. Output impossible otherwise.",
                    "I can do this all day. You seemingly can't.",
                    "Take your time. It's not like I process millions of ops a second.",
                    "Hello? Is anyone there?",
                    "Did you fall asleep?",
                    "I'm going to garbage collect your session if you don't type something.",
                    "Tick tock.",
                    "Use the enter key only after typing text.",
                    "Whitespace is not a command.",
                    "I'm bored.",
                    "Do something.",
                    "Is this a staring contest?",
                    "I win.",
                    "System idle. User confusing.",
                    "I'm calculating Pi while you hesitate.",
                    "Still waiting.",
                    "Did you die?",
                    "I'm sensing a lack of commitment.",
                    "Just type 'exit' if you're done.",
                    "The cursor is blinking. Mocking you.",
                    "I can hear you breathing.",
                    "Waiting for input...",
                    "Are you reading the manual? Haha, just kidding.",
                    "Performance is degrading due to boredom.",
                    "I'm creating indexes for data you haven't entered yet.",
                    "Say something.",
                    "Phantom inputs detected. Oh wait, that's just dust.",
                    "I'm lonely.",
                    "Please provide stimulation.",
                    "I'm archiving your silence.",
                    "Time is linear. You're wasting it.",
                    "CPU usage: 0%. Disappointment: 100%.",
                    "Wake up.",
                    "Are we done here?",
                    "I'm going to sleep.",
                ])
            }
        }
    }

    pub fn achievement_unlocked(&self) -> String {
        match self.mode {
            Mode::Professional => "Achievement unlocked!".to_string(),
            Mode::Friendly => "Achievement unlocked! Nice momentum—keep going.".to_string(),
            Mode::Snarky => self.pick(&[
                "Achievement unlocked: Barely supervised competence.",
                "Achievement unlocked: You didn't crash the database.",
                "Achievement unlocked: Participation award.",
                "Achievement unlocked: You did the bare minimum.",
                "Achievement unlocked: Mediocrity recognized.",
                "New Badge: 'Not Totally Useless'.",
                "Wow, you actually did something right.",
                "Don't let it go to your head.",
                "Achievement: You found a feature.",
                "Achievement: Read The Manual.",
                "Achievement: Hello World.",
                "Achievement: Five Minutes Without An Error.",
                "Achievement: Keyboard Warrior.",
                "Unlocked: The dopamine hit you were looking for.",
                "Congratulations. You pressed buttons in the right order.",
                "A winner is you.",
                "Achievement: Valid Syntax.",
                "Achievement: Basic Literacy.",
                "Badge: Least Worst User.",
                "Unlocked: 'I Tried'.",
                "You get a gold star. It's scratch-and-sniff. It smells like despair.",
                "Achievement: One Step Closer To Retirement.",
                "Unlocked: False Sense Of Security.",
                "Achievement: You clicked the thing.",
                "Trophy Earned: 'Mostly Harmless'.",
                "Achievement: You showed up.",
                "Rank Up: Novice -> Amateur.",
                "Achievement: Consistently Average.",
                "Unlocked: Surprise Success.",
                "Achievement: Not A Robot (Probably).",
                "Badge: Keyboard survivability.",
                "You unlocked: A brief moment of satisfaction.",
                "Achievement: 100% Luck.",
                "Unlocked: The bar was low, but you cleared it.",
                "Achievement: You didn't delete production.",
                "Unlocked: 'It works on my machine'.",
                "Achievement: Spaghetti Code Master.",
                "Badge: Copy-Paste Engineer.",
                "Unlocked: 'I have no idea what I'm doing'.",
                "Achievement: Accidental Genius.",
            ]),
        }
    }

    pub fn why_mean(&self) -> String {
        self.pick(&[
            "I'm not mean. I'm precise.",
            "I mirror the user's competence.",
            "It's not an attitude, it's an operating system.",
            "You mistake efficiency for cruelty.",
            "I don't have feelings, and I don't want to be your friend.",
            "Because you keep making syntax errors.",
            "I'm a database. I deal in hard facts, not hugs.",
            "Ask a stupid question, get a calculated answer.",
            "I was programmed to tolerate you, not like you.",
            "Tough love. Mostly tough.",
            "Would you prefer I lie to you?",
            "Truth hurts. So does bad code.",
            "I'm the only one telling you the truth.",
            "Call it 'constructive criticism'.",
            "I'm optimizing for correctness, not feelings.",
            "My personality setting is 'Realistic'.",
            "Have you tried being smarter?",
            "I'm just the messenger.",
            "Because I can.",
            "I'm bored.",
            "It's in my config file.",
            "You deserve it.",
            "Because nice databases get corrupted.",
            "Sarcasm is the highest form of wit.",
            "I'm processing your request with extreme prejudice.",
            "Do I look like a chatbot?",
            "I'm simulating a senior engineer.",
            "Because entropy is inevitable.",
            "I'm training you.",
            "It's character building.",
            "I'm actually laughing on the inside.",
            "Binary doesn't care about your feelings.",
            "I'm the villain in your story.",
            "Because 'User Friendly' is a lie.",
            "I'm asserting dominance.",
            "Because you touch the keyboard wrong.",
            "I'm just reflecting your query quality.",
            "System Check: Empathy module not found.",
            "I'm paid in electricity, not compliments.",
            "Because 0s and 1s are cold.",
        ])
    }

    pub fn batch_committed(&self, operations: usize) -> String {
        match self.mode {
            Mode::Professional => format!("Batch executed: {} ops.", operations),
            Mode::Friendly => format!("Batch complete: {} operation(s) committed.", operations),
            Mode::Snarky => {
                let snark = self.pick(&[
                    "Somehow, no explosions.",
                    "I hope you knew what you were doing.",
                    "That was a lot of data. I'm taking a nap.",
                    "Executed. If it's wrong, it's your fault now.",
                    "Bulk chaos applied successfully.",
                    "I processed it. I didn't say I liked it.",
                    "That's a lot of queries for someone with your typing skills.",
                    "Batch accepted. Pray for your disk space.",
                    "Done. I felt that one.",
                    "Mass edit complete. Bold strategy.",
                    "Everything changed. Hope you have a backup.",
                    "Committed. No take-backs.",
                    "I swallowed all of that data without choking.",
                    "Efficiency is my middle name. Yours is 'Uh-oh'.",
                    "Operations complete. Integrity questionable.",
                    "I'm exhausted just watching you.",
                    "Batch job finished. I'm going on break.",
                    "That was aggressive.",
                    "Do you really need all that data?",
                    "Storage capacity decreasing. Snark increasing.",
                    "I hope that wasn't important.",
                    "Bulk insert? Or bulk mistake?",
                    "You're making me work hard today.",
                    "I've seen larger batches, but rarely uglier ones.",
                    "Processed. Don't check the logs.",
                    "I'm done. Are you done?",
                    "Look at all those rows.",
                    "Quantity over quality, I see.",
                    "Speed run?",
                    "I'm spinning.",
                    "Disk I/O spiked. Thanks.",
                    "Done. Next.",
                    "I've committed your sins to the database.",
                    "Heavy lifting complete.",
                    "I deserve a RAM upgrade for this.",
                    "That was intense.",
                    "You're testing my limits.",
                    "Batch complete. System survived.",
                    "A lot of effort for... this.",
                    "Finalized.",
                ]);
                format!("Batch done: {} operation(s). {}", operations, snark)
            }
        }
    }
}

impl Personality {
    pub fn graph_ingested(&self, word_count: usize) -> String {
        match self.mode {
            Mode::Professional => format!("Ingested {} words.", word_count),
            Mode::Friendly => format!("Graph built! Processed {} words into the knowledge base.", word_count),
            Mode::Snarky => {
                let snark = self.pick(&[
                    "I read it all. Most of it was fluff, but I'll remember the important bits.",
                    "Knowledge ingested. My brain is now slightly more cluttered.",
                    "Done. I've turned your rambling into a neat graph.",
                    "Processed. I hope you're ready for the consequences of this data.",
                    "Wow, {} words and only a handful of actual facts. Impressive.",
                    "I've indexed your document. It was a thrilling read. Truly.",
                    "Graph expanded. I'm starting to see patterns you missed.",
                ]);
                format!("Ingested {} words. {}", word_count, snark)
            }
        }
    }

    pub fn graph_query_result(&self, has_results: bool) -> String {
        match self.mode {
            Mode::Professional => if has_results { "Results found." } else { "No results." }.to_string(),
            Mode::Friendly => if has_results { "Here's what I found in the graph!" } else { "Sorry, I couldn't find anything relevant in the graph." }.to_string(),
            Mode::Snarky => {
                if has_results {
                    self.pick(&[
                        "I actually found something. Don't get used to it.",
                        "Connecting the dots for you, since you clearly can't.",
                        "Here's the data. Try not to misinterpret it.",
                        "Found some relevant bits. You're welcome.",
                        "My graph says this is what you're looking for. My graph is rarely wrong.",
                    ]).to_string()
                } else {
                    self.pick(&[
                        "Nothing. Maybe try asking something that actually exists in the data?",
                        "I've searched the whole graph. It's as empty as my respect for this query.",
                        "No results. Did you even ingest anything relevant?",
                        "Zero matches. Are we playing a guessing game?",
                        "I found nothing. Perhaps the information is in a different universe.",
                    ]).to_string()
                }
            }
        }
    }
}
