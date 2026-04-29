use ::std::path::PathBuf;

#[derive(Clone, Copy)]
pub enum DeletePromptSelection {
    No,
    Yes,
    DontAskAgain,
}

pub struct UiEffects {
    pub flash_space_freed: bool,
    pub current_path_is_red: bool,
    pub deletion_in_progress: bool,
    pub delete_prompt_selection: DeletePromptSelection,
    pub loading_progress_indicator: u64,
    pub last_read_path: Option<PathBuf>,
    pub status_message: Option<String>,
}

impl UiEffects {
    pub fn new() -> Self {
        Self {
            flash_space_freed: false,
            current_path_is_red: false,
            deletion_in_progress: false,
            delete_prompt_selection: DeletePromptSelection::No,
            loading_progress_indicator: 0,
            last_read_path: None,
            status_message: None,
        }
    }
    pub fn increment_loading_progress_indicator(&mut self) {
        // increasing and decreasing this number will increase
        // the scanning text animation speed
        self.loading_progress_indicator += 3;
    }
}
