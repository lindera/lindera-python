//! # Lindera Python Bindings
//!
//! Python bindings for [Lindera](https://github.com/lindera/lindera), a morphological analysis library for CJK text.
//!
//! Lindera provides high-performance tokenization and morphological analysis for:
//! - Japanese (IPADIC, IPADIC NEologd, UniDic)
//! - Korean (ko-dic)
//! - Chinese (CC-CEDICT)
//!
//! ## Features
//!
//! - **Dictionary management**: Build, load, and use custom dictionaries
//! - **Tokenization**: Multiple tokenization modes (normal, decompose)
//! - **Filters**: Character and token filtering pipeline
//! - **Training**: Train custom morphological models (with `train` feature)
//! - **User dictionaries**: Support for custom user dictionaries
//!
//! ## Examples
//!
//! ```python
//! import lindera
//!
//! # Create a tokenizer
//! tokenizer = lindera.TokenizerBuilder().build()
//!
//! # Tokenize text
//! tokens = tokenizer.tokenize("関西国際空港")
//! for token in tokens:
//!     print(token["text"], token["detail"])
//! ```

pub mod dictionary;
pub mod error;
pub mod metadata;
pub mod mode;
pub mod schema;
pub mod tokenizer;
#[cfg(feature = "train")]
pub mod trainer;
pub mod util;

use pyo3::prelude::*;

use crate::dictionary::{PyDictionary, PyUserDictionary};
use crate::error::PyLinderaError;
use crate::metadata::{PyCompressionAlgorithm, PyMetadata};
use crate::mode::{PyMode, PyPenalty};
use crate::schema::{PyFieldDefinition, PyFieldType, PySchema};
use crate::tokenizer::{PyTokenizer, PyTokenizerBuilder};

/// Returns the version of the lindera-python package.
///
/// # Returns
///
/// Version string in the format "major.minor.patch"
#[pyfunction]
pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

/// Python module definition for lindera.
///
/// This module exports all classes and functions available to Python code.
#[pymodule]
fn lindera(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_class::<PyDictionary>()?;
    module.add_class::<PyUserDictionary>()?;
    module.add_class::<PyTokenizerBuilder>()?;
    module.add_class::<PyTokenizer>()?;
    module.add_class::<PyLinderaError>()?;
    module.add_class::<PyMode>()?;
    module.add_class::<PyPenalty>()?;
    module.add_class::<PyMetadata>()?;
    module.add_class::<PySchema>()?;
    module.add_class::<PyFieldDefinition>()?;
    module.add_class::<PyFieldType>()?;
    module.add_class::<PyCompressionAlgorithm>()?;

    // Dictionary functions
    module.add_function(wrap_pyfunction!(
        crate::dictionary::build_dictionary,
        module
    )?)?;
    module.add_function(wrap_pyfunction!(
        crate::dictionary::build_user_dictionary,
        module
    )?)?;
    module.add_function(wrap_pyfunction!(
        crate::dictionary::load_dictionary,
        module
    )?)?;
    module.add_function(wrap_pyfunction!(
        crate::dictionary::load_user_dictionary,
        module
    )?)?;

    // Trainer functions
    #[cfg(feature = "train")]
    module.add_function(wrap_pyfunction!(crate::trainer::train, module)?)?;
    #[cfg(feature = "train")]
    module.add_function(wrap_pyfunction!(crate::trainer::export, module)?)?;

    module.add_function(wrap_pyfunction!(version, module)?)?;
    Ok(())
}
