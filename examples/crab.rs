use crab_worm_macros::crab_worm;

#[derive(crab_worm)]
pub struct Crab {
	pub name: String,
	pub species: String,
	pub habitat: String,
	pub length_segments: u32,
	pub weight_grams: u32,
	pub age_years: u8,
	pub claw_span_cm: u16,
	pub color: String,
	pub is_nocturnal: bool,
	pub is_alive: bool,
}

fn main() {
	println!("{:#?}", Crab::CRAB_WORM_FIELDS);
}