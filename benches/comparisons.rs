use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::prelude::*;

use fast_inv_sqrt::InvSqrt32;

//Criterion can't run on binary crates, so we include the necessary space.rs code here
use std::ops::{Add, Sub};

pub const FLOATING_POINT_ERROR: f32 = 0.0001;

fn normalize_angle(angle: f32) -> f32
{

    if angle < 0.0
    {

        return angle + 2.0 * std::f32::consts::PI;

    }

    return angle;

}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point
{

    pub x: f32,
    pub y: f32

}

impl Point
{

	pub fn dot(&self, other: &Point) -> f32
	{

		return self.x * other.x + self.y * other.y;

	}

	pub fn scale(&self, s: f32) -> Point
	{

		return Point { x: self.x * s, y: self.y * s };

	}

    //ANCHOR: exclusion
	//Assumes the angle from lower to upper is less than pi. Swap lower and upper and negate it for larger angles
    //Returns false if self is not between lower and upper, true if it is
	pub fn ray_between(&self, lower: Point, upper: Point) -> bool
	{

		//Dot product of upper rotated ccw by pi/2
		let upper_component = self.y * upper.x - self.x * upper.y;

		if upper_component > FLOATING_POINT_ERROR
		{

			return false;

		}

		//Dot product of lower rotated cw by pi/2
		let lower_component = self.x * lower.y - self.y * lower.x;

		if lower_component > FLOATING_POINT_ERROR
		{

			return false;

		}

		return true;

	}
    //ANCHOR_END: exclusion

    pub fn ray_between_atan(&self, lower: f32, upper: f32) -> bool
    {

        let tan = normalize_angle(self.y.atan2(self.x));

        if upper > lower
        {

            return tan < upper && tan > lower;

        }
        else
        {

            return tan > lower || tan < upper;

        }

    }

    // ANCHOR: sorting_function
	//Works so long as all represented angles are between lower and lower+pi
	pub fn sort_from_angle(rays: &mut Vec<Point>, lower: Point)
	{

		rays.sort_unstable_by(|a, b|
		{

            // ANCHOR: compare 
			//We want to order by angle from lower, which is the same as reverse ordering by normalized projections along lower
			//We do some algebra to avoid computing square roots for the normalization, i.e. a dot L/|a|>b dot L/|b| if and only if
			// a dot L*|a dot L|*|b|^2 > b dot L * |b dot L| * |a|^2
			let a_dot_l = lower.dot(a);
			let lhs = a_dot_l.abs() * a_dot_l * (b.x * b.x + b.y * b.y);

			let b_dot_l = lower.dot(b);
			let rhs = b_dot_l.abs() * b_dot_l * (a.x * a.x + a.y * a.y);
            // ANCHOR_END: compare
			return rhs.partial_cmp(&lhs).unwrap();

		});

	}
    // ANCHOR_END: sorting_function

    pub fn sort_from_angle_fisr(rays: &mut Vec<Point>, lower: Point)
    {

        rays.sort_unstable_by(|a, b|
        {

            let lhs = lower.dot(a) * (a.x * a.x + a.y * a.y).inv_sqrt32();
            let rhs = lower.dot(b) * (b.x * b.x + b.y * b.y).inv_sqrt32();

            return rhs.partial_cmp(&rhs).unwrap();

        });

    }

    pub fn sort_from_angle_atan(rays: &mut Vec<Point>, lower: f32, upper: f32)
    {

        if lower < upper
        {

            rays.sort_unstable_by(|a, b|
            {

                let lhs = normalize_angle(a.y.atan2(a.x));
                let rhs = normalize_angle(b.y.atan2(b.x));

                return lhs.partial_cmp(&lhs).unwrap();

            });

        }
        else
        {

            rays.sort_unstable_by(|a, b|
            {

                let lhs = normalize_angle(a.y.atan2(a.x));
                let rhs = normalize_angle(b.y.atan2(b.x));

                if (lhs < upper && rhs < upper) || (lhs > lower && rhs > lower)
                {
    
                    return lhs.partial_cmp(&lhs).unwrap();

                }
                else if lhs < upper && rhs > lower
                {

                    return std::cmp::Ordering::Greater;

                }
                else
                {

                    return std::cmp::Ordering::Less;

                }

            });

        }

    }

}

impl Add for Point
{

	type Output = Point;

	fn add(self, other: Point) -> Point
	{

		return Point { x: self.x + other.x, y: self.y + other.y };

	}

}

impl Sub for Point
{

	type Output = Point;

	fn sub(self, other: Point) -> Point
	{

		return Point { x: self.x - other.x, y: self.y - other.y };

	}

}


fn random_component() -> f32
{

    return rand::thread_rng().gen_range(1.0..10.0);    

}

fn random_rays() -> Vec<Point>
{

    let mut rays = Vec::new();

    for i in 0..100
    {

        rays.push(Point { x: random_component(), y: random_component() });

    }

    return rays;

}

fn exclusion_vector() -> Vec<Point>
{

    let rays = random_rays();

    let upper = Point { x: 1.0, y: 1.0 };
    let lower = Point { x: 1.0, y: -1.0 };
    
    return rays.into_iter().filter(|r| r.ray_between(lower, upper) ).collect();

}

fn exclusion_atan() -> Vec<Point>
{

    let rays = random_rays();

    let upper = 1f32.atan2(1.0);
    let lower = -1f32.atan2(1.0);

    return rays.into_iter().filter(|r| r.ray_between_atan(lower, upper) ).collect();

}

fn exclusion_bench(c: &mut Criterion)
{

    c.bench_function("vector", |b| b.iter(|| exclusion_vector()));

    c.bench_function("atan2", |b| b.iter(|| exclusion_atan()));

}

fn sort_vector()
{

    let mut rays = exclusion_vector();

    let lower = Point { x: 1.0, y: -1.0 };

    Point::sort_from_angle(&mut rays, lower);

}

fn sort_fisr()
{

    let mut rays = exclusion_vector();

    let lower = Point { x: 1.0, y: -1.0 };

    Point::sort_from_angle_fisr(&mut rays, lower);

}

fn sort_atan2()
{

    let mut rays = exclusion_vector();

    let upper = 1f32.atan2(1.0);
    let lower = -1f32.atan2(1.0);

    Point::sort_from_angle_atan(&mut rays, lower, upper);

}

fn sort_bench(c: &mut Criterion)
{

    c.bench_function("vector", |b| b.iter(|| sort_vector()));

    c.bench_function("fisr", |b| b.iter(|| sort_fisr()));

    c.bench_function("atan2", |b| b.iter(|| sort_atan2()));

}

criterion_group!(benches, exclusion_bench, sort_bench);
criterion_main!(benches);
