use reqwest::{Client, Error};
use serde::Deserialize;

// ─── Modelos ──────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, Clone)]
pub struct Organization {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Project {
    pub id: String,
    pub organization_id: String,
    pub name: String,
    pub region: String,
    pub status: String,
    pub created_at: String,
}

// ─── Cliente ──────────────────────────────────────────────────────────────────

#[derive(Clone)]
pub struct SupabaseClient {
    client: Client,
    token: String,
}

impl SupabaseClient {
    pub fn new(token: String) -> Self {
        Self {
            client: Client::new(),
            token,
        }
    }

    /// Lista todas las organizaciones del usuario autenticado.
    pub async fn get_organizations(&self) -> Result<Vec<Organization>, Error> {
        let response = self.client
            .get("https://api.supabase.com/v1/organizations")
            .bearer_auth(&self.token)
            .send()
            .await?;

        let response = response.error_for_status()?;
        let orgs = response.json::<Vec<Organization>>().await?;
        Ok(orgs)
    }

    /// Lista todos los proyectos del usuario autenticado.
    pub async fn get_projects(&self) -> Result<Vec<Project>, Error> {
        let response = self.client
            .get("https://api.supabase.com/v1/projects")
            .bearer_auth(&self.token)
            .send()
            .await?;

        let response = response.error_for_status()?;
        let projects = response.json::<Vec<Project>>().await?;
        Ok(projects)
    }

    /// Pausa un proyecto (solo disponible en Free Tier).
    pub async fn pause_project(&self, ref_id: &str) -> Result<(), String> {
        let url = format!("https://api.supabase.com/v1/projects/{}/pause", ref_id);
        let response = self.client
            .post(&url)
            .bearer_auth(&self.token)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        match response.status() {
            s if s.is_success() => Ok(()),
            reqwest::StatusCode::FORBIDDEN => Err(
                "403: Solo proyectos Free Tier pueden pausarse via API.".to_string()
            ),
            reqwest::StatusCode::UNAUTHORIZED => Err(
                "401: Personal Access Token invalido o expirado.".to_string()
            ),
            s => Err(format!("Error {}: No se pudo pausar el proyecto.", s)),
        }
    }

    /// Reanuda un proyecto previamente pausado.
    pub async fn resume_project(&self, ref_id: &str) -> Result<(), String> {
        let url = format!("https://api.supabase.com/v1/projects/{}/restore", ref_id);
        let response = self.client
            .post(&url)
            .bearer_auth(&self.token)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        match response.status() {
            s if s.is_success() => Ok(()),
            reqwest::StatusCode::FORBIDDEN => Err(
                "403: Sin permisos para reanudar este proyecto.".to_string()
            ),
            reqwest::StatusCode::UNAUTHORIZED => Err(
                "401: Personal Access Token invalido o expirado.".to_string()
            ),
            s => Err(format!("Error {}: No se pudo reanudar el proyecto.", s)),
        }
    }
}
