# simpledi-rs

## What is simpledi-rs?

A simple DI (Dependency Injection) singleton object storage for providing thread-safe shared object management.

## Getting Started

### Cargo.toml

#### Add simpledi-rs dependency.

```toml
[dependencies]
simpledi-rs = "*"
```

### Usage

1. create a DI container (`DIContainerTrait`)
2. create objects, and add them to the container via `create_dep!` macro
3. initialize the DI container
4. inject DI container to DI aware objects which implements `DependencyInjectTrait` via `inject_dep!` macro
5. use the container to get the managed objects 

```rust
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

fn main() {
    let mut container = DIContainer::new(); // (1)

    // add obj to DI
    create_dep!(DIAwareStruct::new(), container); // (2)

    // inject DI container to aware objects
    let container_arc = container.init().unwrap(); // (3)
    inject_dep!(DIAwareStruct, container_arc.clone()); // (4)

    // get container from aware objects
    let injected_obj = container_arc.get::<DIAwareStruct>().unwrap();
}
```

