use crab_worm::crab_worm;

#[derive(crab_worm)]
pub enum Worm {
	LYSARETE_BRASILIENSIS,
	STRONGYLOIDES_STERCORALIS
}

#[derive(crab_worm)]
pub struct Claw {
	pub side: String,
	pub size_cm: u16,
}

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
	pub worms: Vec<Worm>,
	pub nicknames: Vec<String>,
	pub claws: Vec<Claw>,
}

#[derive(crab_worm)]
pub struct Car {
	pub id: u32,
	pub brand: String,
	pub year: u16,
	pub model: String,
	pub owner: String, 
}

#[derive(crab_worm)]
pub struct CPU {
	pub id: u32,
	pub manufacturer: String,
	pub model: String,
	pub cores: u8,
	pub threads: u8,
	pub price: u8,
	pub release_dat: u16, 
}

#[derive(crab_worm)]
pub struct User {
	pub id: u32,
	pub name: String,
	pub email: String,
	pub age: u8,
	pub is_active: bool,
}

fn main() {
	let _ = crab_worm::parasitize();
}