extern crate item01;

use item01::{Service, Services};

#[test]
fn instantiation() {
    struct DefaultService;
    struct CompService;
    struct ArmedService;


    macro_rules! impl_as_ref {
        ($t:ty) => (
            impl AsRef<str> for $t {
                fn as_ref(&self) -> &str {
                    Self::name()
                }
            }
        )
    }

    impl Service for DefaultService {}
    impl_as_ref!(DefaultService);
    impl DefaultService {
        fn name() -> &'static str {
            "Default service"
        }
    }

    impl Service for CompService {}
    impl_as_ref!(CompService);
    impl CompService {
        fn name() -> &'static str {
            "Complementary service"
        }
    }

    impl Service for ArmedService {}
    impl_as_ref!(ArmedService);
    impl ArmedService {
        fn name() -> &'static str {
            "Armed service"
        }
    }

    let mut service = Services::new(|| Box::new(DefaultService) );
    assert_eq!(service.by_name("foo").err(), Some(()));
    assert_eq!(service.by_name("footoo".to_string()).err(), Some(()));

    assert_eq!(service.default().as_ref(), "Default service");
    assert!(service.add_provider(CompService::name(), || Box::new(CompService)).is_none());
    assert!(service.add_provider(CompService::name(), || Box::new(CompService)).is_some());
    assert!(service.add_provider(ArmedService::name(), || Box::new(ArmedService)).is_none());

    assert_eq!(service.by_name(CompService::name()).unwrap().as_ref(), CompService::name());
    assert_eq!(service.by_name(ArmedService::name()).unwrap().as_ref(), ArmedService::name());
}

