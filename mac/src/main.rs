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
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Paragraph},
};
use std::{
    io::{self, stdout},
    process::Command,
    time::{Duration, Instant},
};
use sysinfo::System;

// ── Tokyo Night Storm ─────────────────────────────────────────
const BASE: Color = Color::Rgb(36, 40, 59);
const SKY: Color = Color::Rgb(125, 207, 255);
const SUBTEXT0: Color = Color::Rgb(169, 177, 214);
const SURFACE0: Color = Color::Rgb(52, 56, 80);
const GREEN: Color = Color::Rgb(158, 206, 106);
const YELLOW: Color = Color::Rgb(224, 175, 104);
const RED: Color = Color::Rgb(247, 118, 142);
const MAUVE: Color = Color::Rgb(187, 154, 247);
// ── Nerd Font icons ──────────────────────────────────────────
const ICON_CALENDAR: &str = " ";  // nf-fa-calendar
const ICON_CPU: &str = " ";       // nf-oct-cpu
const ICON_MEM: &str = "󰍛 ";      // nf-md-memory
const ICON_DISK: &str = "󰋊 ";     // nf-md-harddisk
const ICON_OS: &str = " ";       // nf-fa-apple
const ICON_ENTER: &str = "󰌑 ";    // nf-md-keyboard_return
const ICON_HEALTH: &str = "󰄄 ";   // nf-md-heart_pulse

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

// ── Art helpers ───────────────────────────────────────────────
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

// ── System info (one-shot, cached) ───────────────────────────
struct SysSnapshot {
    cpu: String,
    cores: usize,
    ram_gb: f64,
    disk_gb: f64,
    hostname: String,
}

fn take_snapshot() -> SysSnapshot {
    let sys = System::new_all();
    let cpu = sys.cpus().first().map(|c| c.brand().to_string()).unwrap_or_default();
    // Shorten CPU name
    let cpu = cpu.replace("Intel(R) Core(TM) ", "").replace("AMD Ryzen ", "Ryzen ").replace("Apple ", "");
    let cores = sys.cpus().len();
    let ram_gb = sys.total_memory() as f64 / 1_073_741_824.0;
    let disk_gb = sysinfo::Disks::new_with_refreshed_list().iter()
        .map(|d| d.total_space() as f64 / 1_073_741_824.0).sum();
    let hostname = System::host_name().unwrap_or_else(|| "unknown".into());
    SysSnapshot { cpu, cores, ram_gb, disk_gb, hostname }
}

// ── Home screen ──────────────────────────────────────────────
fn render_home(frame: &mut Frame, elapsed: f64, greeting: &str, snap: &SysSnapshot) {
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

            // System info strip
            let info_y = bs + ab.len() as u16 + 4;
            if info_y < area.height.saturating_sub(5) {
                let now = Local::now().format("%a %d %b %H:%M").to_string();
                let info_spans = vec![
                    Span::styled(ICON_CALENDAR, Style::default().fg(Color::Rgb((a*125.0) as u8, (a*207.0) as u8, (a*255.0) as u8))),
                    Span::styled(format!("{}  ", now), Style::default().fg(Color::Rgb(d,d,d))),
                    Span::styled(ICON_OS, Style::default().fg(Color::Rgb((a*158.0) as u8, (a*206.0) as u8, (a*106.0) as u8))),
                    Span::styled(format!("{}  ", snap.hostname), Style::default().fg(Color::Rgb(d,d,d))),
                    Span::styled(ICON_CPU, Style::default().fg(Color::Rgb((a*122.0) as u8, (a*162.0) as u8, (a*247.0) as u8))),
                    Span::styled(format!("{} ({}t)  ", snap.cpu, snap.cores), Style::default().fg(Color::Rgb(d,d,d))),
                    Span::styled(ICON_MEM, Style::default().fg(Color::Rgb((a*187.0) as u8, (a*154.0) as u8, (a*247.0) as u8))),
                    Span::styled(format!("{:.0} GB  ", snap.ram_gb), Style::default().fg(Color::Rgb(d,d,d))),
                    Span::styled(ICON_DISK, Style::default().fg(Color::Rgb((a*115.0) as u8, (a*218.0) as u8, (a*202.0) as u8))),
                    Span::styled(format!("{:.0} GB", snap.disk_gb), Style::default().fg(Color::Rgb(d,d,d))),
                ];
                frame.render_widget(Paragraph::new(Line::from(info_spans)).alignment(Alignment::Center), Rect::new(0,info_y,area.width,1));
            }

            // Footer keybindings with icons
            let py = area.height.saturating_sub(3);
            if py > 0 { frame.render_widget(Paragraph::new(Line::from(vec![
                Span::styled(ICON_ENTER, Style::default().fg(Color::Rgb(br,br,br))),
                Span::styled("Enter", Style::default().fg(Color::Rgb(br,br,br)).add_modifier(Modifier::BOLD)),
                Span::styled("  continue", Style::default().fg(dc)),
            ])).alignment(Alignment::Center), Rect::new(0,py,area.width,1)); }
            let sy2 = area.height.saturating_sub(2);
            if sy2 > py { frame.render_widget(Paragraph::new(Line::from(vec![
                Span::styled(ICON_HEALTH, Style::default().fg(kc)),
                Span::styled("Space", Style::default().fg(kc).add_modifier(Modifier::BOLD)),
                Span::styled("  system health", Style::default().fg(dc)),
            ])).alignment(Alignment::Center), Rect::new(0,sy2,area.width,1)); }
        }
    }
}

// ── Launch btop ──────────────────────────────────────────────
fn launch_btop(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> io::Result<()> {
    // Leave TUI so btop gets the raw terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    // Run btop — blocks until the user quits it (q / Esc)
    let _ = Command::new("btop").status();

    // Re-enter TUI
    enable_raw_mode()?;
    execute!(terminal.backend_mut(), EnterAlternateScreen)?;
    terminal.hide_cursor()?;
    terminal.clear()?;
    Ok(())
}

// ── Main ──────────────────────────────────────────────────────
fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let greeting = get_greeting().to_string();
    let snap = take_snapshot();
    let start = Instant::now();

    loop {
        let elapsed = start.elapsed().as_secs_f64();
        terminal.draw(|frame| render_home(frame, elapsed, &greeting, &snap))?;
        if poll(Duration::from_millis(33))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Enter if elapsed > 2.8 => break,
                    KeyCode::Esc | KeyCode::Char('q') => break,
                    KeyCode::Char(' ') if elapsed > 2.8 => {
                        launch_btop(&mut terminal)?;
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
