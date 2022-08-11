// Copyright (c) 2022 MASSA LABS <info@massa.net>

use massa_models::{constants::MAX_SERIALIZED_OPERATIONS_SIZE_PER_BLOCK, THREAD_COUNT};
use massa_time::MassaTime;
use serde::Deserialize;

/// Protocol Configuration, read from toml user config file
#[derive(Debug, Deserialize, Clone, Copy)]
pub struct ProtocolSettings {
    /// after `ask_block_timeout` milliseconds we try to ask a block to another node
    pub ask_block_timeout: MassaTime,
    /// max known blocks of current nodes we keep in memory (by node)
    pub max_known_blocks_size: usize,
    /// max known blocks of foreign nodes we keep in memory (by node)
    pub max_node_known_blocks_size: usize,
    /// max wanted blocks per node kept in memory
    pub max_node_wanted_blocks_size: usize,
    /// max known operations current node kept in memory
    pub max_known_ops_size: usize,
    /// max known operations of foreign nodes we keep in memory (by node)
    pub max_node_known_ops_size: usize,
    /// max known endorsements by our node that we kept in memory
    pub max_known_endorsements_size: usize,
    /// max known endorsements of foreign nodes we keep in memory (by node)
    pub max_node_known_endorsements_size: usize,
    /// we ask for the same block `max_simultaneous_ask_blocks_per_node` times at the same time
    pub max_simultaneous_ask_blocks_per_node: usize,
    /// Max wait time for sending a Network or Node event.
    pub max_send_wait: MassaTime,
    /// Maximum number of batches in the memory buffer.
    /// Dismiss the new batches if overflow
    pub operation_batch_buffer_capacity: usize,
    /// Start processing batches in the buffer each `operation_batch_proc_period` in millisecond
    pub operation_batch_proc_period: MassaTime,
    /// All operations asked are prune each `operation_asked_pruning_period` millisecond
    pub asked_operations_pruning_period: MassaTime,
    /// Maximum of operations sent in one message.
    pub max_operations_per_message: u64,
}

/// Dinamic protocol configuration mixin static settings and constants configurations.
#[derive(Debug, Deserialize, Clone, Copy)]
pub struct ProtocolConfigs {
    /// running threads count
    pub thread_count: u8,
    /// after `ask_block_timeout` milliseconds we try to ask a block to another node
    pub ask_block_timeout: MassaTime,
    /// max known blocks of current nodes we keep in memory (by node)
    pub max_known_blocks_size: usize,
    /// max known blocks of foreign nodes we keep in memory (by node)
    pub max_node_known_blocks_size: usize,
    /// max wanted blocks per node kept in memory
    pub max_node_wanted_blocks_size: usize,
    /// max known operations current node kept in memory
    pub max_known_ops_size: usize,
    /// max known operations of foreign nodes we keep in memory (by node)
    pub max_node_known_ops_size: usize,
    /// max known endorsements by our node that we kept in memory
    pub max_known_endorsements_size: usize,
    /// max known endorsements of foreign nodes we keep in memory (by node)
    pub max_node_known_endorsements_size: usize,
    /// we ask for the same block `max_simultaneous_ask_blocks_per_node` times at the same time
    pub max_simultaneous_ask_blocks_per_node: usize,
    /// Max wait time for sending a Network or Node event.
    pub max_send_wait: MassaTime,
    /// Maximum number of batches in the memory buffer.
    /// Dismiss the new batches if overflow
    pub operation_batch_buffer_capacity: usize,
    /// Start processing batches in the buffer each `operation_batch_proc_period` in millisecond
    pub operation_batch_proc_period: MassaTime,
    /// All operations asked are prune each `operation_asked_pruning_period` millisecond
    pub asked_operations_pruning_period: MassaTime,
    /// Maximum of operations sent in one message.
    pub max_operations_per_message: u64,
    /// Maximum size in bytes of all serialized operations size in a block
    pub max_serialized_operations_size_per_block: usize,
}

impl From<ProtocolSettings> for ProtocolConfigs {
    fn from(settings: ProtocolSettings) -> Self {
        #[cfg(feature = "sandbox")]
        let thread_count = *THREAD_COUNT;
        #[cfg(not(feature = "sandbox"))]
        let thread_count = THREAD_COUNT;

        Self {
            thread_count,
            ask_block_timeout: settings.ask_block_timeout,
            max_known_blocks_size: settings.max_known_blocks_size,
            max_node_known_blocks_size: settings.max_node_known_blocks_size,
            max_node_wanted_blocks_size: settings.max_node_wanted_blocks_size,
            max_known_ops_size: settings.max_known_ops_size,
            max_node_known_ops_size: settings.max_node_known_ops_size,
            max_known_endorsements_size: settings.max_known_endorsements_size,
            max_node_known_endorsements_size: settings.max_node_known_endorsements_size,
            max_simultaneous_ask_blocks_per_node: settings.max_simultaneous_ask_blocks_per_node,
            max_send_wait: settings.max_send_wait,
            operation_batch_buffer_capacity: settings.operation_batch_buffer_capacity,
            operation_batch_proc_period: settings.operation_batch_proc_period,
            asked_operations_pruning_period: settings.asked_operations_pruning_period,
            max_operations_per_message: settings.max_operations_per_message,
            max_serialized_operations_size_per_block: MAX_SERIALIZED_OPERATIONS_SIZE_PER_BLOCK,
        }
    }
}
