use binrw::{BinRead, BinWrite};
use num_enum::TryFromPrimitiveError;
#[cfg(feature = "dump")]
use serde::{Deserialize, Serialize};

use super::{
    belt::Belt,
    enums::DSPItem,
    station::Station,
    traits::{BPModelId, ItemId, RecipeId},
    visit::{Visit, Visitor},
};

fn b_is(i: ItemId<u16>, f: fn(&DSPItem) -> bool) -> bool {
    i.try_into().as_ref().map(f).unwrap_or(false)
}

#[cfg_attr(feature = "dump", derive(Serialize, Deserialize))]
#[derive(BinRead, BinWrite)]
#[br(import { param_count: usize, building: ItemId<u16> })]
// #[br(pre_assert(param_count <= 32768))] // Arbitrary upper bound to prevent OOM; removed because newer game versions may exceed this
// param_count upper bound removed: newer game versions may exceed 32768
pub enum BuildingParam {
    #[br(pre_assert(b_is(building, DSPItem::is_station)))]
    Station(
        #[br(args { is_interstellar: b_is(building, DSPItem::is_interstellar_station), param_count: param_count })]
         Station,
    ),
    #[br(pre_assert(b_is(building, DSPItem::is_belt)))]
    Belt(
        #[br(if(param_count != 0))]
        #[br(args(param_count))]
        Option<Belt>,
    ),
    Unknown(
        #[br(count = param_count)]
        #[br(little)]
        Vec<u32>,
    ),
}

#[cfg_attr(feature = "dump", derive(Serialize, Deserialize))]
#[derive(BinRead, BinWrite)]
pub struct BuildingHeader {
    // Per-building V2 detection (mirrors the reference JavaScript parser logic):
    //   const index = r.getInt32();
    //   const v2 = index <= -100;
    //
    // If first_int <= -100 → V2 building format:
    //   - first_int is the magic_version sentinel
    //   - index_v2 (the real building index) follows immediately
    //   - tilt_v2 (f32) is present after yaw2
    // Otherwise → V1 building format:
    //   - first_int IS the building index; no extra fields
    #[br(little)]
    pub first_int: i32,

    // V2 only: actual building index (first_int was the sentinel).
    #[br(if(first_int <= -100), little)]
    #[bw(if(*first_int <= -100), little)]
    pub index_v2: u32,

    pub area_index: i8,
    #[br(little)]
    pub local_offset_x: f32,
    #[br(little)]
    pub local_offset_y: f32,
    #[br(little)]
    pub local_offset_z: f32,
    #[br(little)]
    pub local_offset_x2: f32,
    #[br(little)]
    pub local_offset_y2: f32,
    #[br(little)]
    pub local_offset_z2: f32,
    #[br(little)]
    pub yaw: f32,
    #[br(little)]
    pub yaw2: f32,
    // V2 only: tilt angle (0.0 for V1 buildings).
    #[br(if(first_int <= -100), little)]
    #[bw(if(*first_int <= -100), little)]
    pub tilt_v2: f32,
    #[br(little)]
    pub item_id: ItemId<u16>,
    #[br(little)]
    pub model_index: BPModelId<u16>,
    #[br(little)]
    pub output_object_index: u32,
    #[br(little)]
    pub input_object_index: u32,
    pub output_to_slot: i8,
    pub input_from_slot: i8,
    pub output_from_slot: i8,
    pub input_to_slot: i8,
    pub output_offset: i8,
    pub input_offset: i8,
    #[br(little)]
    pub recipe_id: RecipeId<u16>,
    #[br(little)]
    pub filter_id: ItemId<u16>,
    #[br(little)]
    pub parameter_count: u16,
}

impl BuildingHeader {
    /// Returns the building's logical index.
    pub fn index(&self) -> u32 {
        if self.first_int <= -100 {
            self.index_v2
        } else {
            self.first_int as u32
        }
    }
    /// Returns true when this building was encoded in V2 binary format.
    pub fn is_v2(&self) -> bool {
        self.first_int <= -100
    }
    /// Returns the tilt angle (0.0 for V1 buildings).
    pub fn tilt(&self) -> f32 {
        self.tilt_v2
    }
}

#[cfg_attr(feature = "dump", derive(Serialize, Deserialize))]
#[derive(BinRead, BinWrite)]
pub struct Building {
    pub header: BuildingHeader,
    #[br(args { param_count: header.parameter_count as usize, building: header.item_id })]
    pub param: BuildingParam,
}

impl Building {
    pub fn kind(&self) -> Result<DSPItem, TryFromPrimitiveError<DSPItem>> {
        DSPItem::try_from(self.header.item_id)
    }
}

impl Visit for Building {
    fn visit<T: Visitor + ?Sized>(&mut self, visitor: &mut T) {
        match &mut self.param {
            BuildingParam::Station(s) => visitor.visit_station(s),
            BuildingParam::Belt(Some(b)) => visitor.visit_belt(b),
            _ => (),
        }
    }
}
