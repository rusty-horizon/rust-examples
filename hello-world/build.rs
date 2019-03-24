fn main()
{
    std::fs::remove_file("target/pkg.name");
    std::fs::remove_file("target/pkg.authors");
    std::fs::remove_file("target/pkg.version");
    std::fs::write("target/pkg.name", env!("CARGO_PKG_NAME"));
    std::fs::write("target/pkg.authors", env!("CARGO_PKG_AUTHORS"));
    std::fs::write("target/pkg.version", env!("CARGO_PKG_VERSION"));
}