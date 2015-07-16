use std::collections::HashMap;

pub trait Service: AsRef<str> {}

const DEFAULT_PROVIDER_NAME: &'static str = "-- <def> --";


pub struct Services {
    providers: HashMap<String, Box<Fn() -> Box<Service>>>,
}

impl Services {
    pub fn new<P>(default_provider: P) -> Services 
                            where P: Fn() -> Box<Service> + 'static {
        let mut inst = Services {
            providers: HashMap::new()
        };

        inst.providers.insert(DEFAULT_PROVIDER_NAME.to_string(), Box::new(default_provider));
        inst
    }

    pub fn add_provider<S, P>(&mut self, name: S, provider: P) 
                                        -> Option<Box<Fn() -> Box<Service>>> 
                                                where S: AsRef<str>,
                                                      P: Fn() -> Box<Service> + 'static {
        self.providers.insert(name.as_ref().to_owned(), Box::new(provider))
    }

    pub fn by_name<S>(&self, name: S) -> Result<Box<Service>, ()> 
                                                where S: AsRef<str> {
        match self.providers.get(name.as_ref()) {
            Some(constructor) => Ok(constructor()),
            None => Err(()),
        }
    }

    pub fn default(&self) -> Box<Service> {
        self.by_name(DEFAULT_PROVIDER_NAME).unwrap()
    }
}

