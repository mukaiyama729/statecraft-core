use statecraft_core::provider::ReadKey;

struct RobotState {
    pir_detected: bool,
    battery: u8,
}

fn get_pir(state: &RobotState) -> &bool {
    &state.pir_detected
}

fn get_battery(state: &RobotState) -> &u8 {
    &state.battery
}

const PIR_PROVIDER: ReadKey<RobotState, bool> = ReadKey::new(0, get_pir);

const BATTERY_PROVIDER: ReadKey<RobotState, u8> = ReadKey::new(1, get_battery);

#[test]
fn read_provider() {
    let state = RobotState {
        pir_detected: true,
        battery: 82,
    };

    let pir = PIR_PROVIDER.get(&state);
    let battery = BATTERY_PROVIDER.get(&state);

    assert_eq!(*pir, true);
    assert_eq!(*battery, 82);
}

#[test]
fn independent_states() {
    let state_a = RobotState {
        pir_detected: true,
        battery: 82,
    };

    let state_b = RobotState {
        pir_detected: false,
        battery: 30,
    };

    assert_eq!(*BATTERY_PROVIDER.get(&state_a), 82);
    assert_eq!(*BATTERY_PROVIDER.get(&state_b), 30);
}

#[test]
fn provider_id() {
    assert_eq!(PIR_PROVIDER.id().index(), 0);
    assert_eq!(BATTERY_PROVIDER.id().index(), 1);
}
