use ratatui::{
    layout::{Constraint, Direction, Layout, Rect, Alignment},
    style::{Color, Modifier, Style},
    text::{Span, Line},
    widgets::{Block, Borders, BorderType, List, ListItem, Paragraph, Tabs, Clear},
    Frame,
};
use crate::app::{App, ActiveTab};
use chrono::DateTime;

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

pub fn draw(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Length(3), // Tabs
            Constraint::Min(0),    // Content
            Constraint::Length(3), // Footer
        ])
        .split(f.area());

    // Header
    let header = Paragraph::new(Line::from(vec![
        Span::styled(" Supabase Admin CLI ", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
    ]))
    .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded));
    f.render_widget(header, chunks[0]);

    // Tabs
    let titles = vec![
        Line::from(" Proyectos "),
        Line::from(" Organizaciones "),
    ];
    let tab_index = match app.active_tab {
        ActiveTab::Projects => 0,
        ActiveTab::Organizations => 1,
    };
    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded).title(" Vistas "))
        .select(tab_index)
        .style(Style::default().fg(Color::Cyan))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD).fg(Color::Yellow));
    f.render_widget(tabs, chunks[1]);

    // Content
    if app.is_loading {
        let msg = format!("Cargando datos desde Supabase{}", app.loading_dots());
        let loading = Paragraph::new(msg)
            .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded).title(" Cargando "));
        f.render_widget(loading, chunks[2]);
    } else if let Some(err) = &app.error_msg {
        let error_view = Paragraph::new(format!("Error: {}", err))
            .style(Style::default().fg(Color::Red))
            .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded).title(" Error "));
        f.render_widget(error_view, chunks[2]);
    } else {
        match app.active_tab {
            ActiveTab::Projects => {
                let items: Vec<ListItem> = app.projects
                    .iter()
                    .map(|p| {
                        let status_color = match p.status.as_str() {
                            "ACTIVE_HEALTHY" | "ACTIVE" => Color::Green,
                            "PAUSED" => Color::DarkGray,
                            _ => Color::Yellow,
                        };
                        let formatted_date = if let Ok(date) = DateTime::parse_from_rfc3339(&p.created_at) {
                            date.format("%d %b %Y").to_string()
                        } else {
                            p.created_at.clone()
                        };
                        let content = Line::from(vec![
                            Span::styled(format!("{:<25}", p.name), Style::default().add_modifier(Modifier::BOLD)),
                            Span::raw(" | "),
                            Span::styled(format!("{:<15}", p.region), Style::default().fg(Color::LightBlue)),
                            Span::raw(" | "),
                            Span::styled(format!("{:<12}", formatted_date), Style::default().fg(Color::Gray)),
                            Span::raw(" | "),
                            Span::styled(&p.status, Style::default().fg(status_color)),
                        ]);
                        ListItem::new(content)
                    })
                    .collect();

                let spinner = ["○", "◔", "◑", "◗"];
                let projects_title = if let Some(action_msg) = &app.action_in_progress {
                    let spin = spinner[(app.tick as usize / 2) % 4];
                    format!(" {} {}{}  ", spin, action_msg, app.loading_dots())
                } else {
                    " Proyectos (Enter: detalles  |  p: pausar  |  r: reanudar) ".to_string()
                };

                let title_style = if app.action_in_progress.is_some() {
                    Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                };

                let list = List::new(items)
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .border_type(BorderType::Rounded)
                            .title(Span::styled(projects_title, title_style))
                    )
                    .highlight_style(Style::default().bg(Color::DarkGray).add_modifier(Modifier::BOLD))
                    .highlight_symbol("▶ ");

                f.render_stateful_widget(list, chunks[2], &mut app.project_state);
            }
            ActiveTab::Organizations => {
                let items: Vec<ListItem> = app.organizations
                    .iter()
                    .map(|org| {
                        let content = Line::from(vec![
                            Span::styled(format!("{:<30}", org.name), Style::default().add_modifier(Modifier::BOLD)),
                            Span::raw(" | ID: "),
                            Span::styled(&org.id, Style::default().fg(Color::Gray)),
                        ]);
                        ListItem::new(content)
                    })
                    .collect();

                let list = List::new(items)
                    .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded).title(" Organizaciones "))
                    .highlight_style(Style::default().bg(Color::DarkGray).add_modifier(Modifier::BOLD))
                    .highlight_symbol("▶ ");

                f.render_stateful_widget(list, chunks[2], &mut app.org_state);
            }
        }
    }

    // Footer
    let footer_text = if app.active_tab == ActiveTab::Projects {
        " [q]: Salir | [↑/↓]: Nav | [Tab/←/→]: Tab | [?]: Ayuda | [Enter]: Detalles | [p]: Pausar | [r]: Reanudar "
    } else {
        " [q]: Salir | [↑/↓]: Navegar | [Tab/←/→]: Pestaña | [?]: Ayuda "
    };
    let footer = Paragraph::new(footer_text)
        .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded));
    f.render_widget(footer, chunks[3]);

    // Modals
    if app.show_help {
        let block = Block::default().title(" Ayuda ").borders(Borders::ALL).border_type(BorderType::Rounded).style(Style::default().bg(Color::Black));
        let help_text = vec![
            Line::from(Span::styled("Atajos de Teclado", Style::default().add_modifier(Modifier::BOLD).add_modifier(Modifier::UNDERLINED))),
            Line::from(""),
            Line::from(vec![Span::styled("q", Style::default().fg(Color::Yellow)), Span::raw("       : Salir de la aplicación")]),
            Line::from(vec![Span::styled("Esc", Style::default().fg(Color::Yellow)), Span::raw("     : Cerrar ventanas modales / Salir")]),
            Line::from(vec![Span::styled("↑/↓ / k/j", Style::default().fg(Color::Yellow)), Span::raw(": Navegar en las listas")]),
            Line::from(vec![Span::styled("←/→ / Tab", Style::default().fg(Color::Yellow)), Span::raw(": Cambiar de pestaña")]),
            Line::from(vec![Span::styled("Enter", Style::default().fg(Color::Yellow)), Span::raw("   : Ver detalles del proyecto seleccionado")]),
            Line::from(vec![Span::styled("c", Style::default().fg(Color::Yellow)), Span::raw("       : Copiar cadena de conexión (solo en Detalles)")]),
            Line::from(vec![Span::styled("p", Style::default().fg(Color::Red)), Span::raw("       : Pausar (apagar) el proyecto seleccionado")]),
            Line::from(vec![Span::styled("r", Style::default().fg(Color::Green)), Span::raw("       : Reanudar (encender) el proyecto seleccionado")]),
            Line::from(vec![Span::styled("?", Style::default().fg(Color::Yellow)), Span::raw("       : Alternar esta ventana de ayuda")]),
        ];
        let paragraph = Paragraph::new(help_text).block(block).alignment(Alignment::Left);
        let area = centered_rect(60, 55, f.area());
        f.render_widget(Clear, area);
        f.render_widget(paragraph, area);
    } else if let Some((action, ref_id)) = &app.confirm_action {
        let project_name = app.projects.iter()
            .find(|p| &p.id == ref_id)
            .map(|p| p.name.as_str())
            .unwrap_or(ref_id.as_str());

        let (action_label, action_color, action_desc) = if action == "pause" {
            ("PAUSAR (APAGAR)", Color::Red, "El proyecto se detendrá. Los datos se conservan.")
        } else {
            ("REANUDAR (ENCENDER)", Color::Green, "El proyecto se activará nuevamente.")
        };

        let block = Block::default()
            .title(" Confirmación Requerida ")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .style(Style::default().bg(Color::Black));

        let confirm_text = vec![
            Line::from(""),
            Line::from(vec![
                Span::raw("Acción: "),
                Span::styled(action_label, Style::default().fg(action_color).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::raw("Proyecto: "),
                Span::styled(project_name, Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(""),
            Line::from(Span::styled(action_desc, Style::default().fg(Color::Gray))),
            Line::from(""),
            Line::from(Span::styled("¿Deseas continuar?", Style::default().add_modifier(Modifier::BOLD))),
            Line::from(""),
            Line::from(vec![
                Span::styled("  [s/y] Confirmar  ", Style::default().fg(Color::Black).bg(action_color).add_modifier(Modifier::BOLD)),
                Span::raw("   "),
                Span::styled("  [n / Esc] Cancelar  ", Style::default().fg(Color::White).bg(Color::DarkGray)),
            ]),
        ];

        let paragraph = Paragraph::new(confirm_text).block(block).alignment(Alignment::Center);
        let area = centered_rect(50, 45, f.area());
        f.render_widget(Clear, area);
        f.render_widget(paragraph, area);
    } else if app.show_project_details && app.active_tab == ActiveTab::Projects {
        if let Some(i) = app.project_state.selected() {
            if let Some(p) = app.projects.get(i) {
                let block = Block::default().title(" Detalles del Proyecto ").borders(Borders::ALL).border_type(BorderType::Rounded).style(Style::default().bg(Color::Black));

                let api_url = format!("https://{}.supabase.co", p.id);
                let conn_url = format!("postgresql://postgres.[{}]:[TU_CONTRASEÑA]@aws-0-{}.pooler.supabase.com:6543/postgres", p.id, p.region);

                let details_text = vec![
                    Line::from(vec![Span::styled("Nombre: ", Style::default().fg(Color::Cyan)), Span::raw(&p.name)]),
                    Line::from(vec![Span::styled("ID: ", Style::default().fg(Color::Cyan)), Span::raw(&p.id)]),
                    Line::from(vec![Span::styled("Organización ID: ", Style::default().fg(Color::Cyan)), Span::raw(&p.organization_id)]),
                    Line::from(vec![Span::styled("Región: ", Style::default().fg(Color::Cyan)), Span::raw(&p.region)]),
                    Line::from(vec![Span::styled("Estado: ", Style::default().fg(Color::Cyan)), Span::raw(&p.status)]),
                    Line::from(""),
                    Line::from(Span::styled("API URL:", Style::default().fg(Color::Green))),
                    Line::from(api_url),
                    Line::from(""),
                    Line::from(Span::styled("Conexión a Base de Datos:", Style::default().fg(Color::Green))),
                    Line::from(conn_url),
                    Line::from(""),
                    Line::from(Span::styled("[c] Copiar cadena de conexión  |  [Esc] Cerrar", Style::default().fg(Color::DarkGray))),
                ];

                let paragraph = Paragraph::new(details_text).block(block).alignment(Alignment::Left);
                let area = centered_rect(70, 60, f.area());
                f.render_widget(Clear, area);
                f.render_widget(paragraph, area);
            }
        }
    }
}

