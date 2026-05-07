use ratatui::{
    layout::{Constraint, Direction, Layout, Rect, Alignment},
    style::{Color, Modifier, Style},
    text::{Span, Line},
    widgets::{Block, Borders, BorderType, List, ListItem, Paragraph, Tabs, Clear},
    Frame,
};
use crate::app::{App, ActiveTab};
use chrono::DateTime;

// Colores de la paleta principal
const C_ACCENT: Color  = Color::Rgb(0, 200, 150);   // Verde esmeralda
const C_SELECT: Color  = Color::Rgb(30, 50, 80);     // Azul oscuro para selección
const C_SEL_FG: Color  = Color::Rgb(180, 230, 255);  // Texto seleccionado
const C_HEADER: Color  = Color::Rgb(0, 200, 150);    // Header acento
const C_BORDER: Color  = Color::Rgb(60, 80, 110);    // Bordes sutiles
const C_MUTED:  Color  = Color::Rgb(120, 140, 160);  // Texto secundario

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

fn status_badge(status: &str) -> (Span<'static>, Color) {
    match status {
        "ACTIVE_HEALTHY" => (
            Span::styled(" ● ACTIVO  ", Style::default().fg(Color::Black).bg(Color::Rgb(0, 180, 100)).add_modifier(Modifier::BOLD)),
            Color::Rgb(0, 180, 100),
        ),
        "ACTIVE" => (
            Span::styled(" ● ACTIVO  ", Style::default().fg(Color::Black).bg(Color::Rgb(0, 180, 100)).add_modifier(Modifier::BOLD)),
            Color::Rgb(0, 180, 100),
        ),
        "PAUSED" => (
            Span::styled(" ⏸ PAUSADO ", Style::default().fg(Color::White).bg(Color::Rgb(80, 80, 80)).add_modifier(Modifier::BOLD)),
            Color::Rgb(120, 120, 120),
        ),
        "INACTIVE" => (
            Span::styled(" ○ INACTIVO", Style::default().fg(Color::White).bg(Color::Rgb(100, 60, 0)).add_modifier(Modifier::BOLD)),
            Color::Rgb(200, 120, 0),
        ),
        other => (
            Span::styled(format!(" ? {:<8}", other), Style::default().fg(Color::White).bg(Color::Rgb(80, 40, 100)).add_modifier(Modifier::BOLD)),
            Color::Rgb(180, 100, 200),
        ),
    }
}

pub fn draw(f: &mut Frame, app: &mut App) {
    let area = f.area();

    // Layout principal: Header | Tabs | Contenido | Footer
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(area);

    // ══════════════════════════════════════════
    // HEADER
    // ══════════════════════════════════════════
    let header_line = Line::from(vec![
        Span::styled("  ◆ ", Style::default().fg(C_ACCENT)),
        Span::styled("SUPA", Style::default().fg(C_ACCENT).add_modifier(Modifier::BOLD)),
        Span::styled("-ADMIN", Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
        Span::styled("  │  ", Style::default().fg(C_MUTED)),
        Span::styled("Supabase Management CLI", Style::default().fg(C_MUTED)),
    ]);
    let header = Paragraph::new(header_line)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(C_ACCENT))
        );
    f.render_widget(header, chunks[0]);

    // ══════════════════════════════════════════
    // TABS
    // ══════════════════════════════════════════
    let tab_index = match app.active_tab {
        ActiveTab::Projects => 0,
        ActiveTab::Organizations => 1,
    };
    let titles = vec![
        Line::from(vec![
            Span::raw("  "),
            Span::styled("⬡", Style::default().fg(Color::Rgb(0, 180, 100))),
            Span::raw("  Proyectos  "),
        ]),
        Line::from(vec![
            Span::raw("  "),
            Span::styled("⬡", Style::default().fg(Color::Rgb(100, 160, 255))),
            Span::raw("  Organizaciones  "),
        ]),
    ];
    let tabs = Tabs::new(titles)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(C_BORDER))
        )
        .select(tab_index)
        .style(Style::default().fg(C_MUTED))
        .highlight_style(
            Style::default()
                .fg(C_HEADER)
                .add_modifier(Modifier::BOLD)
        );
    f.render_widget(tabs, chunks[1]);

    // ══════════════════════════════════════════
    // CONTENIDO
    // ══════════════════════════════════════════
    if app.is_loading {
        let dots = app.loading_dots();
        let spinner = ["⠋","⠙","⠹","⠸","⠼","⠴","⠦","⠧","⠇","⠏"];
        let spin = spinner[(app.tick as usize) % spinner.len()];
        let msg = Line::from(vec![
            Span::styled(format!("  {}  ", spin), Style::default().fg(C_ACCENT).add_modifier(Modifier::BOLD)),
            Span::styled(format!("Conectando con Supabase{}", dots), Style::default().fg(Color::White)),
        ]);
        let loading = Paragraph::new(msg)
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(C_BORDER))
            );
        f.render_widget(loading, chunks[2]);
    } else if let Some(err) = &app.error_msg {
        let error_view = Paragraph::new(vec![
            Line::from(""),
            Line::from(Span::styled("  ✖  Error", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))),
            Line::from(""),
            Line::from(Span::styled(format!("  {}", err), Style::default().fg(Color::White))),
        ])
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::Red))
        );
        f.render_widget(error_view, chunks[2]);
    } else {
        match app.active_tab {
            ActiveTab::Projects => {
                let items: Vec<ListItem> = app.projects
                    .iter()
                    .map(|p| {
                        let formatted_date = if let Ok(date) = DateTime::parse_from_rfc3339(&p.created_at) {
                            date.format("%d %b %Y").to_string()
                        } else {
                            p.created_at.chars().take(10).collect()
                        };
                        let (badge, _) = status_badge(&p.status);

                        // Línea principal del proyecto
                        let main_line = Line::from(vec![
                            Span::raw("  "),
                            Span::styled(
                                format!("{:<28}", p.name),
                                Style::default().fg(Color::White).add_modifier(Modifier::BOLD)
                            ),
                            Span::styled(" ╎ ", Style::default().fg(C_BORDER)),
                            Span::styled(
                                format!("{:<20}", p.region),
                                Style::default().fg(Color::Rgb(100, 160, 255))
                            ),
                            Span::styled(" ╎ ", Style::default().fg(C_BORDER)),
                            Span::styled(
                                format!("{:<12}", formatted_date),
                                Style::default().fg(C_MUTED)
                            ),
                            Span::styled(" ╎ ", Style::default().fg(C_BORDER)),
                            badge,
                        ]);

                        ListItem::new(main_line)
                    })
                    .collect();

                // Spinner en el título si hay acción en progreso
                let spinner = ["⠋","⠙","⠹","⠸","⠼","⠴","⠦","⠧","⠇","⠏"];
                let (projects_title_text, title_style) = if let Some(action_msg) = &app.action_in_progress {
                    let spin = spinner[(app.tick as usize) % spinner.len()];
                    (
                        format!(" {}  {}{}  ", spin, action_msg, app.loading_dots()),
                        Style::default().fg(Color::Rgb(255, 200, 0)).add_modifier(Modifier::BOLD),
                    )
                } else {
                    let count = app.projects.len();
                    (
                        format!(" ⬡  Proyectos  ({} total) ", count),
                        Style::default().fg(C_ACCENT).add_modifier(Modifier::BOLD),
                    )
                };

                let list = List::new(items)
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .border_type(BorderType::Rounded)
                            .border_style(Style::default().fg(C_BORDER))
                            .title(Span::styled(projects_title_text, title_style))
                    )
                    .highlight_style(
                        Style::default()
                            .bg(C_SELECT)
                            .fg(C_SEL_FG)
                            .add_modifier(Modifier::BOLD)
                    )
                    .highlight_symbol("▶ ");

                f.render_stateful_widget(list, chunks[2], &mut app.project_state);
            }
            ActiveTab::Organizations => {
                let items: Vec<ListItem> = app.organizations
                    .iter()
                    .map(|org| {
                        let content = Line::from(vec![
                            Span::raw("  "),
                            Span::styled(
                                format!("{:<35}", org.name),
                                Style::default().fg(Color::White).add_modifier(Modifier::BOLD)
                            ),
                            Span::styled(" ╎ ", Style::default().fg(C_BORDER)),
                            Span::styled("ID: ", Style::default().fg(C_MUTED)),
                            Span::styled(&org.id, Style::default().fg(Color::Rgb(100, 160, 255))),
                        ]);
                        ListItem::new(content)
                    })
                    .collect();

                let count = app.organizations.len();
                let list = List::new(items)
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .border_type(BorderType::Rounded)
                            .border_style(Style::default().fg(C_BORDER))
                            .title(Span::styled(
                                format!(" ⬡  Organizaciones  ({} total) ", count),
                                Style::default().fg(Color::Rgb(100, 160, 255)).add_modifier(Modifier::BOLD)
                            ))
                    )
                    .highlight_style(
                        Style::default()
                            .bg(C_SELECT)
                            .fg(C_SEL_FG)
                            .add_modifier(Modifier::BOLD)
                    )
                    .highlight_symbol("▶ ");

                f.render_stateful_widget(list, chunks[2], &mut app.org_state);
            }
        }
    }

    // ══════════════════════════════════════════
    // FOOTER
    // ══════════════════════════════════════════
    let keys: &[(&str, &str)] = if app.active_tab == ActiveTab::Projects {
        &[
            ("↑↓", "Nav"),
            ("Tab", "Pestaña"),
            ("Enter", "Detalles"),
            ("p", "Pausar"),
            ("r", "Reanudar"),
            ("?", "Ayuda"),
            ("q", "Salir"),
        ]
    } else {
        &[
            ("↑↓", "Nav"),
            ("Tab", "Pestaña"),
            ("?", "Ayuda"),
            ("q", "Salir"),
        ]
    };

    let mut footer_spans: Vec<Span> = vec![Span::raw("  ")];
    for (i, (key, label)) in keys.iter().enumerate() {
        if i > 0 {
            footer_spans.push(Span::styled("  │  ", Style::default().fg(C_BORDER)));
        }
        footer_spans.push(Span::styled(
            format!("[{}]", key),
            Style::default().fg(C_ACCENT).add_modifier(Modifier::BOLD)
        ));
        footer_spans.push(Span::styled(
            format!(" {}", label),
            Style::default().fg(C_MUTED)
        ));
    }

    let footer = Paragraph::new(Line::from(footer_spans))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(C_BORDER))
        );
    f.render_widget(footer, chunks[3]);

    // ══════════════════════════════════════════
    // MODALES
    // ══════════════════════════════════════════
    if app.show_help {
        let block = Block::default()
            .title(Span::styled(" ◆ Atajos de Teclado ", Style::default().fg(C_ACCENT).add_modifier(Modifier::BOLD)))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(C_ACCENT))
            .style(Style::default().bg(Color::Rgb(10, 15, 25)));

        let row = |key: &'static str, color: Color, desc: &'static str| -> Line<'static> {
            Line::from(vec![
                Span::raw("   "),
                Span::styled(format!("{:<12}", key), Style::default().fg(color).add_modifier(Modifier::BOLD)),
                Span::styled(desc, Style::default().fg(Color::Rgb(200, 210, 220))),
            ])
        };

        let help_text = vec![
            Line::from(""),
            row("q / Esc",   C_MUTED,              "Salir / Cerrar modal"),
            row("↑/↓  k/j",  C_ACCENT,             "Navegar en la lista"),
            row("←/→  Tab",  C_ACCENT,             "Cambiar de pestaña"),
            row("Enter",      Color::White,          "Ver detalles del proyecto"),
            row("c",          Color::White,          "Copiar cadena de conexión"),
            row("p",          Color::Rgb(255,80,80), "Pausar proyecto seleccionado"),
            row("r",          Color::Rgb(80,220,80), "Reanudar proyecto seleccionado"),
            row("?",          C_ACCENT,              "Mostrar / ocultar esta ayuda"),
            Line::from(""),
        ];

        let paragraph = Paragraph::new(help_text).block(block).alignment(Alignment::Left);
        let area = centered_rect(55, 55, f.area());
        f.render_widget(Clear, area);
        f.render_widget(paragraph, area);

    } else if let Some((action, ref_id)) = &app.confirm_action {
        let project_name = app.projects.iter()
            .find(|p| &p.id == ref_id)
            .map(|p| p.name.clone())
            .unwrap_or_else(|| ref_id.clone());

        let (action_label, action_color, action_icon, action_desc) = if action == "pause" {
            ("PAUSAR PROYECTO", Color::Rgb(220, 60, 60), "⏸", "El proyecto se detendrá. Los datos se conservan.")
        } else {
            ("REANUDAR PROYECTO", Color::Rgb(0, 180, 100), "▶", "El proyecto se activará nuevamente.")
        };

        let block = Block::default()
            .title(Span::styled(
                format!(" {} {} ", action_icon, action_label),
                Style::default().fg(action_color).add_modifier(Modifier::BOLD)
            ))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(action_color))
            .style(Style::default().bg(Color::Rgb(10, 15, 25)));

        let confirm_text = vec![
            Line::from(""),
            Line::from(vec![
                Span::styled("   Proyecto:  ", Style::default().fg(C_MUTED)),
                Span::styled(project_name, Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("   ", Style::default()),
                Span::styled(action_desc, Style::default().fg(C_MUTED)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("   ¿Confirmas la acción?", Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::raw("   "),
                Span::styled(
                    format!("  [Enter / s]  {}  ", action_label),
                    Style::default().fg(Color::Black).bg(action_color).add_modifier(Modifier::BOLD)
                ),
                Span::raw("   "),
                Span::styled(
                    "  [n / Esc]  Cancelar  ",
                    Style::default().fg(Color::White).bg(Color::Rgb(50, 55, 65))
                ),
            ]),
        ];

        let paragraph = Paragraph::new(confirm_text).block(block).alignment(Alignment::Left);
        let area = centered_rect(52, 48, f.area());
        f.render_widget(Clear, area);
        f.render_widget(paragraph, area);

    } else if app.show_project_details && app.active_tab == ActiveTab::Projects {
        if let Some(i) = app.project_state.selected() {
            if let Some(p) = app.projects.get(i) {
                let (badge, _) = status_badge(&p.status);
                let api_url = format!("https://{}.supabase.co", p.id);
                let conn_url = format!("postgresql://postgres.{}:[PASSWORD]@aws-0-{}.pooler.supabase.com:6543/postgres", p.id, p.region);

                let block = Block::default()
                    .title(Span::styled(
                        format!(" ◆ {} ", p.name),
                        Style::default().fg(C_ACCENT).add_modifier(Modifier::BOLD)
                    ))
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(C_ACCENT))
                    .style(Style::default().bg(Color::Rgb(10, 15, 25)));

                let lbl = |text: &'static str| Span::styled(
                    format!("   {:<18}", text),
                    Style::default().fg(C_MUTED)
                );

                let details_text = vec![
                    Line::from(""),
                    Line::from(vec![lbl("Estado"),    badge]),
                    Line::from(vec![lbl("ID"),         Span::styled(&p.id, Style::default().fg(Color::Rgb(100, 160, 255)))]),
                    Line::from(vec![lbl("Organización"), Span::styled(&p.organization_id, Style::default().fg(Color::White))]),
                    Line::from(vec![lbl("Región"),     Span::styled(&p.region, Style::default().fg(Color::White))]),
                    Line::from(""),
                    Line::from(vec![
                        Span::styled("   API URL", Style::default().fg(C_ACCENT).add_modifier(Modifier::BOLD)),
                    ]),
                    Line::from(vec![
                        Span::raw("   "),
                        Span::styled(&api_url, Style::default().fg(Color::White)),
                    ]),
                    Line::from(""),
                    Line::from(vec![
                        Span::styled("   Cadena de Conexión", Style::default().fg(C_ACCENT).add_modifier(Modifier::BOLD)),
                    ]),
                    Line::from(vec![
                        Span::raw("   "),
                        Span::styled(&conn_url, Style::default().fg(Color::Rgb(200, 210, 180))),
                    ]),
                    Line::from(""),
                    Line::from(vec![
                        Span::raw("   "),
                        Span::styled("[c] Copiar conexión", Style::default().fg(C_ACCENT)),
                        Span::styled("   │   ", Style::default().fg(C_BORDER)),
                        Span::styled("[Esc] Cerrar", Style::default().fg(C_MUTED)),
                    ]),
                ];

                let paragraph = Paragraph::new(details_text).block(block).alignment(Alignment::Left);
                let area = centered_rect(72, 65, f.area());
                f.render_widget(Clear, area);
                f.render_widget(paragraph, area);
            }
        }
    }
}
