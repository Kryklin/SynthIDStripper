pub mod gptzero;
pub mod kirchenbauer;
pub mod synthid;
pub mod synthid_official;

pub use gptzero::{
    GptZeroClassProbabilities, GptZeroClient, GptZeroDocumentResult, GptZeroPredictRequest,
    GptZeroPredictResponse, GptZeroSentenceResult,
};
pub use kirchenbauer::{
    KirchenbauerConfig, KirchenbauerDetectionResult, KirchenbauerDetector, KirchenbauerWatermarker,
};
pub use synthid::{SynthIdConfig, SynthIdDetectionResult, SynthIdDetector, SynthIdWatermarker};
pub use synthid_official::{
    accumulate_hash, DeepMindDetectionResult, DeepMindSynthIdConfig, DeepMindSynthIdDetector,
};
