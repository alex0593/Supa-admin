use crate::api::{Project, Organization};
use ratatui::widgets::ListState;

#[derive(PartialEq)]
pub enum ActiveTab {
    Projects,
    Organizations,
}

pub enum AppEvent {
    Tick,
    ProjectsLoaded(Vec<Project>),
    OrganizationsLoaded(Vec<Organization>),
    RefreshProjects,
    Error(String),
}

pub struct App {
    pub active_tab: ActiveTab,
    pub projects: Vec<Project>,
    pub project_state: ListState,
    pub organizations: Vec<Organization>,
    pub org_state: ListState,
    pub is_loading: bool,
    pub error_msg: Option<String>,
    pub should_quit: bool,
    pub show_help: bool,
    pub show_project_details: bool,
    pub confirm_action: Option<(String, String)>, // (Acción, Project ID)
    pub tick: u8,                                 // Contador para animaciones
    pub action_in_progress: Option<String>,       // Descripción de acción en curso
}

impl App {
    pub fn new() -> Self {
        Self {
            active_tab: ActiveTab::Projects,
            projects: Vec::new(),
            project_state: ListState::default(),
            organizations: Vec::new(),
            org_state: ListState::default(),
            is_loading: true,
            error_msg: None,
            should_quit: false,
            show_help: false,
            show_project_details: false,
            confirm_action: None,
            tick: 0,
            action_in_progress: None,
        }
    }

    pub fn on_tick(&mut self) {
        self.tick = self.tick.wrapping_add(1);
    }

    /// Retorna una cadena de puntos animados basada en el tick actual.
    /// Ej: "." -> ".." -> "..." -> "." -> ...
    pub fn loading_dots(&self) -> &'static str {
        match (self.tick / 2) % 4 {
            0 => ".",
            1 => "..",
            2 => "...",
            _ => "",
        }
    }

    pub fn toggle_help(&mut self) {
        self.show_help = !self.show_help;
    }

    pub fn toggle_project_details(&mut self) {
        if self.active_tab == ActiveTab::Projects {
            self.show_project_details = !self.show_project_details;
        }
    }

    pub fn next_tab(&mut self) {
        self.active_tab = match self.active_tab {
            ActiveTab::Projects => ActiveTab::Organizations,
            ActiveTab::Organizations => ActiveTab::Projects,
        };
        self.show_project_details = false; // Close details if changing tabs
    }

    pub fn previous_tab(&mut self) {
        self.active_tab = match self.active_tab {
            ActiveTab::Projects => ActiveTab::Organizations,
            ActiveTab::Organizations => ActiveTab::Projects,
        };
        self.show_project_details = false;
    }

    pub fn next(&mut self) {
        match self.active_tab {
            ActiveTab::Projects => {
                let i = match self.project_state.selected() {
                    Some(i) => {
                        if i >= self.projects.len().saturating_sub(1) {
                            0
                        } else {
                            i + 1
                        }
                    }
                    None => 0,
                };
                self.project_state.select(Some(i));
            }
            ActiveTab::Organizations => {
                let i = match self.org_state.selected() {
                    Some(i) => {
                        if i >= self.organizations.len().saturating_sub(1) {
                            0
                        } else {
                            i + 1
                        }
                    }
                    None => 0,
                };
                self.org_state.select(Some(i));
            }
        }
    }

    pub fn previous(&mut self) {
        match self.active_tab {
            ActiveTab::Projects => {
                let i = match self.project_state.selected() {
                    Some(i) => {
                        if i == 0 {
                            self.projects.len().saturating_sub(1)
                        } else {
                            i - 1
                        }
                    }
                    None => 0,
                };
                self.project_state.select(Some(i));
            }
            ActiveTab::Organizations => {
                let i = match self.org_state.selected() {
                    Some(i) => {
                        if i == 0 {
                            self.organizations.len().saturating_sub(1)
                        } else {
                            i - 1
                        }
                    }
                    None => 0,
                };
                self.org_state.select(Some(i));
            }
        }
    }
}
