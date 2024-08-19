#[cfg(all(
    not(feature = "unknown_variants"),
    not(feature = "unknown_variants_slim")
))]
mod default_variants;

#[cfg(feature = "unknown_variants")]
mod unknown_variants;

#[cfg(all(not(feature = "unknown_variants"), feature = "unknown_variants_slim"))]
mod unknown_variants_slim;

fn main() {}
