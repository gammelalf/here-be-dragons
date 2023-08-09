use solar_sim::run;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    pollster::block_on(run())
}
