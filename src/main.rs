use rand::Rng;

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
        println!();
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
        // Update position
        self.x += self.dx;
        self.y += self.dy;
        self.life -= 1;

        // Trunk Logic: Heavily lean upwards, with occasional slight bends
        if self.is_main_trunk {
            self.dy = -1.0;
            if rng.gen_bool(0.4) {
                self.dx += rng.gen_range(-0.3..0.3);
                self.dx = self.dx.clamp(-0.7, 0.7);
            }
        } else {
            // Side branches gradually curve upwards
            if rng.gen_bool(0.2) {
                self.dy -= 0.1;
                self.dx += rng.gen_range(-0.1..0.1);
            }
        }

        // Mapping current direction to character
        let symbol = if self.dx.abs() < 0.4 {
            '|'
        } else if self.dx > 0.0 {
            '\\'
        } else {
            '/'
        };

        canvas.set_pixel(self.x as i32, self.y as i32, symbol);

        // Branching Logic: Trunk and branches can spawn side branches
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

        // Death & Leaves: When life runs out, spawn a cluster of leaves
        if self.life <= 0 {
            draw_leaves(canvas, self.x, self.y, rng);
        }

        None
    }
}

fn draw_pot(canvas: &mut Canvas) {
    let width = canvas.width;
    let height = canvas.height;
    let pot_y = (height - 6) as i32;
    let pot_center = (width / 2) as i32;

    // Top edge
    for x in (pot_center - 6)..=(pot_center + 6) {
        canvas.set_pixel(x, pot_y, '_');
    }
    canvas.set_pixel(pot_center - 7, pot_y, '/');
    canvas.set_pixel(pot_center + 7, pot_y, '\\');

    // Sides
    canvas.set_pixel(pot_center - 7, pot_y + 1, '|');
    canvas.set_pixel(pot_center + 7, pot_y + 1, '|');

    // Bottom edge
    canvas.set_pixel(pot_center - 7, pot_y + 2, '\\');
    for x in (pot_center - 6)..=(pot_center + 6) {
        canvas.set_pixel(x, pot_y + 2, '_');
    }
    canvas.set_pixel(pot_center + 7, pot_y + 2, '/');
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

fn grow_tree(canvas: &mut Canvas) {
    let mut rng = rand::thread_rng();
    let mut agents = Vec::new();

    // Main trunk starting at the center of the pot
    agents.push(BranchAgent {
        x: (canvas.width / 2) as f32,
        y: (canvas.height - 6) as f32,
        dx: 0.0,
        dy: -1.0,
        life: 25,
        is_main_trunk: true,
    });

    while !agents.is_empty() {
        let mut new_agents = Vec::new();
        let mut active_agents = Vec::new();

        for mut agent in agents {
            if agent.life > 0 {
                if let Some(branch) = agent.step(canvas, &mut rng) {
                    new_agents.push(branch);
                }
                if agent.life > 0 {
                    active_agents.push(agent);
                }
            }
        }

        agents = active_agents;
        agents.extend(new_agents);
    }
}

fn main() {
    let mut canvas = Canvas::new(80, 40);
    draw_pot(&mut canvas);
    grow_tree(&mut canvas);
    canvas.display();
}
