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
      {"id": "SorterMKI",   "label": "分拣器 Mk.I"},
      {"id": "SorterMKII",  "label": "分拣器 Mk.II"},
      {"id": "SorterMKIII", "label": "分拣器 Mk.III"}
    ]
  },
  {
    "id": "assembler",
    "label": "制造台 (Assembler)",
    "members": [
      {"id": "AssemblingMachineMkI",   "label": "制造台 Mk.I"},
      {"id": "AssemblingMachineMkII",  "label": "制造台 Mk.II"},
      {"id": "AssemblingMachineMkIII", "label": "制造台 Mk.III"}
    ]
  },
  {
    "id": "smelter",
    "label": "熔炉 (Smelter)",
    "members": [
      {"id": "ArcSmelter",   "label": "电弧熔炉"},
      {"id": "PlaneSmelter", "label": "位面熔炉"}
    ]
  }
]"#.to_owned()
}
