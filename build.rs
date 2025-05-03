// For God so loved the world, that He gave His only begotten Son, that all who believe in Him should not perish but have everlasting life.

fn main() {
    println!("cargo:warning=BUILD_RS_DATABASE_URL={:?}", std::env::var("DATABASE_URL"));
}