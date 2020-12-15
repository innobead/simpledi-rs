#[macro_use]
extern crate simpledi_rs;

use simpledi_rs::di::{DIContainer, DIContainerTrait, DependencyInjectTrait};
use std::sync::Arc;

#[derive(Debug)]
struct DIAwareStruct(Option<Arc<DIContainer>>);

impl DIAwareStruct {
    fn new() -> Self {
        Self { 0: None }
    }
}

impl DependencyInjectTrait for DIAwareStruct {
    fn inject(&mut self, container: Arc<DIContainer>) {
        self.0 = Some(container.clone())
    }
}

#[test]
fn test_di() {
    let mut container = DIContainer::new();

    // add obj to DI
    create_dep!(DIAwareStruct::new(), container);

    // inject DI container to aware objects
    let container_arc = container.init().unwrap();
    let container_ptr = container_arc.as_ref() as *const DIContainer;

    inject_dep!(DIAwareStruct, container_arc.clone());

    // get container from aware objects
    let injected_obj = container_arc.get::<DIAwareStruct>().unwrap();
    let injected_container = injected_obj.0.as_ref().unwrap().as_ref();

    assert_eq!(container_ptr, injected_container as *const DIContainer);
}