#![feature(str_internals, char_indices_offset)]

mod parse;
mod span;
mod tokenize;

use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use ariadne::Source;

use crate::parse::Parser;

#[derive(Clone, Debug)]
pub struct SourceFile {
    id: SourceId,
    name: PathBuf,
    text: String,
}

impl SourceFile {
    pub fn name(&self) -> &Path {
        &self.name
    }

    pub fn id(&self) -> SourceId {
        self.id
    }

    pub fn text(&self) -> &str {
        &self.text
    }
}

mod reports {
    use std::ops::Range;

    use ariadne::{Label, Report, ReportKind};

    use crate::{parse::ParseError, SourceTree, tokenize::TokenType};
    use std::fmt;
    
    struct ExpectedAny(Vec<TokenType>);

    impl fmt::Display for ExpectedAny {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            for (i, v) in self.0.iter().enumerate() {
                let last = i == self.0.len() - 1;
                v.fmt(f)?; 
                if !last {
                    write!(f, ", ")?;
                }
            }
            Ok(())
        }
    }

    pub fn create<'a>(
        err: ParseError,
        source_tree: SourceTree,
    ) -> Report<'a, (String, Range<usize>)> {
        match err {
            ParseError::UnexpectedToken {
                source_id,
                expected_any,
                found,
            } => {
                let source_file = source_tree.find(source_id);
                let name = source_file.name().to_string_lossy().into_owned();
                Report::build(ReportKind::Error, name.clone(), 1)
                    .with_code(0)
                    .with_message("Unexpected character")
                    .with_label(
                        Label::new((name, found.span().begin()..found.span().end())).with_message(
                            format!(
                                "Found {0} but expected any of {1}",
                                found.tok_type(),
                                ExpectedAny(expected_any),
                            ),
                        ),
                    )
                    .finish()
            }
        }
    }
}

const SRC: &'static str = r#"
block add(v0: , v1: i32): i32 {
  v3 = iadd<i32> v0, v1
  return v3
}
"#;

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub struct SourceId(u32);

pub struct SourceTree {
    map: HashMap<SourceId, SourceFile>,
}

impl SourceTree {
    pub fn new() -> Self {
        SourceTree {
            map: HashMap::new(),
        }
    }

    pub fn find(&self, id: SourceId) -> &SourceFile {
        &self.map[&id]
    }

    pub fn register(&mut self, file: UnregisteredSourceFile) -> SourceId {
        let id = SourceId(self.map.len() as u32);
        self.map.insert(
            id,
            SourceFile {
                id,
                name: file.name,
                text: file.text,
            },
        );
        id
    }
}

pub struct UnregisteredSourceFile {
    name: PathBuf,
    text: String,
}

fn main() {
    let mut source_tree = SourceTree::new();

    let source_id = source_tree.register(UnregisteredSourceFile {
        name: PathBuf::from("test.air"),
        text: SRC.to_string(),
    });

    let mut parser = Parser::new(source_tree.find(source_id));

    match parser.node_block() {
        Ok(block) => {
            dbg!(block);
        }
        Err(e) => {
            let source_file = source_tree.find(source_id);
            let name = source_file.name().to_string_lossy().into_owned();
            let source = Source::from(source_file.text());
            reports::create(e, source_tree)
                .print((name, source))
                .unwrap();
        }
    }
}
