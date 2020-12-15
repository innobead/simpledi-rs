#[macro_export]
macro_rules! create_dep {
    ($obj:expr, $container:expr) => {{
        $container.add($obj).unwrap()
    }};

    ($obj:expr, $container:expr, $($t:tt)+) => {{
        $container.add($obj).unwrap()$($t)+
    }};
}

#[macro_export]
macro_rules! inject_dep {
    ($struct:ident, $container:expr) => {{
        unsafe {
            let ptr = $container.get::<$struct>().unwrap() as *const dyn DependencyInjectTrait;
            let ptr = ptr as *mut dyn DependencyInjectTrait;
            let s = &mut *ptr;

            s.inject($container);
        }
    }};
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::di::{DependencyInjectTrait, DIContainer, DIContainerTrait};

    #[derive(Debug)]
    struct DIStruct;

    #[derive(Debug)]
    struct DIAwareStruct(Option<Arc<DIContainer>>);

    impl DIStruct {
        fn new() -> Self {
            Self {}
        }
    }

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
    fn test_create_dep() {
        let mut container = DIContainer::new();

        // add obj to DI
        let obj_ptr = {
            let obj = create_dep!(DIStruct::new(), container);
            obj as *const DIStruct
        };

        // get obj from DI
        let container_arc = container.init().unwrap();
        let injected_obj = container_arc.get::<DIStruct>().unwrap();

        assert_eq!(obj_ptr, injected_obj as *const DIStruct);
    }

    #[test]
    fn test_inject_dep() {
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
}