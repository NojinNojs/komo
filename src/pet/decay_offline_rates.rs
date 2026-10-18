/// Realistic per-minute stat drift while the keeper is offline, tuned by sleep state.
pub(super) fn offline_drop_rates(minutes: u64, sleeping: bool) -> (u8, u8, u8, u8) {
    if minutes <= 20 {
        let step = (minutes / 10) as u8;
        if sleeping {
            (step, 0, (minutes as u8).saturating_mul(2).min(40), 0)
        } else {
            (step.max(1), step, step, step)
        }
    } else {
        let units = (minutes / 12).min(95) as u8;
        if sleeping {
            (
                (units / 2).min(60),
                (units / 3).min(45),
                units.saturating_mul(5).min(100),
                (units / 3).min(45),
            )
        } else {
            (
                units.min(90),
                (units * 2 / 3).min(70),
                (units * 2 / 3).min(75),
                (units * 2 / 3).min(70),
            )
        }
    }
}
