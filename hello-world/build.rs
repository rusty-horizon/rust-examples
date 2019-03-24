fn main()
{
    // Write temp files with project info for Makefile to generate proper output files and NACP info!
    // It's not recommended to edit this as Makefile wouldn't work properly.
    std::fs::remove_file("target/pkg.name");
    std::fs::write("target/pkg.name", env!("CARGO_PKG_NAME"));
    std::fs::remove_file("target/pkg.authors");
    std::fs::write("target/pkg.authors", env!("CARGO_PKG_AUTHORS"));
    std::fs::remove_file("target/pkg.version");
    std::fs::write("target/pkg.version", env!("CARGO_PKG_VERSION"));

    // Custom build process
    build();
}

fn build()
{
    // Add your custom code for build here, if you want to
}