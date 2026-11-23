use statecraft_core::{container::ProviderContainer, provider::ReadKey};

#[derive(Debug)]
struct RobotState {
    pir_detected: bool,
    usb_connected: bool,
    battery: u8,
}

impl RobotState {
    const fn new() -> Self {
        Self {
            pir_detected: false,
            usb_connected: false,
            battery: 82,
        }
    }
}

static PIR_PROVIDER: ReadKey<RobotState, bool> = ReadKey::new(0, |state| &state.pir_detected);

static USB_PROVIDER: ReadKey<RobotState, bool> = ReadKey::new(1, |state| &state.usb_connected);

static BATTERY_PROVIDER: ReadKey<RobotState, u8> = ReadKey::new(2, |state| &state.battery);

#[test]
fn register_and_read() {
    let mut container = ProviderContainer::<RobotState, 8>::new(RobotState::new());

    container.register(&PIR_PROVIDER).unwrap();
    container.register(&USB_PROVIDER).unwrap();
    container.register(&BATTERY_PROVIDER).unwrap();

    let pir = container.read(&PIR_PROVIDER).unwrap();

    let usb = container.read(&USB_PROVIDER).unwrap();

    let battery = container.read(&BATTERY_PROVIDER).unwrap();

    assert!(!*pir);
    assert!(!*usb);
    assert_eq!(*battery, 82);

    assert_eq!(container.registered_count(), 3);
    assert_eq!(container.capacity(), 8);
}

#[test]
fn register_and_read_local_provider() {
    let provider = ReadKey::new(0, |state: &RobotState| &state.battery);
    let mut container = ProviderContainer::<_, 1>::new(RobotState::new());

    container.register(&provider).unwrap();

    assert_eq!(*container.read(&provider).unwrap(), 82);
    assert_eq!(container.registered_count(), 1);
}

#[test]
fn read_borrows_owned_state_independently_of_key_argument() {
    fn read_with_short_key_borrow<'container>(
        container: &'container ProviderContainer<'_, String, 1>,
        provider: &ReadKey<String, String>,
    ) -> &'container String {
        container.read(provider).unwrap()
    }

    let provider = ReadKey::new(0, |state: &String| state);
    let mut container = ProviderContainer::<_, 1>::new(String::from("local state"));
    container.register(&provider).unwrap();

    let value = read_with_short_key_borrow(&container, &provider);

    assert_eq!(value, "local state");
    assert!(core::ptr::eq(value, container.read(&provider).unwrap()));
}
