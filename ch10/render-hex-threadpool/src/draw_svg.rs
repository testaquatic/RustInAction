use svg::{
    node::element::{
        path::{Command, Data, Position},
        Path, Rectangle,
    },
    Document,
};

use crate::parse_args::Operation;

const WIDTH: isize = 400;
pub const HEIGHT: isize = WIDTH;

const HOME_Y: isize = HEIGHT / 2;
const HOME_X: isize = WIDTH / 2;
const STROKE_WIDTH: usize = 5;

enum Orientation {
    North,
    East,
    West,
    South,
}

struct Artist {
    x: isize,
    y: isize,
    heading: Orientation,
}

impl Artist {
    fn new() -> Artist {
        Artist {
            x: HOME_X,
            y: HOME_Y,
            heading: Orientation::North,
        }
    }

    fn home(&mut self) {
        self.x = HOME_X;
        self.y = HOME_Y;
    }

    fn forward(&mut self, distance: isize) {
        match self.heading {
            Orientation::North => self.y += distance,
            Orientation::East => self.x -= distance,
            Orientation::West => self.x += distance,
            Orientation::South => self.y -= distance,
        }
    }

    fn turn_right(&mut self) {
        self.heading = match self.heading {
            Orientation::North => Orientation::East,
            Orientation::East => Orientation::South,
            Orientation::West => Orientation::North,
            Orientation::South => Orientation::West,
        }
    }

    fn turn_left(&mut self) {
        self.heading = match self.heading {
            Orientation::North => Orientation::West,
            Orientation::East => Orientation::North,
            Orientation::West => Orientation::South,
            Orientation::South => Orientation::East,
        }
    }

    fn wrap(&mut self) {
        if self.x < 0 {
            self.x = HOME_X;
            self.heading = Orientation::West;
        } else if self.x > WIDTH {
            self.x = HOME_X;
            self.heading = Orientation::East;
        }

        if self.y < 0 {
            self.y = HOME_Y;
            self.heading = Orientation::North;
        } else if self.y > HEIGHT {
            self.y = HOME_Y;
            self.heading = Orientation::South;
        }
    }
}

pub fn convert(operations: &[Operation]) -> Vec<Command> {
    let mut turtle = Artist::new();
    let start_at_home = Command::Move(Position::Absolute, (HOME_X, HOME_Y).into());
    let mut path_data = Vec::with_capacity(operations.len());
    path_data.push(start_at_home);

    let command_iter = operations.iter().map(|op| {
        match op {
            Operation::Forward(distance) => turtle.forward(*distance),
            Operation::TurnLeft => turtle.turn_left(),
            Operation::TurnRight => turtle.turn_right(),
            Operation::Home => turtle.home(),
            Operation::Noop(byte) => eprintln!("warning: illegal byte encountered: {:?}", byte),
        }

        let command = Command::Line(Position::Absolute, (turtle.x, turtle.y).into());
        turtle.wrap();
        command
    });
    path_data.extend(command_iter);

    path_data
}

pub fn generate_svg(path_data: Vec<Command>) -> Document {
    let background = Rectangle::new()
        .set("x", 0)
        .set("y", 0)
        .set("width", WIDTH)
        .set("height", HEIGHT)
        .set("fill", "white");

    let border = background
        .clone()
        .set("fill-opacity", "0.0")
        .set("stroke", "#CCCCCC")
        .set("stroke-width", 3 * STROKE_WIDTH);

    let sketch = Path::new()
        .set("fill", "none")
        .set("stroke", "#2f2f2f")
        .set("stroke-width", STROKE_WIDTH)
        .set("stroke-opacity", "0.9")
        .set("d", Data::from(path_data));

    Document::new()
        .set("viewBox", (0, 0, HEIGHT, WIDTH))
        .set("height", HEIGHT)
        .set("width", WIDTH)
        .set("style", "style=\"outline: 5px solid #800000;\"")
        .add(background)
        .add(sketch)
        .add(border)
}
