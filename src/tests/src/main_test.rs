fn main() {
    let generator = ferrumc_world_gen::WorldGenerator::new(42069);
    let count = std::env::args().nth(1).unwrap().parse::<i32>().unwrap();
    //let mut chunk = ferrumc_world::chunk_format::Chunk::new(0, 0, "overworld".into());
    for x in i32::MIN..i32::MIN + count {
        std::hint::black_box(generator.generate_chunk(x, x).unwrap());
    }
}
