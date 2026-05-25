use clap::Parser;
use rand::Rng;
use std::io::{stdout, Write};
use std::thread;
use std::time::Duration;

#[derive(Parser, Debug)]
#[command(author, version, about = "Procedural ASCII Bonsai Generator")]
struct Args {
    /// Grow the tree step-by-step with animation
    #[arg(short, long)]
    live: bool,

    /// Animation delay in milliseconds
    #[arg(short, long, default_value_t = 50)]
    delay: u64,

    /// Initial life capacity of the main trunk
    #[arg(short = 'L', long, default_value_t = 25)]
    life: i32,

    /// Custom message to display centered below the pot
    #[arg(short, long)]
    message: Option<String>,
}

const POTS: &[&[&str]] = &[
    &["  (===================)  ", "   \\_________________/   "],
    &[" [=======================]", "  \\_____________________/ "],
    &["  ,-------------------,  ", "  |___________________|  "],
    &["   ___________________   ", "  (___________________)  "],
];

struct Canvas {
    grid: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

impl Canvas {
    fn new(width: usize, height: usize) -> Self {
        Self {
            grid: vec![vec![' '; width]; height],
            width,
            height,
        }
    }

    fn set_pixel(&mut self, x: i32, y: i32, c: char) {
        if x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32 {
            self.grid[y as usize][x as usize] = c;
        }
    }

    fn display(&self) {
        for row in &self.grid {
            let s: String = row.iter().collect();
            println!("{}", s);
        }
    }
}

struct BranchAgent {
    x: f32,
    y: f32,
    dx: f32,
    dy: f32,
    life: i32,
    is_main_trunk: bool,
}

impl BranchAgent {
    fn step(&mut self, canvas: &mut Canvas, rng: &mut impl Rng) -> Option<BranchAgent> {
        self.x += self.dx;
        self.y += self.dy;
        self.life -= 1;

        if self.is_main_trunk {
            self.dy = -1.0;
            if rng.gen_bool(0.4) {
                self.dx += rng.gen_range(-0.3..0.3);
                self.dx = self.dx.clamp(-0.7, 0.7);
            }
        } else {
            if rng.gen_bool(0.2) {
                self.dy -= 0.1;
                self.dx += rng.gen_range(-0.1..0.1);
            }
        }

        let symbol = if self.dx.abs() < 0.4 {
            '|'
        } else if self.dx > 0.0 {
            '\\'
        } else {
            '/'
        };

        canvas.set_pixel(self.x as i32, self.y as i32, symbol);

        let branch_chance = if self.is_main_trunk { 0.4 } else { 0.2 };
        if self.life > 0 && self.life % 4 == 0 && rng.gen_bool(branch_chance) {
            let branch_dx = if rng.gen_bool(0.5) { 1.0 } else { -1.0 };
            return Some(BranchAgent {
                x: self.x,
                y: self.y,
                dx: branch_dx * 0.8,
                dy: -0.2,
                life: (self.life / 2).max(3),
                is_main_trunk: false,
            });
        }

        if self.life <= 0 {
            draw_leaves(canvas, self.x, self.y, rng);
        }

        None
    }
}

fn draw_pot(canvas: &mut Canvas, rng: &mut impl Rng) -> (f32, f32) {
    let pot_index = rng.gen_range(0..POTS.len());
    let pot = POTS[pot_index];

    let pot_height = pot.len();
    let pot_width = pot[0].len();

    // Calculate centering
    let start_x = (canvas.width as i32 - pot_width as i32) / 2;
    let start_y = canvas.height as i32 - pot_height as i32 - 1;

    for (row_idx, line) in pot.iter().enumerate() {
        for (col_idx, c) in line.chars().enumerate() {
            canvas.set_pixel(start_x + col_idx as i32, start_y + row_idx as i32, c);
        }
    }

    // The trunk emerges from the top center of the pot
    let trunk_x = start_x as f32 + (pot_width as f32 / 2.0);
    let trunk_y = start_y as f32 - 1.0;

    (trunk_x, trunk_y)
}

fn draw_leaves(canvas: &mut Canvas, x: f32, y: f32, rng: &mut impl Rng) {
    let leaf_chars = ['*', '&', 'o', '`'];
    let char_to_use = leaf_chars[rng.gen_range(0..leaf_chars.len())];

    for _ in 0..6 {
        let ox = rng.gen_range(-2..3) as i32;
        let oy = rng.gen_range(-2..3) as i32;
        canvas.set_pixel(x as i32 + ox, y as i32 + oy, char_to_use);
    }
}

fn grow_tree(canvas: &mut Canvas, args: &Args) {
    let mut rng = rand::thread_rng();
    let (start_x, start_y) = draw_pot(canvas, &mut rng);

    let mut agents = Vec::new();
    agents.push(BranchAgent {
        x: start_x,
        y: start_y,
        dx: 0.0,
        dy: -1.0,
        life: args.life,
        is_main_trunk: true,
    });

    if args.live {
        print!("\x1B[2J"); // Clear screen
    }

    while !agents.is_empty() {
        let mut next_tick_agents = Vec::new();

        for mut agent in agents {
            if let Some(new_branch) = agent.step(canvas, &mut rng) {
                next_tick_agents.push(new_branch);
            }
            if agent.life > 0 {
                next_tick_agents.push(agent);
            }
        }
        agents = next_tick_agents;

        if args.live {
            print!("\x1B[H"); // Home cursor
            canvas.display();
            println!(); // Trailing newline
            stdout().flush().unwrap();
            thread::sleep(Duration::from_millis(args.delay));
        }
    }
}

fn main() {
    let args = Args::parse();
    let mut canvas = Canvas::new(80, 40);

    grow_tree(&mut canvas, &args);

    if !args.live {
        canvas.display();
        println!();
    }

    if let Some(ref msg) = args.message {
        let msg_len = msg.len();
        let padding = if msg_len < canvas.width {
            (canvas.width - msg_len) / 2
        } else {
            0
        };

        let truncated_msg = if msg_len > canvas.width {
            &msg[..canvas.width]
        } else {
            msg
        };

        println!("{:width$}", truncated_msg, width = padding);
    }
}
