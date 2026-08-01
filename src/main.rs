#[derive(PartialEq)]
enum Status {
    NotStarted,
    InProgress,
    Completed,
    Failed,
}

struct Quest {
    title: String,
    description: String,
    reward_xp: u32,
    status: Status,
    tag: Option<String>,
}

impl Quest {
    fn new(
        title: String,
        description: String,
        reward_xp: u32,
        status: Status,
        tag: Option<String>,
    ) -> Self {
        Self {
            title,
            description,
            reward_xp,
            status,
            tag,
        }
    }

    fn describe(&self) {
        println!(".......Quest Description.......");
        println!("Title: {}", self.title);

        match self.status {
            Status::NotStarted => println!("NotStarted."),
            Status::InProgress => println!("InProgress."),
            Status::Completed => println!("Completed."),
            Status::Failed => println!("Failed."),
        }

        println!("XP: {}", self.reward_xp);

        println!("Tag: {}", self.tag_label());

        if self.description.len() >= 5 {
            println!("Preview: {}.......", &self.description[0..5]);
        };
        println!();
    }

    fn tag_label(&self) -> &str {
        if let Some(tag) = &self.tag {
            return tag;
        };
        "No tag"
    }
}

fn filter_by_status(status: Status, quests: &[Quest]) -> Vec<&Quest> {
    quests
        .iter()
        .filter(|quest| status == quest.status)
        .collect::<Vec<_>>()
}

fn sum_completed_xp(quests: &[Quest]) -> u32 {
    let completed_quests = filter_by_status(Status::Completed, quests);

    completed_quests
        .iter()
        .map(|quest| quest.reward_xp)
        .sum::<u32>()
}

fn main() {
    let quests = vec![
        Quest::new(
            "Mining".to_string(),
            "Start your first mining!".to_string(),
            100_u32,
            Status::NotStarted,
            Some("Congrats".to_string()),
        ),
        Quest::new(
            "Building".to_string(),
            "Finish your first Townhall upgrade!".to_string(),
            10000_u32,
            Status::InProgress,
            Some("Unlock new features.".to_string()),
        ),
        Quest::new(
            "Travaling".to_string(),
            "Finish your first journy!".to_string(),
            1000_u32,
            Status::Completed,
            Some("Find valueable things along thw way.".to_string()),
        ),
        Quest::new(
            "Attack".to_string(),
            "Attack the deamon".to_string(),
            1000_u32,
            Status::Failed,
            None,
        ),
        Quest::new(
            "Login".to_string(),
            "Login Everyday".to_string(),
            10_u32,
            Status::Completed,
            None,
        ),
    ];

    for quest in quests.iter() {
        quest.describe();
    }

    let completed_quests = filter_by_status(Status::Completed, &quests);

    let sum_xp_of_completed = sum_completed_xp(&quests);

    println!("Total quests: {}", quests.len());
    println!();
    println!("Completed Only.........");
    completed_quests.iter().for_each(|quest| quest.describe());
    println!("Total completed XP: {}", sum_xp_of_completed);
}
