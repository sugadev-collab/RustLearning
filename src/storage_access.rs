use crate::messages::success;
use crate::quest::{self, Quest, Status};

fn to_string(status: &Status, xp: &u32, title: &str, note: &Option<String>) -> String {
    let status = quest::status_to_string(&status);
    let note = quest::note_to_string(&note);
    format!("{}|{}|{}|{}", status, xp, title, note)
}
pub fn write(quests: Vec<Quest>, path: &str) -> Result<&'static str, std::io::Error> {
    let mut fmt_str = vec![];
    for quest in quests.into_iter() {
        fmt_str.push(to_string(
            quest.status(),
            quest.xp(),
            quest.title(),
            quest.note(),
        ));
    }

    let fmt_data = fmt_str.join("\n");

    let _ok = std::fs::write(path, fmt_data)?;
    Ok(success::WROTE_SUCCESS)
}

pub fn read(path: &str) -> Result<Vec<Result<QuestStr, &str>>, std::io::Error> {
    let data = std::fs::read_to_string(path)?;

    let quests = data
        .lines()
        .map(|quest| {
            let vec_qust = quest.split("|").collect::<Vec<&str>>();

            map_to_struct(vec_qust)
        })
        .collect();
    Ok(quests)
}

fn map_to_struct(quest: Vec<&str>) -> Result<QuestStr, &'static str> {
    let mut quest_str = QuestStr {
        status: Some("".to_string()),
        xp: "".to_string(),
        title: "".to_string(),
        note: "".to_string(),
    };

    if let Some(status) = quest.get(0) {
        quest_str.status = Some(status.to_string());
    } else {
        return Err("Could not get 'Status' from db.");
    };
    if let Some(xp) = quest.get(1) {
        quest_str.xp = xp.to_string();
    } else {
        return Err("Could not get 'Xp' from db.");
    };
    if let Some(title) = quest.get(2) {
        quest_str.title = title.to_string();
    } else {
        return Err("Could not get 'title' from db.");
    };
    if let Some(note) = quest.get(3) {
        quest_str.note = note.to_string();
    } else {
        return Err("Could not get 'note' from db.");
    };

    Ok(quest_str)
}

pub struct QuestStr {
    pub status: Option<String>,
    pub xp: String,
    pub title: String,
    pub note: String,
}
