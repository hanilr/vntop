use std::time::Duration;
use std::thread;

use vntop::info::*;
use vntop::conf::*;
use vntop::ui::*;

use sysinfo::{System, Disks, Networks};
 
fn main() {
    // Checks if configuration files exist or not.
    // If not then creates and writes.
    is_config(); 

    // Get terminal preferences and draw main frame.
    let terminal = UI::new_with_terminal_size();
    terminal.clean_terminal();

    let win = Parser::new(&Conf::new("window_frame", "txt", "").get()).cook();
    let window = Frame::new(&win[0], &win[1], &win[2], &win[3], 
        &win[4], &win[5], &win[6], &win[7], &win[8], &win[9], 
        &win[10], &win[11], &win[12], &win[13]);

    window.draw(terminal.clone()); // Main Frame

    // Get system informations then draw every widget.
    let mut sys = System::new_all();
    sys.refresh_all();

    let dsk = Disks::new_with_refreshed_list();
    let ntw = Networks::new_with_refreshed_list();

    let mut content_info: Vec<Vec<String>> = vec![];
    content_info.push(Parser::new(&VnSystem::new().raw_info()).cook());
    content_info.push(Parser::new(&VnCpu::new(&sys).raw_info()).cook());
    content_info.push(Parser::new(&VnMemory::new(&sys).raw_info()).cook());
    content_info.push(Parser::new(&VnDisk::new(&dsk).raw_info()).cook());
    content_info.push(Parser::new(&VnNetwork::new(&ntw).raw_info()).cook());
    content_info.push(Parser::new(&VnProcess::new(&sys).raw_info()).cook());

    is_frame(content_info, terminal.width.clone(), terminal.height.clone()); // Sub Frames

    // Goes end of terminal for better view.
    terminal.goto_terminal_end();
    thread::sleep(Duration::from_secs(1));
}