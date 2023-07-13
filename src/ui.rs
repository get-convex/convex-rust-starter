//! Simple curses/TUI-based UI for showing the chat messages in the
//! Convex deployment
use convex::Value;
use std::io::{self, Stdout};
use tui::{
    backend::CrosstermBackend,
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph, Wrap},
    Terminal,
};

const NICK_COLORS: &[Color] = &[
    Color::Cyan,
    Color::Red,
    Color::Green,
    Color::Yellow,
    Color::Magenta,
    Color::Blue,
    Color::Gray,
];

pub struct UI {
    terminal: Terminal<CrosstermBackend<Stdout>>,
    color_reservations: Vec<String>,
}

impl UI {
    pub fn new() -> Self {
        let stdout = io::stdout();
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend).expect("failed to create terminal");
        let mut me = Self {
            terminal,
            color_reservations: vec![],
        };
        me.draw_chat(None);
        me
    }

    pub fn update(&mut self, messages: Vec<Value>) {
        // Grab the last 100 messages -- the window can't possible be bigger than that
        let idx = if messages.len() > 100 {
            messages.len() - 100
        } else {
            0
        };
        let messages = &messages[idx..];
        let mut text = vec![];
        for m in messages {
            if let Value::Object(o) = m {
                let author = if let Value::String(author) = o.get("author").unwrap() {
                    author
                } else {
                    panic!("wrong type")
                };
                let body = if let Value::String(body) = o.get("body").unwrap() {
                    body
                } else {
                    panic!("wrong type")
                };
                text.push(Spans::from(vec![
                    Span::styled(
                        format!("<{}>", author),
                        Style::default().fg(self.get_author_color(author)),
                    ),
                    Span::raw(" "),
                    Span::raw(body.clone()),
                ]));
            } else {
                panic!("Weird, unexpected return value type");
            }
        }
        let p = Paragraph::new(text).wrap(Wrap { trim: false });

        self.draw_chat(Some((p, messages.len() as u16)));
    }

    fn get_author_color(&mut self, author: &str) -> Color {
        for i in 0..self.color_reservations.len() {
            if self.color_reservations[i] == author {
                return NICK_COLORS[i % NICK_COLORS.len()];
            }
        }
        let idx = self.color_reservations.len();
        self.color_reservations.push(author.to_owned());
        NICK_COLORS[idx % NICK_COLORS.len()]
    }

    fn draw_chat(&mut self, inner: Option<(Paragraph, u16)>) {
        self.terminal.clear().expect("failed to clear terminal");
        self.terminal
            .draw(|f| {
                let size = f.size();
                let block = Block::default().title("Convex Chat").borders(Borders::ALL);
                let block_area = block.inner(size);
                f.render_widget(block, size);
                if let Some((mut p, lines)) = inner {
                    let height = block_area.height - 2;
                    if lines > height {
                        p = p.scroll((lines - height, 0));
                    }
                    f.render_widget(p, block_area);
                }
            })
            .expect("failed to draw to terminal");
    }
}
