/// Main function for compiling Protocol Buffers (protobuf) definitions.
///
/// This function uses the `tonic_build` crate to compile the specified
/// protobuf file (`proto/template.proto`) into Rust code. It then returns
/// a `Result` indicating whether the compilation was successful or if an error
/// occurred.
///
/// # Errors
///
/// If an error occurs during the compilation process, it will be returned as
/// an `Err` variant of the `Result`, containing an error type that implements
/// the `std::error::Error` trait.
///
/// # Examples
///
/// ```rust
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     # Compile the protobuf definitions
///     tonic_build::compile_protos("proto/template.proto")?;
///     Ok(())
/// }
/// ```
fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/template.proto")?;
    Ok(())
}
