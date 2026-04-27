use colored::*;

// Loop print optimized macro.
macro_rules! printf {
    ($($arg:tt)*) => {
        print!($($arg)*);
        //std::io::stdout().flush().unwrap();
    };
}

// Ascii escape sequence terminal cursor manipulation macro.
macro_rules! gotoxy {
    ($y:expr, $x:expr) => {
        print!("\x1B[{};{}H", $y, $x);
    };
}

// Ascii escape sequence terminal clean.
macro_rules! clean {
    () => {
        print!("\x1B[2J\x1B[1;1H");
        use std::io::Write;
        std::io::stdout().flush().unwrap();
    };
}

// UI base structure.
#[derive(Clone)]
pub struct UI {
    pub width: u8,
    pub height: u8,
    pub pos_x: u8,
    pub pos_y: u8,
}

impl UI {
    pub fn new(width: u8, height: u8, pos_x: u8, pos_y: u8) -> UI {
        UI {
            width: width,
            height: height,
            pos_x: pos_x,
            pos_y: pos_y,
        }
    }

    pub fn goto_terminal_end(&self) {
        gotoxy!(self.height, 0);
    }

    pub fn goto(&self, pos_y: u8, pos_x: u8) {
        gotoxy!(pos_y, pos_x);
    }

    pub fn clean_terminal(&self) {
        clean!();
    }
}

// Every frames base structure.
#[derive(Clone)]
pub struct Frame {
    // Name, Name's Left and Right Symbols
    pub name: String,
    pub nl_sym: String,
    pub nr_sym: String,

    // Horizontal and Vertical Symbols
    h_sym: String,
    v_sym: String,
    
    // Left Upper, Right Upper, Left Down and Right Down Symbols
    lu_sym: String,
    ru_sym: String,
    pub ld_sym: String,
    pub rd_sym: String,

    // Horizontal and Vertical Foreground and Background Colors
    h_fg: String,
    pub v_fg: String,
    h_bg: String,
    pub v_bg: String,

    // Background Color
    pub b_bg: String,
}

impl Frame {
    pub fn new(name: &str, nl_sym: &str, nr_sym: &str, h_sym: &str, v_sym: &str, lu_sym: &str, ru_sym: &str, ld_sym: &str,
        rd_sym: &str, h_fg: &str, v_fg: &str, h_bg: &str, v_bg: &str, b_bg: &str) -> Frame {
        Frame {
            // Name, Name's Left and Right Symbols
            name: name.to_string(),
            nl_sym: nl_sym.to_string(),
            nr_sym: nr_sym.to_string(),

            // Horizontal and Vertical Symbols
            h_sym: h_sym.to_string(),
            v_sym: v_sym.to_string(),

            // Left Upper, Right Upper, Left Down and Right Down Symbols
            lu_sym: lu_sym.to_string(),
            ru_sym: ru_sym.to_string(),
            ld_sym: ld_sym.to_string(),
            rd_sym: rd_sym.to_string(),

            // Horizontal and Vertical Foreground and Background Colors
            h_fg: h_fg.to_string(),
            v_fg: v_fg.to_string(),
            h_bg: h_bg.to_string(),
            v_bg: v_bg.to_string(),

            // Background Color
            b_bg: b_bg.to_string(),
        }
    }

    pub fn draw(&self, ui: UI) {
        for y in 0..ui.height {
            gotoxy!(ui.pos_y + 1 + y, ui.pos_x + 1);

            if y == 0 {
                printf!("{}", self.lu_sym.color(self.v_fg.clone()).on_color(self.v_bg.clone()));
                for _ in 0..(ui.width - 2) {
                    printf!("{}", self.h_sym.color(self.h_fg.clone()).on_color(self.h_bg.clone()));
                }
                printf!("{}", self.ru_sym.color(self.v_fg.clone()).on_color(self.v_bg.clone()));

                let title_str = format!("{}{}{}", self.nl_sym, self.name, self.nr_sym);
                let t_len = title_str.chars().count() as u8;
                let offset = (ui.width.saturating_sub(t_len)) / 2;
                
                gotoxy!(ui.pos_y + 1, ui.pos_x + 1 + offset);
                printf!("{}", title_str.color(self.h_fg.clone()).on_color(self.h_bg.clone()));

            } else if y == ui.height - 1 { 
                printf!("{}", self.ld_sym.color(self.v_fg.clone()).on_color(self.v_bg.clone()));

                for _ in 0..(ui.width - 2) {
                    printf!("{}", self.h_sym.color(self.h_fg.clone()).on_color(self.h_bg.clone()));
                }
                printf!("{}", self.rd_sym.color(self.v_fg.clone()).on_color(self.v_bg.clone()));
                
            } else {
                printf!("{}", self.v_sym.color(self.v_fg.clone()).on_color(self.v_bg.clone()));

                for _ in 0..(ui.width - 2) {
                    printf!("{}", " ".on_color(self.b_bg.clone()));
                }
                printf!("{}", self.v_sym.color(self.v_fg.clone()).on_color(self.v_bg.clone()));
            }
        }
    }
}

// Every information block's structure.
pub struct StatWidget {
    ui: UI,
    frame: Frame,
    content: Vec<String>,
    content_fg: String,
    content_bg: String,
    names: Vec<String>,
    names_fg: String,
    names_bg: String,
}

impl StatWidget {
    pub fn new(ui: UI, frame: Frame, content: Vec<String>, content_fg: &str,
        content_bg: &str, names: Vec<String>, names_fg: &str, names_bg: &str) -> StatWidget {
        StatWidget { 
            ui: ui,
            frame: frame,
            content: content,
            content_fg: content_fg.to_string(),
            content_bg: content_bg.to_string(),
            names: names,
            names_fg: names_fg.to_string(),
            names_bg: names_bg.to_string(),
        }
    }

    pub fn draw(&self) {
        let widget = Frame::new(&self.frame.name, &self.frame.nl_sym,
            &self.frame.nr_sym, &self.frame.h_sym, &self.frame.v_sym,
            &self.frame.lu_sym, &self.frame.ru_sym, &self.frame.ld_sym,
            &self.frame.rd_sym, &self.frame.h_fg, &self.frame.v_fg,
            &self.frame.h_bg, &self.frame.v_bg, &self.frame.b_bg);

        widget.draw(self.ui.clone());
        let max_lines = (self.ui.height.saturating_sub(2)) as usize;

        for i in 0..self.content.len() {
            if i >= max_lines { break; } 

            let mut val = self.content.get(i).unwrap_or(&String::new()).clone();

            if self.frame.name != "Processes" {
                let name_len = self.names.get(i).map_or(0, |n| n.chars().count());
                let max_val_len = (self.ui.width as usize).saturating_sub(name_len + 6);
                if val.chars().count() > max_val_len {
                    val = format!("{}...", val.chars().take(max_val_len.saturating_sub(3)).collect::<String>());
                }
            }

            gotoxy!(self.ui.pos_y + 2 + i as u8, self.ui.pos_x + 3); 
            if self.frame.name == "Processes" {
                printf!("{}", val.color(self.content_fg.clone()).on_color(self.content_bg.clone()));
            } else {
                let name = self.names.get(i).unwrap_or(&String::new()).clone();
                printf!("{}{}{}", 
                    name.color(self.names_fg.clone()).on_color(self.names_bg.clone()),
                    ": ".color(self.names_fg.clone()).on_color(self.names_bg.clone()),
                    val.color(self.content_fg.clone()).on_color(self.content_bg.clone())
                );
            }
        }
    }
}