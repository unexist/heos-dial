//
/// @package heos-dial
///
/// @file HEOS tui
/// @copyright 2024-present Christoph Kappel <christoph@unexist.dev>
/// @version $Id$
///
/// This program can be distributed under the terms of the GNU GPLv3.
/// See the file LICENSE for details.
///

use color_eyre::Result;
use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Constraint, Layout, Rect},
    style::{
        palette::tailwind::{BLUE, GREEN, SLATE},
        Color, Modifier, Style, Stylize,
    },
    symbols,
    text::Line,
    widgets::{
        Block, Borders, HighlightSpacing, List, ListItem, ListState, Padding, Paragraph,
        StatefulWidget, Widget, Wrap,
    },
    DefaultTerminal,
};
use heos_lib::HeosDevice;

const DEV_HEADER_STYLE: Style = Style::new().fg(SLATE.c100).bg(BLUE.c800);
const NORMAL_ROW_BG: Color = SLATE.c950;
const ALT_ROW_BG_COLOR: Color = SLATE.c900;
const SELECTED_STYLE: Style = Style::new().bg(SLATE.c800).add_modifier(Modifier::BOLD);
const TEXT_FG_COLOR: Color = SLATE.c200;
const COMPLETED_TEXT_FG_COLOR: Color = GREEN.c500;

pub struct App {
    should_exit: bool,
    dev_list: DeviceList,
}

struct DeviceList {
    items: Vec<HeosDevice>,
    state: ListState,
}

impl Default for App {
    fn default() -> Self {
        Self {
            should_exit: false,
            dev_list: DeviceList::from_iter([
                ("Studio1", "10.0.8.24", "844263156"),
                ("Studio2", "10.0.8.24", "844263156"),
                ("Fitness", "10.0.8.24", "844263156"),
                ("1st Floor", "10.0.8.24", "844263156"),
                ("Floor", "10.0.8.24", "844263156"),
                ("Kitchen", "10.0.8.24", "844263156"),
                ("Living Room (AVR)", "10.0.8.37", "-474905601"),
            ]),
        }
    }
}

impl FromIterator<(&'static str, &'static str, &'static str)> for DeviceList {
    fn from_iter<I: IntoIterator<Item = (&'static str, &'static str, &'static str)>>(iter: I) -> Self {
        let items = iter
            .into_iter()
            .map(|(name, url, pid)| HeosDevice::new(name, url, pid).unwrap())
            .collect();
        let state = ListState::default();

        Self { items, state }
    }
}

impl App {
    pub(crate) fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        while !self.should_exit {
            terminal.draw(|frame| frame.render_widget(&mut self, frame.area()))?;

            if let Event::Key(key) = event::read()? {
                self.handle_key(key);
            };
        }

        Ok(())
    }

    fn handle_key(&mut self, key: KeyEvent) {
        if KeyEventKind::Press != key.kind {
            return;
        }

        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => self.should_exit = true,
            KeyCode::Char('h') | KeyCode::Left => self.select_none(),
            KeyCode::Char('j') | KeyCode::Down => self.select_next(),
            KeyCode::Char('k') | KeyCode::Up => self.select_previous(),
            KeyCode::Char('g') | KeyCode::Home => self.select_first(),
            KeyCode::Char('G') | KeyCode::End => self.select_last(),
            KeyCode::Char('l') | KeyCode::Right | KeyCode::Enter => {
                self.toggle_status();
            }
            _ => {}
        }
    }

    fn select_none(&mut self) {
        self.dev_list.state.select(None);
    }

    fn select_next(&mut self) {
        self.dev_list.state.select_next();
    }
    fn select_previous(&mut self) {
        self.dev_list.state.select_previous();
    }

    fn select_first(&mut self) {
        self.dev_list.state.select_first();
    }

    fn select_last(&mut self) {
        self.dev_list.state.select_last();
    }

    fn toggle_status(&mut self) {
        if let Some(i) = self.dev_list.state.selected() {
            println!("Selected status: {}", self.dev_list.items[i].stream.is_some());
        }
    }
}

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [header_area, main_area, footer_area] = Layout::vertical([
            Constraint::Length(2),
            Constraint::Fill(1),
            Constraint::Length(1),
        ])
            .areas(area);

        let [list_area, item_area] =
            Layout::horizontal([Constraint::Fill(1), Constraint::Fill(5)]).areas(main_area);

        App::render_header(header_area, buf);
        App::render_footer(footer_area, buf);

        self.render_list(list_area, buf);
        self.render_selected_item(item_area, buf);
    }
}

/// Rendering logic for the app
impl App {
    fn render_header(area: Rect, buf: &mut Buffer) {
        Paragraph::new("Heos devices")
            .bold()
            .centered()
            .render(area, buf);
    }

    fn render_footer(area: Rect, buf: &mut Buffer) {
        Paragraph::new("Use ↓↑ to move, ← to unselect, → to change status, g/G to go top/bottom.")
            .centered()
            .render(area, buf);
    }

    fn render_list(&mut self, area: Rect, buf: &mut Buffer) {
        let block = Block::new()
            .title(Line::raw("Device List").centered())
            .borders(Borders::all())
            .border_set(symbols::border::PLAIN)
            .border_style(DEV_HEADER_STYLE)
            .bg(NORMAL_ROW_BG);

        let items: Vec<ListItem> = self
            .dev_list
            .items
            .iter()
            .enumerate()
            .map(|(i, dev_item)| {
                let color = alternate_colors(i);
                convert_dev_to_list_item(dev_item).bg(color)
            })
            .collect();

        let list = List::new(items)
            .block(block)
            .highlight_style(SELECTED_STYLE)
            .highlight_symbol(">")
            .highlight_spacing(HighlightSpacing::Always);

        StatefulWidget::render(list, area, buf, &mut self.dev_list.state);
    }

    fn render_selected_item(&self, area: Rect, buf: &mut Buffer) {
        let info = if let Some(i) = self.dev_list.state.selected() {
            match self.dev_list.items[i].stream {
                Some(_) => format!("✓ : {}", self.dev_list.items[i].name),
                None => format!("󰵙 : {}", self.dev_list.items[i].name),
            }
        } else {
            "Nothing selected...".to_string()
        };

        // We show the list item's info under the list in this paragraph
        let block = Block::new()
            .title(Line::raw("Device Info").centered())
            .borders(Borders::all())
            .border_set(symbols::border::PLAIN)
            .border_style(DEV_HEADER_STYLE)
            .bg(NORMAL_ROW_BG)
            .padding(Padding::horizontal(1));

        // We can now render the item info
        Paragraph::new(info)
            .block(block)
            .fg(TEXT_FG_COLOR)
            .wrap(Wrap { trim: false })
            .render(area, buf);
    }
}

const fn alternate_colors(i: usize) -> Color {
    if i % 2 == 0 {
        NORMAL_ROW_BG
    } else {
        ALT_ROW_BG_COLOR
    }
}

fn convert_dev_to_list_item(value: &HeosDevice) -> ListItem {
    let line = match value.stream {
        Some(_) => {
            Line::styled(format!(" ✓ {}", value.name), COMPLETED_TEXT_FG_COLOR)
        }
        None => Line::styled(format!(" 󰵙 {}", value.name), TEXT_FG_COLOR),
    };

    ListItem::new(line)
}
