// todo: move into own state worker

use std::rc::Rc;

#[derive(Debug)]
pub struct Material {}

#[derive(Debug)]
pub struct Part {
    material: Rc<Material>,
    amount: u32,
}

#[derive(Debug)]
pub struct Routing {
    material: Rc<Material>,
    parts: Box<[Part]>,
}

pub fn get_routing(routing_id: String) -> Routing {
    // todo: where to define the materials and the routings?

    // materials

    let rice = Rc::new(Material {});

    let water = Rc::new(Material {});

    let oil = Rc::new(Material {});

    let bell_pepper = Rc::new(Material {});

    let boiled_rice = Rc::new(Material {});

    let fried_pepper = Rc::new(Material {});

    let mixed_rice_pan = Rc::new(Material {});

    // routings

    let boiled_rice_routing = Routing {
        material: Rc::clone(&boiled_rice),
        parts: Box::new([
            Part {
                material: Rc::clone(&water),
                amount: 1,
            },
            Part {
                material: Rc::clone(&rice),
                amount: 500,
            },
        ]),
    };

    let fried_pepper_routing = Routing {
        material: Rc::clone(&fried_pepper),
        parts: Box::new([
            Part {
                material: Rc::clone(&oil),
                amount: 10,
            },
            Part {
                material: Rc::clone(&bell_pepper),
                amount: 4,
            },
        ]),
    };

    let mixed_rice_pan_routing = Routing {
        material: Rc::clone(&mixed_rice_pan),
        parts: Box::new([
            Part {
                material: Rc::clone(&boiled_rice),
                amount: 1,
            },
            Part {
                material: Rc::clone(&fried_pepper),
                amount: 1,
            },
        ]),
    };

    mixed_rice_pan_routing
}
