use crate::api::{Project, Organization};
use ratatui::widgets::ListState;

// ─── Tipos de eventos del canal asíncrono ─────────────────────────────────────

pub enum AppEvent {
    /// Tick periódico para animar la UI (cada ~250ms).
    Tick,
    /// Proyectos cargados exitosamente desde la API.
    ProjectsLoaded(Vec<Project>),
    /// Organizaciones cargadas exitosamente desde la API.
    OrganizationsLoaded(Vec<Organization>),
    /// Solicita recargar la lista de proyectos.
    RefreshProjects,
    /// Error al comunicarse con la API.
    Error(String),
}

// ─── Pestaña activa ───────────────────────────────────────────────────────────

#[derive(PartialEq)]
pub enum ActiveTab {
    Projects,
    Organizations,
}

// ─── Estado global de la aplicación ─────────────────────────────────────────

pub struct App {
    // Navegación
    pub active_tab: ActiveTab,

    // Datos de proyectos
    pub projects: Vec<Project>,
    pub project_state: ListState,

    // Datos de organizaciones
    pub organizations: Vec<Organization>,
    pub org_state: ListState,

    // Estados de UI
    pub is_loading: bool,
    pub error_msg: Option<String>,
    pub should_quit: bool,
    pub show_help: bool,
    pub show_project_details: bool,

    // Confirmación de acción destructiva: (tipo: "pause"/"resume", project_id)
    pub confirm_action: Option<(String, String)>,

    // Animaciones
    pub tick: u8,
    pub action_in_progress: Option<String>,
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

    /// Avanza el contador de animación en cada tick.
    pub fn on_tick(&mut self) {
        self.tick = self.tick.wrapping_add(1);
    }

    /// Genera puntos de carga animados según el tick actual: `.` `..` `...` ``
    pub fn loading_dots(&self) -> &'static str {
        match (self.tick / 2) % 4 {
            0 => ".",
            1 => "..",
            2 => "...",
            _ => "",
        }
    }

    // ─── Navegación ────────────────────────────────────────────────────────────

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
            ActiveTab::Projects      => ActiveTab::Organizations,
            ActiveTab::Organizations => ActiveTab::Projects,
        };
        self.show_project_details = false;
    }

    pub fn previous_tab(&mut self) {
        // Con 2 pestañas, siguiente y anterior son lo mismo
        self.next_tab();
    }

    pub fn next(&mut self) {
        match self.active_tab {
            ActiveTab::Projects => {
                let len = self.projects.len();
                if len == 0 { return; }
                let i = self.project_state.selected()
                    .map(|i| if i >= len - 1 { 0 } else { i + 1 })
                    .unwrap_or(0);
                self.project_state.select(Some(i));
            }
            ActiveTab::Organizations => {
                let len = self.organizations.len();
                if len == 0 { return; }
                let i = self.org_state.selected()
                    .map(|i| if i >= len - 1 { 0 } else { i + 1 })
                    .unwrap_or(0);
                self.org_state.select(Some(i));
            }
        }
    }

    pub fn previous(&mut self) {
        match self.active_tab {
            ActiveTab::Projects => {
                let len = self.projects.len();
                if len == 0 { return; }
                let i = self.project_state.selected()
                    .map(|i| if i == 0 { len - 1 } else { i - 1 })
                    .unwrap_or(0);
                self.project_state.select(Some(i));
            }
            ActiveTab::Organizations => {
                let len = self.organizations.len();
                if len == 0 { return; }
                let i = self.org_state.selected()
                    .map(|i| if i == 0 { len - 1 } else { i - 1 })
                    .unwrap_or(0);
                self.org_state.select(Some(i));
            }
        }
    }
}
