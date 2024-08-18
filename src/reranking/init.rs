use std::path::{Path, PathBuf};

use ort::{ExecutionProviderDispatch, Session};
use tokenizers::Tokenizer;

use crate::{RerankerModel, TokenizerFiles, DEFAULT_CACHE_DIR};

use super::{DEFAULT_MAX_LENGTH, DEFAULT_RE_RANKER_MODEL};

pub struct TextRerank {
    pub tokenizer: Tokenizer,
    pub(crate) session: Session,
    pub(crate) need_token_type_ids: bool,
}

/// Options for initializing the reranking model
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct RerankInitOptions {
    pub model_name: RerankerModel,
    pub execution_providers: Vec<ExecutionProviderDispatch>,
    pub max_length: usize,
    pub cache_dir: PathBuf,
    pub show_download_progress: bool,
}

impl RerankInitOptions {
    pub fn new(model_name: RerankerModel) -> Self {
        Self {
            model_name,
            ..Default::default()
        }
    }

    pub fn with_max_length(mut self, max_length: usize) -> Self {
        self.max_length = max_length;
        self
    }

    pub fn with_cache_dir(mut self, cache_dir: PathBuf) -> Self {
        self.cache_dir = cache_dir;
        self
    }

    pub fn with_execution_providers(
        mut self,
        execution_providers: Vec<ExecutionProviderDispatch>,
    ) -> Self {
        self.execution_providers = execution_providers;
        self
    }

    pub fn with_show_download_progress(mut self, show_download_progress: bool) -> Self {
        self.show_download_progress = show_download_progress;
        self
    }
}

impl Default for RerankInitOptions {
    fn default() -> Self {
        Self {
            model_name: DEFAULT_RE_RANKER_MODEL,
            execution_providers: Default::default(),
            max_length: DEFAULT_MAX_LENGTH,
            cache_dir: Path::new(DEFAULT_CACHE_DIR).to_path_buf(),
            show_download_progress: true,
        }
    }
}

/// Options for initializing UserDefinedRerankerModel
///
/// Model files are held by the UserDefinedRerankerModel struct
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct RerankInitOptionsUserDefined {
    pub execution_providers: Vec<ExecutionProviderDispatch>,
    pub max_length: usize,
}

impl Default for RerankInitOptionsUserDefined {
    fn default() -> Self {
        Self {
            execution_providers: Default::default(),
            max_length: DEFAULT_MAX_LENGTH,
        }
    }
}

/// Convert RerankInitOptions to RerankInitOptionsUserDefined
///
/// This is useful for when the user wants to use the same options for both the default and user-defined models
impl From<RerankInitOptions> for RerankInitOptionsUserDefined {
    fn from(options: RerankInitOptions) -> Self {
        RerankInitOptionsUserDefined {
            execution_providers: options.execution_providers,
            max_length: options.max_length,
        }
    }
}

/// Struct for "bring your own" reranking models
///
/// The onnx_file and tokenizer_files are expecting the files' bytes
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct UserDefinedRerankingModel {
    pub onnx_file: Vec<u8>,
    pub tokenizer_files: TokenizerFiles,
}

impl UserDefinedRerankingModel {
    pub fn new(onnx_file: Vec<u8>, tokenizer_files: TokenizerFiles) -> Self {
        Self {
            onnx_file,
            tokenizer_files,
        }
    }
}

/// Rerank result.
#[derive(Debug, PartialEq, Clone)]
pub struct RerankResult {
    pub document: Option<String>,
    pub score: f32,
    pub index: usize,
}
