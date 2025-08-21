fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_prost_build::configure()
        .build_client(false)
        .message_attribute(
            "IdentityCard",
            "#[derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema)]\n#[serde(deny_unknown_fields)]",
        )
        .compile_protos(&["proto/identity.proto"], &[])?;

    Ok(())
}
