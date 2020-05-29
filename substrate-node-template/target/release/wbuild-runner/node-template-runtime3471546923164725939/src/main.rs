
				use substrate_wasm_builder::build_project_with_default_rustflags;

				fn main() {
					build_project_with_default_rustflags(
						"/home/sriharikapu/substrate-node-template/target/release/build/node-template-runtime-546937c1c409955e/out/wasm_binary.rs",
						"/home/sriharikapu/substrate-node-template/runtime/Cargo.toml",
						"-Clink-arg=--export=__heap_base -C link-arg=--import-memory ",
					)
				}
			