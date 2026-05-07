use reqwest::{Client, Error};
use serde::Deserialize;

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

    pub async fn get_organizations(&self) -> Result<Vec<Organization>, Error> {
        let response = self.client
            .get("https://api.supabase.com/v1/organizations")
            .bearer_auth(&self.token)
            .send()
            .await?;
            
        // Validar si fue un error HTTP
        let response = response.error_for_status()?;
            
        let orgs = response.json::<Vec<Organization>>().await?;
        Ok(orgs)
    }

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

    pub async fn pause_project(&self, ref_id: &str) -> Result<(), Error> {
        let url = format!("https://api.supabase.com/v1/projects/{}/pause", ref_id);
        let response = self.client
            .post(&url)
            .bearer_auth(&self.token)
            .send()
            .await?;
            
        response.error_for_status()?;
        Ok(())
    }

    pub async fn resume_project(&self, ref_id: &str) -> Result<(), Error> {
        let url = format!("https://api.supabase.com/v1/projects/{}/restore", ref_id);
        let response = self.client
            .post(&url)
            .bearer_auth(&self.token)
            .send()
            .await?;
            
        response.error_for_status()?;
        Ok(())
    }
}
