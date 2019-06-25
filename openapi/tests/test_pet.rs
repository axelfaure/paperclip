#[macro_use]
extern crate lazy_static;

use paperclip::v2::{
    self,
    codegen::{CrateMeta, DefaultEmitter, Emitter, EmitterState},
    models::{Api, DefaultSchema},
};

use std::fs::{self, File};
use std::io::Read;

lazy_static! {
    static ref ROOT: String = String::from(env!("CARGO_MANIFEST_DIR"));
    static ref SCHEMA: Api<DefaultSchema> = {
        let fd = File::open(ROOT.clone() + "/tests/pet-v2.yaml").expect("file?");
        let raw: Api<DefaultSchema> = v2::from_reader(fd).expect("deserializing spec");
        raw.resolve().expect("resolution")
    };
    static ref CODEGEN: () = {
        let mut state = EmitterState::default();
        state.working_dir = (&*ROOT).into();
        state.working_dir.push("tests");
        state.working_dir.push("test_pet");
        if !state.working_dir.exists() {
            fs::create_dir_all(&state.working_dir).expect("workdir");
        }

        let mut meta = CrateMeta::default();
        meta.authors = Some(vec!["Me <me@example.com>".into()]);
        state.set_meta(meta);

        let emitter = DefaultEmitter::from(state);
        emitter.generate(&SCHEMA).expect("codegen");
    };
}

fn assert_file_contains_content_at(path: &str, matching_content: &str, index: usize) {
    let _ = &*CODEGEN;

    let mut contents = String::new();
    let mut fd = File::open(path).expect("missing file");
    fd.read_to_string(&mut contents).expect("reading file");

    assert_eq!(contents.find(matching_content), Some(index));
}

#[test]
fn test_lib_creation() {
    assert_file_contains_content_at(
        &(ROOT.clone() + "/tests/test_pet/lib.rs"),
        "
#[macro_use]
extern crate failure_derive;
#[macro_use]
extern crate serde_derive;

pub mod category {
    include!(\"./category.rs\");
}

pub mod pet {
    include!(\"./pet.rs\");
}

pub mod tag {
    include!(\"./tag.rs\");
}

pub mod client {
    use futures::{Future, future};
",
        0,
    );
}

#[test]
fn test_manifest_creation() {
    assert_file_contains_content_at(
        &(ROOT.clone() + "/tests/test_pet/Cargo.toml"),
        "[package]
name = \"test_pet\"
version = \"0.1.0\"
authors = [\"Me <me@example.com>\"]
edition = \"2018\"

[lib]
path = \"lib.rs\"

[dependencies]
failure = \"0.1\"
failure_derive = \"0.1\"
futures = \"0.1\"
parking_lot = \"0.8\"
reqwest = \"0.9\"
serde = \"1.0\"
serde_derive = \"1.0\"

[workspace]
",
        0,
    );
}

#[test]
fn test_array_response() {
    // If an operation returns an array of objects, then we bind that
    // operation to that object and do `Sendable<Output<Vec<Object>>>`.
    assert_file_contains_content_at(
        &(ROOT.clone() + "/tests/test_pet/pet.rs"),
        "
/// Builder created by [`Pet::list_pets`](./struct.Pet.html#method.list_pets) method for a `GET` operation associated with `Pet`.
#[derive(Debug, Clone)]
pub struct PetGetBuilder;


impl crate::client::Sendable for PetGetBuilder {
    type Output = Vec<Pet>;

    const METHOD: reqwest::Method = reqwest::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        \"/pets\".into()
    }
}
",
        2715,
    );
}