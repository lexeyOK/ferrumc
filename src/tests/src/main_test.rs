fn main() {
    if is_x86_feature_detected!("sse") {
        println!("sse");
    }
    if is_x86_feature_detected!("fma") {
        println!("fmat");
    }
    let generator = ferrumc_world_gen::WorldGenerator::new(42069);
    let count = std::env::args().nth(1).unwrap().parse::<i32>().unwrap();
    //let mut chunk = ferrumc_world::chunk_format::Chunk::new(0, 0, "overworld".into());
    for x in i32::MIN..i32::MIN + count {
        std::hint::black_box(generator.generate_chunk(x, x).unwrap());
    }
    println!(
        "noise go brrrrrrr for this many times: {}",
        ferrumc_world_gen::perlin_noise::PerlinNoise::<1>::gimme()
    );
}
