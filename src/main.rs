use std::time::Duration;
use std::thread;
use std::io::Write;

use vntop::info::*;
use vntop::conf::*;
use vntop::ui::*;

use sysinfo::{System, Disks, Networks};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crossterm::terminal;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
 
fn main() {
    // Checks if configuration files exist or not.
    // If not then creates and writes.
    is_config(); 

    enable_raw_mode().expect("Terminal raw mode issue.");
    let live = thread::spawn(move || {
        // Get system informations then draw every widget.
        let mut sys = System::new_all();
        let mut dsk = Disks::new_with_refreshed_list();
        let mut ntw = Networks::new_with_refreshed_list();

        // Process sorting variable.
        let mut sort_cpu = true;

        // Get terminal preferences and draw main frame.
        let (width, height) = terminal::size().unwrap_or((80, 24));
        let mut terminal = UI::new(width as u8, height as u8, 0, 0);
        terminal.clean_terminal();

        let win = Parser::new(&Conf::new("window_frame", "txt", "").get()).cook();
        let window = Frame::new(&win[0], &win[1], &win[2], &win[3], 
            &win[4], &win[5], &win[6], &win[7], &win[8], &win[9], 
            &win[10], &win[11], &win[12], &win[13]);

        window.draw(terminal.clone()); // Main Frame
        is_keymap(terminal.clone(), window.clone(), "yellow"); // Keymap

        loop {
            if event::poll(Duration::from_millis(50)).unwrap() {
                let mut last_resize = None;
                        
                while event::poll(Duration::from_secs(0)).unwrap() {
                    if let Event::Key(key) = event::read().unwrap() {
                        if key.kind == KeyEventKind::Press {
                            match key.code {
                                KeyCode::Char('q') => return,
                                KeyCode::F(1) => {
                                    sort_cpu = !sort_cpu;
                                }
                                KeyCode::F(2) => {
                                    // Process Search
                                }
                                _ => {}
                            }
                        }
                    } else if let Event::Resize(nw, nh) = event::read().unwrap() {
                        last_resize = Some((nw, nh));
                    }
                }
            
                if let Some((new_width, new_height)) = last_resize {
                    terminal.width = new_width as u8;
                    terminal.height = new_height as u8;
                    terminal.clean_terminal();
                    window.draw(terminal.clone());
                    is_keymap(terminal.clone(), window.clone(), "yellow");
                }
            }

            sys.refresh_all(); // First refresh for start time.
            thread::sleep(Duration::from_millis(100));
            sys.refresh_all(); // Second refresh for measure cpu usage.

            dsk.refresh(false);
            ntw.refresh(false);
            
            let mut content_info: Vec<Vec<String>> = vec![];
            content_info.push(Parser::new(&VnSystem::new().raw_info()).cook());
            content_info.push(Parser::new(&VnCpu::new(&sys).raw_info()).cook());
            content_info.push(Parser::new(&VnMemory::new(&sys).raw_info()).cook());
            content_info.push(Parser::new(&VnDisk::new(&dsk).raw_info()).cook());
            content_info.push(Parser::new(&VnNetwork::new(&ntw).raw_info()).cook());
            content_info.push(Parser::new(&VnProcess::new(&sys).raw_info(sort_cpu)).cook());
            
            is_frame(content_info, terminal.width.clone(), terminal.height.clone()); // Sub Frames
            
            // Goes end of terminal for better view.
            terminal.goto_terminal_end();
            std::io::stdout().flush().unwrap();
            thread::sleep(Duration::from_millis(850));
        }
    });
    let _ = live.join();
    disable_raw_mode().unwrap();
}