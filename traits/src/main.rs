// Polymorphism allows to call methods on an interface,
// without worrying about the concrete types that implement that interface.

// Traits in rust is similar to inheritence in Object oriented programming.
// With traits only functionality can be shared, data cannot be shared.

trait Park {
    fn park(&self);

    // Associated function
    fn get_default_color() -> String {
        "White".to_owned()
    }
}

// park is the Super trait of Vehicle trait =>
// Any type that implements Vehicle trait must also implement Park trait.
// trait Vehicle: Park + AnotherTrait {
//     fn drive(&self);
// }

trait Paint {
    fn paint(&self, color: String) {
        println!("Painting vehicle with color: {}", color);
    }
}

struct VehicleInfo {
    make: String,
    model: String,
    year: u16,
}

struct Car {
   info: VehicleInfo
}

impl Park for Car {
    fn park(&self){
        println!("parking Car");
    }
}

impl Paint for Car {}

struct Truck {
    info: VehicleInfo
}

impl Truck {
    fn unload(&self){
        println!("unloading Truck");
    }
}

impl Park for Truck {
    fn park(&self){
        println!("parking Truck");
    }
}

impl Paint for Truck {}

struct House {}

impl Paint for House {
    fn paint(&self, color: String) {
        println!("Painting House with color: {}", color);
    }
}

// Trait Bounds = Generic types can be used with traits.

// T is some type that implements the Paint trait.
// fn paint_red<T: Paint>(object : &T) {
//     object.paint("Red".to_string());
// }

fn paint_vehicle_red<T>(object : &T) where T: Paint + Park {
    object.paint("Red".to_string());
}
// "where" approach is used if we wants to use multiple traits.

// object is of type that implements Paint trait.
fn paint_red(object : &impl Paint) {
    object.paint("Red".to_string());
}

// returns some type that implements Paint trait.
fn create_paitable_house() -> impl Paint {
    House {}
}


// impl Paint = syntax sugar for using a generic with a trait bound.
// when returing a generic, that generic must be substitiuted with a concrete type at compile time.
// here we have two concerete types(Car and House).
// to handle this, we use Trait objects (dyn Trait).
// dyn = dynamic dispatch.(when compiler does not figure out
// which concrete method to be called at compile time.)
// the concrete type will be determined at runtime. => increases runtime performance cost.
// trait objects must be behind some pointer(stored in heap).

fn create_paitable_object(vehicle: bool) -> Box<dyn Paint> {
    if vehicle {
        Box::new(Car {
            info : VehicleInfo {
                make: "Toyota".to_string(),
                model: "Corolla".to_string(),
                year: 2020
            }
        })
    } else {
        Box::new(House {})
    }
}

fn paint_black(object: &dyn Paint) {
    object.paint("Black".to_string());
}

// Summary
// Trait bounds for static dispatch.
// Trait objects for dynamic dispatch.
// Derive attribute(#[derive(_, _)]) is used to automatically implement traits which has associated derive macro.

fn main() {
    let car = Car {
        info : VehicleInfo {
            make: "Toyota".to_string(),
            model: "Corolla".to_string(),
            year: 2020
        }
    };

    let house = House {};

    let paintable_house = create_paitable_house();

    let object = create_paitable_object(true);

    let paintable_objects: Vec<&dyn Paint> = vec![&car, &house];

    paint_red(&car);
    paint_red(&house);
    paint_red(&paintable_house);

    paint_vehicle_red(&car);
    paint_black(object.as_ref());
}

