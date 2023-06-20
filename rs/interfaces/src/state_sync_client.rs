use ic_types::{
    artifact::{StateSyncArtifactId, StateSyncMessage},
    chunkable::{ArtifactChunk, ChunkId, Chunkable},
    NodeId,
};

pub trait StateSyncClient: Send + Sync {
    /// Returns the Id of the latest available state or None if no state is available.
    fn latest_state(&self) -> Option<StateSyncArtifactId>;
    /// Initiates new state sync for the specified Id. Returns None if the state should not be synced.
    /// If `Some(..)` is returned a new state sync is initated.
    /// Callers of this interface need to uphold the following: `start_state_sync` is not called again
    /// before the previously returned object is dropped.
    /// TODO: (NET-1469) In the future the mentiond caller restriction should be lifted.
    fn start_state_sync(
        &self,
        id: &StateSyncArtifactId,
    ) -> Option<Box<dyn Chunkable + Send + Sync>>;
    /// Get a specifc chunk from the specified state.
    fn chunk(&self, id: &StateSyncArtifactId, chunk_id: ChunkId) -> Option<ArtifactChunk>;
    /// Finish a state sync by delivering the `StateSyncMessage` returned in `Chunkable::add_chunks`.
    /// TODO: (NET-1469) In the future peer_id should be removed from this interface since it has no relevance.
    fn deliver_state_sync(&self, msg: StateSyncMessage, peer_id: NodeId);
}
