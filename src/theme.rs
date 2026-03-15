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

    Pacman,

    Heartbeat,

    DVD,

    WaterLevel,

    Fish,

    Snake,

    Waves,

    FillUp,

    Arrows,

    FishBounce,

    DotWaves,

    Banner(String),
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

            Theme::Pacman => {
                let percent = if total == 0 { 0.0 } else { current as f64 / total as f64 };
                let pos = (percent * width as f64) as usize;
                
                // Toggle mouth every iteration
                let mouth = if current % 2 == 0 { "ᗧ" } else { "O" };
                
                let eaten = " ".repeat(pos);
                let food = "•".repeat(width.saturating_sub(pos + 1));
                
                format!("{}{}{}", eaten, mouth, food)
            }

            Theme::Heartbeat => {
                let percent = if total == 0 { 0.0 } else { current as f64 / total as f64 };
                let pos = (percent * width as f64) as usize;
                
                // The EKG pulse pattern
                let ekg = "-/\\--"; 
                // The "heart" blip - alternates between a dot and a check/pulse
                let blip = if current % 4 == 0 { "✓" } else { "•" };
                
                let mut line = String::with_capacity(width);
                for i in 0..width {
                    if i == pos {
                        line.push_str(blip);
                    } else if i < pos {
                        // Trail: Repeat the EKG pattern
                        let char_vec: Vec<char> = ekg.chars().collect();
                        line.push(char_vec[i % char_vec.len()]);
                    } else {
                        // Flatline ahead
                        line.push('-');
                    }
                }
                
                // Ensure we stay within width
                line.chars().take(width).collect()
            }

            Theme::DVD => {
                let logo = "DVD";
                let logo_len = logo.len();
                
                if width <= logo_len {
                    return logo.to_string();
                }

                let travel_dist = width - logo_len;
                // Move back and forth based on the current iteration
                let cycle = current % (travel_dist * 2);
                let pos = if cycle < travel_dist {
                    cycle
                } else {
                    (travel_dist * 2) - cycle
                };

                let left_pad = " ".repeat(pos);
                let right_pad = " ".repeat(width - pos - logo_len);
                
                format!("{}{}{}", left_pad, logo, right_pad)
            }

            Theme::WaterLevel => {
                // Fills from bottom-up using vertical blocks
                let levels = [' ', '▂', '▃', '▄', '▅', '▆', '▇', '█'];
                let percent = if total == 0 { 0.0 } else { current as f64 / total as f64 };
                let idx = (percent * (levels.len() - 1) as f64) as usize;
                levels[idx].to_string().repeat(width)
            }

            Theme::Fish => {
                let fish_chars: Vec<char> = "><(((°>".chars().collect();
                let fish_len = fish_chars.len();
                
                // Calculate position based on percentage (0.0 to 1.0)
                let percent = if total == 0 { 0.0 } else { current as f64 / total as f64 };
                
                // We calculate 'pos' so that at 100%, the fish head is at the far right
                let pos = (percent * (width + fish_len) as f64) as i32;
                let start = pos - fish_len as i32;

                let mut res = String::with_capacity(width);
                for i in 0..width {
                    let i_i32 = i as i32;
                    if i_i32 >= start && i_i32 < start + fish_len as i32 {
                        let idx = (i_i32 - start) as usize;
                        res.push(fish_chars[idx]);
                    } else {
                        res.push('~');
                    }
                }
                res
            }

            Theme::Snake => {
                // A snake slithering across (~~~~~🐍)
                let travel = width + 6; // body + emoji
                let pos = current % travel;
                // We'll just simplify the rendering for the showcase
                let mut line: Vec<char> = " ".repeat(width).chars().collect();
                for i in 0..6 {
                    let p = pos as i32 - i as i32;
                    if p >= 0 && p < width as i32 {
                        line[p as usize] = if i == 0 { 'S' } else { '~' }; // Using 'S' to avoid emoji width bugs in some terminals
                    }
                }
                line.into_iter().collect()
            }

            Theme::Waves => {
                let wave_chars: Vec<char> = "▁▅▇██▇▅▁".chars().collect();
                let n = wave_chars.len();
                let mut res = String::with_capacity(width);
                
                for i in 0..width {
                    // (i + current) shifts the pattern leftwards over time
                    // creating the "Right to Left" visual flow
                    let idx = (i + current) % n;
                    res.push(wave_chars[idx]);
                }
                res
            }

            Theme::FillUp => {
                let levels = [' ', '▂', '▃', '▄', '▅', '▆', '▇', '█'];
                let percent = if total == 0 { 0.0 } else { current as f64 / total as f64 };
                
                // total_points represents the "height" across the whole bar
                // If width is 10, total_points goes from 0 to 80 (10 blocks * 8 levels)
                let total_points = (percent * width as f64 * (levels.len() - 1) as f64) as usize;
                
                let full_blocks_count = total_points / (levels.len() - 1);
                let partial_height_idx = total_points % (levels.len() - 1);

                let mut bar = String::with_capacity(width * 3);
                
                // 1. Add the fully grown blocks
                for _ in 0..full_blocks_count {
                    bar.push('█');
                }

                // 2. Add the single block that is currently "growing" upwards
                if full_blocks_count < width {
                    bar.push(levels[partial_height_idx]);
                    
                    // 3. Fill the rest with empty space to maintain width
                    for _ in (full_blocks_count + 1)..width {
                        bar.push(' ');
                    }
                }

                bar
            }

            Theme::Arrows => {
                let percent = if total == 0 { 0.0 } else { current as f64 / total as f64 };
                let pos = (percent * width as f64) as usize;
                let mut bar = ">".repeat(pos.saturating_sub(1));
                if pos > 0 { bar.push('>'); }
                bar.push_str(&" ".repeat(width.saturating_sub(pos)));
                bar
            }

            Theme::FishBounce => {
                let fish_chars: Vec<char> = "><(((°>".chars().collect();
                let fish_len = fish_chars.len();
                let travel_dist = width.saturating_sub(fish_len);
                
                if travel_dist == 0 { return "><(((°>".to_string(); }

                let cycle = current % (travel_dist * 2);
                let pos = if cycle < travel_dist { cycle } else { (travel_dist * 2) - cycle };
                
                // Flip the fish icon based on direction!
                let fish_icon = if cycle < travel_dist { "><(((°>" } else { "<°)))><" };

                format!("{}{}{}", " ".repeat(pos), fish_icon, " ".repeat(width - pos - fish_len))
            }

            Theme::DotWaves => {
                let dots: Vec<char> = "⠁⠈⠐⠠⢀⡀⠄⠂".chars().collect();
                let mut res = String::with_capacity(width * 3); // Braille is 3 bytes
                for i in 0..width {
                    // Right-to-left flow
                    let idx = (i + current) % dots.len();
                    res.push(dots[idx]);
                }
                res
            }

            Theme::Banner(text) => {
                let text_chars: Vec<char> = text.chars().collect();
                let n = text_chars.len();
                let mut res = String::with_capacity(width);
                
                for i in 0..width {
                    // This creates a continuous scrolling banner effect
                    let idx = (i + current) % n;
                    res.push(text_chars[idx]);
                }
                res
            }
        }
    }
}