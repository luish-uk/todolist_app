use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame, buffer::Buffer, layout::Rect, style::{Modifier, Stylize}, symbols::border, text::{Line, Text}, widgets::{Block, List, ListState, Paragraph, StatefulWidget, Widget}
};

use color_eyre::{
    eyre::{bail, WrapErr},
    Result,
};


//use crate::jsontodos;
use crate::json_todos::Todos;





// default function is not overriden
#[derive(Debug, Default)]
pub struct App {
    todos: Todos,
    list_state: ListState,
    exit: bool,
}





impl App {
    //
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        let _ = self.todos.load("todos.txt", 0);

        while !self.exit{
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events().wrap_err("handle events failed")?;
        }
        Ok(())
    }
    
    fn draw(&mut self, frame: &mut Frame) {
        self.render(frame.area(), frame.buffer_mut());
    }

    fn handle_events(&mut self) -> Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => self
                .handle_key_event(key_event)
                .wrap_err_with(|| format!("handling key event failed:\n{key_event:#?}")),
            _ => Ok(()),
            }
        }

    fn handle_key_event(&mut self, key_event: KeyEvent) -> Result<()> {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Up => self.list_state.select_previous(),
            KeyCode::Down => self.list_state.select_next(),
            _ => {}
        }
        Ok(())
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}

impl App {
    fn render(&mut self, area: Rect, buf: &mut Buffer) { 
        
        let title = Line::from(" Counter App Tutorial ".bold());
        let instructions = Line::from(vec![
            " moving selection ".into(),
            "<Up>".blue().bold(),
            "/".into(),
            "<Down>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);
        
        

        let list  = List::new(self.todos.clone().list_format());
      
        StatefulWidget::render(
            list
                .block(block)
                .highlight_style(Modifier::REVERSED),
            area,
            buf,
            &mut self.list_state,
        );
    }
}


