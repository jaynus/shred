extern crate shred;
#[macro_use]
extern crate shred_derive;

use shred::{DispatcherBuilder, Read, ResourceId, System, SystemData, World, Write};

#[derive(Debug, Default)]
struct ResA;

#[derive(Debug, Default)]
struct ResB;

#[derive(SystemData)]
struct Data<'a> {
    a: Read<'a, ResA>,
    b: Write<'a, ResB>,
}

struct EmptySystem;

impl<'a> System<'a> for EmptySystem {
    type SystemData = Data<'a>;

    fn run(&mut self, bundle: Data<'a>) {
        println!("{:?}", &*bundle.a);
        println!("{:?}", &*bundle.b);
    }
}

fn main() {
    let mut resources = World::empty();
    let mut dispatcher = DispatcherBuilder::new()
        .with(EmptySystem, "empty", &[])
        .build();
    dispatcher.setup(&mut resources);

    dispatcher.dispatch_seq(&resources);
}
