#[derive(Clone, Default)]
pub enum Style {
    /// [████------] | Deterministic | Fixed-character block bar.
    /// Parameters: (filled_char, empty_char)
    Classic(char, char),

    /// [███▌      ] | Deterministic | High-resolution sub-pixel progress using Unicode 1/8th blocks.
    #[default]
    Fractional,

    /// [ █▂    ] | Deterministic | Sequential vertical fill of individual cells from left to right.
    VerticalFill,

    /// [  ███   ] | Indeterminate | Ping-pong animation of a sliding block.
    /// Parameters: (block_width, filled_char, empty_char)
    Bouncing(usize, char, char),

    /// [ ██████ ] | Deterministic | Linear RGB interpolation between two 24-bit hex colors.
    /// Parameters: (start_hex: String, end_hex: String)
    Gradient(String, String),

    /// [ ━━━━━━ ] | Deterministic | Minimalist TrueColor slim-line bar for modern terminal emulators.
    /// Parameters: (fill_hex: String, background_hex: String)
    ModernSlim(String, String),

    /// [ ━━━    ] | Indeterminate | Temporal-based color-shifting pattern for indeterminate states.
    /// Parameters: (primary_hex: String, secondary_hex: String)
    Marquee(String, String),

    /// [ | ] | Indeterminate | Sequential ASCII rotation: `|`, `/`, `-`, `\`.
    AsciiSpinner,

    /// [ ⠋ ] | Indeterminate | Modern high-density rotation using 8-dot Braille patterns.
    BrailleSpinner,

    /// [ Hello! ] | Indeterminate | Horizontal marquee for strings longer than the bar width.
    TextTicker(String),

    /// [  DVD   ] | Indeterminate | Bouncing logo logic that mirrors terminal screen-saver behavior.
    DVD,

    /// [ ᗧ • • ] | Deterministic | Progression-based animation with alternating "mouth" states.
    Pacman,

    /// [ -/\--• ] | Deterministic | Dynamic EKG line with a leading pulse blip.
    EKG,

    /// [ ▂▃▅▆▇ ] | Deterministic | Global vertical fill level across the entire bar width.
    WaterLevel,

    /// [ ><(((°> ] | Deterministic | Linear translation of an entity from left to right.
    Fish,

    /// [ ▁▅▇██▇▅ ] | Indeterminate | Procedural sine-wave oscillation using block height variations.
    Waves,

    /// [ >>>>   ] | Deterministic | Head-heavy directional bar using ASCII chevrons.
    Arrows,

    /// [ 🚀~~~~ ] | Deterministic | Leading emoji entity with ANSI-colored exhaust and starfield.
    Rocket,

    /// [ <°)))>< ] | Indeterminate | Ping-pong entity movement with directional sprite-flipping.
    FishBounce,

    /// [ ⠁⠈⠐⠠ ] | Indeterminate | High-speed rippling effect utilizing temporal Braille offsets.
    DotWaves,

    /// [ ~~~🐱  ] | Deterministic | 256-color rainbow trail with a trailing cat.
    NyanCat,
}

impl Style {
    fn hex_to_rgb(hex: &str) -> (u8, u8, u8) {
        let hex = hex.trim_start_matches('#');

        let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
        let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
        let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);

        (r, g, b)
    }

    /// Renders the visual portion of the progress bar based on the active style
    pub fn render(&self, width: usize, current: usize, total: usize) -> String {
        match self {
            Style::Classic(fill, empty) => {
                let percent = if total == 0 {
                    1.0
                } else {
                    current as f64 / total as f64
                };
                let filled_len = (percent * width as f64).round() as usize;
                let empty_len = width.saturating_sub(filled_len);

                format!(
                    "{}{}",
                    fill.to_string().repeat(filled_len),
                    empty.to_string().repeat(empty_len)
                )
            }

            Style::Fractional => {
                let percent = if total == 0 {
                    1.0
                } else {
                    current as f64 / total as f64
                };
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

            Style::AsciiSpinner => {
                let chars = ['|', '/', '-', '\\'];
                let c = chars[current % chars.len()];
                let padding = width.saturating_sub(2);
                format!("{} {}", c, " ".repeat(padding))
            }

            Style::BrailleSpinner => {
                let chars = ['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'];
                let c = chars[current % chars.len()];
                let padding = width.saturating_sub(2);
                format!("{} {}", c, " ".repeat(padding))
            }

            Style::Bouncing(block_width, fill, empty) => {
                if width <= *block_width {
                    return fill.to_string().repeat(width);
                }

                // Calculate back-and-forth movement
                let travel_distance = width - block_width;
                let cycle = current % (travel_distance * 2);
                let pos = if cycle < travel_distance {
                    cycle
                } else {
                    (travel_distance * 2) - cycle
                };

                let left_empty = empty.to_string().repeat(pos);
                let block = fill.to_string().repeat(*block_width);
                let right_empty = empty.to_string().repeat(width - pos - block_width);

                format!("{}{}{}", left_empty, block, right_empty)
            }

            Style::Pacman => {
                let percent = if total == 0 {
                    0.0
                } else {
                    current as f64 / total as f64
                };
                let pos = (percent * width as f64) as usize;

                // Toggle mouth every iteration
                let mouth = if current % 2 == 0 { "ᗧ" } else { "O" };

                let eaten = " ".repeat(pos);
                let food = "•".repeat(width.saturating_sub(pos + 1));

                format!("{}{}{}", eaten, mouth, food)
            }

            Style::EKG => {
                let percent = if total == 0 {
                    0.0
                } else {
                    current as f64 / total as f64
                };
                let pos = (percent * width as f64) as usize;

                // The EKG pulse pattern
                let ekg = "-/\\--";
                // The "heart" blip - alternates between a dot and a check/pulse
                let blip = if current % 4 == 0 { "✓" } else { "•" };

                let mut line = String::with_capacity(width);
                for i in 0..width {
                    match i.cmp(&pos) {
                        std::cmp::Ordering::Equal => {
                            line.push_str(blip);
                        }
                        std::cmp::Ordering::Less => {
                            // Trail: Repeat the EKG pattern
                            let char_vec: Vec<char> = ekg.chars().collect();
                            line.push(char_vec[i % char_vec.len()]);
                        }
                        std::cmp::Ordering::Greater => {
                            // Flatline ahead
                            line.push('-');
                        }
                    }
                }

                // Ensure we stay within width
                line.chars().take(width).collect()
            }

            Style::DVD => {
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

            Style::WaterLevel => {
                // Fills from bottom-up using vertical blocks
                let levels = [' ', '▂', '▃', '▄', '▅', '▆', '▇', '█'];
                let percent = if total == 0 {
                    0.0
                } else {
                    current as f64 / total as f64
                };
                let idx = (percent * (levels.len() - 1) as f64) as usize;
                levels[idx].to_string().repeat(width)
            }

            Style::Fish => {
                let fish_chars: Vec<char> = "><(((°>".chars().collect();
                let fish_len = fish_chars.len();

                // Calculate position based on percentage (0.0 to 1.0)
                let percent = if total == 0 {
                    0.0
                } else {
                    current as f64 / total as f64
                };

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

            Style::Waves => {
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

            Style::VerticalFill => {
                let levels = [' ', '▂', '▃', '▄', '▅', '▆', '▇', '█'];
                let percent = if total == 0 {
                    0.0
                } else {
                    current as f64 / total as f64
                };

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

            Style::Arrows => {
                let percent = if total == 0 {
                    0.0
                } else {
                    current as f64 / total as f64
                };
                let pos = (percent * width as f64) as usize;
                let mut bar = ">".repeat(pos.saturating_sub(1));
                if pos > 0 {
                    bar.push('>');
                }
                bar.push_str(&" ".repeat(width.saturating_sub(pos)));
                bar
            }

            Style::Rocket => {
                let percent = if total == 0 {
                    0.0
                } else {
                    current as f64 / total as f64
                };
                let pos = (percent * width as f64) as usize;

                let mut bar = String::with_capacity(width);

                for i in 0..width {
                    match i.cmp(&pos) {
                        std::cmp::Ordering::Equal => {
                            bar.push('🚀');
                        }
                        std::cmp::Ordering::Less => {
                            // The exhaust trail (fire and smoke)
                            if (pos - i) < 3 {
                                bar.push_str("\x1b[38;5;208m~\x1b[0m"); // Orange fire
                            } else {
                                bar.push_str("\x1b[38;5;244m.\x1b[0m"); // Gray smoke
                            }
                        }
                        std::cmp::Ordering::Greater => {
                            // Stars in the distance (randomly twinkle using current)
                            if (i + current) % 7 == 0 {
                                bar.push('*');
                            } else {
                                bar.push(' ');
                            }
                        }
                    }
                }

                bar
            }

            Style::FishBounce => {
                let fish_chars: Vec<char> = "><(((°>".chars().collect();
                let fish_len = fish_chars.len();
                let travel_dist = width.saturating_sub(fish_len);

                if travel_dist == 0 {
                    return "><(((°>".to_string();
                }

                let cycle = current % (travel_dist * 2);
                let pos = if cycle < travel_dist {
                    cycle
                } else {
                    (travel_dist * 2) - cycle
                };

                // Flip the fish icon based on direction!
                let fish_icon = if cycle < travel_dist {
                    "><(((°>"
                } else {
                    "<°)))><"
                };

                format!(
                    "{}{}{}",
                    " ".repeat(pos),
                    fish_icon,
                    " ".repeat(width - pos - fish_len)
                )
            }

            Style::DotWaves => {
                let dots: Vec<char> = "⠁⠈⠐⠠⢀⡀⠄⠂".chars().collect();
                let mut res = String::with_capacity(width * 3); // Braille is 3 bytes
                for i in 0..width {
                    // Right-to-left flow
                    let idx = (i + current) % dots.len();
                    res.push(dots[idx]);
                }
                res
            }

            Style::TextTicker(text) => {
                let text_chars: Vec<char> = text.chars().collect();
                let n = text_chars.len();
                let mut res = String::with_capacity(width);

                for i in 0..width {
                    // This creates a continuous scrolling TextTicker effect
                    let idx = (i + current) % n;
                    res.push(text_chars[idx]);
                }
                res
            }

            Style::NyanCat => {
                let cat = "🐱";
                let percent = if total == 0 {
                    0.0
                } else {
                    current as f64 / total as f64
                };
                let pos = (percent * width as f64) as usize;

                let colors = [
                    "\x1b[38;5;196m", // Red
                    "\x1b[38;5;208m", // Orange
                    "\x1b[38;5;226m", // Yellow
                    "\x1b[38;5;46m",  // Green
                    "\x1b[38;5;21m",  // Blue
                    "\x1b[38;5;93m",  // Purple
                ];

                let mut trail = String::new();
                for i in 0..pos {
                    let color = colors[i % colors.len()];
                    // Alternate waves for nyan tail
                    let char = if (i + current) % 2 == 0 { "~" } else { "-" };
                    trail.push_str(&format!("{}{}", color, char));
                }

                format!(
                    "{}\x1b[0m{}\x1b[0m{}",
                    trail,
                    cat,
                    " ".repeat(width.saturating_sub(pos))
                )
            }

            Style::Gradient(start_hex, end_hex) => {
                let (r1, g1, b1) = Self::hex_to_rgb(start_hex);
                let (r2, g2, b2) = Self::hex_to_rgb(end_hex);

                let percent = if total == 0 {
                    1.0
                } else {
                    current as f64 / total as f64
                };
                let filled_len = (percent * width as f64) as usize;

                let mut bar = String::new();
                for i in 0..width {
                    if i < filled_len {
                        // Interpolate colors
                        let t = i as f64 / width as f64;
                        let r = (r1 as f64 + t * (r2 as f64 - r1 as f64)) as u8;
                        let g = (g1 as f64 + t * (g2 as f64 - g1 as f64)) as u8;
                        let b = (b1 as f64 + t * (b2 as f64 - b1 as f64)) as u8;
                        bar.push_str(&format!("\x1b[38;2;{};{};{}m█\x1b[0m", r, g, b));
                    } else {
                        bar.push(' ');
                    }
                }
                bar
            }

            Style::ModernSlim(fill_hex, empty_hex) => {
                let (r1, g1, b1) = Self::hex_to_rgb(fill_hex);
                let (r2, g2, b2) = Self::hex_to_rgb(empty_hex);

                let percent = if total == 0 {
                    0.0
                } else {
                    current as f64 / total as f64
                };
                let filled_len = (percent * width as f64) as usize;
                let empty_len = width.saturating_sub(filled_len);

                let mut bar = String::new();

                // '━' sits in the middle of the line height
                let bar_char = '━';

                // Filled portion (Red)
                bar.push_str(&format!("\x1b[38;2;{};{};{}m", r1, g1, b1));
                bar.push_str(&bar_char.to_string().repeat(filled_len));

                // Empty portion (Gray)
                bar.push_str(&format!("\x1b[38;2;{};{};{}m", r2, g2, b2));
                bar.push_str(&bar_char.to_string().repeat(empty_len));

                bar.push_str("\x1b[0m");
                bar
            }

            Style::Marquee(color1_hex, color2_hex) => {
                let (r1, g1, b1) = Self::hex_to_rgb(color1_hex);
                let (r2, g2, b2) = Self::hex_to_rgb(color2_hex);

                // Speed: higher divisor = slower movement
                let time = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_millis()
                    / 80;

                let mut bar = String::new();
                let bar_char = '━';
                let pattern_width = 6; // Total width of one color cycle

                for i in 0..width {
                    // Offset the index by time to create the "Marquee" motion
                    // Using (i + time) makes it slide left; (time - i) would slide right
                    let pos = (i + time as usize) % pattern_width;

                    // If pos is in the first half of the pattern, use Color 1, else Color 2
                    if pos < pattern_width / 2 {
                        bar.push_str(&format!("\x1b[38;2;{};{};{}m{}", r1, g1, b1, bar_char));
                    } else {
                        bar.push_str(&format!("\x1b[38;2;{};{};{}m{}", r2, g2, b2, bar_char));
                    }
                }

                bar.push_str("\x1b[0m");
                bar
            }
        }
    }
}
