// ok i dont know much about this use create, mod,const , static, other things releated to this so can you explain or add this in future learning
pub mod success {
    pub const SUCCESS: &str = "Opration success.";
    // pub const ADD_FMT_QUEST: &str = "Quest created Sucessfully.";
    pub const WROTE_SUCCESS: &str = "Quest Wrote Successfully.";
}

pub mod failed {
    pub const ADD_FMT_TITLE_ERROR: &str = "Quest title is Mandatory.";
    //  pub const ADD_FMT_XP_ERROR: &str = "Xp is Mandatory.";
    pub const ADD_FMT_XP_PARSE_ERROR: &str = "Xp can be only numbers.";
    pub const ACTIVE_ERROR: &str = "Only 'Planned' Quest can be activated.";
    pub const COMPLETE_ERROR: &str = "Only 'Active' quest can be completed.";
    pub const UNKNOWN_STATUS_IN_DB_ERROR: &str = "Unknown Status in DB.";
    pub const UNKNOWN_NOTE_IN_DB_ERROR: &str = "Unknown Note in DB.";
}

pub mod warning {
    pub const ADD_FMT_NOTE_WARNING: &str = "Note is Missing But ist Optional.";
    pub const ACTIVE_WARNING: &str = "Quest is Already 'Active'.";
    pub const COMPLETE_WARNING: &str = "Quet is Already 'Completed'.";
}
