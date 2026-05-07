mod api;
mod app;
mod config;
mod ui;

use anyhow::Result;
use app::{App, AppEvent};
use config::Config;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{
    io::{self, Write},
    time::Duration,
};
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> Result<()> {
    let mut config = Config::load()?;

    if config.supabase_token.is_empty() {
        println!("========================================");
        println!("    ¡Bienvenido a Supabase Admin CLI!   ");
        println!("========================================");
        println!("Para comenzar, necesitamos tu Personal Access Token de Supabase.");
        println!("Puedes generarlo en: https://supabase.com/dashboard/account/tokens\n");
        print!("Introduce tu Token: ");
        io::stdout().flush()?;

        let mut token = String::new();
        io::stdin().read_line(&mut token)?;
        
        config.supabase_token = token.trim().to_string();
        config.save()?;
        println!("\n¡Token guardado exitosamente!");
    }

    // Configurar terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let api_client = api::SupabaseClient::new(config.supabase_token);
    
    // Iniciar hilo para fetch API asíncrono (Proyectos)
    let (tx, mut rx) = mpsc::channel(32);
    let tx_clone = tx.clone();
    let api_client_clone1 = api_client.clone();
    
    tokio::spawn(async move {
        match api_client_clone1.get_projects().await {
            Ok(projects) => {
                let _ = tx_clone.send(AppEvent::ProjectsLoaded(projects)).await;
            }
            Err(e) => {
                let _ = tx_clone.send(AppEvent::Error(e.to_string())).await;
            }
        }
    });

    // Iniciar hilo para fetch API asíncrono (Organizaciones)
    let tx_clone_org = tx.clone();
    let api_client_clone2 = api_client.clone();
    
    tokio::spawn(async move {
        match api_client_clone2.get_organizations().await {
            Ok(orgs) => {
                let _ = tx_clone_org.send(AppEvent::OrganizationsLoaded(orgs)).await;
            }
            Err(e) => {
                let _ = tx_clone_org.send(AppEvent::Error(e.to_string())).await;
            }
        }
    });

    // Iniciar hilo de eventos tick
    let tx_tick = tx.clone();
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(Duration::from_millis(250)).await;
            if tx_tick.send(AppEvent::Tick).await.is_err() {
                break;
            }
        }
    });

    // Bucle principal
    loop {
        terminal.draw(|f| ui::draw(f, &mut app))?;

        if crossterm::event::poll(Duration::from_millis(10))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == event::KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => {
                            app.should_quit = true;
                        }
                        KeyCode::Esc => {
                            if app.show_help {
                                app.show_help = false;
                            } else if app.confirm_action.is_some() {
                                app.confirm_action = None;
                            } else if app.show_project_details {
                                app.show_project_details = false;
                            } else {
                                app.should_quit = true;
                            }
                        }
                        KeyCode::Char('?') => {
                            app.toggle_help();
                        }
                        KeyCode::Enter => {
                            if app.confirm_action.is_some() {
                                if let Some((action, ref_id)) = app.confirm_action.take() {
                                    let label = if action == "pause" {
                                        "Pausando proyecto".to_string()
                                    } else {
                                        "Reanudando proyecto".to_string()
                                    };
                                    app.action_in_progress = Some(label);
                                    let tx_action = tx.clone();
                                    let client = api_client.clone();
                                    let rid = ref_id.clone();
                                    tokio::spawn(async move {
                                        let result = if action == "pause" {
                                            client.pause_project(&rid).await
                                        } else {
                                            client.resume_project(&rid).await
                                        };
                                        match result {
                                            Ok(_) => {
                                                tokio::time::sleep(std::time::Duration::from_millis(1500)).await;
                                                let _ = tx_action.send(AppEvent::RefreshProjects).await;
                                            }
                                            Err(e) => {
                                                let _ = tx_action.send(AppEvent::Error(e.to_string())).await;
                                            }
                                        }
                                    });
                                }
                            } else {
                                app.toggle_project_details();
                            }
                        }
                        KeyCode::Char('c') => {
                            if app.show_project_details {
                                if let Some(i) = app.project_state.selected() {
                                    if let Some(p) = app.projects.get(i) {
                                        let conn_url = format!("postgresql://postgres.[{}]:[TU_CONTRASEÑA]@aws-0-{}.pooler.supabase.com:6543/postgres", p.id, p.region);
                                        if let Ok(mut clipboard) = arboard::Clipboard::new() {
                                            let _ = clipboard.set_text(conn_url);
                                        }
                                    }
                                }
                            }
                        }
                        // Confirmar acción (s/y)
                        KeyCode::Char('s') | KeyCode::Char('y') => {
                            if let Some((action, ref_id)) = app.confirm_action.take() {
                                let label = if action == "pause" {
                                    "Pausando proyecto".to_string()
                                } else {
                                    "Reanudando proyecto".to_string()
                                };
                                app.action_in_progress = Some(label);
                                let tx_action = tx.clone();
                                let client = api_client.clone();
                                let rid = ref_id.clone();
                                tokio::spawn(async move {
                                    let result = if action == "pause" {
                                        client.pause_project(&rid).await
                                    } else {
                                        client.resume_project(&rid).await
                                    };
                                    match result {
                                        Ok(_) => {
                                            tokio::time::sleep(std::time::Duration::from_millis(1500)).await;
                                            let _ = tx_action.send(AppEvent::RefreshProjects).await;
                                        }
                                        Err(e) => {
                                            let _ = tx_action.send(AppEvent::Error(e.to_string())).await;
                                        }
                                    }
                                });
                            }
                        }
                        // Cancelar acción (n)
                        KeyCode::Char('n') => {
                            if app.confirm_action.is_some() {
                                app.confirm_action = None;
                            }
                        }
                        // Pausar proyecto (p)
                        KeyCode::Char('p') => {
                            if app.active_tab == app::ActiveTab::Projects && app.confirm_action.is_none() {
                                if let Some(i) = app.project_state.selected() {
                                    if let Some(p) = app.projects.get(i) {
                                        app.confirm_action = Some(("pause".to_string(), p.id.clone()));
                                    }
                                }
                            }
                        }
                        // Reanudar proyecto (r)
                        KeyCode::Char('r') => {
                            if app.active_tab == app::ActiveTab::Projects && app.confirm_action.is_none() {
                                if let Some(i) = app.project_state.selected() {
                                    if let Some(p) = app.projects.get(i) {
                                        app.confirm_action = Some(("resume".to_string(), p.id.clone()));
                                    }
                                }
                            }
                        }
                        KeyCode::Down | KeyCode::Char('j') => {
                            app.next();
                        }
                        KeyCode::Up | KeyCode::Char('k') => {
                            app.previous();
                        }
                        KeyCode::Right | KeyCode::Tab | KeyCode::Char('l') => {
                            app.next_tab();
                        }
                        KeyCode::Left | KeyCode::BackTab | KeyCode::Char('h') => {
                            app.previous_tab();
                        }
                        _ => {}
                    }
                }
            }
        }

        while let Ok(event) = rx.try_recv() {
            match event {
                AppEvent::ProjectsLoaded(projects) => {
                    app.projects = projects;
                    app.is_loading = false;
                    if !app.projects.is_empty() {
                        app.project_state.select(Some(0));
                    }
                }
                AppEvent::OrganizationsLoaded(orgs) => {
                    app.organizations = orgs;
                    if !app.organizations.is_empty() {
                        app.org_state.select(Some(0));
                    }
                }
                AppEvent::RefreshProjects => {
                    app.action_in_progress = None; // Limpiar spinner
                    let tx_refresh = tx.clone();
                    let client_refresh = api_client.clone();
                    tokio::spawn(async move {
                        match client_refresh.get_projects().await {
                            Ok(projects) => {
                                let _ = tx_refresh.send(AppEvent::ProjectsLoaded(projects)).await;
                            }
                            Err(e) => {
                                let _ = tx_refresh.send(AppEvent::Error(e.to_string())).await;
                            }
                        }
                    });
                }
                AppEvent::Error(err) => {
                    app.error_msg = Some(err);
                    app.is_loading = false;
                    app.action_in_progress = None;
                }
                AppEvent::Tick => {
                    app.on_tick();
                }
            }
        }

        if app.should_quit {
            break;
        }
    }

    // Restaurar terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
