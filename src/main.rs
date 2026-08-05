#[derive(PartialEq)]
enum QuestStatus {
    Planned,
    Active,
    Completed,
}
struct Quest {
    title: String,
    reward_xp: u32,
    status: QuestStatus,
    note: Option<String>,
}
impl Quest {
    fn new(title: String, reward_xp: u32, note: Option<String>) -> Self {
        Self {
            title,
            reward_xp,
            status: QuestStatus::Planned,
            note,
        }
    }

    fn title(&self) -> &str {
        &self.title
    }

    fn active(&mut self) {
        self.status = QuestStatus::Active;
    }
    fn complete(&mut self) {
        self.status = QuestStatus::Completed;
    }
    fn _change_status(&mut self, status: QuestStatus) {
        self.status = status
    }
    fn into_reward_xp(self) -> u32 {
        println!("Consuming the quest...");
        self.reward_xp
    }

    fn print_note(&self) {
        if let Some(note) = &self.note {
            println!("Note: {}", note);
        }
    }
    fn print_status(&self) {
        match self.status {
            QuestStatus::Planned => println!("Will start soon."),
            QuestStatus::Active => println!("Ongoing."),
            QuestStatus::Completed => println!("done."),
        }
    }
}

fn sum_of_completed_xp(quests: &[Quest]) -> u32 {
    quests
        .iter()
        .filter(|quest| quest.status == QuestStatus::Completed)
        .map(|quest| quest.reward_xp)
        .sum::<u32>()
}

fn main() {
    let mut quests = vec![
        Quest::new(
            "Making Chatgpt From scratch".to_string(),
            1000_u32,
            Option::Some("Be careful and dont make mistake".to_string()),
        ),
        Quest::new(
            "Train Chat gpt".to_string(),
            100_000_000_u32,
            Some("Be nice with electricity bill".to_string()),
        ),
        Quest::new(
            "Ordering and Reciving Flood of Ram supply".to_string(),
            1000_u32,
            Some("HBM memories are importent Not DDR5".to_string()),
        ),
        Quest::new("Make Nvidia Grate again".to_string(), 100_000_000_u32, None),
    ];

    if let Some(quest) = quests.get_mut(2) {
        quest.active();
        quest.complete();
        quest.print_note();
        println!()
    }
    println!();
    quests.iter().for_each(|quest| {
        println!("Title: {}", quest.title());
        quest.print_status();
    });

    let sum = sum_of_completed_xp(&quests);

    println!("Sum of completed XP: {}Xp", sum);

    println!("proof that the quests are only borrowed..");

    for quest in quests.iter() {
        println!("Title: {}", quest.title)
    }

    let consuming_quest = Quest::new(
        "Chad gpt is comming".to_string(),
        1_000_000_000_u32,
        Some("Wait for it...".to_string()),
    );

    println!(
        "Consumed Quest but xp is here: {}xp",
        consuming_quest.into_reward_xp()
    );
    // println!("This not gonna work: {}", consuming_quest.title)
}
