use solar_sim::run;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    pollster::block_on(run())
}
