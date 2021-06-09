use ggez::*;
use glam::*;

use ggez::graphics::{DrawMode, DrawParam, Mesh};

use std::path;
use std::env;

mod space;
mod sight;

use space::{Point, Segment, Triangle};

const WORLD_WIDTH: f32 = 700.0;
const WORLD_HEIGHT: f32 = 500.0;

//Used for rotating the line of sight each frame
const SIN_THETA: f32 = 0.01308959557;
const COS_THETA: f32 = 0.99991432757;

struct State 
{

    walls: Vec<Segment>,
    sight: Vec<Triangle>,
    location: Point,
    upper: Point,
    lower: Point,
    static_mesh: Mesh,
    sight_mesh: Mesh

}

impl State
{

    pub fn new(ctx: &mut Context) -> GameResult<State>
    {

        //Some example walls to obstruct our view
        let mut walls: Vec<Segment> = Vec::new();
        walls.push(Segment::new(Point { x: 5.0, y: 5.0 }, Point { x: 5.0 + WORLD_WIDTH, y: 5.0 }));
        walls.push(Segment::new(Point { x: 5.0 + WORLD_WIDTH, y: 5.0 }, Point { x: 5.0 + WORLD_WIDTH, y: 5.0 + WORLD_HEIGHT }));
        walls.push(Segment::new(Point { x: 5.0 + WORLD_WIDTH, y: 5.0 + WORLD_HEIGHT }, Point { x: 5.0, y: 5.0 + WORLD_HEIGHT }));
        walls.push(Segment::new(Point { x: 5.0, y: 5.0 + WORLD_HEIGHT }, Point { x: 5.0, y: 5.0 }));
        walls.push(Segment::new(Point { x: 100.0, y: 50.0 }, Point { x: 50.0, y: 100.0 }));
        walls.push(Segment::new(Point { x: 200.0, y: 450.0 }, Point { x: 400.0, y: 450.0 }));
        walls.push(Segment::new(Point { x: 300.0, y: 350.0 }, Point { x: 350.0, y: 425.0 }));
        walls.push(Segment::new(Point { x: 400.0, y: 10.0 }, Point { x: 400.0, y: 50.0 }));
        walls.push(Segment::new(Point { x: 400.0, y: 50.0 }, Point { x: 600.0, y: 250.0 }));
        walls.push(Segment::new(Point { x: 50.0, y: 300.0 }, Point { x: 75.0, y: 300.0 }));
        walls.push(Segment::new(Point { x: 75.0, y: 300.0 }, Point { x: 75.0, y: 325.0 }));
        walls.push(Segment::new(Point { x: 75.0, y: 325.0 }, Point { x: 50.0, y: 325.0 }));
        walls.push(Segment::new(Point { x: 50.0, y: 325.0 }, Point { x: 50.0, y: 300.0 }));
        walls.push(Segment::new(Point { x: 280.0, y: 240.0}, Point { x: 280.0, y: 260.0 }));
        walls.push(Segment::new(Point { x: 450.0, y: 5.0 + WORLD_HEIGHT * 0.5 }, Point {x : 700.0, y: 5.0 + WORLD_HEIGHT * 0.5 }));

        let mb = &mut graphics::MeshBuilder::new();
        for segment in walls.iter()
        {

            mb.line(&[Vec2::new(segment.start.x, segment.start.y), Vec2::new(segment.end.x, segment.end.y)], 2.0, [1.0, 0.0, 0.0, 1.0].into())?;

        }

        mb.circle
        (

            DrawMode::fill(),
            Vec2::new(5.0 + WORLD_WIDTH * 0.5, 5.0 + WORLD_HEIGHT * 0.5),
            10.0,
            1.0,
            [0.0, 0.0, 1.0, 1.0].into()

        )?;

        let static_mesh = mb.build(ctx)?;

        let sight_mb = &mut graphics::MeshBuilder::new();
        sight_mb.circle
        (

            DrawMode::fill(),
            Vec2::new(0.0, 0.0),
            1.0,
            1.0,
            [0.0, 0.0, 0.0, 0.0].into()

        )?; //This only exists since there seems to be no way to initialize an empty mesh

        let sight_mesh = sight_mb.build(ctx)?;

        return Ok(State
        {

            walls: walls,
            sight: Vec::new(),
            location: Point { x: 5.0 + WORLD_WIDTH * 0.5, y: 5.0 + WORLD_HEIGHT * 0.5 },
            upper: Point { x: (3f32).sqrt(), y: 1.0 },
            lower: Point { x: 1.0, y: 0.0 },
            static_mesh: static_mesh,
            sight_mesh: sight_mesh

        });

    }

}

impl event::EventHandler for State
{

    fn update(&mut self, ctx: &mut Context) -> GameResult
    {

        //Update our field of view
        self.upper.x = self.upper.x * COS_THETA - self.upper.y * SIN_THETA;
        self.upper.y = self.upper.x * SIN_THETA + self.upper.y * COS_THETA;

        self.lower.x = self.lower.x * COS_THETA - self.lower.y * SIN_THETA;
        self.lower.y = self.lower.x * SIN_THETA + self.lower.y * COS_THETA;

        //Generate our new line of sight
        self.sight = sight::generate_line_of_sight(self.location, self.upper, self.lower, &self.walls); 
        
        let mb = &mut graphics::MeshBuilder::new();
        for triangle in self.sight.iter()
        {

            mb.polygon
            (

                DrawMode::fill(),
                &[

                    Vec2::new(triangle.vertices[0].x, triangle.vertices[0].y),
                    Vec2::new(triangle.vertices[2].x, triangle.vertices[2].y),
                    Vec2::new(triangle.vertices[1].x, triangle.vertices[1].y)

                ],
                [1.0, 1.0, 1.0, 0.8].into()

            )?;

        }

        self.sight_mesh = mb.build(ctx)?;

        return Ok(());

    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult
    {

        graphics::clear(ctx, [0.0, 0.0, 0.0, 1.0].into());

        graphics::draw(ctx, &self.sight_mesh, DrawParam::default())?;

        graphics::draw(ctx, &self.static_mesh, DrawParam::default())?;

        graphics::present(ctx)?;

        return Ok(());

    }

}

fn main() -> GameResult
{

    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR")
    {

        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");

        path

    } 
    else
    {

        path::PathBuf::from("./resources")

    };

    let cb =  ContextBuilder::new("line_of_sight", "basstabs").add_resource_path(resource_dir).window_setup(ggez::conf::WindowSetup::default().title("Line of Sight Example"));
    let (mut ctx, event_loop) = cb.build()?;

    let state = State::new(&mut ctx)?;

    event::run(ctx, event_loop, state);

}
