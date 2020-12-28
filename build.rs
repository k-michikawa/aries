fn main() -> Result<(), Box<dyn std::error::Error>> {
    // compile protos
    tonic_build::compile_protos("proto/product.proto")?;
    Ok(())
}
