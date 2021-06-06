use std::ops::{Add, Sub};

pub const FLOATING_POINT_ERROR: f32 = 0.0001;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point
{

    pub x: f32,
    pub y: f32

}

impl Point
{

	pub fn dot(&self, other: Point) -> f32
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

	//Works so long as all represented angles are between from and from+pi
	pub fn sort_from_angle(rays: &mut Vec<Point>, from: Point)
	{

		rays.sort_unstable_by(|a, b|
		{

			//We want to order by angle from lower, which is the same as reverse ordering by normalized projections along lower
			//We do some algebra to avoid computing square roots for the normalization, i.e. a dot L/|a|>b dot L/|b| if and only if
			// a dot L*|a dot L|*|b|^2 > b dot L * |b dot L| * |a|^2
			let a_dot_f = from.dot(*a);
			let lhs = a_dot_f.abs() * a_dot_f * (b.x * b.x + b.y * b.y);

			let b_dot_f = from.dot(*b);
			let rhs = b_dot_f.abs() * b_dot_f * (a.x * a.x + a.y * a.y);

			return rhs.partial_cmp(&lhs).unwrap();

		});

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

pub struct Segment
{

	pub start: Point,
	pub end: Point

}

impl Segment
{

	pub fn new(start: Point, end: Point) -> Segment
	{

		if start.x == end.x && start.y == end.y
		{

			panic!("Cannot create line segment between point and itself.");

		}

		return Segment { start, end };

	}

	pub fn raycast(&self, location: Point, ray: Point) -> Option<f32>
	{

		//Ensure the ray can be raycast
		if ray.x == 0.0 && ray.y == 0.0
		{

			panic!("Cannot raycast the zero vector");

		}

		let rise = self.end.y - self.start.y;
		let run = self.end.x - self.start.x;

		let denominator = rise * ray.x - run * ray.y;
		if denominator.abs() < FLOATING_POINT_ERROR //The ray and the segment are parallel, so there is no intersection to find
		{

			return None;

		}

		let segment_param = (location.y * ray.x + self.start.x * ray.y - location.x * ray.y - self.start.y * ray.x) / denominator;
		if segment_param < -FLOATING_POINT_ERROR || segment_param > 1.0 + FLOATING_POINT_ERROR //The lines intersect outside the segment, so there is no intersection
		{

			return None;

		}

		let ray_param;
		if ray.x == 0.0
		{

			ray_param = (self.start.y - location.y + rise * segment_param) / ray.y;

		}
		else
		{

			ray_param = (self.start.x - location.x + run * segment_param) / ray.x;

		}

		if ray_param < -FLOATING_POINT_ERROR //The opposite of the ray intersects the segment, not the ray itself
		{

			return None;

		}

		return Some(ray_param);

	}

}

#[derive(Debug)]
pub struct Triangle
{

	pub vertices: [Point; 3]

}

impl Triangle
{

	pub fn new(a: Point, b: Point, c: Point) -> Triangle
	{

		return Triangle { vertices: [a, b, c] };

	}
	
}

#[cfg(test)]
mod tests
{

	use super::*;

	#[test]
	fn angle_sort()
	{

		let mut rays = vec![Point { x: 1.0, y: 1.0 }, Point { x: 0.0, y: 1.0 }, Point { x: 2.0, y: 4.0 }, Point { x: -1.0, y: 1.0 }, Point { x: 1.0, y: 0.2 } ];
		Point::sort_from_angle(&mut rays, Point { x: 1.0, y: 0.0 });

		assert_eq!(rays, vec![Point { x: 1.0, y: 0.2 }, Point { x: 1.0, y: 1.0 }, Point { x: 2.0, y: 4.0 }, Point { x: 0.0, y: 1.0 }, Point { x: -1.0, y: 1.0 }]);

	}
    
    // ANCHOR: ray_test
	#[test]
	fn ray_between()
	{

		let ray1 = Point { x: 2.5, y: 0.0 };
		let ray2 = Point { x: 0.0, y: 1.0 };
		let ray3 = Point { x: -1.0, y: 2.0 };
		let ray4 = Point { x: -1.0, y: -1.1 };
		let ray5 = Point { x: 3.7, y: -2.0 };
		let ray6 = Point { x: -2.0, y: 0.0 };
		let ray7 = Point { x: 0.0, y: -30.0 };
		let ray8 = Point { x: 10.0, y: 1.0 };

		assert!(ray8.ray_between(ray1, ray2));
		assert!(ray6.ray_between(ray3, ray4));
		assert!(ray5.ray_between(ray7, ray1));
		assert!(ray4.ray_between(ray3, ray5));

		assert!(!ray3.ray_between(ray1, ray2));
		assert!(!ray1.ray_between(ray3, ray4));
		assert!(!ray2.ray_between(ray7, ray1));
		assert!(!ray8.ray_between(ray3, ray5));

	}
    // ANCHOR_END: ray_test

	#[test]
	fn raycast()
	{

		let segment1 = Segment::new(Point { x: 10.0, y: 0.0 }, Point { x: 0.0, y: 0.0 });
		let segment2 = Segment::new(Point { x: 0.0, y: 0.0 }, Point { x: 0.0, y: 10.0 });
		let segment3 = Segment::new(Point { x: 1.0, y: 9.0 }, Point { x: 2.0, y: 5.0});

		let cast1 = segment1.raycast(Point { x: 1.0, y: -1.0 }, Point { x: 0.0, y: 1.0 }).unwrap();
		let cast2 = segment2.raycast(Point { x: 3.0, y: 4.0 }, Point { x: -30.0, y: 0.0 }).unwrap();
		let cast3 = segment3.raycast(Point { x: 0.0, y: 9.0 }, Point { x: 1.0, y: -1.0 }).unwrap();

		assert_eq!(1.0, cast1);
		assert_eq!(1.0 / 10.0, cast2);
		assert_eq!(4.0 / 3.0, cast3);

		let cast4 = segment1.raycast(Point { x: 2.0, y: 0.0 }, Point { x: 1.0, y: 0.0 });
		let cast5 = segment2.raycast(Point { x: 1.0, y: 11.0 }, Point { x: -1.0, y: 0.0 });
		let cast6 = segment3.raycast(Point { x: 0.0, y: 9.0 }, Point { x: -1.0, y: 1.0 });

		assert!(cast4.is_none());
		assert!(cast5.is_none());
		assert!(cast6.is_none());

	}

}
