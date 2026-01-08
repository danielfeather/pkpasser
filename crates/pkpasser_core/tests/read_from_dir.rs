use pkpasser_core::apple::Manifest;

#[test]
fn manifest_from_folder() -> Result<(), Box<dyn std::error::Error>> {
    let dir = std::env::current_dir()?;

    let _manifest = Manifest::from_dir(&dir)?;

    Ok(())
}
