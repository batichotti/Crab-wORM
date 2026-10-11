pub use crab_worm_macros::crab_worm;

pub fn parasitize() {
    println!(
        r#"
        .          . 
         \        /
          \      /
       ___/      \___
      /   \      /   \
     /     \____/     \
    |   .-.      .-.   |
    |  ( o )    ( o )  |
    |   '-'  __  '-'   |
     \      /  \      /
      '.___/____\___.'
          /|    |\
         / |    | \
        /  |____|  \
       /___/    \___\
          /      \
         /________\

       Crab-wORM v0.1.0
    ~ The wORM is awakening ~
"#
    );

    crab_worm_core::parasitize();
}

pub fn help() {
    println!("Crab-wORM provides the crab_worm macro and runtime support for building ORM behavior.");
    println!("Use #[crab_worm] on supported code, then call parasitize() to initialize it.");
}