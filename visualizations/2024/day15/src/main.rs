use std::{error::Error, io, time::Duration};

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use rand::RngCore;
use ratatui::{buffer::Buffer, layout::Rect, style::{Color, Style}, text::{self, Line, Span}, widgets::{Paragraph, Widget}, DefaultTerminal, Frame};
use rustc_hash::FxHashSet;


#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum WideTile {
    Robot,
    Obstacle,
    LeftBox,
    RightBox,
    Air,
}

#[derive(Debug)]
struct App {
    game: Game,
    exit: bool
}

#[derive(Debug)]
struct Game {
    map: Vec<WideTile>,
    width: usize,
    robot_position: usize,
}

const AIR_WEIGHT: u32 = 70;
const BOX_WEIGHT: u32 = 35;
const OBSTACLE_WEIGHT: u32 = 4;
const MAX_WEIGHT: u32 = AIR_WEIGHT + BOX_WEIGHT + OBSTACLE_WEIGHT;

impl Game {

    fn new(width: usize, height: usize) -> Game {
        let mut map: Vec<WideTile> = Vec::with_capacity(width * 2 * height);

        let mut rng = rand::thread_rng();

        for i in 0..width * height {

            if i % width < 1 || i % width > width - 2 || i / width == 0 || i / width == height - 1 {
                map.push(WideTile::Obstacle);
                map.push(WideTile::Obstacle);
                continue;
            }

            let roll = rng.next_u32() % MAX_WEIGHT;

            if roll < AIR_WEIGHT {
                map.push(WideTile::Air);
                map.push(WideTile::Air);
            } else if roll < AIR_WEIGHT + BOX_WEIGHT {
                map.push(WideTile::LeftBox);
                map.push(WideTile::RightBox);
            } else {
                map.push(WideTile::Obstacle);
                map.push(WideTile::Obstacle);
            }
        }

        let mut random_pos = width * height;
        while map[random_pos] != WideTile::Air {
            random_pos += 2;
        }

        map[random_pos] = WideTile::Robot;

        Game {
            map,
            width,
            robot_position: random_pos
        }
    }

    #[inline(always)]
    fn is_horizontal_movement(direction: isize) -> bool {
        direction == 1 || direction == -1
    }

    fn can_move(&self, position: usize, direction: isize) -> bool {
        let next_index = (position as isize + direction) as usize;
        match self.map[next_index] {
            WideTile::Robot => panic!("Moved self"),
            WideTile::Obstacle => false,
            WideTile::LeftBox => if Self::is_horizontal_movement(direction) {
                self.can_move(next_index, direction)
            } else {
                self.can_move(next_index, direction) && self.can_move(next_index + 1, direction)
            },
            WideTile::RightBox => if Self::is_horizontal_movement(direction) {
                self.can_move(next_index, direction)
            } else {
                self.can_move(next_index, direction) && self.can_move(next_index - 1, direction)
            },
            WideTile::Air => true,
        }
    }

    fn do_move(&mut self, position: usize, direction: isize, moved: &mut FxHashSet<usize>) {
        if !moved.insert(position) {
            return;
        }

        let next_index = (position as isize + direction) as usize;
        match self.map[position] {
            WideTile::Robot => self.do_move(next_index, direction, moved),
            WideTile::Obstacle => panic!("Moved wall"),
            WideTile::LeftBox => if Self::is_horizontal_movement(direction) {
                self.do_move(next_index, direction, moved);
            } else {
                self.do_move(next_index, direction, moved); 
                self.do_move(position + 1, direction, moved);
            },
            WideTile::RightBox => if Self::is_horizontal_movement(direction) {
                self.do_move(next_index, direction, moved);
            } else {
                self.do_move(next_index, direction, moved);
                self.do_move(position - 1, direction, moved);
            },
            WideTile::Air => return,
        }

        self.map.swap(position, next_index);
    }

    #[inline(always)]
    fn doubled_width(&self) -> usize {
        self.width * 2
    }

    fn random_move(&self) -> isize {
        let r: u8 = rand::random();

        match r & 3 {
            0 => 1,
            1 => -1,
            2 => self.doubled_width() as isize,
            3 => -(self.doubled_width() as isize),
            _ => panic!("Illegal value")
        }
    }

    fn step_random(&mut self) {
        let potential_move = self.random_move();
        if self.can_move(self.robot_position, potential_move) {
            self.do_move(self.robot_position, potential_move, &mut FxHashSet::default());
            self.robot_position = (self.robot_position as isize + potential_move) as usize;
        }
    }

    fn render(&self) -> Paragraph {
        let mut lines = Vec::new();
        let mut texts = Vec::new();

        for (i, tile) in self.map.iter().enumerate() {

            match tile {
                WideTile::Robot => texts.push(Span::styled("@", Style::new().fg(Color::LightGreen))),
                WideTile::Obstacle => texts.push(Span::styled("#", Style::new().fg(Color::Blue))),
                WideTile::LeftBox => texts.push(Span::styled("[", Style::new().fg(Color::LightRed))),
                WideTile::RightBox => texts.push(Span::styled("]", Style::new().fg(Color::LightRed))),
                WideTile::Air => texts.push(Span::styled(" ", Style::new().fg(Color::LightRed))),
            }
            
            if i % self.doubled_width() == self.doubled_width() - 1 {
                lines.push(Line::from(texts));
                texts = Vec::new();
            }
        }

        Paragraph::new(lines)
    }
}

impl App {

    fn new(width: usize, height: usize) -> App {
        return App {
            game: Game::new(width, height),
            exit: false
        }
    }

    fn process(&mut self) {
        self.game.step_random();
    }

    fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<(), io::Error> {
        while !self.exit {
            self.process();
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    const INITIAL_WAIT: Duration = Duration::from_millis(0);
    const THEN_WAIT: Duration = Duration::ZERO;

    fn handle_events(&mut self) -> io::Result<()> {
        let mut duration = Self::INITIAL_WAIT;
        while event::poll(duration)? {
            match event::read()? {
                Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    if key_event.code == KeyCode::Esc {
                        self.exit = true;
                    }
                }
                _ => {}
            };
            duration = Self::THEN_WAIT;
        }

        Ok(())
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.game.render().centered().render(area, buf);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    color_eyre::install()?;
    let mut terminal = ratatui::init();
    let mut app = App::new(30, 30);
    let result = app.run(&mut terminal);
    ratatui::restore();
    result?;

    Ok(())
}