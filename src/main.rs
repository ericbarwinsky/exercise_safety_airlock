/// Evaluates whether the starship's outer airlock door can be safely opened.
///
/// # Arguments
/// * `main_power` - Status of the primary power grid (bool)
/// * `backup_power` - Status of the backup power grid (bool)
/// * `captain_override` - Manual override triggered by the captain (bool)
/// * `life_support_failure` - Critical failure flag from life support (bool)
/// * `officer_a` - Presence of security officer A (bool)
/// * `officer_b` - Presence of security officer B (bool)
/// * `high_external_pressure` - High pressure detected outside the airlock (bool)
/// * `inner_door_open` - Status of the inner airlock door (bool)
///
/// # Returns
/// * `true` if the airlock can open safely, `false` otherwise.
fn can_open_airlock(
    main_power: bool,
    backup_power: bool,
    captain_override: bool,
    life_support_failure: bool,
    officer_a: bool,
    officer_b: bool,
    high_external_pressure: bool,
    inner_door_open: bool,
) -> bool {
    // 1. NAND Lockout: Prevent opening if high pressure AND inner door are open simultaneously.
    // Logic: !(A && B)
    let safety_lockout = !(high_external_pressure && inner_door_open);

    // 2. Dual Authorization (XOR): Exactly one security officer must be present.
    let dual_auth = officer_a != officer_b;

    // 3. Power Condition (AND): Both primary and backup power must be active.
    let power_ok = main_power && backup_power;

    // 4. Emergency Override (OR): Captain override or life support failure.
    let emergency = captain_override || life_support_failure;

    // The safety lockout is mandatory in all scenarios.
    // In an emergency, safety_lockout is sufficient. Otherwise, power and dual_auth are required.
    safety_lockout && (emergency || (power_ok && dual_auth))
}

fn main() {
    println!("--- Starship Airlock Security System ---");

    // Test the function
    let result = can_open_airlock(true, true, false, false, true, false, false, false);
    println!("Can open airlock? {}", result);
}