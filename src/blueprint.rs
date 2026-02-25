use std::fmt::Write as _;
use std::io::{Cursor, Read, Write};
use std::str::FromStr;

use crate::data::blueprint::BlueprintData;
use crate::data::visit::{Visit, Visitor};
use crate::error::{some_error, Error};
use crate::version::with_game_version;
use base64::engine::GeneralPurpose;
use base64::Engine;
use binrw::{BinReaderExt, BinWrite};
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
#[cfg(feature = "dump")]
use serde::{Deserialize, Serialize};

use crate::md5::{Algo, MD5Hash, MD5};

#[cfg_attr(feature = "dump", derive(Serialize, Deserialize))]
pub struct Blueprint {
    pub layout: u32,
    pub icons: [u32; 5],
    pub timestamp: u64,
    pub game_version: String,
    pub icon_text: String,
    pub desc: String,
    pub data: BlueprintData,
}

const B64: GeneralPurpose = base64::engine::general_purpose::STANDARD;

impl Blueprint {
    fn int<T: FromStr>(data: &str, what: &str) -> Result<T, Error> {
        str::parse(data).map_err(|_| format!("Failed to parse {}", what).into())
    }

    fn unpack_data(b64data: &str) -> anyhow::Result<(BlueprintData, Vec<u8>)> {
        let zipped_data = B64
            .decode(b64data)
            .map_err(|_| some_error("Failed to base64 decode blueprint"))?;
        let mut d = GzDecoder::new(zipped_data.as_slice());
        let mut data = vec![];
        d.read_to_end(&mut data)?;
        // Binary V2 format is detected per-building inside BuildingHeader
        // by checking whether the first i32 <= -100. No global flag needed here.
        let mut c = Cursor::new(data);
        let out = c.read_le()?;
        Ok((out, c.into_inner()))
    }

    fn hash_str_to_hash(d: &str) -> anyhow::Result<MD5Hash> {
        if d.len() != 32 {
            return Err(some_error(format!(
                "Unexpected hash length, expected 32, got {}",
                d.len()
            )));
        }
        Ok((0..16)
            .map(|x| (2 * x..2 * x + 2))
            .map(|x| &d[x])
            .map(|x| u8::from_str_radix(x, 16))
            .collect::<Result<Vec<_>, _>>()?
            .try_into()
            .unwrap())
    }

    fn hash(data: &str) -> MD5Hash {
        MD5::new(Algo::MD5F).process(data.as_bytes())
    }

    fn pack_data(&self, level: Compression) -> anyhow::Result<String> {
        let mut e = GzEncoder::new(Vec::new(), level);
        let mut ws = Cursor::new(vec![]);
        self.data.write_le(&mut ws)?;
        e.write_all(&ws.into_inner()).unwrap();
        let gzipped_data = e.finish().unwrap();
        Ok(B64.encode(gzipped_data.as_slice()))
    }

    pub fn new(data: &str) -> anyhow::Result<Self> {
        let (me, _) = Self::new_with_raw_bp(data)?;
        Ok(me)
    }

    pub fn new_with_raw_bp(data: &str) -> anyhow::Result<(Self, Vec<u8>)> {
        let data_and_hash: Vec<&str> = data.rsplitn(2, "\"").collect();
        if data_and_hash.len() != 2 {
            return Err(some_error("Did not find hash delimiter"));
        }
        let [mut hash, mut data]: [&str; 2] = data_and_hash.try_into().unwrap();
        hash = hash.trim();
        data = data.trim();

        // NOTICE: we hash the blueprint without the trailing quote!
        let hash = Self::hash_str_to_hash(hash)?;
        let our_hash = Self::hash(data);
        if hash != our_hash {
            return Err(some_error(format!(
                "Blueprint hash does not match calculated hash: {:x?} != {:x?}",
                hash, our_hash
            )));
        }

        const PREFIX: &str = "BLUEPRINT:";
        if data.len() < PREFIX.len() || &data[0..PREFIX.len()] != PREFIX {
            let ml = std::cmp::min(PREFIX.len(), data.len());
            return Err(some_error(format!("Unexpected prefix: {}", &data[0..ml])));
        }
        data = &data[PREFIX.len()..];

        // The '"' character separates the CSV header (including desc) from the
        // base-64 payload.  V10 (>= 0.10.30.22239) inserts extra empty comma-
        // separated fields between desc and '"'; using find('"') handles both
        // the old format (12 fields) and the new format (15 fields) correctly.
        let quote_pos = data
            .find('"')
            .ok_or_else(|| some_error("No '\"' delimiter found in blueprint data"))?;
        let csv_part = &data[..quote_pos];
        let b64data = &data[quote_pos + 1..];

        let fields: Vec<&str> = csv_part.split(',').collect();
        if fields.len() < 12 {
            return Err(some_error(format!(
                "Expected at least 12 CSV elements, got {}",
                fields.len()
            )));
        }

        let [_fixed0_1, layout]: [&str; 2] = fields[0..2].try_into().unwrap();
        let icons = &fields[2..7];
        let [_fixed0_2, timestamp, game_version, icon_text]: [&str; 4] =
            fields[7..11].try_into().unwrap();
        // fields[11] is always the description; fields[12..] are V10 reserved extras.
        let desc = fields[11];

        let layout = Self::int(layout, "layout")?;
        let icons: Vec<u32> = icons
            .into_iter()
            .map(|x| Self::int(*x, "icon"))
            .collect::<Result<Vec<_>, _>>()?;
        let timestamp = Self::int(timestamp, "timestamp")?;
        // fixed0_1 / fixed0_2 are ignored: V10 uses 1 instead of 0.

        let (data, raw_bp) = with_game_version(game_version, || Self::unpack_data(b64data))?;

        Ok((
            Self {
                layout,
                icons: icons.try_into().unwrap(),
                timestamp,
                game_version: game_version.into(),
                icon_text: icon_text.into(),
                desc: desc.into(),
                data,
            },
            raw_bp,
        ))
    }
    pub fn into_bp_string(&self, level: u32) -> anyhow::Result<String> {
        let icons = self.icons.map(|x| x.to_string()).join(",");
        // Build the header + payload inside with_game_version so is_v10() is
        // set correctly for both pack_data (binary) and the CSV format changes.
        let mut out = with_game_version(&self.game_version, || -> anyhow::Result<String> {
            let b64data = self.pack_data(Compression::new(level))?;
            // V10: fixed0_1 = 1, and 2 extra empty fields after desc before '"'
            let (fixed0_1, extra) = if crate::version::is_v10() {
                ("1", ",,,")
            } else {
                ("0", "")
            };
            Ok(format!(
                "BLUEPRINT:{},{},{},0,{},{},{},{}{}\"{}",
                fixed0_1, self.layout, icons, self.timestamp,
                self.game_version, self.icon_text, self.desc, extra, b64data,
            ))
        })?;
        let hash = Self::hash(&out);
        write!(&mut out, "\"").unwrap();
        for b in hash {
            write!(&mut out, "{:02X}", b).unwrap();
        }
        Ok(out)
    }

    #[cfg(feature = "dump")]
    pub fn new_from_json(json: &str) -> anyhow::Result<Self> {
        Ok(serde_json::from_str(json)?)
    }

    #[cfg(feature = "dump")]
    pub fn dump_json(&self) -> anyhow::Result<Vec<u8>> {
        Ok(serde_json::to_vec(self)?)
    }

    pub fn get_description(&self) -> anyhow::Result<String> {
        Ok(urlencoding::decode(&self.desc)?.into_owned())
    }

    pub fn set_icon_text(&mut self, text: &str) {
        self.icon_text = urlencoding::encode(text).into_owned();
    }

    pub fn get_icon_text(&self) -> anyhow::Result<String> {
        Ok(urlencoding::decode(&self.icon_text)?.into_owned())
    }
}

impl Visit for Blueprint {
    fn visit<T: Visitor + ?Sized>(&mut self, visitor: &mut T) {
        visitor.visit_blueprint_data(&mut self.data)
    }
}

#[cfg(test)]
mod test {
    use super::Blueprint;

    /// Parse → serialize → re-parse; both parse steps must succeed.
    fn round_trip(raw: &str) -> Blueprint {
        let bp = Blueprint::new(raw).expect("parse failed");
        let out = bp.into_bp_string(6).expect("serialize failed");
        Blueprint::new(&out).expect("round-trip parse failed");
        bp
    }

    /// Collect all *.txt files from the examples/ directory at test time.
    fn all_example_files() -> Vec<std::path::PathBuf> {
        let mut dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        dir.push("examples");
        std::fs::read_dir(&dir)
            .unwrap_or_else(|_| panic!("cannot read examples dir: {}", dir.display()))
            .filter_map(|entry| {
                let entry = entry.ok()?;
                let path = entry.path();
                if path.extension()?.to_str()? == "txt" {
                    Some(path)
                } else {
                    None
                }
            })
            .collect()
    }

    /// Every *.txt file in examples/ must parse and round-trip without errors.
    #[test]
    fn all_blueprints_round_trip() {
        let files = all_example_files();
        assert!(!files.is_empty(), "No .txt files found in examples/");
        for path in files {
            let raw_bytes = std::fs::read(&path)
                .unwrap_or_else(|e| panic!("cannot read {}: {}", path.display(), e));
            let raw = std::str::from_utf8(&raw_bytes)
                .unwrap_or_else(|_| panic!("{} is not valid UTF-8", path.display()))
                .trim();
            let bp = round_trip(raw);
            eprintln!(
                "{}: game_version={} buildings={}",
                path.file_name().unwrap().to_string_lossy(),
                bp.game_version,
                bp.data.buildings.len()
            );
        }
    }

    /// Take an existing V1 blueprint, change its game_version to a V10 value,
    /// re-serialize (which writes magic_version+tilt per building), then
    /// re-parse the resulting V10 string.  Verifies the full V10 encode/decode
    /// cycle without requiring a real game-generated V10 blueprint file.
    #[test]
    fn v10_encode_decode_round_trip() {
        // Use the first .txt file we can find.
        let files = all_example_files();
        assert!(!files.is_empty(), "No .txt files found in examples/");
        let path = &files[0];
        let raw_bytes = std::fs::read(path).unwrap();
        let raw = std::str::from_utf8(&raw_bytes).unwrap().trim();

        // Parse as original (pre-V10) version.
        let mut bp = Blueprint::new(raw).expect("initial parse failed");
        let original_count = bp.data.buildings.len();

        // Force V10 game version — from here all serialisation/parsing uses V10 format.
        bp.game_version = "0.10.34.28470".to_string();

        // Re-serialise: is_v10()=true → CSV has fixed0_1=1 + extra commas,
        // and binary has magic_version + tilt per building.
        let v10_str = bp.into_bp_string(6).expect("V10 serialize failed");

        // Confirm the CSV prefix changed (fixed0_1 should now be 1).
        assert!(v10_str.starts_with("BLUEPRINT:1,"), "expected V10 fixed0_1=1");

        // Re-parse the V10 string — must succeed and have the same building count.
        let bp2 = Blueprint::new(&v10_str).expect("V10 parse failed");
        assert_eq!(
            bp2.game_version, "0.10.34.28470",
            "game_version mismatch after V10 round-trip"
        );
        assert_eq!(
            bp2.data.buildings.len(),
            original_count,
            "building count changed during V10 round-trip"
        );
        eprintln!(
            "V10 round-trip OK: {} buildings, game_version={}",
            bp2.data.buildings.len(),
            bp2.game_version
        );
    }
}
