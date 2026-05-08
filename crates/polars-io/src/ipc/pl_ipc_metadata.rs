pub static POLARS_IPC_METADATA_KEY: &str = "__POLARS_IPC_METADATA";

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PlIpcMetadata {
    pub record_batch_num_rows: Vec<u64>,
}
