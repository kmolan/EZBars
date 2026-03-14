#[derive(Clone)]
pub enum Theme {
    /// A standard solid bar: [████------]
    Standard(char, char),
    
    /// A high-resolution bar using fractional block characters
    Smooth,
    
    /// The classic retro ASCII spinner: | / - \
    Spinner,

    /// Braille dot spinner (Claude style)
    Claude,
    
    /// A bouncing block for indeterminate progress (like KITT / Cylon)
    Bouncing { block_width: usize, fill: char, empty: char },
}

impl Default for Theme {
    fn default() -> Self {
        Theme::Smooth
    }
}

impl Theme {
    /// Returns the standard bar with default '█' and '-'
    pub fn standard() -> Self {
        Theme::Standard('█', '-')
    }

    /// Returns a bouncing bar with a default width of 4 and '█' block
    pub fn bouncing() -> Self {
        Theme::Bouncing { 
            block_width: 4, 
            fill: '█', 
            empty: ' ' 
        }
    }
}

impl Theme {
    /// Renders the visual portion of the progress bar based on the active theme
    pub fn render(&self, width: usize, current: usize, total: usize) -> String {
        match self {
            Theme::Standard(fill, empty) => {
                let percent = if total == 0 { 1.0 } else { current as f64 / total as f64 };
                let filled_len = (percent * width as f64).round() as usize;
                let empty_len = width.saturating_sub(filled_len);
                
                format!("{}{}", fill.to_string().repeat(filled_len), empty.to_string().repeat(empty_len))
            }
            
            Theme::Smooth => {
                let percent = if total == 0 { 1.0 } else { current as f64 / total as f64 };
                // Calculate total out of 8ths (since there are 8 fractional block chars)
                let total_eighths = (percent * width as f64 * 8.0).round() as usize;
                let full_blocks = total_eighths / 8;
                let remainder = total_eighths % 8;

                let fractions = [' ', '▏', '▎', '▍', '▌', '▋', '▊', '▉'];
                
                let mut bar = "█".repeat(full_blocks);
                
                // Cap off the leading edge with the correct fractional block
                if full_blocks < width {
                    bar.push(fractions[remainder]);
                    let empty_len = width.saturating_sub(full_blocks + 1);
                    bar.push_str(&" ".repeat(empty_len));
                }
                bar
            }

            Theme::Spinner => {
                let chars = ['|', '/', '-', '\\'];
                let c = chars[current % chars.len()];
                let padding = width.saturating_sub(2);
                format!("{} {}", c, " ".repeat(padding))
            }

            Theme::Claude => {
                let chars = ['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'];
                let c = chars[current % chars.len()];
                let padding = width.saturating_sub(2);
                format!("{} {}", c, " ".repeat(padding))
            }

            Theme::Bouncing { block_width, fill, empty } => {
                if width <= *block_width {
                    return fill.to_string().repeat(width);
                }
                
                // Calculate back-and-forth movement
                let travel_distance = width - block_width;
                let cycle = current % (travel_distance * 2);
                let pos = if cycle < travel_distance { cycle } else { (travel_distance * 2) - cycle };

                let left_empty = empty.to_string().repeat(pos);
                let block = fill.to_string().repeat(*block_width);
                let right_empty = empty.to_string().repeat(width - pos - block_width);

                format!("{}{}{}", left_empty, block, right_empty)
            }
        }
    }
}