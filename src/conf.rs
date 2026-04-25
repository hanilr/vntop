use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use crate::ui::*;

// Informations of frames and contents.
pub const DEFAULT_CONFIGS: [(&str, &str, &str); 13] = [
        ("window_frame", "txt", "Variation: Table of Processes\n┤\n├\n─\n│\n┌\n┐\n└\n┘\ncyan\ncyan\nblack\nblack\nblack"),
        ("system_frame", "txt", "System\n┤\n├\n─\n│\n┌\n┐\n└\n┘\nblue\nblue\nblack\nblack\nblack"),
        ("cpu_frame", "txt", "CPU\n┤\n├\n─\n│\n┌\n┐\n└\n┘\nred\nred\nblack\nblack\nblack"),
        ("memory_frame", "txt", "Memory\n┤\n├\n─\n│\n┌\n┐\n└\n┘\ngreen\ngreen\nblack\nblack\nblack"),
        ("disk_frame", "txt", "Disks\n┤\n├\n─\n│\n┌\n┐\n└\n┘\nyellow\nyellow\nblack\nblack\nblack"),
        ("network_frame", "txt", "Networks\n┤\n├\n─\n│\n┌\n┐\n└\n┘\npurple\npurple\nblack\nblack\nblack"),
        ("process_frame", "txt", "Processes\n┤\n├\n─\n│\n┌\n┐\n└\n┘\nwhite\nwhite\nblack\nblack\nblack"),
        ("system_content", "txt", "Name\nHost\n1970\nUptime\nArch\nOS\nLong OS\nKernel\nLong Kernel\nblue\nwhite"),
        ("cpu_content", "txt", "Name\nUsage\nFreq\nBrand\nVendor\nLogical\nPhysical\nTotal Usage\nCore\nred\nwhite"),
        ("memory_content", "txt", "Total\nAvailable\nUsed\nFree\nTotal Swap\nUsed Swap\ngreen\nwhite"),
        ("disk_content", "txt", "Name\nMount\nKind\nFile Type\nTotal\nAvailable\nyellow\nwhite"),
        ("network_content", "txt", "Name\nMAC\nReceived\nTransmitted\nTotal Received\nTotal Transmitted\npurple\nwhite"),
        ("process_content", "txt", "Name\nUID\nPID\nCPU Usage\nMemory Usage\nStart Time\nStatus\ngray\nwhite"),
];

// Configurations base structure.
pub struct Conf {
    name: String,
    extension: String,
    content: String,
}

impl Conf {
    pub fn new(name: &str, extension: &str, content: &str) -> Conf {
        Conf {
            name: name.to_string(),
            extension: extension.to_string(),
            content: content.to_string(),
        }
    }

    pub fn is_exist(&self) -> bool {
        let file = format!("conf/{}.{}", self.name, self.extension);
        let path = Path::new(&file);
        path.exists()
    }

    pub fn apply(&self) {
        let mut f = File::create(format!("conf/{}.{}", self.name, self.extension))
            .expect("File not created.");
        
        f.write_all(self.content.as_bytes())
            .expect("File not wrote.");
    }

    pub fn get(&mut self) -> String {
        self.content = fs::read_to_string(format!("conf/{}.{}", self.name, self.extension) as String)
            .expect("File not read.");

        self.content.clone()
    }
}

// Content and data parser structure.
pub struct Parser {
    raw: String,
    cooked: Option<Vec<String>>,
}

impl Parser {
    pub fn new(raw: &str) -> Parser {
        Parser {
            raw: raw.to_string(),
            cooked: None,
        }
    }

    pub fn cook(&mut self) -> Vec<String> {
        let result = self.raw.lines()
            .map(|s| s.to_string())
            .collect();

        self.cooked = Some(result);
        self.cooked.as_ref().unwrap().clone()
    }
}

// Check if configuration files exist.
// If exist makes with file rules.
// If not creates and writes default configuration settings from `DEFAULT_CONFIGS`
pub fn is_config() {
    fs::create_dir_all("conf").expect("Folder not created.");
    for(name, extension, content) in DEFAULT_CONFIGS {
        let conf = Conf::new(name, extension, content);
        if !conf.is_exist() {
            conf.apply();
        }
    }
}

// Controls every frame and informations in screen.
pub fn is_frame(content_info: Vec<Vec<String>>, term_w: u8, term_h: u8) {
    let mut frames: Vec<Frame> = vec![];
    let mut contents: Vec<Vec<String>> = vec![];
    
    for (name, extension, _) in DEFAULT_CONFIGS {
        let mut conf = Conf::new(name, extension, "");

        if name.ends_with("_frame") && !name.starts_with("window") {
            let frame_raw = conf.get();
            let frame_cooked = Parser::new(&frame_raw).cook();

            frames.push(Frame::new(&frame_cooked[0], &frame_cooked[1],
                &frame_cooked[2], &frame_cooked[3], &frame_cooked[4],
                &frame_cooked[5], &frame_cooked[6], &frame_cooked[7],
                &frame_cooked[8], &frame_cooked[9], &frame_cooked[10],
                &frame_cooked[11], &frame_cooked[12], &frame_cooked[13]));
                
        } else if name.ends_with("_content") {
            let content_raw = conf.get();
            contents.push(Parser::new(&content_raw).cook());
        } 
    }

    let mut req_widths = vec![0; 6];
    let mut req_heights = vec![0; 6];

    for i in 0..6 {
        let title_len = frames[i].nl_sym.chars().count() 
                      + frames[i].name.chars().count() 
                      + frames[i].nr_sym.chars().count();
        let mut max_len = title_len + 4; 

        let content_len = content_info[i].len();
        req_heights[i] = content_len as u8 + 2; 

        for j in 0..content_len {
            let name_len = contents[i].get(j).map_or(0, |n| n.chars().count());
            let val_len = content_info[i].get(j).map_or(0, |v| v.chars().count());
            
            let line_needed = name_len + val_len + 6; 
            if line_needed > max_len {
                max_len = line_needed;
            }
        }
        
        let mut final_w = max_len as u8;
        
       let max_w = if i == 1 {
            term_w.saturating_sub(req_widths[0]).saturating_sub(6)
        } else {
            term_w.saturating_sub(6) 
        };
        
        if final_w > max_w { final_w = max_w; } 
        req_widths[i] = final_w;
    }

    let gap_x = 2; 
    let mut current_y = 2;
    let mut current_x = 2;
    let mut max_y_in_row = 2;

    for i in 0..5 {
        if i == 2 || current_x + req_widths[i] > term_w.saturating_sub(2) {
            current_x = 2;
            current_y = max_y_in_row + 1;
        }

        let dynamic_ui = UI::new(req_widths[i], req_heights[i], current_x, current_y);
        if current_y + req_heights[i] > max_y_in_row {
            max_y_in_row = current_y + req_heights[i];
        }

        let content_len = contents[i].len();
        let fg_color = &contents[i][content_len - 2];
        let bg_color = &contents[i][content_len - 1];

        let widget = StatWidget::new(
            dynamic_ui, 
            frames[i].clone(),
            content_info[i].clone(), 
            fg_color, 
            &frames[i].b_bg,
            contents[i].clone(), 
            bg_color, 
            &frames[i].b_bg
        );
        widget.draw();

        current_x += req_widths[i] + gap_x; 
    }

    let proc_idx = 5;
    let proc_y = max_y_in_row + 1; 
    let proc_h = term_h.saturating_sub(proc_y).saturating_sub(1); 
    let proc_w = term_w.saturating_sub(4); 

    if proc_h > 3 { 
        let dynamic_ui = UI::new(proc_w, proc_h, 2, proc_y);
        
        let content_len = contents[proc_idx].len();
        let fg_color = &contents[proc_idx][content_len - 2];
        let bg_color = &contents[proc_idx][content_len - 1];

        let widget = StatWidget::new(
            dynamic_ui, 
            frames[proc_idx].clone(),
            content_info[proc_idx].clone(), 
            fg_color, 
            &frames[proc_idx].b_bg,
            contents[proc_idx].clone(), 
            bg_color, 
            &frames[proc_idx].b_bg
        );
        widget.draw();
    }
}