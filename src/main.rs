use std::io::{self, Write};

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

/// The main interactive CLI loop for the starship airlock security system.
fn main() {
    println!("========================================");
    println!("  STARSHIP AIRLOCK SECURITY SYSTEM v1.0 ");
    println!("========================================");
    println!("Please answer the following sensor prompts (true/false):\n");

    let main_power = prompt_boolean("1. Main power active? (true/false): ");
    let backup_power = prompt_boolean("2. Backup power active? (true/false): ");
    let captain_override = prompt_boolean("3. Captain override engaged? (true/false): ");
    let life_support_failure = prompt_boolean("4. Life support failure reported? (true/false): ");
    let officer_a = prompt_boolean("5. Security officer A present? (true/false): ");
    let officer_b = prompt_boolean("6. Security officer B present? (true/false): ");
    let high_external_pressure = prompt_boolean("7. High external pressure detected? (true/false): ");
    let inner_door_open = prompt_boolean("8. Inner airlock door open? (true/false): ");

    let state = AirlockState {
        main_power,
        backup_power,
        captain_override,
        life_support_failure,
        officer_a,
        officer_b,
        high_external_pressure,
        inner_door_open,
    };

    println!("\n----------------------------------------");
    let result = can_open_airlock(&state);
    if result {
        println!(" RESULT: [OPEN] Airlock door can be safely opened.");
    } else {
        println!(" RESULT: [LOCKED] Airlock door remains locked for safety.");
    }
    println!("----------------------------------------");
}

/// Helper function to prompt the user for a boolean value with error handling.
fn prompt_boolean(question: &str) -> bool {
    loop {
        print!("{}", question);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                match input.trim().to_lowercase().parse::<bool>() {
                    Ok(val) => return val,
                    Err(_) => println!("Invalid input. Please type 'true' or 'false'."),
                }
            }
            Err(_) => println!("Failed to read input. Try again."),
        }
    }
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
    fn test_dual_auth_failure_both_officers() {
        let state = AirlockState {
            main_power: true,
            backup_power: true,
            captain_override: false,
            life_support_failure: false,
            officer_a: true,
            officer_b: true,
            high_external_pressure: false,
            inner_door_open: false,
        };
        assert_eq!(can_open_airlock(&state), false);
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
            high_external_pressure: true,
            inner_door_open: true,
        };
        assert_eq!(can_open_airlock(&state), false);
    }

    #[test]
    fn test_emergency_override_bypasses_power() {
        let state = AirlockState {
            main_power: false,
            backup_power: false,
            captain_override: true,
            life_support_failure: false,
            officer_a: false,
            officer_b: false,
            high_external_pressure: false,
            inner_door_open: false,
        };
        assert_eq!(can_open_airlock(&state), true);
    }
}