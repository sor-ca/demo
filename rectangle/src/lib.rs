#[derive(Debug)]
struct Rectangle {
    name: String,
    width: u32,
    hight: u32,
    }
impl Rectangle {
    fn square(size: u32)-> Rectangle {
        Rectangle { width: size, hight: size,
            name: String::from("Rectangle"),}
    }
    pub fn greeting_rect(&self)->String {
        format!("Hello {}!", self.name)
    }   
    fn area(&self) -> u32 {
        self.width*self.hight
        }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width>other.width && self.hight>other.hight
    }
}
pub fn greeting(name:  &str)->String {
    format!("Hello {}!", name)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            name: String::from("Rectangle"),
            width: 5,
            hight: 3,
        };
        let smaller = Rectangle {
            name: String::from("Rectangle"),
            width: 3,
            hight: 1,
        };
        assert!(larger.can_hold(&smaller));
    }
    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            name: String::from("Rectangle"),
            width: 5,
            hight: 3,
        };
        let smaller = Rectangle {
            name: String::from("Rectangle"),
            width: 3,
            hight: 1,
        };
        assert!(!smaller.can_hold(&larger));
    }
    #[test]
    fn area_equal() {
        let rect = Rectangle {
            name: String::from("Rectangle"),
            width: 5,
            hight: 3,
        };
        assert_eq!(5*3, rect.area());
    }
    #[test]
    fn square_test() {
        let sq_rect = square(5);
        assert!(sq_rect.width==sq_rect.hight);
    }
    #[test]
    fn greeting_contains_name() {
        let result = greeting("Rectangle");
        assert!(result.contains("Rectangle"));
    }
    #[test]
    fn greeting_rect_test() {
        let rect = Rectangle  {
            name: String::from("Rectangle"),
            width: 5,
            hight: 3,
        };
        let result = greeting_rect(rect);
        assert!(result.contains("Rectangle"));
    }
        
}

