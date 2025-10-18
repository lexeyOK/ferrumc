use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct WorldGen {
    // biome: BTreeMap<String, Biome>,
    // biome_parameters: BtreeMap<String,Biome>,
    pub density_function: BTreeMap<String, DensityFunction>,
    pub noise: BTreeMap<String, Noise>,
    pub noise_settings: BTreeMap<String, NoiseSettings>,
    pub world_preset: BTreeMap<String, WorldPreset>,
}

#[derive(Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Noise {
    #[serde(rename = "firstOctave")]
    pub first_octave: i32,
    pub amplitudes: Vec<f64>,
}

#[derive(Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct WorldPreset {
    pub dimensions: BTreeMap<String, Dimension>,
}

#[derive(Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Dimension {
    #[serde(rename = "type")]
    pub dimension_type: String,
    pub generator: Generator,
}

#[derive(Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Generator {
    #[serde(rename = "minecraft:flat")]
    Flat { settings: FlatGeneratorSettings },
    #[serde(rename = "minecraft:noise")]
    Noise {
        settings: NoiseGeneratorSettings,
        biome_source: NoiseGeneratorBiomeSource,
    },
    #[serde(rename = "minecraft:debug")]
    Debug,
}

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct FlatGeneratorSettings {
    pub layers: Vec<FlatGeneratorLayer>,
    pub biome: String,
    pub lakes: Option<bool>,
    pub features: Option<bool>,
    pub structure_overrides: Option<Vec<String>>,
}

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct FlatGeneratorLayer {
    pub height: i32,
    pub block: String,
}

#[derive(Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum NoiseGeneratorBiomeSource {
    #[serde(rename = "minecraft:checkerboard")]
    CheckerBoard { biomes: Vec<String>, scale: i32 },
    #[serde(rename = "minecraft:fixed")]
    Fixed { biome: String },
    #[serde(rename = "minecraft:multi_noise")]
    MultiNoise(MultiNoiseSource),
    #[serde(rename = "minecraft:the_end")]
    TheEnd,
}

#[derive(Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MultiNoiseSource {
    Preset { preset: String },
    Inline { biomes: Vec<Biome> },
}

#[derive(Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Biome {
    pub biome: String,
    pub parameters: BiomeParameters,
}

#[derive(Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct BiomeParameters {
    pub temperature: BiomeParameter,
    pub continentalness: BiomeParameter,
    pub depth: BiomeParameter,
    pub erosion: BiomeParameter,
    pub humidity: BiomeParameter,
    pub offset: BiomeParameter,
    pub weirdness: BiomeParameter,
}

#[derive(Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BiomeParameter {
    Object { min: f32, max: f32 },
    Tuple(f32, f32),
    Single(f32),
}

#[derive(Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NoiseGeneratorSettings {
    Id(String),
    Inline(NoiseSettings),
}

#[derive(Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct NoiseSettings {
    pub sea_level: i32,
    pub default_block: BlockState,
    pub default_fluid: BlockState,
    pub disable_mob_generation: bool,
    pub ore_veins_enabled: bool,
    pub aquifers_enabled: bool,
    pub legacy_random_source: bool,
    pub spawn_target: Vec<BiomeParameters>,
    pub noise: NoiseParams,
    pub noise_router: NoiseRouter,
    pub surface_rule: SurfaceRule,
}

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct NoiseParams {
    pub min_y: i32,
    pub height: i32,
    pub size_horizontal: u8,
    pub size_vertical: u8,
}

#[derive(Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct NoiseRouter {
    #[serde(bound = "")]
    pub initial_density_without_jaggedness: DensityFunction,
    #[serde(bound = "")]
    pub final_density: DensityFunction,
    #[serde(bound = "")]
    pub barrier: DensityFunction,
    #[serde(bound = "")]
    pub fluid_level_floodedness: DensityFunction,
    #[serde(bound = "")]
    pub fluid_level_spread: DensityFunction,
    #[serde(bound = "")]
    pub lava: DensityFunction,
    #[serde(bound = "")]
    pub vein_toggle: DensityFunction,
    #[serde(bound = "")]
    pub vein_ridged: DensityFunction,
    #[serde(bound = "")]
    pub vein_gap: DensityFunction,
    #[serde(bound = "")]
    pub temperature: DensityFunction,
    #[serde(bound = "")]
    pub vegetation: DensityFunction,
    #[serde(bound = "")]
    pub continents: DensityFunction,
    #[serde(bound = "")]
    pub erosion: DensityFunction,
    #[serde(bound = "")]
    pub depth: DensityFunction,
    #[serde(bound = "")]
    pub ridges: DensityFunction,
}

#[derive(Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DensityFunction {
    Id(String),
    Constant(f64),
    #[serde(bound = "")]
    Impl(DensityFunctionImpl),
}

#[derive(Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum DensityFunctionImpl {
    #[serde(rename = "minecraft:interpolated")]
    Interpolated {
        #[serde(bound = "")]
        argument: Box<DensityFunction>,
    },
    #[serde(rename = "minecraft:flat_cache")]
    FlatCache {
        #[serde(bound = "")]
        argument: Box<DensityFunction>,
    },
    #[serde(rename = "minecraft:cache_2d")]
    Cache2D {
        #[serde(bound = "")]
        argument: Box<DensityFunction>,
    },
    #[serde(rename = "minecraft:cache_once")]
    CacheOnce {
        #[serde(bound = "")]
        argument: Box<DensityFunction>,
    },
    #[serde(rename = "minecraft:cache_all_in_cell")]
    CacheAllInCell {
        #[serde(bound = "")]
        argument: Box<DensityFunction>,
    },
    #[serde(rename = "minecraft:abs")]
    Abs {
        #[serde(bound = "")]
        argument: Box<DensityFunction>,
    },
    #[serde(rename = "minecraft:square")]
    Square {
        #[serde(bound = "")]
        argument: Box<DensityFunction>,
    },
    #[serde(rename = "minecraft:cube")]
    Cube {
        #[serde(bound = "")]
        argument: Box<DensityFunction>,
    },
    #[serde(rename = "minecraft:half_negative")]
    HalfNegative {
        #[serde(bound = "")]
        argument: Box<DensityFunction>,
    },
    #[serde(rename = "minecraft:quarter_negative")]
    QuarterNegative {
        #[serde(bound = "")]
        argument: Box<DensityFunction>,
    },
    #[serde(rename = "minecraft:squeeze")]
    Squeeze {
        #[serde(bound = "")]
        argument: Box<DensityFunction>,
    },
    #[serde(rename = "minecraft:invert")]
    Invert {
        #[serde(bound = "")]
        argument: Box<DensityFunction>,
    },
    #[serde(rename = "minecraft:add")]
    Add {
        #[serde(bound = "")]
        argument1: Box<DensityFunction>,
        #[serde(bound = "")]
        argument2: Box<DensityFunction>,
    },
    #[serde(rename = "minecraft:mul")]
    Mul {
        #[serde(bound = "")]
        argument1: Box<DensityFunction>,
        #[serde(bound = "")]
        argument2: Box<DensityFunction>,
    },
    #[serde(rename = "minecraft:min")]
    Min {
        #[serde(bound = "")]
        argument1: Box<DensityFunction>,
        #[serde(bound = "")]
        argument2: Box<DensityFunction>,
    },
    #[serde(rename = "minecraft:max")]
    Max {
        #[serde(bound = "")]
        argument1: Box<DensityFunction>,
        #[serde(bound = "")]
        argument2: Box<DensityFunction>,
    },
    #[serde(rename = "minecraft:blend_alpha")]
    BlendAlpha,
    #[serde(rename = "minecraft:blend_offset")]
    BlendOffset,
    #[serde(rename = "minecraft:blend_density")]
    BlendDensity {
        #[serde(bound = "")]
        argument: Box<DensityFunction>,
    },
    #[serde(rename = "minecraft:beardifier")]
    Beardifier,
    #[serde(rename = "minecraft:old_blended_noise")]
    OldBlendedNoise {
        xz_scale: f32,
        y_scale: f32,
        xz_factor: f32,
        y_factor: f32,
        smear_scale_multiplier: f32,
    },
    #[serde(rename = "minecraft:noise")]
    Noise {
        noise: String,
        xz_scale: f64,
        y_scale: f64,
    },
    #[serde(rename = "minecraft:end_islands")]
    EndIslands,
    #[serde(rename = "minecraft:weird_scaled_sampler")]
    WeirdScaledSampler {
        rarity_value_mapper: String,
        noise: String,
        #[serde(bound = "")]
        input: Box<DensityFunction>,
    },
    #[serde(rename = "minecraft:shifted_noise")]
    ShiftedNoise {
        noise: String,
        xz_scale: f64,
        y_scale: f64,
        #[serde(bound = "")]
        shift_x: Box<DensityFunction>,
        #[serde(bound = "")]
        shift_y: Box<DensityFunction>,
        #[serde(bound = "")]
        shift_z: Box<DensityFunction>,
    },
    #[serde(rename = "minecraft:range_choice")]
    RangeChoice {
        #[serde(bound = "")]
        input: Box<DensityFunction>,
        min_inclusive: f64,
        max_exclusive: f64,
        #[serde(bound = "")]
        when_in_range: Box<DensityFunction>,
        #[serde(bound = "")]
        when_out_of_range: Box<DensityFunction>,
    },
    #[serde(rename = "minecraft:shift_a")]
    ShiftA { argument: String },
    #[serde(rename = "minecraft:shift_b")]
    ShiftB { argument: String },
    #[serde(rename = "minecraft:shift")]
    Shift { argument: String },
    #[serde(rename = "minecraft:clamp")]
    Clamp {
        #[serde(bound = "")]
        input: Box<DensityFunction>,
        min: f64,
        max: f64,
    },
    #[serde(rename = "minecraft:spline")]
    Spline { spline: Spline },
    #[serde(rename = "minecraft:constant")]
    Constant { argument: f64 },
    #[serde(rename = "minecraft:y_clamped_gradient")]
    YClampedGradient {
        from_y: i32,
        to_y: i32,
        from_value: f64,
        to_value: f64,
    },
    #[serde(rename = "minecraft:find_top_surface")]
    FindTopSurface {
        #[serde(bound = "")]
        density: Box<DensityFunction>,
        #[serde(bound = "")]
        upper_bound: Box<DensityFunction>,
        lower_bound: i32,
        cell_height: i32,
    },
}

#[derive(Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Spline {
    Const(f32),
    SplineImpl {
        coordinate: Box<DensityFunction>,
        points: Vec<SplinePoint>,
    },
}

#[derive(Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct SplinePoint {
    pub location: f32,
    #[serde(bound = "")]
    pub value: Box<Spline>,
    pub derivative: f32,
}

#[derive(Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SurfaceRule {
    #[serde(rename = "minecraft:bandlands")]
    BandLands,
    #[serde(rename = "minecraft:block")]
    Block { result_state: BlockState },
    #[serde(rename = "minecraft:condition")]
    Condition {
        if_true: SurfaceCondition,
        #[serde(bound = "")]
        then_run: Box<SurfaceRule>,
    },
    #[serde(rename = "minecraft:sequence")]
    Sequence {
        #[serde(bound = "")]
        sequence: Vec<SurfaceRule>,
    },
}

#[derive(Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SurfaceCondition {
    #[serde(rename = "minecraft:above_preliminary_surface")]
    AbovePreliminarySurface,
    #[serde(rename = "minecraft:biome")]
    Biome { biome_is: Vec<String> },
    #[serde(rename = "minecraft:hole")]
    Hole,
    #[serde(rename = "minecraft:noise_threshold")]
    NoiseThreshold {
        max_threshold: f64,
        min_threshold: f64,
        noise: String,
    },
    #[serde(rename = "minecraft:not")]
    Not {
        #[serde(bound = "")]
        invert: Box<SurfaceCondition>,
    },
    #[serde(rename = "minecraft:steep")]
    Steep,
    #[serde(rename = "minecraft:stone_depth")]
    StoneDepth {
        surface_type: SurfaceType,
        offset: i32,
        add_surface_depth: bool,
        secondary_depth_range: i32,
    },
    #[serde(rename = "minecraft:temperature")]
    Temperature,
    #[serde(rename = "minecraft:vertical_gradient")]
    VerticalGradient {
        random_name: String,
        true_at_and_below: VerticalAnchor,
        false_at_and_above: VerticalAnchor,
    },
    #[serde(rename = "minecraft:water")]
    Water {
        offset: i32,
        surface_depth_multiplier: i32,
        add_stone_depth: bool,
    },
    #[serde(rename = "minecraft:y_above")]
    YAbove {
        anchor: VerticalAnchor,
        surface_depth_multiplier: i32,
        add_stone_depth: bool,
    },
}

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum SurfaceType {
    #[serde(rename = "floor")]
    Floor,
    #[serde(rename = "ceiling")]
    Ceiling,
}

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(untagged)]
pub enum VerticalAnchor {
    Absolute { absolute: i32 },
    AboveBottom { above_bottom: i32 },
    BelowTop { below_top: i32 },
}

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct BlockState {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Properties")]
    pub properties: Option<BTreeMap<String, String>>,
}
