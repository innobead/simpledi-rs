use std::any::{Any, type_name};
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::{Arc, Mutex};

use anyhow::Result;
use log::{debug, trace};

/// DependencyInjectTrait, enable objects able to inject DI container for getting interested objects from the container
pub trait DependencyInjectTrait {
    fn inject(&mut self, container: Arc<DIContainer>);
}

/// DIContainerExtTrait, an extended util trait for crate::di::DIContainer for having fast functions to get the managed objects in DI system
pub trait DIContainerExtTrait {
    fn get<T>(&self) -> Option<&T> where T: 'static + Debug;
}

/// DIContainer, a singleton container manages all DI managed objects
#[derive(Debug)]
pub struct DIContainer {
    objects: HashMap<String, Box<dyn Any>>,
    mutex: Mutex<u8>,
}

unsafe impl Send for DIContainer {}

unsafe impl Sync for DIContainer {}

pub trait DIContainerTrait {
    fn new() -> Self;

    fn init(self) -> Result<Arc<Self>>;

    fn add<T>(&mut self, obj: T) -> Result<&mut T>
        where
            T: 'static + Debug;

    fn get<T>(&self) -> Option<&T>
        where
            T: 'static + Debug;

    fn get_mut<T>(&mut self) -> Option<&mut T>
        where
            T: 'static + Debug;
}

impl DIContainerTrait for DIContainer {
    fn new() -> Self {
        debug!("creating DI system container");

        DIContainer {
            objects: Default::default(),
            mutex: Mutex::new(0),
        }
    }

    fn init(self) -> Result<Arc<Self>> {
        debug!("initializing DI system");

        Ok(Arc::new(self))
    }

    fn add<T>(&mut self, obj: T) -> Result<&mut T>
        where
            T: 'static + Debug,
    {
        debug!("adding {:?} to DI system", obj);

        let _ = self.mutex.lock();
        let key = type_name::<T>().to_string();

        self.objects.insert(key, Box::new(obj));

        Ok(self.get_mut::<T>().unwrap())
    }

    fn get<T>(&self) -> Option<&T>
        where
            T: 'static + Debug,
    {
        let name = type_name::<T>().to_string();
        trace!("getting immutable {:?} from DI system", name);

        self.objects
            .get(&name)
            .unwrap()
            .downcast_ref()
    }

    fn get_mut<T>(&mut self) -> Option<&mut T>
        where
            T: 'static + Debug,
    {
        let name = type_name::<T>().to_string();
        trace!("getting mutable {:?} from DI system", name);

        self.objects
            .get_mut(&name)
            .unwrap()
            .downcast_mut()
    }
}

impl DIContainerExtTrait for Option<Arc<DIContainer>> {
    fn get<T>(&self) -> Option<&T> where T: 'static + Debug {
        self.as_ref().unwrap().get::<T>()
    }
}