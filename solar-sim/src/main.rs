use solar_sim::run;

fn main() {
    pollster::block_on(run());
}
