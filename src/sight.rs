use super::space::{Point, Segment, Triangle};

pub fn generate_line_of_sight(location: Point, upper: Point, lower: Point, segments: &Vec<Segment>) -> Vec<Triangle>
{

    // ANCHOR: section1
    let mut rays: Vec<Point> = Vec::new();
	rays.push(lower);
	rays.push(upper);

	//Collect the rays we need to project
	for segment in segments.iter()
	{

		let ray = segment.start - location;

		if ray.ray_between(lower, upper)
		{

			rays.push(ray);

		}

        let ray = segment.end - location;

        if ray.ray_between(lower, upper)
        {

            rays.push(ray);

        }

	}
    // ANCHOR_END: section1

    // ANCHOR: sort
    //Sort the rays from lower to upper
	Point::sort_from_angle(&mut rays, lower);
    // ANCHOR_END: sort
    //
    // ANCHOR: final
	//Actually create the triangles
	let mut line_of_sight: Vec<Triangle> = Vec::new();

	for i in 0..rays.len()-1
	{

		let mut shortest_current = 0.0;
		let mut shortest_next = 0.0;

		for segment in segments.iter()
		{

			let cast_current = segment.raycast(location, rays[i]);
			let cast_next = segment.raycast(location, rays[i + 1]);

			if cast_current.is_some() && cast_next.is_some() && (shortest_current == 0.0 || cast_current.unwrap() < shortest_current)
			{

				shortest_current = cast_current.unwrap();
				shortest_next = cast_next.unwrap();

			}

		}

		line_of_sight.push(Triangle::new(location, location + rays[i].scale(shortest_current), location + rays[i + 1].scale(shortest_next)));

	}

    return line_of_sight;
    // ANCHOR_END: final

}
