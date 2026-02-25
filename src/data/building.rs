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
#[br(little)]
pub struct BuildingHeader {
    // DSP >= 0.10.30.22239 ("V10") prepends a magic_version i32 before the
    // building index and appends a tilt f32 after yaw2.  Detection is done via
    // the global is_v10() flag set from the CSV game_version field, which is
    // the same approach used by the dspbp-master reference implementation.
    #[br(if(crate::version::is_v10()))]
    #[bw(if(crate::version::is_v10()))]
    pub magic_version: i32,

    pub index: i32,
    pub area_index: i8,
    pub local_offset_x: f32,
    pub local_offset_y: f32,
    pub local_offset_z: f32,
    pub local_offset_x2: f32,
    pub local_offset_y2: f32,
    pub local_offset_z2: f32,
    pub yaw: f32,
    pub yaw2: f32,
    // Present only in V10 blueprints.
    #[br(if(crate::version::is_v10()))]
    #[bw(if(crate::version::is_v10()))]
    pub tilt: f32,
    pub item_id: ItemId<u16>,
    pub model_index: BPModelId<u16>,
    pub output_object_index: u32,
    pub input_object_index: u32,
    pub output_to_slot: i8,
    pub input_from_slot: i8,
    pub output_from_slot: i8,
    pub input_to_slot: i8,
    pub output_offset: i8,
    pub input_offset: i8,
    pub recipe_id: RecipeId<u16>,
    pub filter_id: ItemId<u16>,
    pub parameter_count: u16,
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
