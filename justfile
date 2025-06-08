# Create a Codama IDL
codama-create-idl:
    bun run scripts/createCodamaIDL.ts

# Generate (render) Rust client code
codama-generate-rs:
    bun run scripts/generateCode.ts

# Build Holosim's documentation
doc-holosim:
    cargo doc -p staratlas-holosim --no-deps --open
