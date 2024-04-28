fn main() -> Result<(), Box<dyn std::error::Error>> {
    vergen::EmitBuilder::builder().all_build().all_git().fail_on_error().emit()?;
    Ok(())
}
