use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{palette::tailwind::SLATE, Stylize},
    symbols,
    text::Line,
    widgets::{Block, Borders},
    DefaultTerminal, Frame,
};
use tui_bar_graph::BarGraph;

#[derive(Debug, Default)]
pub struct App {
    /// Is the application running?
    exit: bool,
}

impl App {
    /// Construct a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Run the application's main loop.
    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.render(frame))?;
            self.handle_crossterm_events()?;
        }
        Ok(())
    }

    /// Renders the user interface.
    fn render(&mut self, frame: &mut Frame) {
        let bg_block = Block::new().bg(SLATE.c800);
        frame.render_widget(bg_block, frame.area());

        let [header, top, mid, _] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Fill(1),
            Constraint::Fill(2),
            Constraint::Fill(1),
        ])
        .spacing(1)
        .margin(4)
        .areas(frame.area());

        let title = Line::raw("Blocks without borders")
            .fg(SLATE.c900)
            .bg(SLATE.c100);
        frame.render_widget(title, header);

        let [left, right] = Layout::horizontal([Constraint::Fill(1); 2])
            .spacing(2)
            .areas(top);

        render_graph("CPU", frame, left);
        render_graph("GPU", frame, right);

        let [left, right] = Layout::horizontal([Constraint::Fill(1), Constraint::Fill(2)])
            .spacing(2)
            .areas(mid);
        render_disk(frame, left);
        render_memory(frame, right);
    }

    /// Reads the crossterm events and updates the state of [`App`].
    ///
    /// If your application needs to perform work in between handling events, you can use the
    /// [`event::poll`] function to check if there are any events available with a timeout.
    fn handle_crossterm_events(&mut self) -> Result<()> {
        match event::read()? {
            // it's important to check KeyEventKind::Press to avoid handling key release events
            Event::Key(key) if key.kind == KeyEventKind::Press => self.on_key_event(key),
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
            _ => {}
        }
        Ok(())
    }

    /// Handles the key events and updates the state of [`App`].
    fn on_key_event(&mut self, key: KeyEvent) {
        match (key.modifiers, key.code) {
            (_, KeyCode::Esc | KeyCode::Char('q'))
            | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => self.quit(),
            // Add other key handlers here.
            _ => {}
        }
    }

    /// Set running to false to quit the application.
    fn quit(&mut self) {
        self.exit = true;
    }
}

fn render_graph(name: &str, frame: &mut Frame, area: Rect) {
    let block = Block::new()
        .borders(Borders::TOP)
        .border_set(symbols::border::FULL)
        .title(Line::raw(name).centered().fg(SLATE.c900).bg(SLATE.c300))
        .border_style(SLATE.c300)
        .bg(SLATE.c900);
    frame.render_widget(&block, area);

    let inner = block.inner(area);
    let data = (0..inner.width * 2)
        .map(|_| rand::random::<f64>())
        .collect::<Vec<f64>>();
    let graph = BarGraph::new(data).with_gradient(colorgrad::preset::plasma());
    frame.render_widget(graph, inner);
}

fn render_disk(frame: &mut Frame, area: Rect) {
    let block = Block::new()
        .borders(Borders::TOP)
        .border_set(symbols::border::FULL)
        .title(Line::raw("Disk").centered().fg(SLATE.c900).bg(SLATE.c300))
        .border_style(SLATE.c300)
        .bg(SLATE.c900);
    frame.render_widget(&block, area);
}

fn render_memory(frame: &mut Frame, area: Rect) {
    let block = Block::new()
        .borders(Borders::TOP)
        .border_set(symbols::border::FULL)
        .title(Line::raw("Memory").centered().fg(SLATE.c900).bg(SLATE.c300))
        .border_style(SLATE.c300)
        .bg(SLATE.c900);
    frame.render_widget(&block, area);

    let inner = block.inner(area);
    let data = (0..inner.width * 2)
        .map(|_| rand::random::<f64>())
        .collect::<Vec<f64>>();
    let graph = BarGraph::new(data).with_gradient(colorgrad::preset::blues());
    frame.render_widget(graph, inner);
}
