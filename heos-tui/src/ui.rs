///
/// @package heos-dial
///
/// @file HEOS tui
/// @copyright 2024-present Christoph Kappel <christoph@unexist.dev>
/// @version $Id$
///
/// This program can be distributed under the terms of the GNU GPLv3.
/// See the file LICENSE for details.
///

use ratatui::{style::{Color, Style}, symbols, widgets::{Block, Paragraph}};
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::prelude::{Line, Modifier, StatefulWidget, Stylize, Widget};
use ratatui::style::palette::tailwind::{GREEN, SLATE};
use ratatui::text::Span;
use ratatui::widgets::{Borders, Gauge, HighlightSpacing, List, ListItem, Padding, Wrap};
use tui_logger::{TuiLoggerLevelOutput, TuiLoggerWidget};
use heos_lib::HeosDevice;
use std::cmp::PartialEq;
use ratatui::style::palette::material::RED;
use crate::app::{App, Focus};

const HEADER_STYLE: Style = Style::new().fg(SLATE.c100).bg(SLATE.c800);
const SELECTED_STYLE: Style = Style::new().fg(GREEN.c100).bg(SLATE.c800).add_modifier(Modifier::BOLD);
const NORMAL_TEXT_FG_COLOR: Color = SLATE.c200;
const ACTIVE_TEXT_FG_COLOR: Color = GREEN.c100;
const ATTENTION_TEXT_FG_COLOR: Color = RED.c200;
const NORMAL_ROW_BG_COLOR: Color = SLATE.c950;
const ALT_ROW_BG_COLOR: Color = SLATE.c900;
const VOLUME_GAUGE_COLOR: Color = GREEN.c100;

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [header_area, main_area, footer_area] = Layout::vertical([
            Constraint::Length(2),
            Constraint::Fill(1),
            Constraint::Length(1),
        ]).areas(area);

        let [lists_area, item_area] =
            Layout::horizontal([Constraint::Fill(1), Constraint::Fill(5)]).areas(main_area);

        let [dev_list_area, group_list_area] =
            Layout::vertical([Constraint::Fill(4), Constraint::Fill(2)]).areas(lists_area);

        let [text_area, gauge_area, log_area] =
            Layout::vertical([Constraint::Fill(3), Constraint::Fill(1), Constraint::Fill(2)])
                .areas(item_area);

        render_header(header_area, buf);
        render_footer(footer_area, buf);

        render_dev_list(self, dev_list_area, buf);
        render_group_list(self, group_list_area, buf);

        render_selected_item(self, text_area, buf);
        render_gauge(self, gauge_area, buf);
        render_logger(self, log_area, buf);
    }
}

fn render_header(area: Rect, buf: &mut Buffer) {
    Paragraph::new("Heos devices")
        .bold()
        .centered()
        .render(area, buf);
}

fn render_footer(area: Rect, buf: &mut Buffer) {
    let lines = Line::from(vec![
        Span::raw("Use ↓ /↑ to move, ← /→  to lower/raise volume, g/d to select lists, p to play, s to stop."),
    ]);

    Paragraph::new(lines)
        .centered()
        .render(area, buf);
}

fn render_dev_list(app: &mut App, area: Rect, buf: &mut Buffer) {
    let style = match app.focus_state {
        Focus::Devices => SELECTED_STYLE,
        _ => HEADER_STYLE,
    };

    let block = Block::new()
        .title(Line::raw("Device List (d)").centered())
        .borders(Borders::all())
        .border_set(symbols::border::PLAIN)
        .border_style(style)
        .bg(NORMAL_ROW_BG_COLOR);

    let dev_list = app.dev_list.read().unwrap();

    let mut items: Vec<ListItem> = dev_list
        .iter()
        .enumerate()
        .map(|(i, dev_item)| {
            let color = alternate_colors(i);

            let line = match dev_item.volume {
                x if 0 < x => Line::styled(
                    format!("{:^5} {}", "🔊", dev_item.name), ACTIVE_TEXT_FG_COLOR),
                _ => Line::styled(
                    format!("{:^5} {}", "🔈", dev_item.name), NORMAL_TEXT_FG_COLOR),
            };

            ListItem::new(line).bg(color)
        })
        .collect();

    /* Check whether list is empty */
    if items.is_empty() {
        items.push(ListItem::new("No devices found"));
    }

    let list = List::new(items)
        .block(block)
        .highlight_style(SELECTED_STYLE)
        .highlight_symbol(">")
        .highlight_spacing(HighlightSpacing::Always);

    StatefulWidget::render(list, area, buf, &mut app.dev_list_state);
}

impl PartialEq for Focus {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}

fn render_group_list(app: &mut App, area: Rect, buf: &mut Buffer) {
    let style = match app.focus_state {
        Focus::Groups => SELECTED_STYLE,
        _ => HEADER_STYLE,
    };

    let block = Block::new()
        .title(Line::raw("Group List (g)").centered())
        .borders(Borders::all())
        .border_set(symbols::border::PLAIN)
        .border_style(style)
        .bg(NORMAL_ROW_BG_COLOR);

    let group_list = app.group_list.read().unwrap();

    let mut items: Vec<ListItem> = group_list
        .iter()
        .enumerate()
        .map(|(i, group_item)| {
            let color = alternate_colors(i);

            let line = Line::styled(format!("{:^5} {}", "∑",
                                            group_item.name), NORMAL_TEXT_FG_COLOR);

            ListItem::new(line).bg(color)
        })
        .collect();

    /* Check whether list is empty */
    if items.is_empty() {
        items.push(ListItem::new("No groups found"));
    }

    let list = List::new(items)
        .block(block)
        .highlight_style(SELECTED_STYLE)
        .highlight_symbol(">")
        .highlight_spacing(HighlightSpacing::Always);

    StatefulWidget::render(list, area, buf, &mut app.group_list_state);
}

fn render_selected_item(app: &App, area: Rect, buf: &mut Buffer) {
    let title = title_block("Device Info");
    let style = Style::new().italic();

    let mut lines = vec![];

    if let Some(dev) = get_selected_device(app) {
        lines.push(Line::styled(match dev.volume {
             x if 0 < x => format!("{:^4} : {}", "🔊", dev.name),
            _ => format!("{:^4} : {}", "🔈", dev.name),
        }, style));

        lines.push(Line::styled(format!("{:^5} : {}", "™", dev.model), style));
        lines.push(Line::styled(format!("{:^5} : {}", "🖧", dev.base_url), style));
        lines.push(Line::styled(format!("{:^4} : {}", "🆔", dev.player_id), style));
    } else {
        lines.push(Line::raw("Nothing selected yet".to_string()));
    }

    Paragraph::new(lines)
        .block(title)
        .fg(NORMAL_TEXT_FG_COLOR)
        .wrap(Wrap { trim: false })
        .render(area, buf);
}

fn render_gauge(app: &App, area: Rect, buf: &mut Buffer) {
    let title = title_block("Volume");
    let vol = match get_selected_device(app) {
        Some(dev) => dev.volume,
        None => 0,
    };

    Gauge::default()
        .block(title)
        .gauge_style(VOLUME_GAUGE_COLOR)
        .percent(vol)
        .render(area, buf);
}

fn render_logger(_app: &App, area: Rect, buf: &mut Buffer) {
    TuiLoggerWidget::default()
        .block(title_block("Heos Logs"))
        .style_error(Style::default().fg(ATTENTION_TEXT_FG_COLOR))
        .style_debug(Style::default().fg(ACTIVE_TEXT_FG_COLOR))
        .style_warn(Style::default().fg(ATTENTION_TEXT_FG_COLOR))
        .style_info(Style::default().fg(NORMAL_TEXT_FG_COLOR))
        .output_separator('|')
        .output_timestamp(Some("%F %H:%M:%S%.3f".to_string()))
        .output_level(Some(TuiLoggerLevelOutput::Long))
        .output_target(false)
        .output_file(false)
        .output_line(false)
        .render(area, buf);
}

const fn alternate_colors(i: usize) -> Color {
    if 0 == i % 2 {
        NORMAL_ROW_BG_COLOR
    } else {
        ALT_ROW_BG_COLOR
    }
}

fn title_block(title: &str) -> Block {
    let title = Line::from(title).centered();

    Block::default()
        .title(title)
        .borders(Borders::all())
        .border_set(symbols::border::PLAIN)
        .border_style(HEADER_STYLE)
        .bg(NORMAL_ROW_BG_COLOR)
        .padding(Padding::horizontal(1))
}

fn get_selected_device(app: &App) -> Option<HeosDevice> {
    if let Some(i) = app.dev_list_state.selected() {
        return app.dev_list.read().unwrap().get(i).cloned();
    }

    None
}
