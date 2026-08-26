use crate::messages::failed;

pub enum Status {
    Planned,
    Active,
    Completed,
}

pub struct Quest {
    title: String,
    xp: u32,
    status: Status,
    note: Option<String>,
}

impl Quest {
    pub fn new(title: String, xp: u32, note: Option<String>, status: Status) -> Self {
        Self {
            title,
            xp,
            status,
            note,
        }
    }

    fn change_status(&mut self, status: Status) {
        self.status = status;
    }

    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn xp(&self) -> &u32 {
        &self.xp
    }

    pub fn status(&self) -> &Status {
        &self.status
    }

    pub fn note(&self) -> &Option<String> {
        &self.note
    }
}
pub fn status_to_string(status: &Status) -> String {
    match status {
        Status::Planned => "Planned".to_string(),
        Status::Active => "Active".to_string(),
        Status::Completed => "Completed".to_string(),
    }
}
pub fn note_to_string(note: &Option<String>) -> &str {
    match note {
        Some(note) => note,
        None => "Nothing.",
    }
}

pub fn string_to_status(status: String) -> Result<Status, &'static str> {
    let status = status.trim();
    if status.eq("Planned") {
        return Ok(Status::Planned);
    } else if status.eq("Active") {
        return Ok(Status::Active);
    } else if status.eq("Completed") {
        return Ok(Status::Completed);
    };
    Err(failed::UNKNOWN_STATUS_IN_DB_ERROR)
}

pub fn string_to_note(note: &str) -> Result<Option<String>, &'static str> {
    let note = note.trim().to_string();
    if !note.is_empty() {
        if note.eq("Nothing.") {
            return Ok(None);
        }
        return Ok(Some(note));
    };
    Err(failed::UNKNOWN_NOTE_IN_DB_ERROR)
}

pub fn print_list(quests: &[Quest]) {
    for i in 0..quests.len() {
        let quest = &quests[i];

        println!(
            "Q{} :- Status: {}, Title: {}, Reward: {}Xp, Note: {}",
            i + 1,
            status_to_string(&quest.status),
            quest.title,
            quest.xp,
            note_to_string(&quest.note)
        )
    }
}

pub fn summary(quests: &[Quest]) {
    let n_completed = quests
        .iter()
        .filter(|quest| matches!(quest.status, Status::Completed))
        .count();
    println!(
        "Total quests: {}, You completed: {}",
        quests.len(),
        n_completed
    )
}

pub mod validate {

    use crate::messages::failed;
    use crate::messages::success;
    use crate::messages::warning;
    use crate::quest::Status;
    use crate::quest::string_to_note;
    use crate::quest::string_to_status;

    use super::Quest;

    pub fn new(
        title: String,
        xp: String,
        note: String,
        status: Option<String>,
    ) -> Result<(Quest, &'static str), &'static str> {
        let f_title: String = title.trim().to_string();
        let mut message: &str = "";
        if f_title.is_empty() {
            return Err(failed::ADD_FMT_TITLE_ERROR);
        }
        let f_xp: u32 = match xp.trim().parse::<u32>() {
            Ok(xp) => xp,
            Err(_e) => return Err(failed::ADD_FMT_XP_PARSE_ERROR),
        };

        let note_trim = note.trim();
        let f_note: Option<String>;
        let mut f_status = Status::Planned;
        if !note_trim.is_empty() {
            match string_to_note(note_trim) {
                Ok(note) => f_note = note,
                Err(message) => {
                    println!("{}", message);
                    f_note = None
                }
            }
        } else {
            f_note = None;
            message = warning::ADD_FMT_NOTE_WARNING
        };

        if let Some(_status) = status {
            f_status = string_to_status(_status)?;
        }

        Ok((Quest::new(f_title, f_xp, f_note, f_status), message))
    }

    pub fn active(quest: &mut Quest) -> Result<&str, &str> {
        if matches!(quest.status, Status::Planned) {
            quest.change_status(Status::Active);
            return Ok(success::SUCCESS);
        } else if matches!(quest.status, Status::Active) {
            return Err(warning::ACTIVE_WARNING);
        };
        Err(failed::ACTIVE_ERROR)
    }

    pub fn complete(quest: &mut Quest) -> Result<&str, &str> {
        if matches!(quest.status, Status::Active) {
            quest.change_status(Status::Completed);
            return Ok("Sucess!");
        } else if matches!(quest.status, Status::Completed) {
            return Err(warning::COMPLETE_WARNING);
        };
        Err(failed::COMPLETE_ERROR)
    }
}
