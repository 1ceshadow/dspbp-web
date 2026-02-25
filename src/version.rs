use std::cell::Cell;

// DSP 0.10.30.22239 introduced CSV format changes (extra fields, fixed0_1=1).
// Binary V2 format detection is now done per-building by checking whether the
// first i32 of each BuildingHeader is <= -100 (a sentinel value). No global
// binary-version flag is needed.

const V10_VERSION: [u32; 4] = [0, 10, 30, 22239];

thread_local! {
    static IS_CSV_V10: Cell<bool> = const { Cell::new(false) };
}

/// True when the CSV game_version is >= 0.10.30.22239.
pub fn is_v10() -> bool {
    IS_CSV_V10.with(|v| v.get())
}

fn parse_version(s: &str) -> [u32; 4] {
    let mut parts = [0u32; 4];
    for (i, part) in s.split('.').take(4).enumerate() {
        parts[i] = part.parse().unwrap_or(0);
    }
    parts
}

/// Run `f` with IS_CSV_V10 set from `game_version`.
/// The flag is restored to its previous value when `f` returns.
pub fn with_game_version<T, F: FnOnce() -> T>(game_version: &str, f: F) -> T {
    let csv_v10 = parse_version(game_version) >= V10_VERSION;
    let prev_csv = IS_CSV_V10.with(|v| { let p = v.get(); v.set(csv_v10); p });
    let result = f();
    IS_CSV_V10.with(|v| v.set(prev_csv));
    result
}
