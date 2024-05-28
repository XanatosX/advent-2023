#[derive(Clone, Debug)]
pub struct Point
{
    x: i32,
    y: i32,
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl  Point {
    pub fn new(x: i32, y: i32) -> Point
    {
        Point{x, y}
    }

    pub fn get_x(&self) -> &i32
    {
        &self.x
    }

    pub fn get_y(&self) -> &i32
    {
        &self.y
    }
}