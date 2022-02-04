struct User {
    username : String,
    email : String,
    active : bool,
    age : u32
}

#[derive(Debug)]
struct Rectangle {
    width : u32,
    height : u32
}

impl Rectangle {
    fn area_3(&self) -> u32{ // Inside the constructors of the impl (Implementation) the first argument is always &self
        self.width * self.height
    }
        // &self is used for inidicating the following is a constructor
    fn can_hold(&self,other : &Rectangle) -> bool{
        self.width > other.width && self.height > other.height
    }
}


// Associated Functions
impl Rectangle{
    fn square(size : u32) -> Rectangle{
        Rectangle{
            width : size,
            height : size
        }
    }
}

fn main() {
    let mut user1 = User{
        email : String::from("harshiljani2002@gmail.com"),
        username : String::from("Harshil Jani"),
        active : true,
        age : 19
    };
    user1.username = String::from("Beast Incarnate");
    let name = user1.username;
    println!("{}",name);

    let mut user2 = build_user(String::from("Robert"), String::from(""),42);
    println!("{} is {} year's Old",user2.username,user2.age);

    let user3 = User{
        username : String::from("Can Yaman"),
        email : String::from("CanBay@tukish.com"),
        ..user2  // Other  fields will be taken from Instance of User2, So put it into User 3 using ..user2
    };

    // Tuple Structs

    struct Colors(i32, i32, i32); // (Red, Green, Blue)
    struct Point(i32, i32, i32); // (X, Y, Z)

    {
        let height = 32;
        let width = 64;
        println!("{} is the area.",area(height,width));
    }
    {
        let rect = Rectangle{
            width : 20,
            height : 324,
        };
        // println!("{}",rect);  -> std::fmt::Display error
        println!("{:#?}",rect);  // Don't forget to add #[derive(Debug)] at very top of struct Rectangle
        println!("{} IS THE AREA.",area_2(&rect));
        println!("{} is the Area",rect.area_3()); // Refer impl Rectangle
    
        let rect1 = Rectangle{
            width : 4,
            height : 24,
        };
        let rect2 = Rectangle{
            width : 42,
            height : 42,
        };
        println!("The rect can hold rect1 : {}",rect.can_hold(&rect1));
        println!("The rect can hold rect2 : {}",rect.can_hold(&rect2));

        let rect3 = Rectangle::square(12);  // This is how we call any associated functions
        println!("{:#?}",rect3);
    }
} 

fn build_user(username:String, email:String, age: u32) -> User {
    User{
        email : email,
        username : username,
        active : true,
        age :  age
    }
}

fn area(width: u32, height: u32) -> u32{
    width*height
}

fn area_2(rectangle: &Rectangle) -> u32{
    rectangle.height * rectangle.width
}