use statecraft_core::{container::ProviderContainer, provider::NotifierProvider, state::Field};

struct Battery;
impl Field<u8> for Battery {
    type Value = u8;
    fn get(state: &u8) -> &u8 {
        state
    }
    fn get_mut(state: &mut u8) -> &mut u8 {
        state
    }
}

struct BatteryLogic {
    drain: u8,
}

impl BatteryLogic {
    fn tick(&self, container: &mut ProviderContainer<u8>) {
        container
            .update::<Battery, _>(|battery| *battery = battery.saturating_sub(self.drain))
            .unwrap();
    }
}

#[test]
fn notifier_preserves_logic_without_provider_keys() {
    const NOTIFIER: NotifierProvider<Battery, BatteryLogic> =
        NotifierProvider::new(BatteryLogic { drain: 10 });
    let mut container = ProviderContainer::new(82);

    NOTIFIER.logic().tick(&mut container);

    assert_eq!(*container.read::<Battery>().unwrap(), 72);
    assert_eq!(NOTIFIER.logic().drain, 10);
}
