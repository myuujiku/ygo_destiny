use once_cell::sync::OnceCell;

use super::*;

static CORE_BOOSTERS: OnceCell<Vec<String>> = OnceCell::new();

struct TaggedSet {
    name: String,
    tags: Vec<String>,
}

impl TaggedSet {
    fn new(name: String) -> Self {
        Self {
            name: name,
            tags: vec![],
        }
    }

    fn contains(&self, search_text: &str) -> bool {
        self.name.as_str().contains(search_text)
    }

    fn starts_with(&self, search_text: &str) -> bool {
        self.name.as_str().starts_with(search_text)
    }

    fn add_tag(&mut self, tag: &str) {
        self.tags.push(tag.to_string());
    }

    fn eval(&mut self, search_text: &str, eval_to: &str) -> bool {
        if self.contains(search_text) {
            self.add_tag(eval_to);
            return true;
        } else {
            return false;
        }
    }

    fn eval_default(&mut self, search_text: &str) -> bool {
        self.eval(search_text, search_text)
    }

    fn multi_eval(&mut self, args: Vec<(&str, Option<&str>)>) -> bool {
        args.iter().any(|&arg| {
            if arg.1.is_some() {
                self.eval(arg.0, arg.1.unwrap())
            } else {
                self.eval_default(arg.0)
            }
        })
    }

    fn find_at_start(&mut self, search_text: &str, eval_to: &str) -> bool {
        if self.starts_with(search_text) {
            self.add_tag(eval_to);
            return true;
        } else {
            return false;
        }
    }
}

fn get_core_boosters() -> Vec<String> {
    let mut core_boosters: Vec<String> = Vec::new();

    for url in [
        "https://yugipedia.com/wiki/Category:TCG_Core_Boosters",
        "https://yugipedia.com/wiki/Category:International_Core_Boosters",
    ] {
        let raw_html = get_response(url).unwrap();

        let document = scraper::Html::parse_document(raw_html.as_str());
        let selector = scraper::Selector::parse("div.mw-category li > a").unwrap();

        for element in document.select(&selector) {
            core_boosters.push(element.value().attr("title").unwrap().to_string());
        }
    }

    return core_boosters;
}

// Evaluate which tags a set should get...
pub fn eval_tags(name: String) -> Vec<String> {
    let mut tagset = TaggedSet::new(name);

    // Core Booster
    if CORE_BOOSTERS.get_or_init(|| {get_core_boosters()}).contains(&tagset.name) {
        tagset.add_tag("Core Booster");
        tagset.add_tag("Booster Pack");
    }

    // Tournament Packs
    let mut tp_found = false;
    for tp in ["Tournament", "Champion", "Turbo", "Astral"] {
        if tagset.find_at_start(tp, format!("{} Pack", tp).as_str()) {
            tp_found = true;
            break;
        }
    }

    if !tp_found && tagset.contains("OTS Tournament Pack") {
        tp_found = true;
        tagset.add_tag("OTS Pack");
        tagset.eval("(POR)", "Portugal");
    }

    if tp_found {
        tagset.add_tag("Tournament");
        tagset.add_tag("Booster Pack");
    }

    // Hidden Arsenal
    if tagset.eval_default("Hidden Arsenal") {
        tagset.add_tag("Booster Pack");
    }
    tagset.eval_default("Duel Terminal");

    // Other packs
    if tagset.multi_eval(vec![
        ("Duelist Pack", None),
        ("Legendary Duelists", None),
        ("Battle Pack", None),
        ("Battles of Legend", None),
    ]) {
        tagset.add_tag("Booster Pack");
    } else {
        tagset.multi_eval(vec![("Special Edition", None), ("Collector Box", None)]);
    }

    // Decks
    if tagset.eval_default("Deck") {
        tagset.multi_eval(vec![
            ("Structure Deck", None),
            ("Starter Deck", None),
            ("God Deck", Some("Starter Deck")),
        ]);
    }

    // Speed Duel
    if tagset.eval_default("Speed Duel") {
        tagset.eval("Box", "Collector Box");
    }

    // Tins
    if tagset.eval_default("Tin") {
        tagset.eval("Tins", "Collector Box");
    }
    tagset.eval_default("Mega Pack");

    // Gold Series
    if tagset.contains("Gold") {
        for t in ["Series", "Premium", "Maximum", "Gold Edition"] {
            if tagset.eval(t, "Gold Series") {
                break;
            }
        }
    }

    // Promos
    if tagset.eval("promotional", "Promotional Cards") {
        tagset.multi_eval(vec![
            ("Shonen Jump", None),
            ("Volume", Some("Manga")),
            ("WSJ Jump", Some("Shonen Jump")),
            ("Premiere!", Some("Premiere")),
            ("World Championship", Some("Video Game")),
            ("Tag Force", Some("Video Game")),
            ("Mattel", Some("Mattel Action Figure")),
            ("Power of Chaos", Some("Video Game")),
            ("McDonald's", None),
            ("5D's Duel Transer", Some("Video Game")),
            ("5D's Stardust Accelerator", Some("Video Game")),
            ("5D's Wheelie Breakers", Some("Video Game")),
            ("Capsule Monster Coliseum", Some("Video Game")),
            ("Dark Duel Stories", Some("Video Game")),
            ("Forbidden Memories", Some("Video Game")),
            ("GX Card Almanac", Some("Video Game")),
            ("GX Duel Academy", Some("Video Game")),
            ("GX Spirit Caller", Some("Video Game")),
            ("Legacy of the Duelist", Some("Video Game")),
            ("Nightmare Troubadour", Some("Video Game")),
            ("Reshef of Destruction", Some("Video Game")),
            ("Dawn of Destiny", Some("Video Game")),
            ("Duelists of the Roses", Some("Video Game")),
            ("Eternal Duelist Soul", Some("Video Game")),
            ("Falsebound Kingdom", Some("Video Game")),
            ("Sacred Cards", Some("Video Game")),
        ]);
    }
    if tagset.name.to_ascii_lowercase().as_str().contains("prize") {
        tagset.add_tag("Prize");
    }
    tagset.eval_default("Sneak Peak");
    tagset.eval_default("The Lost Art");

    if tagset.tags.is_empty() {
        tagset.add_tag("Misc.");
    } else {
        tagset.tags.sort_unstable();
    }

    tagset.tags
}
