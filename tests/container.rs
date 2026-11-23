use statecraft_core::{container::ProviderContainer, state::Field};

struct RobotState {
    pir_detected: bool,
    usb_connected: bool,
    battery: u8,
}

struct Pir;
impl Field<RobotState> for Pir {
    type Value = bool;
    fn get(state: &RobotState) -> &bool {
        &state.pir_detected
    }
    fn get_mut(state: &mut RobotState) -> &mut bool {
        &mut state.pir_detected
    }
}

struct Usb;
impl Field<RobotState> for Usb {
    type Value = bool;
    fn get(state: &RobotState) -> &bool {
        &state.usb_connected
    }
    fn get_mut(state: &mut RobotState) -> &mut bool {
        &mut state.usb_connected
    }
}

struct Battery;
impl Field<RobotState> for Battery {
    type Value = u8;
    fn get(state: &RobotState) -> &u8 {
        &state.battery
    }
    fn get_mut(state: &mut RobotState) -> &mut u8 {
        &mut state.battery
    }
}

#[test]
fn read_and_update_fields_without_registration() {
    let mut container = ProviderContainer::new(RobotState {
        pir_detected: false,
        usb_connected: true,
        battery: 82,
    });

    assert!(!*container.read::<Pir>().unwrap());
    assert!(*container.read::<Usb>().unwrap());
    assert_eq!(*container.read::<Battery>().unwrap(), 82);

    container.set::<Pir>(true).unwrap();
    container.set::<Usb>(false).unwrap();
    container.set::<Battery>(90).unwrap();
    let previous = container
        .update::<Battery, _>(|battery| {
            let previous = *battery;
            *battery -= 10;
            previous
        })
        .unwrap();

    assert_eq!(previous, 90);
    assert_eq!(*container.read::<Battery>().unwrap(), 80);
    assert!(*container.read::<Pir>().unwrap());
    assert!(!*container.read::<Usb>().unwrap());
}

#[test]
fn independent_containers() {
    let mut first = ProviderContainer::new(RobotState {
        pir_detected: false,
        usb_connected: false,
        battery: 82,
    });
    let second = ProviderContainer::new(RobotState {
        pir_detected: true,
        usb_connected: true,
        battery: 30,
    });

    first.set::<Battery>(100).unwrap();

    assert_eq!(*first.read::<Battery>().unwrap(), 100);
    assert_eq!(*second.read::<Battery>().unwrap(), 30);
}

#[test]
fn read_borrows_owned_state_without_copying() {
    struct Text;
    impl Field<String> for Text {
        type Value = String;
        fn get(state: &String) -> &String {
            state
        }
        fn get_mut(state: &mut String) -> &mut String {
            state
        }
    }

    let mut container = ProviderContainer::new(String::from("local state"));
    let value = container.read::<Text>().unwrap();
    assert_eq!(value, "local state");
    assert!(core::ptr::eq(value, container.read::<Text>().unwrap()));

    let suffix = String::from(" updated");
    container
        .update::<Text, _>(move |text| text.push_str(&suffix))
        .unwrap();
    assert_eq!(container.read::<Text>().unwrap(), "local state updated");
}
