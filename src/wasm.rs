use std::collections::HashMap;

use wasm_bindgen::prelude::*;

use crate::{
    blueprint::Blueprint,
    data::{
        enums::{DSPItem, DSPRecipe},
        traits::{DSPEnum, TryFromUserString},
        visit::Visitor,
    },
    edit::{stats::GetStats, EditBlueprint},
    error::some_error,
    locale,
};

#[wasm_bindgen(start)]
pub fn init() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

fn parse_comma_list(s: &str) -> anyhow::Result<Vec<(String, String)>> {
    s.split(',')
        .filter(|v| !v.trim().is_empty())
        .map(|v| {
            let p: Vec<&str> = v.split(':').collect();
            if p.len() != 2 {
                return Err(some_error(format!(
                    "Invalid replacement: \"{}\". Expected \"From:To\".",
                    v
                )));
            }
            Ok((p[0].trim().to_owned(), p[1].trim().to_owned()))
        })
        .collect()
}

fn parse_building_map(s: &str) -> anyhow::Result<HashMap<DSPItem, DSPItem>> {
    let list = parse_comma_list(s)?;
    let mut map = HashMap::new();
    for (a, b) in list {
        let a = DSPItem::try_from_user_string(&a)?;
        let b = DSPItem::try_from_user_string(&b)?;
        map.insert(a, b);
    }
    Ok(map)
}

fn parse_item_map(s: &str) -> anyhow::Result<HashMap<DSPItem, DSPItem>> {
    parse_building_map(s)
}

fn parse_recipe_map(s: &str) -> anyhow::Result<HashMap<DSPRecipe, DSPRecipe>> {
    let list = parse_comma_list(s)?;
    let mut map = HashMap::new();
    for (a, b) in list {
        let a = DSPRecipe::try_from_user_string(&a)?;
        let b = DSPRecipe::try_from_user_string(&b)?;
        map.insert(a, b);
    }
    Ok(map)
}

/// Edit a blueprint.
///
/// All replacement strings use the format "From:To,From2:To2,...".
/// Empty strings are ignored.
///
/// - `replace_building`: upgrade/downgrade buildings (e.g. "ConveyorBeltMKI:ConveyorBeltMKII")
/// - `replace_item`: replace items (e.g. "IronIngot:CopperIngot")
/// - `replace_recipe`: replace recipes
/// - `replace_both`: replace items + auto-match recipes
/// - `compression_level`: 1-9, default 6
///
/// Returns the new blueprint string, or throws a JS error on failure.
#[wasm_bindgen]
pub fn edit_blueprint(
    bp_string: &str,
    replace_building: &str,
    replace_item: &str,
    replace_recipe: &str,
    replace_both: &str,
    compression_level: u32,
) -> Result<String, JsValue> {
    let inner = || -> anyhow::Result<String> {
        let mut bp = EditBlueprint::new(Blueprint::new(bp_string)?);

        let mut item_replace: HashMap<DSPItem, DSPItem> = HashMap::new();
        let mut recipe_replace: HashMap<DSPRecipe, DSPRecipe> = HashMap::new();
        let mut building_replace: HashMap<DSPItem, DSPItem> = HashMap::new();

        if !replace_both.is_empty() {
            let mut r = parse_item_map(replace_both)?;
            let mut r2: HashMap<DSPRecipe, DSPRecipe> = r
                .iter()
                .filter_map(|(k, v)| {
                    let k = DSPRecipe::for_item(k)?;
                    let v = DSPRecipe::for_item(v)?;
                    Some((k, v))
                })
                .collect();
            item_replace.extend(r.drain());
            recipe_replace.extend(r2.drain());
        }
        if !replace_item.is_empty() {
            item_replace.extend(parse_item_map(replace_item)?);
        }
        if !replace_recipe.is_empty() {
            recipe_replace.extend(parse_recipe_map(replace_recipe)?);
        }
        if !replace_building.is_empty() {
            building_replace = parse_building_map(replace_building)?;
        }

        if !item_replace.is_empty() {
            bp.replace_item(item_replace);
        }
        if !recipe_replace.is_empty() {
            bp.replace_recipe(recipe_replace);
        }
        if !building_replace.is_empty() {
            bp.replace_building(building_replace)?;
        }

        bp.0.into_bp_string(compression_level)
    };

    inner().map_err(|e| JsValue::from_str(&format!("{:#}", e)))
}

/// Return a JSON string describing the buildings in the blueprint.
/// Format: { "buildings": { "AssemblingMachineMkI": 5, ... } }
#[wasm_bindgen]
pub fn blueprint_info(bp_string: &str) -> Result<String, JsValue> {
    let inner = || -> anyhow::Result<String> {
        let mut bp = Blueprint::new(bp_string)?;
        let mut stats = GetStats::new();
        stats.visit_blueprint(&mut bp);
        Ok(format!("{}", stats.0))
    };
    inner().map_err(|e| JsValue::from_str(&format!("{:#}", e)))
}

/// Return a JSON string describing all supported upgrade groups.
/// This is a static definition, used by the frontend to render the UI.
#[wasm_bindgen]
pub fn upgrade_groups() -> String {
    // Each group: { id, label, members: [{id, label}] }
    r#"[
  {
    "id": "belt",
    "label": "传送带 (Conveyor Belt)",
    "members": [
      {"id": "ConveyorBeltMKI",   "label": "传送带 Mk.I"},
      {"id": "ConveyorBeltMKII",  "label": "传送带 Mk.II"},
      {"id": "ConveyorBeltMKIII", "label": "传送带 Mk.III"}
    ]
  },
  {
    "id": "sorter",
    "label": "分拣器 (Sorter)",
    "members": [
      {"id": "SorterMKI",      "label": "分拣器 Mk.I"},
      {"id": "SorterMKII",     "label": "分拣器 Mk.II"},
      {"id": "SorterMKIII",    "label": "分拣器 Mk.III"},
      {"id": "AutomaticPiler", "label": "堆叠分拣器"}
    ]
  },
  {
    "id": "assembler",
    "label": "制造台 (Assembler)",
    "members": [
      {"id": "AssemblingMachineMkI",    "label": "制造台 Mk.I"},
      {"id": "AssemblingMachineMkII",   "label": "制造台 Mk.II"},
      {"id": "AssemblingMachineMkIII",  "label": "制造台 Mk.III"},
      {"id": "RecomposingAssembler",    "label": "重组式制造台"}
    ]
  },
  {
    "id": "smelter",
    "label": "熔炉 (Smelter)",
    "members": [
      {"id": "ArcSmelter",        "label": "电弧熔炉"},
      {"id": "PlaneSmelter",      "label": "位面熔炉"},
      {"id": "NegentropySmelter", "label": "负熵熔炉"}
    ]
  },
  {
    "id": "chemplant",
    "label": "化工厂 (Chemical Plant)",
    "members": [
      {"id": "ChemicalPlant",        "label": "化工厂"},
      {"id": "QuantumChemicalPlant", "label": "量子化工厂"}
    ]
  },
  {
    "id": "lab",
    "label": "研究站 (Matrix Lab)",
    "members": [
      {"id": "MatrixLab",        "label": "矩阵研究站"},
      {"id": "SelfevolutionLab", "label": "自演化研究站"}
    ]
  }
]"#.to_owned()
}

/// Return a JSON array of all in-game items with their IDs and Chinese names.
/// Format: [{"id": 1001, "name": "铁矿"}, ...]
#[wasm_bindgen]
pub fn item_list() -> String {
    let items = locale::all_items_cn();
    let entries: Vec<String> = items
        .iter()
        .map(|(id, name)| format!("{{\"id\":{},\"name\":\"{}\"}}", id, name))
        .collect();
    format!("[{}]", entries.join(","))
}

/// Get the 5 blueprint icon slots as a JSON array of u32 values.
/// Icons use DSP's encoding: items 1000-19999, recipes 20000+, signals 0-999.
#[wasm_bindgen]
pub fn get_blueprint_icons(bp_string: &str) -> Result<String, JsValue> {
    let inner = || -> anyhow::Result<String> {
        let bp = Blueprint::new(bp_string)?;
        let icons: Vec<u32> = bp.icons.to_vec();
        Ok(serde_json::to_string(&icons)?)
    };
    inner().map_err(|e| JsValue::from_str(&format!("{:#}", e)))
}

/// Set individual blueprint icon slots.
/// `icons_json` is a JSON array of up to 5 objects: [{"slot": 0, "value": 1001}, ...]
/// slot is 0-4; value uses DSP encoding (item id, or recipe id + 20000, or 0 to clear).
#[wasm_bindgen]
pub fn set_blueprint_icons(
    bp_string: &str,
    icons_json: &str,
    compression_level: u32,
) -> Result<String, JsValue> {
    let inner = || -> anyhow::Result<String> {
        let mut bp = Blueprint::new(bp_string)?;
        let updates: Vec<serde_json::Value> = serde_json::from_str(icons_json)?;
        for item in &updates {
            let slot = item["slot"]
                .as_u64()
                .ok_or_else(|| crate::error::some_error("missing \"slot\" field"))? as usize;
            let value = item["value"]
                .as_u64()
                .ok_or_else(|| crate::error::some_error("missing \"value\" field"))? as u32;
            if slot >= 5 {
                anyhow::bail!("icon slot {} out of range (0-4)", slot);
            }
            bp.icons[slot] = value;
        }
        bp.into_bp_string(compression_level)
    };
    inner().map_err(|e| JsValue::from_str(&format!("{:#}", e)))
}
