# 🚀 Supa-Admin

> **Administra tus proyectos de Supabase directamente desde la terminal.**

Una herramienta TUI (Terminal User Interface) de alto rendimiento construida en **Rust** para gestionar tus proyectos y organizaciones de Supabase sin salir de tu consola.

![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange?style=flat-square&logo=rust)
![License](https://img.shields.io/badge/License-MIT-blue?style=flat-square)
![Platform](https://img.shields.io/badge/Platform-Windows%20%7C%20Linux%20%7C%20macOS-lightgrey?style=flat-square)

---

## ✨ Características

| Funcionalidad | Descripción |
|---|---|
| 📋 **Listado de Proyectos** | Visualiza todos tus proyectos con nombre, región, fecha y estado en tiempo real |
| 🏢 **Listado de Organizaciones** | Accede a todas tus organizaciones desde una pestaña dedicada |
| 🔍 **Vista de Detalles** | Panel con URL de API, cadena de conexión PostgreSQL y metadata del proyecto |
| 📋 **Portapapeles** | Copia la cadena de conexión con una sola tecla |
| ⏸️ **Pausar / ▶️ Reanudar** | Controla el estado de tus proyectos con confirmación interactiva |
| 🔄 **Recarga Automática** | Tras cada acción, la lista se actualiza automáticamente |
| ⚡ **Animaciones de Estado** | Spinner animado en el título del panel durante las operaciones en curso |
| ❓ **Menú de Ayuda** | Overlay de atajos de teclado disponible con `?` en cualquier momento |

---

## 🚀 Instalación

### Requisitos

- [Rust y Cargo](https://rustup.rs/) (1.70 o superior)
- Un **Personal Access Token** de Supabase → [Generar token](https://supabase.com/dashboard/account/tokens)

### Clonar y ejecutar

```bash
git clone https://github.com/TU_USUARIO/Supa-Admin.git
cd Supa-Admin
cargo run --release
```

### Primera ejecución

Al iniciar por primera vez, la aplicación te pedirá tu Personal Access Token de Supabase. Este se guarda de forma segura en el directorio de configuración de tu sistema operativo (`~/.config` en Linux/macOS, `%APPDATA%` en Windows) y no necesitas introducirlo de nuevo.

---

## ⌨️ Controles

| Tecla | Acción |
|---|---|
| `↑` / `k` | Navegar hacia arriba |
| `↓` / `j` | Navegar hacia abajo |
| `←` / `h` / `Shift+Tab` | Pestaña anterior |
| `→` / `l` / `Tab` | Pestaña siguiente |
| `Enter` | Ver detalles del proyecto seleccionado |
| `c` | Copiar cadena de conexión al portapapeles *(en panel de detalles)* |
| `p` | **Pausar** el proyecto seleccionado *(pide confirmación)* |
| `r` | **Reanudar** el proyecto seleccionado *(pide confirmación)* |
| `?` | Mostrar / ocultar menú de ayuda |
| `Esc` | Cerrar modal activo / Salir |
| `q` | Salir de la aplicación |

---

## 🛠️ Stack Tecnológico

| Librería | Uso |
|---|---|
| [Ratatui](https://ratatui.rs/) | Framework TUI para la interfaz de terminal |
| [Crossterm](https://github.com/crossterm-rs/crossterm) | Eventos de teclado y control de terminal multiplataforma |
| [Tokio](https://tokio.rs/) | Runtime asíncrono para peticiones concurrentes |
| [Reqwest](https://docs.rs/reqwest) | Cliente HTTP para la Management API de Supabase |
| [Serde](https://serde.rs/) | Serialización / deserialización de respuestas JSON |
| [Chrono](https://docs.rs/chrono) | Formateo de fechas legibles |
| [Arboard](https://docs.rs/arboard) | Soporte nativo de portapapeles (Windows, Linux, macOS) |
| [Directories](https://docs.rs/directories) | Rutas de configuración multiplataforma |

---

## 🚧 Roadmap

- [x] Pestañas de navegación (Proyectos / Organizaciones)
- [x] Listado de Organizaciones
- [x] Vista detallada del proyecto (URL API + cadena de conexión PostgreSQL)
- [x] Pausar y Reanudar proyectos con confirmación interactiva
- [x] Copiar cadena de conexión al portapapeles
- [x] Animaciones de estado (spinner en título del panel)
- [ ] Visualización de Logs en tiempo real
- [ ] Gestión de Secretos y Variables de Entorno
- [ ] Panel de Métricas (CPU, memoria, almacenamiento)
- [ ] Listado e interacción con Backups

---

## 📄 Licencia

Distribuido bajo la licencia **MIT**. Consulta el archivo `LICENSE` para más información.
