#![allow(unused)]
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use serde_json;
use serde_path_to_error;
use zip::ZipArchive;

use std::collections::BTreeMap;
use std::io::{Cursor, Seek};
use std::path::PathBuf;

mod codegen;
mod data;

use crate::datapacks::data::WorldGen;

fn world_gen_from_datapack(archive: &mut ZipArchive<impl std::io::Read + Seek>) -> WorldGen {
    let file_names = archive.file_names().map(PathBuf::from).collect::<Vec<_>>();
    fn collect_from_archive<T: serde::de::DeserializeOwned>(
        archive: &mut ZipArchive<impl std::io::Read + Seek>,
        file_names: &[PathBuf],
        path_pat: &str,
    ) -> BTreeMap<String, T> {
        file_names
            .iter()
            .filter_map(|name| {
                if name.extension().is_some() && name.starts_with(path_pat) {
                    let file = archive.by_path(name).unwrap();
                    return Some((
                        format!(
                            "minecraft:{}",
                            name.strip_prefix(path_pat)
                                .expect(&format!("starts with {path_pat}"))
                                .to_str()
                                .expect("a valid utf-8 string")
                                .split_once(".json")
                                .unwrap()
                                .0
                        ),
                        serde_path_to_error::deserialize(
                            &mut serde_json::de::Deserializer::from_reader(file),
                        )
                        .expect("valid datapack format"),
                    ));
                }
                None
            })
            .collect()
    }

    WorldGen {
        density_function: collect_from_archive(
            archive,
            &file_names,
            "data/minecraft/worldgen/density_function/",
        ),
        noise: collect_from_archive(archive, &file_names, "data/minecraft/worldgen/noise/"),
        noise_settings: collect_from_archive(
            archive,
            &file_names,
            "data/minecraft/worldgen/noise_settings/",
        ),
        world_preset: collect_from_archive(
            archive,
            &file_names,
            "data/minecraft/worldgen/world_preset/",
        ),
    }
}

pub fn build_density_function(_input: TokenStream) -> TokenStream {
    let zip_file_bytes = include_bytes!("../../../../../assets/vanila.zip");
    let zip_file_cursor = Cursor::new(zip_file_bytes);
    let mut archive =
        ZipArchive::new(zip_file_cursor).expect("assets/vanila.zip is a valid zip file");
    let worldgen = world_gen_from_datapack(&mut archive);
    _input
}

#[test]
pub fn compile_datapack() {
    use crate::datapacks::data::{Generator, NoiseGeneratorSettings};
    let zip_file_bytes = include_bytes!("../../../../../assets/vanila.zip");
    let zip_file_cursor = Cursor::new(zip_file_bytes);
    let mut archive = ZipArchive::new(zip_file_cursor).expect("vanila.zip to be a zip archive");
    let worldgen = world_gen_from_datapack(&mut archive);
    let preset = worldgen.world_preset.get("minecraft:normal").unwrap();
    let Generator::Noise { ref settings, .. } = preset
        .dimensions
        .get("minecraft:overworld")
        .unwrap()
        .generator
    else {
        unreachable!("minecraft:overworld dimension should exist")
    };
    let NoiseGeneratorSettings::Id(noise_id) = settings else {
        unreachable!("This should be noise_id");
    };
    let noise = worldgen
        .noise_settings
        .get(noise_id)
        .expect(&format!("{} exists", &noise_id));
    let router = &noise.noise_router;
    let final_density = &router.final_density;
    let token_stream = final_density.to_function(&worldgen, format_ident!("final_density"));
    let file_string = token_stream.to_string();
    println!("{}", file_string);
}
