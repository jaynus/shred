extern crate shred;
#[macro_use]
extern crate shred_derive;

use shred::{Dispatcher, DispatcherBuilder, Fetch, FetchMut, Resource, Resources, System};

#[derive(Debug)]
struct Res;

impl Resource for Res {}

#[derive(SystemData)]
struct DummyData<'a> {
    _res: Fetch<'a, Res>,
}

#[derive(SystemData)]
struct DummyDataMut<'a> {
    _res: FetchMut<'a, Res>,
}

struct DummySys;

impl<'a, C> System<'a, C> for DummySys {
    type SystemData = DummyData<'a>;

    fn work(&mut self, _data: Self::SystemData, _context: C) {}
}

struct DummySysMut;

impl<'a, C> System<'a, C> for DummySysMut {
    type SystemData = DummyDataMut<'a>;

    fn work(&mut self, _data: Self::SystemData, _context: C) {}
}

#[test]
fn dispatch_builder() {
    DispatcherBuilder::<()>::new()
        .add(DummySys, "a", &[])
        .add(DummySys, "b", &["a"])
        .add(DummySys, "c", &["a"])
        .finish();
}

#[test]
#[should_panic(expected = "No such system registered")]
fn dispatch_builder_invalid() {
    DispatcherBuilder::<()>::new()
        .add(DummySys, "a", &[])
        .add(DummySys, "b", &["z"])
        .finish();
}

#[test]
fn dispatch_basic() {
    let mut res = Resources::new();
    res.add(Res, ());

    let mut d: Dispatcher<_> = DispatcherBuilder::new()
        .add(DummySys, "a", &[])
        .add(DummySys, "b", &["a"])
        .finish();

    d.dispatch(&mut res, ());
}

#[test]
fn dispatch_rw_block() {
    let mut res = Resources::new();
    res.add(Res, ());

    let mut d: Dispatcher<_> = DispatcherBuilder::new()
        .add(DummySys, "a", &[])
        .add(DummySysMut, "b", &[])
        .finish();

    d.dispatch(&mut res, ());
}

#[test]
fn dispatch_rw_block_rev() {
    let mut res = Resources::new();
    res.add(Res, ());

    let mut d: Dispatcher<_> = DispatcherBuilder::new()
        .add(DummySysMut, "a", &[])
        .add(DummySys, "b", &[])
        .finish();

    d.dispatch(&mut res, ());
}

#[test]
fn dispatch_sequential() {
    let mut res = Resources::new();
    res.add(Res, ());

    let mut d: Dispatcher<_> = DispatcherBuilder::new()
        .add(DummySysMut, "a", &[])
        .add(DummySys, "b", &[])
        .finish();

    d.dispatch_seq(&mut res, ());
}
