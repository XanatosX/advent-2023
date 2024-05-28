
use advent_shared::geometry::point::Point;

#[derive(Clone, Debug)]
pub struct SchematicNumber {
    number: i32,
    start_position: Point
}

impl SchematicNumber {
    pub fn new(number: i32, x: i32, y: i32) -> SchematicNumber 
    {
        SchematicNumber{number, start_position: Point::new(x, y)}
    }

    pub fn get_number(&self) -> &i32
    {
        &self.number
    }

    pub fn get_start_position(&self) -> &Point{
        &self.start_position
    }

    pub fn get_length(&self) -> i32 {
        self.number.to_string().len() as i32
    }

    pub fn point_does_touch(&self, point: &Point) -> bool
    {
        let check_start = Point::new(self.start_position.get_x() - 1, self.start_position.get_y() - 1);
        let end_point = Point::new(self.start_position.get_x() + self.get_length(), self.start_position.get_y() + 1);
        //println!("{:?}", self.number);
        //println!("{:?}", check_start);
        //println!("{:?}", end_point);
        if point.get_x() >= check_start.get_x() && point.get_x() <= end_point.get_x() {
            if point.get_y() >= check_start.get_y() && point.get_y() <= end_point.get_y() {
                return true;
            }
        }

        false
    }
}