use std::default;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame, buffer::Buffer, layout::Rect, style::{Modifier, Stylize}, symbols::border, text::{Line}, widgets::{Block, List, ListState, StatefulWidget}
};

use color_eyre::{
    eyre::{WrapErr},
    Result,
};


//use crate::jsontodos;
use crate::{app::MenuState::Edit, json_todos::Todos};

#[derive(Debug)]
enum MenuState{
    Main,
    Edit(usize),//FIXME: the name of this state will be changed
}

impl Default for MenuState {
    fn default() -> Self {
        MenuState::Main
    }
}

// default function is not overriden
#[derive(Debug, Default)]
pub struct App {
    todos: Todos,
    filename: String,
    list_state: ListState,
    menu_state: MenuState,
    exit: bool,
}





impl App {
    //
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        self.filename = "todos.txt".to_string();
        let _ = self.todos.load(&self.filename, 0);

        while !self.exit{
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events().wrap_err("handle events failed")?;
        }
        Ok(())
    }
    
    fn draw(&mut self, frame: &mut Frame) {
        match self.menu_state{
            Edit(id) => {
                self.todos.flip(id);
                self.menu_state = MenuState::Main;
                },
            _ => {}
            
        }
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
            KeyCode::Char('f') => {
                match self.list_state.selected() {
                    Some(id) => self.menu_state = Edit(id.try_into().unwrap()),
                    None => ()
                }
            },
            _ => {}
        }
        Ok(())
    }

    fn exit(&mut self) {
        self.todos.save(&self.filename);
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
            " flip marking ".into(),
            "<F> ".blue().bold(),
            " Quit&Save ".into(),
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


