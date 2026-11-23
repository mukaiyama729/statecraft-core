use statecraft_core::{
    container::ProviderContainer,
    provider::{ReadKey, StateProvider},
};

struct RobotState {
    battery: u8,
    pir_detected: bool,
}

fn read_battery(s: &RobotState) -> &u8 {
    &s.battery
}

fn write_battery(s: &mut RobotState) -> &mut u8 {
    &mut s.battery
}

static BATTERY_READ: ReadKey<RobotState, u8> = ReadKey::new(0, read_battery);

static BATTERY_PROVIDER: StateProvider<RobotState, u8> =
    StateProvider::new(&BATTERY_READ, write_battery);

#[test]
fn test_state_provider() {
    let state = RobotState {
        battery: 80,
        pir_detected: false,
    };

    let mut container = ProviderContainer::<RobotState, 8>::new(state);

    // Provider 登録
    container.register_state(&BATTERY_PROVIDER).unwrap();

    // 読み取り
    let battery = container.read_state(&BATTERY_PROVIDER).unwrap();

    assert_eq!(*battery, 80);

    // 更新
    container.set(&BATTERY_PROVIDER, 90).unwrap();

    assert_eq!(*container.read_state(&BATTERY_PROVIDER).unwrap(), 90);

    // 現在値に基づく更新
    container
        .update(&BATTERY_PROVIDER, |battery| {
            *battery = battery.saturating_sub(10);
        })
        .unwrap();

    assert_eq!(*container.read_state(&BATTERY_PROVIDER).unwrap(), 80);
}
