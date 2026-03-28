use chrono::Local;
use crossterm::{
    event::{self, Event, KeyCode, poll},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use rand::RngExt;
use ratatui::{
    Frame, Terminal,
    backend::CrosstermBackend,
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols::Marker,
    text::{Line, Span},
    widgets::{Axis, Block, BorderType, Borders, Chart, Dataset, Gauge, Paragraph, Row, Table, Widget},
};
use std::{
    collections::VecDeque,
    io::{self, stdout},
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};
use sysinfo::{Disks, Networks, System};

// ── Tokyo Night Storm ─────────────────────────────────────────
const BASE: Color = Color::Rgb(36, 40, 59);
const SKY: Color = Color::Rgb(125, 207, 255);
const SUBTEXT0: Color = Color::Rgb(169, 177, 214);
const SUBTEXT1: Color = Color::Rgb(192, 202, 245);
const OVERLAY0: Color = Color::Rgb(115, 130, 175);
const SURFACE0: Color = Color::Rgb(52, 56, 80);
const SURFACE1: Color = Color::Rgb(65, 72, 104);
const GREEN: Color = Color::Rgb(158, 206, 106);
const YELLOW: Color = Color::Rgb(224, 175, 104);
const RED: Color = Color::Rgb(247, 118, 142);
const MAUVE: Color = Color::Rgb(187, 154, 247);
const TEAL: Color = Color::Rgb(115, 218, 202);
const PEACH: Color = Color::Rgb(255, 158, 100);
const BLUE: Color = Color::Rgb(122, 162, 247);

const GLITCH_CHARS: &[char] = &[
    '░','▒','▓','█','╳','◈','◆','▪','⬡','⊘','⊗','∆','∇','∞','≈','≠',
    '⌬','⏣','ᚠ','ᚢ','ᚦ','ᚨ','Ⱥ','Ɇ','₭','Ħ','Ǥ','€','Ø','Ɏ','Ⱬ','Đ',
];
const ART_NOAH: &[&str] = &[
    " ███╗   ██╗  ██████╗   █████╗  ██╗  ██╗",
    " ████╗  ██║ ██╔═══██╗ ██╔══██╗ ██║  ██║",
    " ██╔██╗ ██║ ██║   ██║ ███████║ ███████║",
    " ██║╚██╗██║ ██║   ██║ ██╔══██║ ██╔══██║",
    " ██║ ╚████║ ╚██████╔╝ ██║  ██║ ██║  ██║",
    " ╚═╝  ╚═══╝  ╚═════╝  ╚═╝  ╚═╝ ╚═╝  ╚═╝",
];
const ART_GALLEGO: &[&str] = &[
    "  ██████╗  █████╗ ██╗     ██╗     ███████╗ ██████╗  ██████╗ ",
    " ██╔════╝ ██╔══██╗██║     ██║     ██╔════╝██╔════╝ ██╔═══██╗",
    " ██║  ███╗███████║██║     ██║     █████╗  ██║  ███╗██║   ██║",
    " ██║   ██║██╔══██║██║     ██║     ██╔══╝  ██║   ██║██║   ██║",
    " ╚██████╔╝██║  ██║███████╗███████╗███████╗╚██████╔╝╚██████╔╝",
    "  ╚═════╝ ╚═╝  ╚═╝╚══════╝╚══════╝╚══════╝ ╚═════╝  ╚═════╝",
];
const ART_NOAH_SM: &[&str] = &["█▄ █  ██  ▄▀▄ █ █","█ ▀█ █  █ █▀█ █▀█","▀  ▀  ▀▀  ▀ ▀ ▀ ▀"];
const ART_GALLEGO_SM: &[&str] = &["▄▀  ▄▀▄ █  █  ██ ▄▀  ██","█ █ █▀█ █  █  █▀ █ █ █ █"," ▀▀ ▀ ▀ ▀▀ ▀▀ ▀▀  ▀▀  ▀▀"];

fn rblock(title: &str, color: Color) -> Block<'_> {
    Block::default().borders(Borders::ALL).border_type(BorderType::Rounded)
        .border_style(Style::default().fg(SURFACE1))
        .title(Span::styled(format!(" {} ", title), Style::default().fg(color).add_modifier(Modifier::BOLD)))
}

// ── Greetings ─────────────────────────────────────────────────
fn get_greeting() -> &'static str {
    let hour = Local::now().format("%H").to_string().parse::<u32>().unwrap_or(12);
    let mut rng = rand::rng();
    let g: &[&str] = match hour {
        5..=8 => &["Rise and grind.","Coffee's ready, let's code.","Fresh morning, fresh commits.",
            "Early bird gets the merge.","Sun's up, let's ship.","Dawn patrol. Let's go.",
            "Up before the bugs.","The world is quiet. Perfect for building."],
        9..=11 => &["Morning momentum — let's use it.","Good morning. What are we building?",
            "Caffeinated and ready.","Prime coding hours. Let's go.","Clear head, clean code.",
            "The morning is yours.","Cogitating... ready."],
        12..=14 => &["Afternoon push incoming.","Back at it.","Post-lunch productivity mode.",
            "Halfway through — let's finish strong.","Second wind activated.","Brewing something good."],
        15..=17 => &["Late afternoon, locked in.","Golden hour coding.","Let's close out strong.",
            "Almost EOD. One more feature?","Power hour.","Final stretch. Let's go."],
        18..=21 => &["Evening session, let's build.","Night owl mode activated.",
            "Good evening. Let's create something.","The quiet hours are the best hours.",
            "Moonlight coding.","Evening energy. Let's ship."],
        _ => &["Late night, big commits.","Burning the midnight oil.","Can't sleep? Let's code.",
            "Silent night, loud commits.","3 AM clarity hits different.",
            "When the world sleeps, we ship.","Late night, locked in."],
    };
    let generic: &[&str] = &["Let's make something cool.","Ready when you are.","What are we breaking today?",
        "The terminal awaits.","Your move.","Ship it.","Let's cook.","Vibing. Let's build."];
    if rng.random_bool(0.3) { generic[rng.random_range(0..generic.len())] }
    else { g[rng.random_range(0..g.len())] }
}

// ── Data structs ──────────────────────────────────────────────
struct GpuInfo { name: String, util_pct: u32, vram_used: u64, vram_total: u64, temp: u32, fan_pct: Option<u32>, power_w: Option<u32> }
struct TempSensor { label: String, temp_c: f32 }
struct BoardInfo { board_name: String, bios_vendor: String, bios_version: String, bios_date: String }
struct DropboxStatus { status: String, last_activity: String, is_syncing: bool }

/// Progress from a background file-walk of ~/Dropbox using `dropbox filestatus`.
#[derive(Clone)]
struct DropboxSyncProgress { synced: u64, total: u64, scanning: bool }

fn read_sysfs(p: &str) -> Option<String> { std::fs::read_to_string(p).ok().map(|s| s.trim().to_string()).filter(|s| !s.is_empty()) }

fn get_board_info() -> BoardInfo {
    BoardInfo {
        board_name: read_sysfs("/sys/class/dmi/id/board_name").unwrap_or_else(|| "Unknown".into()),
        bios_vendor: read_sysfs("/sys/class/dmi/id/bios_vendor").unwrap_or_else(|| "Unknown".into()),
        bios_version: read_sysfs("/sys/class/dmi/id/bios_version").unwrap_or_default(),
        bios_date: read_sysfs("/sys/class/dmi/id/bios_date").unwrap_or_default(),
    }
}

fn get_temps() -> Vec<TempSensor> {
    let mut temps = Vec::new();
    let Ok(entries) = std::fs::read_dir("/sys/class/hwmon") else { return temps; };
    for entry in entries.flatten() {
        let base = entry.path();
        let name = read_sysfs(&format!("{}/name", base.display())).unwrap_or_default();
        for i in 1..=10 {
            if let Some(val) = read_sysfs(&format!("{}/temp{}_input", base.display(), i)) {
                if let Ok(mc) = val.parse::<f32>() {
                    let label = read_sysfs(&format!("{}/temp{}_label", base.display(), i)).unwrap_or_else(|| format!("{} #{}", name, i));
                    temps.push(TempSensor { label, temp_c: mc / 1000.0 });
                }
            }
        }
    }
    temps.sort_by(|a, b| b.temp_c.partial_cmp(&a.temp_c).unwrap_or(std::cmp::Ordering::Equal));
    temps.truncate(8);
    temps
}

fn get_fans() -> Vec<(String, u32)> {
    let mut fans = Vec::new();
    let Ok(entries) = std::fs::read_dir("/sys/class/hwmon") else { return fans; };
    for entry in entries.flatten() {
        let base = entry.path();
        let name = read_sysfs(&format!("{}/name", base.display())).unwrap_or_default();
        for i in 1..=5 {
            if let Some(val) = read_sysfs(&format!("{}/fan{}_input", base.display(), i)) {
                if let Ok(rpm) = val.parse::<u32>() { if rpm > 0 { fans.push((format!("{} fan{}", name, i), rpm)); } }
            }
        }
    }
    fans
}

fn get_gpu() -> Option<GpuInfo> {
    let out = std::process::Command::new("nvidia-smi")
        .args(["--query-gpu=name,utilization.gpu,memory.used,memory.total,temperature.gpu,fan.speed,power.draw",
            "--format=csv,noheader,nounits"]).output().ok()?;
    if !out.status.success() { return None; }
    let s = String::from_utf8_lossy(&out.stdout);
    let p: Vec<&str> = s.trim().split(", ").collect();
    if p.len() >= 5 { Some(GpuInfo {
        name: p[0].trim().into(), util_pct: p[1].trim().parse().unwrap_or(0),
        vram_used: p[2].trim().parse::<u64>().unwrap_or(0)*1024*1024,
        vram_total: p[3].trim().parse::<u64>().unwrap_or(1)*1024*1024,
        temp: p[4].trim().parse().unwrap_or(0),
        fan_pct: p.get(5).and_then(|s| s.trim().parse().ok()),
        power_w: p.get(6).and_then(|s| s.trim().split('.').next()?.parse().ok()),
    }) } else { None }
}

fn get_dropbox() -> DropboxStatus {
    let output = std::process::Command::new("dropbox").arg("status").output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_else(|_| "Not installed".into());
    let status = output.lines().next().unwrap_or("Unknown").to_string();
    let is_syncing = !status.contains("Up to date") && !status.contains("isn't running")
        && !status.contains("Not installed");
    let last_activity = std::fs::metadata(
        std::env::var("HOME").unwrap_or_default() + "/.dropbox/instance1/sync/nucleus.sqlite3-wal")
        .and_then(|m| m.modified())
        .map(|t| {
            let elapsed = t.elapsed().unwrap_or_default();
            if elapsed.as_secs() < 60 { "just now".into() }
            else if elapsed.as_secs() < 3600 { format!("{}m ago", elapsed.as_secs() / 60) }
            else if elapsed.as_secs() < 86400 { format!("{}h ago", elapsed.as_secs() / 3600) }
            else { format!("{}d ago", elapsed.as_secs() / 86400) }
        }).unwrap_or_else(|_| "unknown".into());
    DropboxStatus { status, last_activity, is_syncing }
}

/// Walk ~/Dropbox recursively and call `dropbox filestatus` in batches to count
/// synced vs total files.  Designed to run in a background thread.
fn scan_dropbox_progress(result: Arc<Mutex<DropboxSyncProgress>>) {
    let home = std::env::var("HOME").unwrap_or_default();
    let dropbox_dir = std::path::PathBuf::from(&home).join("Dropbox");
    if !dropbox_dir.is_dir() { return; }

    // Collect all file paths first
    let mut paths: Vec<std::path::PathBuf> = Vec::new();
    let mut dirs = vec![dropbox_dir];
    while let Some(dir) = dirs.pop() {
        if let Ok(entries) = std::fs::read_dir(&dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    // Skip .dropbox.cache
                    if path.file_name().map(|n| n.to_string_lossy().starts_with('.')).unwrap_or(false) { continue; }
                    dirs.push(path);
                } else {
                    paths.push(path);
                }
            }
        }
    }

    let total = paths.len() as u64;
    if total == 0 { return; }

    // Update total immediately so UI can show "scanning 0/N"
    if let Ok(mut p) = result.lock() {
        p.total = total;
        p.synced = 0;
        p.scanning = true;
    }

    // Query filestatus in batches of 200
    let mut synced: u64 = 0;
    for chunk in paths.chunks(200) {
        let args: Vec<&std::ffi::OsStr> = std::iter::once(std::ffi::OsStr::new("filestatus"))
            .chain(chunk.iter().map(|p| p.as_os_str()))
            .collect();
        if let Ok(out) = std::process::Command::new("dropbox").args(&args).output() {
            let text = String::from_utf8_lossy(&out.stdout);
            for line in text.lines() {
                if line.contains("up to date") { synced += 1; }
            }
        }
        // Update progress as we go so the UI is live
        if let Ok(mut p) = result.lock() {
            p.synced = synced;
        }
    }

    if let Ok(mut p) = result.lock() {
        p.synced = synced;
        p.scanning = false;
    }
}

fn restart_dropbox_sync() {
    // Run in background thread to avoid blocking the render loop
    std::thread::spawn(|| {
        let _ = std::process::Command::new("dropbox").arg("stop").output();
        std::thread::sleep(Duration::from_millis(500));
        let _ = std::process::Command::new("dropbox").arg("start").output();
    });
}

// ── Vertical core bars widget ─────────────────────────────────
struct CoreBars { percentages: Vec<f32> }
impl Widget for CoreBars {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let blocks = [' ', '▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];
        let bar_w = 2u16; // 2 chars per core
        let max_cores = ((area.width) / bar_w) as usize;
        for (i, &pct) in self.percentages.iter().take(max_cores).enumerate() {
            let x = area.x + (i as u16 * bar_w);
            let color = usage_color(pct as f64);
            for row in 0..area.height {
                let y = area.y + area.height - 1 - row;
                let row_pct = ((row as f64 + 1.0) / area.height as f64) * 100.0;
                let ch = if (pct as f64) >= row_pct { '█' }
                    else if (pct as f64) >= row_pct - (100.0 / area.height as f64) {
                        let frac = ((pct as f64 - (row_pct - 100.0 / area.height as f64)) / (100.0 / area.height as f64) * 8.0) as usize;
                        blocks[frac.min(8)]
                    } else { ' ' };
                for dx in 0..bar_w.min(area.width - (i as u16 * bar_w)) {
                    if let Some(cell) = buf.cell_mut((x + dx, y)) { cell.set_char(ch); cell.set_style(Style::default().fg(color)); }
                }
            }
        }
    }
}

// ── Stacked memory bar widget ─────────────────────────────────
struct MemBar { used: f64, cached: f64, label: String }
impl Widget for MemBar {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.width < 10 || area.height == 0 { return; }
        let bw = area.width as f64;
        let uw = (self.used * bw) as u16;
        let cw = (self.cached * bw) as u16;
        let fw = area.width.saturating_sub(uw + cw);
        let y = area.y;
        for dx in 0..uw { if let Some(c) = buf.cell_mut((area.x + dx, y)) { c.set_char('█'); c.set_style(Style::default().fg(GREEN)); } }
        for dx in 0..cw { if let Some(c) = buf.cell_mut((area.x + uw + dx, y)) { c.set_char('█'); c.set_style(Style::default().fg(YELLOW)); } }
        for dx in 0..fw { if let Some(c) = buf.cell_mut((area.x + uw + cw + dx, y)) { c.set_char('░'); c.set_style(Style::default().fg(SURFACE0)); } }
        // Label overlay
        let label_x = area.x + (area.width.saturating_sub(self.label.len() as u16)) / 2;
        for (i, ch) in self.label.chars().enumerate() {
            if let Some(c) = buf.cell_mut((label_x + i as u16, y)) { c.set_char(ch); c.set_style(Style::default().fg(SUBTEXT1).add_modifier(Modifier::BOLD)); }
        }
    }
}

// ── Health monitor ────────────────────────────────────────────
const HISTORY_LEN: usize = 120;
const TAB_NAMES: &[&str] = &["Overview", "Processes", "Disks", "System"];

struct HealthMonitor {
    sys: System, disks: Disks, networks: Networks,
    cpu_history: VecDeque<f64>, time_points: VecDeque<f64>,
    mem_history: VecDeque<f64>,
    top_procs: Vec<(String, u32, f32, u64)>,
    gpu_info: Option<GpuInfo>, temps: Vec<TempSensor>, fans: Vec<(String, u32)>,
    board: BoardInfo, dropbox: DropboxStatus,
    dropbox_progress: Arc<Mutex<DropboxSyncProgress>>,
    dropbox_scan_running: bool,
    dropbox_last_scan_done: Instant,
    uptime: u64, os_name: String, hostname: String, kernel: String,
    net_rx_total: u64, net_tx_total: u64,
    scroll_offset: usize, tab_idx: usize,
    health_start: Instant, last_dropbox_refresh: Instant, initialized: bool,
}

impl HealthMonitor {
    fn new() -> Self {
        let mut sys = System::new_all(); sys.refresh_cpu_usage();
        Self {
            sys, disks: Disks::new_with_refreshed_list(), networks: Networks::new_with_refreshed_list(),
            cpu_history: VecDeque::with_capacity(HISTORY_LEN), time_points: VecDeque::with_capacity(HISTORY_LEN),
            mem_history: VecDeque::with_capacity(HISTORY_LEN),
            top_procs: Vec::new(), gpu_info: None, temps: Vec::new(), fans: Vec::new(),
            board: get_board_info(), dropbox: get_dropbox(),
            dropbox_progress: Arc::new(Mutex::new(DropboxSyncProgress { synced: 0, total: 0, scanning: false })),
            dropbox_scan_running: false,
            dropbox_last_scan_done: Instant::now() - Duration::from_secs(60),
            uptime: System::uptime(), os_name: System::name().unwrap_or_else(|| "Unknown".into()),
            hostname: System::host_name().unwrap_or_else(|| "unknown".into()),
            kernel: System::kernel_version().unwrap_or_default(),
            net_rx_total: 0, net_tx_total: 0,
            scroll_offset: 0, tab_idx: 0, health_start: Instant::now(), last_dropbox_refresh: Instant::now() - Duration::from_secs(60), initialized: false,
        }
    }
    fn refresh(&mut self) {
        self.sys.refresh_cpu_usage(); self.sys.refresh_memory();
        self.sys.refresh_processes(sysinfo::ProcessesToUpdate::All, true);
        self.disks.refresh(false); self.networks.refresh(false);
        let elapsed = self.health_start.elapsed().as_secs_f64();
        let global = self.sys.global_cpu_usage() as f64;
        let mem_pct = self.sys.used_memory() as f64 / self.sys.total_memory().max(1) as f64 * 100.0;
        if self.cpu_history.len() >= HISTORY_LEN { self.cpu_history.pop_front(); self.time_points.pop_front(); self.mem_history.pop_front(); }
        self.cpu_history.push_back(global); self.time_points.push_back(elapsed); self.mem_history.push_back(mem_pct);
        let nc = self.sys.cpus().len().max(1) as f32;
        let mut procs: Vec<_> = self.sys.processes().iter()
            .map(|(pid, p)| (p.name().to_string_lossy().to_string(), pid.as_u32(), p.cpu_usage()/nc, p.memory()))
            .filter(|(_, _, cpu, _)| *cpu > 0.05).collect();
        procs.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap_or(std::cmp::Ordering::Equal));
        procs.truncate(30); self.top_procs = procs;
        self.net_rx_total = self.networks.iter().map(|(_, d)| d.total_received()).sum();
        self.net_tx_total = self.networks.iter().map(|(_, d)| d.total_transmitted()).sum();
        self.uptime = System::uptime(); self.gpu_info = get_gpu();
        self.temps = get_temps(); self.fans = get_fans();
        // Refresh dropbox status text every 5s
        if self.last_dropbox_refresh.elapsed() >= Duration::from_secs(5) {
            self.dropbox = get_dropbox();
            self.last_dropbox_refresh = Instant::now();
        }
        // Kick off a background file-walk scan if not already running
        // Re-scan every 30s, or immediately on first load
        if !self.dropbox_scan_running && self.dropbox_last_scan_done.elapsed() >= Duration::from_secs(30) {
            let done = self.dropbox_progress.lock().map(|p| !p.scanning).unwrap_or(true);
            if done {
                self.dropbox_scan_running = true;
                let progress = Arc::clone(&self.dropbox_progress);
                std::thread::spawn(move || {
                    scan_dropbox_progress(progress);
                });
            }
        }
        // When background scan finishes, record the time and allow future scans
        if self.dropbox_scan_running {
            let done = self.dropbox_progress.lock().map(|p| !p.scanning && p.total > 0).unwrap_or(false);
            if done {
                self.dropbox_scan_running = false;
                self.dropbox_last_scan_done = Instant::now();
            }
        }
        self.initialized = true;
    }
}

// ── Helpers ───────────────────────────────────────────────────
fn usage_color(p: f64) -> Color { match p as u32 { 0..=40 => GREEN, 41..=70 => YELLOW, 71..=90 => PEACH, _ => RED } }
fn temp_color(c: f32) -> Color { if c < 50.0 { GREEN } else if c < 70.0 { YELLOW } else if c < 85.0 { PEACH } else { RED } }
fn fmt_b(b: u64) -> String {
    if b >= 1_073_741_824 { format!("{:.1}G", b as f64/1_073_741_824.0) }
    else if b >= 1_048_576 { format!("{:.0}M", b as f64/1_048_576.0) }
    else if b >= 1024 { format!("{:.0}K", b as f64/1024.0) }
    else { format!("{}B", b) }
}
fn glitch_art_line(line: &str, progress: f64) -> Line<'static> {
    let mut rng = rand::rng(); let chars: Vec<char> = line.chars().collect(); let len = chars.len() as f64;
    Line::from(chars.iter().enumerate().map(|(i, &ch)| {
        if ch == ' ' { return Span::styled(" ", Style::default()); }
        let cp = ((progress*(len+8.0)-i as f64)/8.0).clamp(0.0,1.0);
        if cp > 0.92 { Span::styled(ch.to_string(), Style::default().fg(SKY).add_modifier(Modifier::BOLD)) }
        else if cp > 0.55 { if rng.random_bool(cp) { Span::styled(ch.to_string(), Style::default().fg(MAUVE)) }
            else { Span::styled(GLITCH_CHARS[rng.random_range(0..GLITCH_CHARS.len())].to_string(), Style::default().fg(SUBTEXT0)) } }
        else if cp > 0.05 { let c = [RED,SKY,YELLOW,GREEN,MAUVE][rng.random_range(0..5)];
            Span::styled(GLITCH_CHARS[rng.random_range(0..GLITCH_CHARS.len())].to_string(), Style::default().fg(c)) }
        else { Span::styled(" ", Style::default()) }
    }).collect::<Vec<_>>())
}
fn resolved_art_line(line: &str) -> Line<'static> {
    Line::from(line.chars().map(|ch| if ch==' ' { Span::styled(" ", Style::default()) }
        else { Span::styled(ch.to_string(), Style::default().fg(SKY).add_modifier(Modifier::BOLD)) }).collect::<Vec<_>>())
}

// ── Screens ───────────────────────────────────────────────────
#[derive(PartialEq)] enum Screen { Home, Health }

fn render_home(frame: &mut Frame, elapsed: f64, greeting: &str) {
    let area = frame.area();
    frame.render_widget(Block::default().style(Style::default().bg(BASE)), area);
    let gd = 2.8; let p = (elapsed/gd).clamp(0.0,1.0);
    let e = if p < 0.5 { 2.0*p*p } else { 1.0-(-2.0*p+2.0).powi(2)/2.0 };
    let sm = area.width < 40;
    let at: &[&str] = if sm { &ART_NOAH_SM } else { &ART_NOAH };
    let ab: &[&str] = if sm { &ART_GALLEGO_SM } else { &ART_GALLEGO };
    let th = at.len()+1+ab.len();
    let sy = if area.height > (th as u16+8) { ((area.height as f64-th as f64-4.0)/3.0).max(1.0) as u16 } else { 1 };
    for (i,l) in at.iter().enumerate() {
        let y = sy+i as u16; if y >= area.height.saturating_sub(3) { break; }
        let al = if e<1.0 { glitch_art_line(l,e) } else { resolved_art_line(l) };
        frame.render_widget(Paragraph::new(al).alignment(Alignment::Center), Rect::new(0,y,area.width,1));
    }
    let bs = sy+at.len() as u16+1;
    for (i,l) in ab.iter().enumerate() {
        let y = bs+i as u16; if y >= area.height.saturating_sub(3) { break; }
        let gp = ((e-0.15)/0.85).clamp(0.0,1.0);
        let al = if gp<1.0 { glitch_art_line(l,gp) } else { resolved_art_line(l) };
        frame.render_widget(Paragraph::new(al).alignment(Alignment::Center), Rect::new(0,y,area.width,1));
    }
    if elapsed < gd && elapsed > 0.2 {
        for s in [((elapsed*5.0) as u16)%area.height, ((elapsed*7.5+area.height as f64/2.0) as u16)%area.height] {
            if s < area.height { frame.render_widget(Paragraph::new(Line::from(Span::styled("─".repeat(area.width as usize), Style::default().fg(SURFACE0)))), Rect::new(0,s,area.width,1)); }
        }
    }
    if elapsed > gd {
        let rv = ((elapsed-gd)/0.8).clamp(0.0,1.0);
        if rv > 0.15 {
            let a = ((rv-0.15)/0.4).clamp(0.0,1.0); let b = (a*200.0) as u8;
            let gy = bs+ab.len() as u16+2;
            if gy < area.height.saturating_sub(3) {
                let cv = if (elapsed*2.0).sin()>0.0 { "█" } else { " " };
                frame.render_widget(Paragraph::new(Line::from(vec![
                    Span::styled(greeting, Style::default().fg(Color::Rgb(b,b,(b as f64*1.05).min(255.0) as u8)).add_modifier(Modifier::ITALIC)),
                    Span::styled(cv, Style::default().fg(SKY)),
                ])).alignment(Alignment::Center), Rect::new(0,gy,area.width,1));
            }
        }
        if rv > 0.5 {
            let a = ((rv-0.5)/0.3).clamp(0.0,1.0);
            let d = (a*108.0) as u8; let br = (a*200.0) as u8;
            let kc = Color::Rgb((a*137.0) as u8,(a*180.0) as u8,(a*250.0) as u8);
            let dc = Color::Rgb(d,d,d);
            let py = area.height.saturating_sub(3);
            if py > 0 { frame.render_widget(Paragraph::new(Line::from(vec![
                Span::styled("Press ", Style::default().fg(dc)),
                Span::styled("Enter", Style::default().fg(Color::Rgb(br,br,br)).add_modifier(Modifier::BOLD)),
                Span::styled(" to continue", Style::default().fg(dc)),
            ])).alignment(Alignment::Center), Rect::new(0,py,area.width,1)); }
            let sy2 = area.height.saturating_sub(2);
            if sy2 > py { frame.render_widget(Paragraph::new(Line::from(vec![
                Span::styled("[", Style::default().fg(dc)),
                Span::styled("space", Style::default().fg(kc).add_modifier(Modifier::BOLD)),
                Span::styled("] system health", Style::default().fg(dc)),
            ])).alignment(Alignment::Center), Rect::new(0,sy2,area.width,1)); }
        }
    }
}

// ── Health screen ─────────────────────────────────────────────
fn render_health(frame: &mut Frame, mon: &HealthMonitor, elapsed: f64) {
    let area = frame.area();
    frame.render_widget(Block::default().style(Style::default().bg(BASE)), area);
    if !mon.initialized { return; }
    let chunks = Layout::vertical([Constraint::Length(3), Constraint::Min(6), Constraint::Length(2)]).split(area);

    // Header with Dropbox status
    let h = mon.uptime/3600; let m = (mon.uptime%3600)/60;
    let now = Local::now().format("%H:%M:%S").to_string();
    let db_color = if mon.dropbox.status.contains("Up to date") { GREEN } else if mon.dropbox.is_syncing { PEACH } else { YELLOW };
    // Spinning icon when syncing: ◐ ◓ ◑ ◒
    let spin_chars = ['◐', '◓', '◑', '◒'];
    let db_icon = if mon.dropbox.is_syncing {
        let idx = (elapsed * 3.0) as usize % spin_chars.len();
        spin_chars[idx].to_string()
    } else if mon.dropbox.status.contains("Up to date") {
        "✓".to_string()
    } else {
        "⟳".to_string()
    };
    frame.render_widget(Paragraph::new(Line::from(vec![
        Span::styled(" SYSTEM HEALTH ", Style::default().fg(SKY).add_modifier(Modifier::BOLD)),
        Span::styled(format!("│ {} │ up {}h {}m │ {} │ ", mon.hostname, h, m, now), Style::default().fg(OVERLAY0)),
        Span::styled(format!("{} ", db_icon), Style::default().fg(db_color)),
        Span::styled(format!("{} ({})", mon.dropbox.status, mon.dropbox.last_activity), Style::default().fg(db_color)),
    ])), Rect::new(0, chunks[0].y, area.width, 1));

    // Tab bar — arrow key navigable, no numbers
    let ty = chunks[0].y + 1;
    let mut ts: Vec<Span> = vec![Span::styled(" ", Style::default())];
    for (i, name) in TAB_NAMES.iter().enumerate() {
        if i == mon.tab_idx {
            ts.push(Span::styled(format!(" ● {} ", name), Style::default().fg(BASE).bg(SKY).add_modifier(Modifier::BOLD)));
        } else {
            ts.push(Span::styled(format!("   {}  ", name), Style::default().fg(OVERLAY0)));
        }
    }
    frame.render_widget(Paragraph::new(Line::from(ts)), Rect::new(0,ty,area.width,1));
    frame.render_widget(Paragraph::new(Line::from(Span::styled("─".repeat(area.width as usize), Style::default().fg(SURFACE1)))), Rect::new(0,ty+1,area.width,1));

    match mon.tab_idx {
        0 => render_overview(frame, chunks[1], mon),
        1 => render_processes(frame, chunks[1], mon),
        2 => render_disks(frame, chunks[1], mon),
        3 => render_system(frame, chunks[1], mon, elapsed),
        _ => {}
    }

    // Footer
    let blink = (elapsed*2.0).sin() > 0.0;
    let mut f: Vec<Span> = vec![
        Span::styled(" [", Style::default().fg(OVERLAY0)), Span::styled("esc", Style::default().fg(SKY).add_modifier(Modifier::BOLD)),
        Span::styled("] back  ", Style::default().fg(OVERLAY0)),
        Span::styled("[", Style::default().fg(OVERLAY0)), Span::styled("← →", Style::default().fg(SKY).add_modifier(Modifier::BOLD)),
        Span::styled("] tabs  ", Style::default().fg(OVERLAY0)),
    ];
    if mon.tab_idx == 1 {
        f.extend([Span::styled("[", Style::default().fg(OVERLAY0)), Span::styled("↑↓", Style::default().fg(SKY).add_modifier(Modifier::BOLD)),
            Span::styled("] scroll  ", Style::default().fg(OVERLAY0))]);
    }
    f.extend([
        Span::styled("[", Style::default().fg(OVERLAY0)), Span::styled("s", Style::default().fg(TEAL).add_modifier(Modifier::BOLD)),
        Span::styled("] sync  ", Style::default().fg(OVERLAY0)),
        Span::styled("  live", Style::default().fg(GREEN).add_modifier(Modifier::BOLD)),
        Span::styled(if blink { " ●" } else { "  " }, Style::default().fg(GREEN)),
    ]);
    frame.render_widget(Paragraph::new(Line::from(f)), chunks[2]);
}

fn render_overview(frame: &mut Frame, area: Rect, mon: &HealthMonitor) {
    let cores: Vec<f32> = mon.sys.cpus().iter().map(|c| c.cpu_usage()).collect();
    let has_gpu = mon.gpu_info.is_some();

    // Two-column layout
    let cols = Layout::horizontal([Constraint::Fill(3), Constraint::Fill(2)]).split(area);

    // Left column: CPU chart + memory
    let mem_cached = (mon.sys.total_memory() - mon.sys.used_memory() - mon.sys.available_memory().min(mon.sys.total_memory() - mon.sys.used_memory())) as f64 / mon.sys.total_memory().max(1) as f64;
    let mem_used = mon.sys.used_memory() as f64 / mon.sys.total_memory().max(1) as f64;

    let mut left_c = vec![Constraint::Min(8), Constraint::Length(3)]; // CPU chart, Memory
    if mon.sys.total_swap() > 0 { left_c.push(Constraint::Length(3)); }
    left_c.push(Constraint::Length(4)); // Network
    let left = Layout::vertical(left_c).split(cols[0]);

    // CPU Chart (Braille) — takes most space
    render_cpu_chart(frame, left[0], mon, &cores);

    // Memory — stacked bar
    let mem_title = format!("Memory {} / {}", fmt_b(mon.sys.used_memory()), fmt_b(mon.sys.total_memory()));
    let mem_blk = rblock(&mem_title, TEAL);
    let mem_inner = mem_blk.inner(left[1]);
    frame.render_widget(mem_blk, left[1]);
    if mem_inner.height > 0 {
        let label = format!("{:.0}%", mem_used * 100.0);
        frame.render_widget(MemBar { used: mem_used, cached: mem_cached.max(0.0), label }, mem_inner);
    }

    // Swap
    let mut li = 2;
    if mon.sys.total_swap() > 0 {
        let sw_r = mon.sys.used_swap() as f64 / mon.sys.total_swap().max(1) as f64;
        frame.render_widget(Gauge::default()
            .block(rblock(&format!("Swap {} / {}", fmt_b(mon.sys.used_swap()), fmt_b(mon.sys.total_swap())), MAUVE))
            .gauge_style(Style::default().fg(usage_color(sw_r*100.0)).bg(SURFACE0))
            .label(Span::styled(format!("{:.0}%", sw_r*100.0), Style::default().fg(SUBTEXT1).add_modifier(Modifier::BOLD)))
            .ratio(sw_r.clamp(0.0,1.0)), left[li]);
        li += 1;
    }

    // Network
    render_net(frame, left[li], mon);

    // Right column: Core bars + GPU + Thermals
    let core_h = ((cores.len() as u16 * 2 + 2) / 2).clamp(4, 12);
    let mut right_c = vec![Constraint::Length(core_h)]; // Core bars
    if has_gpu { right_c.push(Constraint::Length(7)); }
    if !mon.temps.is_empty() { right_c.push(Constraint::Length((mon.temps.len() as u16 + mon.fans.len() as u16 + 2).min(10))); }
    right_c.push(Constraint::Min(0));
    let right = Layout::vertical(right_c).split(cols[1]);

    // Core bars (vertical)
    let core_title = format!("Cores {:.0}%", mon.sys.global_cpu_usage());
    let core_blk = rblock(&core_title, GREEN);
    let core_inner = core_blk.inner(right[0]);
    frame.render_widget(core_blk, right[0]);
    frame.render_widget(CoreBars { percentages: cores }, core_inner);

    let mut ri = 1;
    if let Some(ref gpu) = mon.gpu_info { render_gpu(frame, right[ri], gpu); ri += 1; }
    if !mon.temps.is_empty() && ri < right.len() - 1 { render_temps(frame, right[ri], &mon.temps, &mon.fans); }
}

fn render_cpu_chart(frame: &mut Frame, area: Rect, mon: &HealthMonitor, _cores: &[f32]) {
    if mon.cpu_history.len() < 2 { return; }
    let cpu_data: Vec<(f64,f64)> = mon.time_points.iter().zip(mon.cpu_history.iter()).map(|(&t,&v)| (t,v)).collect();
    let mem_data: Vec<(f64,f64)> = mon.time_points.iter().zip(mon.mem_history.iter()).map(|(&t,&v)| (t,v)).collect();
    let x_min = mon.time_points.front().copied().unwrap_or(0.0);
    let x_max = mon.time_points.back().copied().unwrap_or(60.0);
    let elapsed = x_max - x_min;
    let x_labels = vec![
        Span::styled(format!("-{:.0}s", elapsed), Style::default().fg(OVERLAY0)),
        Span::styled("now", Style::default().fg(SUBTEXT1)),
    ];
    let datasets = vec![
        Dataset::default().name("CPU").data(&cpu_data).marker(Marker::Braille).style(Style::default().fg(SKY)),
        Dataset::default().name("MEM").data(&mem_data).marker(Marker::Braille).style(Style::default().fg(TEAL)),
    ];
    frame.render_widget(Chart::new(datasets)
        .block(rblock("CPU & Memory History", SKY))
        .x_axis(Axis::default().bounds([x_min, x_max]).labels(x_labels).style(Style::default().fg(OVERLAY0)))
        .y_axis(Axis::default().bounds([0.0,100.0])
            .labels(vec![Span::styled("0",Style::default().fg(OVERLAY0)),Span::styled("50",Style::default().fg(OVERLAY0)),Span::styled("100%",Style::default().fg(OVERLAY0))])
            .style(Style::default().fg(OVERLAY0))), area);
}

fn render_gpu(frame: &mut Frame, area: Rect, gpu: &GpuInfo) {
    let gpu_title = format!("GPU {}°C", gpu.temp);
    let blk = rblock(&gpu_title, MAUVE);
    let inner = blk.inner(area); frame.render_widget(blk, area);
    if inner.height == 0 { return; }
    let mut y = inner.y;
    let nd = if gpu.name.len() > inner.width as usize - 2 { &gpu.name[..inner.width as usize-2] } else { &gpu.name };
    frame.render_widget(Paragraph::new(Span::styled(format!(" {}", nd), Style::default().fg(SUBTEXT0))), Rect::new(inner.x,y,inner.width,1)); y+=1;
    if y < inner.y+inner.height {
        let bw = (inner.width as usize).saturating_sub(12); let f = (gpu.util_pct as usize*bw)/100; let c = usage_color(gpu.util_pct as f64);
        frame.render_widget(Paragraph::new(Line::from(vec![Span::styled(" USE ",Style::default().fg(OVERLAY0)),
            Span::styled("█".repeat(f),Style::default().fg(c)),Span::styled("░".repeat(bw.saturating_sub(f)),Style::default().fg(SURFACE0)),
            Span::styled(format!("{:>3}%",gpu.util_pct),Style::default().fg(c))])), Rect::new(inner.x,y,inner.width,1)); y+=1;
    }
    if y < inner.y+inner.height {
        let vp = (gpu.vram_used as f64/gpu.vram_total.max(1) as f64)*100.0;
        let bw = (inner.width as usize).saturating_sub(12); let f = (vp as usize*bw)/100; let c = usage_color(vp);
        frame.render_widget(Paragraph::new(Line::from(vec![Span::styled(" MEM ",Style::default().fg(OVERLAY0)),
            Span::styled("█".repeat(f),Style::default().fg(c)),Span::styled("░".repeat(bw.saturating_sub(f)),Style::default().fg(SURFACE0)),
            Span::styled(format!("{:>3}% {}/{}",vp as u32,fmt_b(gpu.vram_used),fmt_b(gpu.vram_total)),Style::default().fg(c))])), Rect::new(inner.x,y,inner.width,1)); y+=1;
    }
    if y < inner.y+inner.height {
        let mut info: Vec<Span> = vec![Span::styled(" ",Style::default())];
        if let Some(fan) = gpu.fan_pct { info.push(Span::styled(format!("FAN {}%",fan),Style::default().fg(SUBTEXT0))); info.push(Span::styled("  ",Style::default())); }
        if let Some(pw) = gpu.power_w { info.push(Span::styled(format!("PWR {}W",pw),Style::default().fg(SUBTEXT0))); }
        frame.render_widget(Paragraph::new(Line::from(info)), Rect::new(inner.x,y,inner.width,1));
    }
}

fn render_temps(frame: &mut Frame, area: Rect, temps: &[TempSensor], fans: &[(String, u32)]) {
    let blk = rblock("Thermals", PEACH); let inner = blk.inner(area); frame.render_widget(blk, area);
    for (i,t) in temps.iter().enumerate() {
        let y = inner.y+i as u16; if y >= inner.y+inner.height { break; }
        let c = temp_color(t.temp_c); let label = if t.label.len()>16 { &t.label[..16] } else { &t.label };
        frame.render_widget(Paragraph::new(Line::from(vec![
            Span::styled(format!(" {:>16} ",label),Style::default().fg(OVERLAY0)),
            Span::styled(format!("{:.0}°C",t.temp_c),Style::default().fg(c).add_modifier(Modifier::BOLD)),
        ])), Rect::new(inner.x,y,inner.width,1));
    }
    let fs = inner.y+temps.len() as u16;
    for (i,(name,rpm)) in fans.iter().enumerate() {
        let y = fs+i as u16; if y >= inner.y+inner.height { break; }
        frame.render_widget(Paragraph::new(Line::from(vec![
            Span::styled(format!(" {:>16} ",name),Style::default().fg(OVERLAY0)),
            Span::styled(format!("{} RPM",rpm),Style::default().fg(TEAL)),
        ])), Rect::new(inner.x,y,inner.width,1));
    }
}

fn render_net(frame: &mut Frame, area: Rect, mon: &HealthMonitor) {
    let blk = rblock("Network", BLUE); let inner = blk.inner(area); frame.render_widget(blk, area);
    if inner.height >= 1 {
        frame.render_widget(Paragraph::new(Line::from(vec![
            Span::styled(" ▲ TX ",Style::default().fg(PEACH)), Span::styled(fmt_b(mon.net_tx_total),Style::default().fg(SUBTEXT1)),
            Span::styled("    ▼ RX ",Style::default().fg(GREEN)), Span::styled(fmt_b(mon.net_rx_total),Style::default().fg(SUBTEXT1)),
        ])), Rect::new(inner.x,inner.y,inner.width,1));
    }
    if inner.height >= 2 {
        let ifaces: Vec<String> = mon.networks.iter()
            .filter(|(_,d)| d.total_received()>0||d.total_transmitted()>0)
            .map(|(n,d)| format!("{}(↑{} ↓{})",n,fmt_b(d.transmitted()),fmt_b(d.received()))).collect();
        frame.render_widget(Paragraph::new(Span::styled(format!(" {}",ifaces.join("  ")),Style::default().fg(OVERLAY0))),
            Rect::new(inner.x,inner.y+1,inner.width,1));
    }
}

fn render_processes(frame: &mut Frame, area: Rect, mon: &HealthMonitor) {
    let cols = Layout::horizontal([Constraint::Fill(3), Constraint::Fill(2)]).split(area);

    // Table
    let hdr = Row::new(vec![Span::styled("PID",Style::default().fg(SKY).add_modifier(Modifier::BOLD)),
        Span::styled("PROCESS",Style::default().fg(SKY).add_modifier(Modifier::BOLD)),
        Span::styled("CPU%",Style::default().fg(SKY).add_modifier(Modifier::BOLD)),
        Span::styled("MEMORY",Style::default().fg(SKY).add_modifier(Modifier::BOLD))]).height(1);
    let vis = cols[0].height.saturating_sub(4) as usize;
    let off = mon.scroll_offset.min(mon.top_procs.len().saturating_sub(vis));
    let rows: Vec<Row> = mon.top_procs.iter().skip(off).take(vis).map(|(name,pid,cpu,mem)| {
        Row::new(vec![Span::styled(format!("{}",pid),Style::default().fg(OVERLAY0)),
            Span::styled(if name.len()>28 { format!("{}…",&name[..27]) } else { name.clone() },Style::default().fg(SUBTEXT1)),
            Span::styled(format!("{:.1}",cpu),Style::default().fg(usage_color(*cpu as f64))),
            Span::styled(fmt_b(*mem),Style::default().fg(SUBTEXT0))])
    }).collect();
    frame.render_widget(Table::new(rows, [Constraint::Length(8),Constraint::Min(15),Constraint::Length(8),Constraint::Length(10)])
        .header(hdr).block(rblock(&format!("Processes ({})", mon.top_procs.len()), PEACH)), cols[0]);

    // Resource breakdown — top 8 as colored bars
    let blk = rblock("Resource Usage", YELLOW); let inner = blk.inner(cols[1]); frame.render_widget(blk, cols[1]);
    let top: Vec<_> = mon.top_procs.iter().take(8).collect();
    let max_cpu = top.first().map(|p| p.2).unwrap_or(1.0).max(0.1);
    let colors = [SKY, MAUVE, TEAL, PEACH, GREEN, YELLOW, RED, BLUE];
    for (i,(name,_,cpu,mem)) in top.iter().enumerate() {
        let y = inner.y+(i as u16*2);
        if y+1 >= inner.y+inner.height { break; }
        let label = if name.len()>18 { &name[..18] } else { name };
        frame.render_widget(Paragraph::new(Line::from(vec![
            Span::styled(format!(" {} ",label),Style::default().fg(SUBTEXT1)),
            Span::styled(format!("{:.1}% {}",cpu,fmt_b(*mem)),Style::default().fg(OVERLAY0)),
        ])), Rect::new(inner.x,y,inner.width,1));
        let bw = (inner.width as usize).saturating_sub(2);
        let f = ((*cpu/max_cpu)*bw as f32) as usize;
        let c = colors[i%colors.len()];
        frame.render_widget(Paragraph::new(Line::from(vec![
            Span::styled(" ",Style::default()),
            Span::styled("█".repeat(f),Style::default().fg(c)),
            Span::styled("░".repeat(bw.saturating_sub(f)),Style::default().fg(SURFACE0)),
        ])), Rect::new(inner.x,y+1,inner.width,1));
    }
}

fn render_disks(frame: &mut Frame, area: Rect, mon: &HealthMonitor) {
    let disks: Vec<_> = mon.disks.iter()
        .filter(|d| d.total_space()>100_000_000 && !d.mount_point().to_string_lossy().starts_with("/snap")).collect();
    let mut cs: Vec<Constraint> = disks.iter().map(|_| Constraint::Length(3)).collect();
    cs.push(Constraint::Min(0));
    let secs = Layout::vertical(cs).split(area);
    for (i,d) in disks.iter().enumerate() {
        if i >= secs.len()-1 { break; }
        let used = d.total_space()-d.available_space();
        let ratio = used as f64/d.total_space().max(1) as f64;
        frame.render_widget(Gauge::default()
            .block(rblock(&format!("{} ({}) [{}]", d.mount_point().to_string_lossy(), d.name().to_string_lossy(), d.file_system().to_string_lossy()), BLUE))
            .gauge_style(Style::default().fg(usage_color(ratio*100.0)).bg(SURFACE0))
            .label(Span::styled(format!("{} / {} ({:.0}%)",fmt_b(used),fmt_b(d.total_space()),ratio*100.0),
                Style::default().fg(SUBTEXT1).add_modifier(Modifier::BOLD)))
            .ratio(ratio.clamp(0.0,1.0)), secs[i]);
    }
}

fn render_system(frame: &mut Frame, area: Rect, mon: &HealthMonitor, elapsed: f64) {
    let cols = Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)]).split(area);

    // Left: System info
    let blk = rblock("Hardware & OS", MAUVE); let inner = blk.inner(cols[0]); frame.render_widget(blk, cols[0]);
    let info: Vec<(&str, String)> = vec![
        ("Hostname", mon.hostname.clone()), ("OS", mon.os_name.clone()),
        ("Kernel", mon.kernel.clone()), ("Board", mon.board.board_name.clone()),
        ("BIOS", format!("{} v{}", mon.board.bios_vendor, mon.board.bios_version)),
        ("BIOS Date", mon.board.bios_date.clone()),
        ("CPU", mon.sys.cpus().first().map(|c| c.brand().to_string()).unwrap_or_default()),
        ("Cores", format!("{}", mon.sys.cpus().len())),
        ("RAM", fmt_b(mon.sys.total_memory())), ("Swap", fmt_b(mon.sys.total_swap())),
        ("Uptime", { let h=mon.uptime/3600; let m=(mon.uptime%3600)/60; format!("{}h {}m",h,m) }),
        ("Shell", std::env::var("SHELL").unwrap_or_default().rsplit('/').next().unwrap_or("").to_string()),
    ];
    for (i,(label,value)) in info.iter().enumerate() {
        let y = inner.y+i as u16; if y >= inner.y+inner.height { break; }
        frame.render_widget(Paragraph::new(Line::from(vec![
            Span::styled(format!(" {:>10}  ",label),Style::default().fg(SKY).add_modifier(Modifier::BOLD)),
            Span::styled(value,Style::default().fg(SUBTEXT1)),
        ])), Rect::new(inner.x,y,inner.width,1));
    }

    // Right: GPU + Dropbox + services
    let mut right_c: Vec<Constraint> = Vec::new();
    if mon.gpu_info.is_some() { right_c.push(Constraint::Length(8)); }
    right_c.push(Constraint::Length(7)); // Dropbox (extra row for progress bar)
    right_c.push(Constraint::Min(0));
    let right = Layout::vertical(right_c).split(cols[1]);
    let mut ri = 0;

    if let Some(ref gpu) = mon.gpu_info {
        let blk = rblock("GPU Details", MAUVE); let inner = blk.inner(right[ri]); frame.render_widget(blk, right[ri]);
        let gpu_info: Vec<(&str, String)> = vec![
            ("GPU", gpu.name.clone()),
            ("VRAM", format!("{} / {}", fmt_b(gpu.vram_used), fmt_b(gpu.vram_total))),
            ("Temp", format!("{}°C", gpu.temp)),
            ("Fan", gpu.fan_pct.map(|f| format!("{}%",f)).unwrap_or("N/A".into())),
            ("Power", gpu.power_w.map(|p| format!("{}W",p)).unwrap_or("N/A".into())),
        ];
        for (i,(label,value)) in gpu_info.iter().enumerate() {
            let y = inner.y+i as u16; if y >= inner.y+inner.height { break; }
            frame.render_widget(Paragraph::new(Line::from(vec![
                Span::styled(format!(" {:>10}  ",label),Style::default().fg(MAUVE).add_modifier(Modifier::BOLD)),
                Span::styled(value,Style::default().fg(SUBTEXT1)),
            ])), Rect::new(inner.x,y,inner.width,1));
        }
        ri += 1;
    }

    // Dropbox
    let spin_frames = ['◐', '◓', '◑', '◒'];
    let prog = mon.dropbox_progress.lock().map(|p| p.clone())
        .unwrap_or(DropboxSyncProgress { synced: 0, total: 0, scanning: false });
    let db_spin = if mon.dropbox.is_syncing {
        let idx = (elapsed * 3.0) as usize % spin_frames.len();
        format!(" {} Syncing", spin_frames[idx])
    } else { String::new() };
    let db_title = if mon.dropbox.is_syncing { format!("Dropbox{}", db_spin) } else { "Dropbox".into() };
    let db_title_color = if mon.dropbox.is_syncing { PEACH } else { BLUE };
    let db_blk = rblock(&db_title, db_title_color); let db_inner = db_blk.inner(right[ri]); frame.render_widget(db_blk, right[ri]);
    let db_color = if mon.dropbox.status.contains("Up to date") { GREEN } else if mon.dropbox.is_syncing { PEACH } else { YELLOW };
    let db_info: Vec<(&str, String, Color)> = vec![
        ("Status", mon.dropbox.status.clone(), db_color),
        ("Activity", mon.dropbox.last_activity.clone(), SUBTEXT1),
        ("Action", "[s] restart sync".into(), OVERLAY0),
    ];
    for (i,(label,value,c)) in db_info.iter().enumerate() {
        let y = db_inner.y+i as u16; if y >= db_inner.y+db_inner.height { break; }
        frame.render_widget(Paragraph::new(Line::from(vec![
            Span::styled(format!(" {:>10}  ",label),Style::default().fg(BLUE).add_modifier(Modifier::BOLD)),
            Span::styled(value,Style::default().fg(*c)),
        ])), Rect::new(db_inner.x,y,db_inner.width,1));
    }
    // Sync progress bar — always shown, based on actual file scan
    let progress_y = db_inner.y + 3;
    if progress_y < db_inner.y + db_inner.height {
        let (ratio, label): (f64, String) = if prog.total > 0 {
            let pct = prog.synced as f64 / prog.total as f64;
            let suffix = if prog.scanning { " scanning..." } else { "" };
            (pct, format!("{}/{}  {:.0}%{}", prog.synced, prog.total, pct * 100.0, suffix))
        } else if prog.scanning {
            (0.0, "Scanning files...".into())
        } else {
            (0.0, "Waiting...".into())
        };
        let bar_color = if ratio >= 1.0 { GREEN } else { PEACH };
        let bar_area = Rect::new(db_inner.x + 1, progress_y, db_inner.width.saturating_sub(2), 1);
        let gauge = Gauge::default()
            .gauge_style(Style::default().fg(bar_color).bg(SURFACE0))
            .ratio(ratio.clamp(0.0, 1.0))
            .label(Span::styled(label, Style::default().fg(SUBTEXT1).add_modifier(Modifier::BOLD)));
        frame.render_widget(gauge, bar_area);
    }
}

// ── Main ──────────────────────────────────────────────────────
fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let greeting = get_greeting().to_string();
    let start = Instant::now();
    let mut screen = Screen::Home;
    let mut monitor = HealthMonitor::new();
    let mut last_refresh = Instant::now() - Duration::from_secs(10);

    loop {
        let elapsed = start.elapsed().as_secs_f64();
        if screen == Screen::Health && last_refresh.elapsed() >= Duration::from_millis(1000) {
            monitor.refresh(); last_refresh = Instant::now();
        }
        terminal.draw(|frame| match &screen {
            Screen::Home => render_home(frame, elapsed, &greeting),
            Screen::Health => render_health(frame, &monitor, elapsed),
        })?;
        if poll(Duration::from_millis(33))? {
            if let Event::Key(key) = event::read()? {
                match (&screen, key.code) {
                    (Screen::Home, KeyCode::Enter) if elapsed > 2.8 => break,
                    (Screen::Home, KeyCode::Esc)|(Screen::Home, KeyCode::Char('q')) => break,
                    (Screen::Home, KeyCode::Char(' ')) if elapsed > 2.8 => {
                        monitor.refresh(); last_refresh = Instant::now(); screen = Screen::Health;
                    }
                    (Screen::Health, KeyCode::Esc)|(Screen::Health, KeyCode::Backspace)|(Screen::Health, KeyCode::Char('q')) => screen = Screen::Home,
                    (Screen::Health, KeyCode::Right)|(Screen::Health, KeyCode::Tab) => {
                        monitor.tab_idx = (monitor.tab_idx+1)%TAB_NAMES.len(); monitor.scroll_offset = 0;
                    }
                    (Screen::Health, KeyCode::Left)|(Screen::Health, KeyCode::BackTab) => {
                        monitor.tab_idx = if monitor.tab_idx==0 { TAB_NAMES.len()-1 } else { monitor.tab_idx-1 }; monitor.scroll_offset = 0;
                    }
                    (Screen::Health, KeyCode::Up) => monitor.scroll_offset = monitor.scroll_offset.saturating_sub(1),
                    (Screen::Health, KeyCode::Down) => monitor.scroll_offset += 1,
                    (Screen::Health, KeyCode::Char('s')) => {
                        restart_dropbox_sync();
                        monitor.dropbox = DropboxStatus { status: "Restarting sync...".into(), last_activity: "now".into(), is_syncing: true };
                        if let Ok(mut p) = monitor.dropbox_progress.lock() {
                            p.synced = 0; p.total = 0; p.scanning = false;
                        }
                        monitor.dropbox_scan_running = false;
                        monitor.dropbox_last_scan_done = Instant::now() - Duration::from_secs(60);
                        monitor.last_dropbox_refresh = Instant::now();
                    }
                    _ => {}
                }
            }
        }
    }
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}
