/// Represents the current sensor and control states of the starship airlock.
pub struct AirlockState {
    pub main_power: bool,
    pub backup_power: bool,
    pub captain_override: bool,
    pub life_support_failure: bool,
    pub officer_a: bool,
    pub officer_b: bool,
    pub high_external_pressure: bool,
    pub inner_door_open: bool,
}

/// Evaluates whether the starship's outer airlock door can be safely opened
/// based on the provided `AirlockState`.
///
/// # Arguments
/// * `state` - A reference to an `AirlockState` struct containing all sensor readings.
///
/// # Returns
/// * `true` if the airlock can open safely, `false` otherwise.
fn can_open_airlock(state: &AirlockState) -> bool {
    // 1. NAND Lockout: Prevent opening if high pressure AND inner door are open simultaneously.
    let safety_lockout = !(state.high_external_pressure && state.inner_door_open);

    // 2. Dual Authorization (XOR): Exactly one security officer must be present.
    let dual_auth = state.officer_a != state.officer_b;

    // 3. Power Condition (AND): Both primary and backup power must be active.
    let power_ok = state.main_power && state.backup_power;

    // 4. Emergency Override (OR): Captain override or life support failure.
    let emergency = state.captain_override || state.life_support_failure;

    // The safety lockout is mandatory in all scenarios.
    // In an emergency, safety_lockout is sufficient. Otherwise, power and dual_auth are required.
    safety_lockout && (emergency || (power_ok && dual_auth))
}

fn main() {
    println!("--- Starship Airlock Security System ---");

    // Initialize the state cleanly using struct syntax
    let normal_state = AirlockState {
        main_power: true,
        backup_power: true,
        captain_override: false,
        life_support_failure: false,
        officer_a: true,
        officer_b: false,
        high_external_pressure: false,
        inner_door_open: false,
    };

    let result = can_open_airlock(&normal_state);
    println!("Can open airlock? {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_operation_success() {
        let state = AirlockState {
            main_power: true,
            backup_power: true,
            captain_override: false,
            life_support_failure: false,
            officer_a: true,
            officer_b: false,
            high_external_pressure: false,
            inner_door_open: false,
        };
        assert_eq!(can_open_airlock(&state), true);
    }

    #[test]
    fn test_safety_lockout_blocks_everything() {
        let state = AirlockState {
            main_power: true,
            backup_power: true,
            captain_override: true,
            life_support_failure: false,
            officer_a: true,
            officer_b: false,
            high_external_pressure: true, // Danger!
            inner_door_open: true,  // Danger!
        };
        assert_eq!(can_open_airlock(&state), false);
    }
}