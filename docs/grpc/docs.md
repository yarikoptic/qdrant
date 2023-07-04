# Protocol Documentation
<a name="top"></a>

## Table of Contents

- [collections.proto](#collections-proto)
    - [AliasDescription](#qdrant-AliasDescription)
    - [AliasOperations](#qdrant-AliasOperations)
    - [ChangeAliases](#qdrant-ChangeAliases)
    - [CollectionClusterInfoRequest](#qdrant-CollectionClusterInfoRequest)
    - [CollectionClusterInfoResponse](#qdrant-CollectionClusterInfoResponse)
    - [CollectionConfig](#qdrant-CollectionConfig)
    - [CollectionDescription](#qdrant-CollectionDescription)
    - [CollectionInfo](#qdrant-CollectionInfo)
    - [CollectionInfo.PayloadSchemaEntry](#qdrant-CollectionInfo-PayloadSchemaEntry)
    - [CollectionOperationResponse](#qdrant-CollectionOperationResponse)
    - [CollectionParams](#qdrant-CollectionParams)
    - [CollectionParamsDiff](#qdrant-CollectionParamsDiff)
    - [CreateAlias](#qdrant-CreateAlias)
    - [CreateCollection](#qdrant-CreateCollection)
    - [DeleteAlias](#qdrant-DeleteAlias)
    - [DeleteCollection](#qdrant-DeleteCollection)
    - [GetCollectionInfoRequest](#qdrant-GetCollectionInfoRequest)
    - [GetCollectionInfoResponse](#qdrant-GetCollectionInfoResponse)
    - [HnswConfigDiff](#qdrant-HnswConfigDiff)
    - [ListAliasesRequest](#qdrant-ListAliasesRequest)
    - [ListAliasesResponse](#qdrant-ListAliasesResponse)
    - [ListCollectionAliasesRequest](#qdrant-ListCollectionAliasesRequest)
    - [ListCollectionsRequest](#qdrant-ListCollectionsRequest)
    - [ListCollectionsResponse](#qdrant-ListCollectionsResponse)
    - [LocalShardInfo](#qdrant-LocalShardInfo)
    - [MoveShard](#qdrant-MoveShard)
    - [OptimizerStatus](#qdrant-OptimizerStatus)
    - [OptimizersConfigDiff](#qdrant-OptimizersConfigDiff)
    - [PayloadIndexParams](#qdrant-PayloadIndexParams)
    - [PayloadSchemaInfo](#qdrant-PayloadSchemaInfo)
    - [ProductQuantization](#qdrant-ProductQuantization)
    - [QuantizationConfig](#qdrant-QuantizationConfig)
    - [RemoteShardInfo](#qdrant-RemoteShardInfo)
    - [RenameAlias](#qdrant-RenameAlias)
    - [Replica](#qdrant-Replica)
    - [ScalarQuantization](#qdrant-ScalarQuantization)
    - [ShardTransferInfo](#qdrant-ShardTransferInfo)
    - [TextIndexParams](#qdrant-TextIndexParams)
    - [UpdateCollection](#qdrant-UpdateCollection)
    - [UpdateCollectionClusterSetupRequest](#qdrant-UpdateCollectionClusterSetupRequest)
    - [UpdateCollectionClusterSetupResponse](#qdrant-UpdateCollectionClusterSetupResponse)
    - [VectorParams](#qdrant-VectorParams)
    - [VectorParamsMap](#qdrant-VectorParamsMap)
    - [VectorParamsMap.MapEntry](#qdrant-VectorParamsMap-MapEntry)
    - [VectorsConfig](#qdrant-VectorsConfig)
    - [WalConfigDiff](#qdrant-WalConfigDiff)
  
    - [CollectionStatus](#qdrant-CollectionStatus)
    - [CompressionRatio](#qdrant-CompressionRatio)
    - [Distance](#qdrant-Distance)
    - [PayloadSchemaType](#qdrant-PayloadSchemaType)
    - [QuantizationType](#qdrant-QuantizationType)
    - [ReplicaState](#qdrant-ReplicaState)
    - [TokenizerType](#qdrant-TokenizerType)
  
- [collections_service.proto](#collections_service-proto)
    - [Collections](#qdrant-Collections)
  
- [json_with_int.proto](#json_with_int-proto)
    - [ListValue](#qdrant-ListValue)
    - [Struct](#qdrant-Struct)
    - [Struct.FieldsEntry](#qdrant-Struct-FieldsEntry)
    - [Value](#qdrant-Value)
  
    - [NullValue](#qdrant-NullValue)
  
- [points.proto](#points-proto)
    - [BatchResult](#qdrant-BatchResult)
    - [ClearPayloadPoints](#qdrant-ClearPayloadPoints)
    - [Condition](#qdrant-Condition)
    - [CountPoints](#qdrant-CountPoints)
    - [CountResponse](#qdrant-CountResponse)
    - [CountResult](#qdrant-CountResult)
    - [CreateFieldIndexCollection](#qdrant-CreateFieldIndexCollection)
    - [DeleteFieldIndexCollection](#qdrant-DeleteFieldIndexCollection)
    - [DeletePayloadPoints](#qdrant-DeletePayloadPoints)
    - [DeletePointVectors](#qdrant-DeletePointVectors)
    - [DeletePoints](#qdrant-DeletePoints)
    - [FieldCondition](#qdrant-FieldCondition)
    - [Filter](#qdrant-Filter)
    - [GeoBoundingBox](#qdrant-GeoBoundingBox)
    - [GeoPoint](#qdrant-GeoPoint)
    - [GeoRadius](#qdrant-GeoRadius)
    - [GetPoints](#qdrant-GetPoints)
    - [GetResponse](#qdrant-GetResponse)
    - [GroupId](#qdrant-GroupId)
    - [GroupsResult](#qdrant-GroupsResult)
    - [HasIdCondition](#qdrant-HasIdCondition)
    - [IsEmptyCondition](#qdrant-IsEmptyCondition)
    - [IsNullCondition](#qdrant-IsNullCondition)
    - [LookupLocation](#qdrant-LookupLocation)
    - [Match](#qdrant-Match)
    - [NamedVectors](#qdrant-NamedVectors)
    - [NamedVectors.VectorsEntry](#qdrant-NamedVectors-VectorsEntry)
    - [NestedCondition](#qdrant-NestedCondition)
    - [PayloadExcludeSelector](#qdrant-PayloadExcludeSelector)
    - [PayloadIncludeSelector](#qdrant-PayloadIncludeSelector)
    - [PointGroup](#qdrant-PointGroup)
    - [PointId](#qdrant-PointId)
    - [PointStruct](#qdrant-PointStruct)
    - [PointStruct.PayloadEntry](#qdrant-PointStruct-PayloadEntry)
    - [PointVectors](#qdrant-PointVectors)
    - [PointsIdsList](#qdrant-PointsIdsList)
    - [PointsOperationResponse](#qdrant-PointsOperationResponse)
    - [PointsSelector](#qdrant-PointsSelector)
    - [QuantizationSearchParams](#qdrant-QuantizationSearchParams)
    - [Range](#qdrant-Range)
    - [ReadConsistency](#qdrant-ReadConsistency)
    - [RecommendBatchPoints](#qdrant-RecommendBatchPoints)
    - [RecommendBatchResponse](#qdrant-RecommendBatchResponse)
    - [RecommendGroupsResponse](#qdrant-RecommendGroupsResponse)
    - [RecommendPointGroups](#qdrant-RecommendPointGroups)
    - [RecommendPoints](#qdrant-RecommendPoints)
    - [RecommendResponse](#qdrant-RecommendResponse)
    - [RepeatedIntegers](#qdrant-RepeatedIntegers)
    - [RepeatedStrings](#qdrant-RepeatedStrings)
    - [RetrievedPoint](#qdrant-RetrievedPoint)
    - [RetrievedPoint.PayloadEntry](#qdrant-RetrievedPoint-PayloadEntry)
    - [ScoredPoint](#qdrant-ScoredPoint)
    - [ScoredPoint.PayloadEntry](#qdrant-ScoredPoint-PayloadEntry)
    - [ScrollPoints](#qdrant-ScrollPoints)
    - [ScrollResponse](#qdrant-ScrollResponse)
    - [SearchBatchPoints](#qdrant-SearchBatchPoints)
    - [SearchBatchResponse](#qdrant-SearchBatchResponse)
    - [SearchGroupsResponse](#qdrant-SearchGroupsResponse)
    - [SearchParams](#qdrant-SearchParams)
    - [SearchPointGroups](#qdrant-SearchPointGroups)
    - [SearchPoints](#qdrant-SearchPoints)
    - [SearchResponse](#qdrant-SearchResponse)
    - [SetPayloadPoints](#qdrant-SetPayloadPoints)
    - [SetPayloadPoints.PayloadEntry](#qdrant-SetPayloadPoints-PayloadEntry)
    - [UpdatePointVectors](#qdrant-UpdatePointVectors)
    - [UpdateResult](#qdrant-UpdateResult)
    - [UpsertPoints](#qdrant-UpsertPoints)
    - [ValuesCount](#qdrant-ValuesCount)
    - [Vector](#qdrant-Vector)
    - [Vectors](#qdrant-Vectors)
    - [VectorsSelector](#qdrant-VectorsSelector)
    - [WithLookup](#qdrant-WithLookup)
    - [WithPayloadSelector](#qdrant-WithPayloadSelector)
    - [WithVectorsSelector](#qdrant-WithVectorsSelector)
    - [WriteOrdering](#qdrant-WriteOrdering)
  
    - [FieldType](#qdrant-FieldType)
    - [ReadConsistencyType](#qdrant-ReadConsistencyType)
    - [UpdateStatus](#qdrant-UpdateStatus)
    - [WriteOrderingType](#qdrant-WriteOrderingType)
  
- [points_service.proto](#points_service-proto)
    - [Points](#qdrant-Points)
  
- [qdrant.proto](#qdrant-proto)
    - [HealthCheckReply](#qdrant-HealthCheckReply)
    - [HealthCheckRequest](#qdrant-HealthCheckRequest)
  
    - [Qdrant](#qdrant-Qdrant)
  
- [snapshots_service.proto](#snapshots_service-proto)
    - [CreateFullSnapshotRequest](#qdrant-CreateFullSnapshotRequest)
    - [CreateSnapshotRequest](#qdrant-CreateSnapshotRequest)
    - [CreateSnapshotResponse](#qdrant-CreateSnapshotResponse)
    - [DeleteFullSnapshotRequest](#qdrant-DeleteFullSnapshotRequest)
    - [DeleteSnapshotRequest](#qdrant-DeleteSnapshotRequest)
    - [DeleteSnapshotResponse](#qdrant-DeleteSnapshotResponse)
    - [ListFullSnapshotsRequest](#qdrant-ListFullSnapshotsRequest)
    - [ListSnapshotsRequest](#qdrant-ListSnapshotsRequest)
    - [ListSnapshotsResponse](#qdrant-ListSnapshotsResponse)
    - [SnapshotDescription](#qdrant-SnapshotDescription)
  
    - [Snapshots](#qdrant-Snapshots)
  
- [Scalar Value Types](#scalar-value-types)



<a name="collections-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## collections.proto



<a name="qdrant-AliasDescription"></a>

### AliasDescription



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| alias_name | [string](#string) |  | Name of the alias |
| collection_name | [string](#string) |  | Name of the collection |






<a name="qdrant-AliasOperations"></a>

### AliasOperations



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| create_alias | [CreateAlias](#qdrant-CreateAlias) |  |  |
| rename_alias | [RenameAlias](#qdrant-RenameAlias) |  |  |
| delete_alias | [DeleteAlias](#qdrant-DeleteAlias) |  |  |






<a name="qdrant-ChangeAliases"></a>

### ChangeAliases



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| actions | [AliasOperations](#qdrant-AliasOperations) | repeated | List of actions |
| timeout | [uint64](#uint64) | optional | Wait timeout for operation commit in seconds, if not specified - default value will be supplied |






<a name="qdrant-CollectionClusterInfoRequest"></a>

### CollectionClusterInfoRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| collection_name | [string](#string) |  | Name of the collection |






<a name="qdrant-CollectionClusterInfoResponse"></a>

### CollectionClusterInfoResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| peer_id | [uint64](#uint64) |  | ID of this peer |
| shard_count | [uint64](#uint64) |  | Total number of shards |
| local_shards | [LocalShardInfo](#qdrant-LocalShardInfo) | repeated | Local shards |
| remote_shards | [RemoteShardInfo](#qdrant-RemoteShardInfo) | repeated | Remote shards |
| shard_transfers | [ShardTransferInfo](#qdrant-ShardTransferInfo) | repeated | Shard transfers |






<a name="qdrant-CollectionConfig"></a>

### CollectionConfig



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| params | [CollectionParams](#qdrant-CollectionParams) |  | Collection parameters |
| hnsw_config | [HnswConfigDiff](#qdrant-HnswConfigDiff) |  | Configuration of vector index |
| optimizer_config | [OptimizersConfigDiff](#qdrant-OptimizersConfigDiff) |  | Configuration of the optimizers |
| wal_config | [WalConfigDiff](#qdrant-WalConfigDiff) |  | Configuration of the Write-Ahead-Log |
| quantization_config | [QuantizationConfig](#qdrant-QuantizationConfig) | optional | Configuration of the vector quantization |






<a name="qdrant-CollectionDescription"></a>

### CollectionDescription



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| name | [string](#string) |  | Name of the collection |






<a name="qdrant-CollectionInfo"></a>

### CollectionInfo



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| status | [CollectionStatus](#qdrant-CollectionStatus) |  | operating condition of the collection |
| optimizer_status | [OptimizerStatus](#qdrant-OptimizerStatus) |  | status of collection optimizers |
| vectors_count | [uint64](#uint64) |  | number of vectors in the collection |
| segments_count | [uint64](#uint64) |  | Number of independent segments |
| config | [CollectionConfig](#qdrant-CollectionConfig) |  | Configuration |
| payload_schema | [CollectionInfo.PayloadSchemaEntry](#qdrant-CollectionInfo-PayloadSchemaEntry) | repeated | Collection data types |
| points_count | [uint64](#uint64) |  | number of points in the collection |
| indexed_vectors_count | [uint64](#uint64) | optional | number of indexed vectors in the collection. |






<a name="qdrant-CollectionInfo-PayloadSchemaEntry"></a>

### CollectionInfo.PayloadSchemaEntry



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| key | [string](#string) |  |  |
| value | [PayloadSchemaInfo](#qdrant-PayloadSchemaInfo) |  |  |






<a name="qdrant-CollectionOperationResponse"></a>

### CollectionOperationResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| result | [bool](#bool) |  | if operation made changes |
| time | [double](#double) |  | Time spent to process |






<a name="qdrant-CollectionParams"></a>

### CollectionParams



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| shard_number | [uint32](#uint32) |  | Number of shards in collection |
| on_disk_payload | [bool](#bool) |  | If true - point&#39;s payload will not be stored in memory |
| vectors_config | [VectorsConfig](#qdrant-VectorsConfig) | optional | Configuration for vectors |
| replication_factor | [uint32](#uint32) | optional | Number of replicas of each shard that network tries to maintain |
| write_consistency_factor | [uint32](#uint32) | optional | How many replicas should apply the operation for us to consider it successful |






<a name="qdrant-CollectionParamsDiff"></a>

### CollectionParamsDiff



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| replication_factor | [uint32](#uint32) | optional | Number of replicas of each shard that network tries to maintain |
| write_consistency_factor | [uint32](#uint32) | optional | How many replicas should apply the operation for us to consider it successful |






<a name="qdrant-CreateAlias"></a>

### CreateAlias



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| collection_name | [string](#string) |  | Name of the collection |
| alias_name | [string](#string) |  | New name of the alias |






<a name="qdrant-CreateCollection"></a>

### CreateCollection



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| collection_name | [string](#string) |  | Name of the collection |
| hnsw_config | [HnswConfigDiff](#qdrant-HnswConfigDiff) | optional | Configuration of vector index |
| wal_config | [WalConfigDiff](#qdrant-WalConfigDiff) | optional | Configuration of the Write-Ahead-Log |
| optimizers_config | [OptimizersConfigDiff](#qdrant-OptimizersConfigDiff) | optional | Configuration of the optimizers |
| shard_number | [uint32](#uint32) | optional | Number of shards in the collection, default is 1 for standalone, otherwise equal to the number of nodes. Minimum is 1 |
| on_disk_payload | [bool](#bool) | optional | If true - point&#39;s payload will not be stored in memory |
| timeout | [uint64](#uint64) | optional | Wait timeout for operation commit in seconds, if not specified - default value will be supplied |
| vectors_config | [VectorsConfig](#qdrant-VectorsConfig) | optional | Configuration for vectors |
| replication_factor | [uint32](#uint32) | optional | Number of replicas of each shard that network tries to maintain, default = 1 |
| write_consistency_factor | [uint32](#uint32) | optional | How many replicas should apply the operation for us to consider it successful, default = 1 |
| init_from_collection | [string](#string) | optional | Specify name of the other collection to copy data from |
| quantization_config | [QuantizationConfig](#qdrant-QuantizationConfig) | optional | Quantization configuration of vector |






<a name="qdrant-DeleteAlias"></a>

### DeleteAlias



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| alias_name | [string](#string) |  | Name of the alias |






<a name="qdrant-DeleteCollection"></a>

### DeleteCollection



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| collection_name | [string](#string) |  | Name of the collection |
| timeout | [uint64](#uint64) | optional | Wait timeout for operation commit in seconds, if not specified - default value will be supplied |






<a name="qdrant-GetCollectionInfoRequest"></a>

### GetCollectionInfoRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| collection_name | [string](#string) |  | Name of the collection |






<a name="qdrant-GetCollectionInfoResponse"></a>

### GetCollectionInfoResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| result | [CollectionInfo](#qdrant-CollectionInfo) |  |  |
| time | [double](#double) |  | Time spent to process |






<a name="qdrant-HnswConfigDiff"></a>

### HnswConfigDiff



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| m | [uint64](#uint64) | optional | Number of edges per node in the index graph. Larger the value - more accurate the search, more space required. |
| ef_construct | [uint64](#uint64) | optional | Number of neighbours to consider during the index building. Larger the value - more accurate the search, more time required to build the index. |
| full_scan_threshold | [uint64](#uint64) | optional | Minimal size (in KiloBytes) of vectors for additional payload-based indexing. If the payload chunk is smaller than `full_scan_threshold` additional indexing won&#39;t be used - in this case full-scan search should be preferred by query planner and additional indexing is not required. Note: 1 Kb = 1 vector of size 256 |
| max_indexing_threads | [uint64](#uint64) | optional | Number of parallel threads used for background index building. If 0 - auto selection. |
| on_disk | [bool](#bool) | optional | Store HNSW index on disk. If set to false, the index will be stored in RAM. |
| payload_m | [uint64](#uint64) | optional | Number of additional payload-aware links per node in the index graph. If not set - regular M parameter will be used. |






<a name="qdrant-ListAliasesRequest"></a>

### ListAliasesRequest







<a name="qdrant-ListAliasesResponse"></a>

### ListAliasesResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| aliases | [AliasDescription](#qdrant-AliasDescription) | repeated |  |
| time | [double](#double) |  | Time spent to process |






<a name="qdrant-ListCollectionAliasesRequest"></a>

### ListCollectionAliasesRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| collection_name | [string](#string) |  | Name of the collection |






<a name="qdrant-ListCollectionsRequest"></a>

### ListCollectionsRequest







<a name="qdrant-ListCollectionsResponse"></a>

### ListCollectionsResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| collections | [CollectionDescription](#qdrant-CollectionDescription) | repeated |  |
| time | [double](#double) |  | Time spent to process |






<a name="qdrant-LocalShardInfo"></a>

### LocalShardInfo



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| shard_id | [uint32](#uint32) |  | Local shard id |
| points_count | [uint64](#uint64) |  | Number of points in the shard |
| state | [ReplicaState](#qdrant-ReplicaState) |  | Is replica active |






<a name="qdrant-MoveShard"></a>

### MoveShard



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| shard_id | [uint32](#uint32) |  | Local shard id |
| from_peer_id | [uint64](#uint64) |  |  |
| to_peer_id | [uint64](#uint64) |  |  |






<a name="qdrant-OptimizerStatus"></a>

### OptimizerStatus



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| ok | [bool](#bool) |  |  |
| error | [string](#string) |  |  |






<a name="qdrant-OptimizersConfigDiff"></a>

### OptimizersConfigDiff



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| deleted_threshold | [double](#double) | optional | The minimal fraction of deleted vectors in a segment, required to perform segment optimization |
| vacuum_min_vector_number | [uint64](#uint64) | optional | The minimal number of vectors in a segment, required to perform segment optimization |
| default_segment_number | [uint64](#uint64) | optional | Target amount of segments the optimizer will try to keep. Real amount of segments may vary depending on multiple parameters:

- Amount of stored points. - Current write RPS.

It is recommended to select the default number of segments as a factor of the number of search threads, so that each segment would be handled evenly by one of the threads. |
| max_segment_size | [uint64](#uint64) | optional | Do not create segments larger this size (in kilobytes). Large segments might require disproportionately long indexation times, therefore it makes sense to limit the size of segments.

If indexing speed is more important - make this parameter lower. If search speed is more important - make this parameter higher. Note: 1Kb = 1 vector of size 256 If not set, will be automatically selected considering the number of available CPUs. |
| memmap_threshold | [uint64](#uint64) | optional | Maximum size (in kilobytes) of vectors to store in-memory per segment. Segments larger than this threshold will be stored as read-only memmaped file.

Memmap storage is disabled by default, to enable it, set this threshold to a reasonable value.

To disable memmap storage, set this to `0`.

Note: 1Kb = 1 vector of size 256 |
| indexing_threshold | [uint64](#uint64) | optional | Maximum size (in kilobytes) of vectors allowed for plain index, exceeding this threshold will enable vector indexing

Default value is 20,000, based on &lt;https://github.com/google-research/google-research/blob/master/scann/docs/algorithms.md&gt;.

To disable vector indexing, set to `0`.

Note: 1kB = 1 vector of size 256. |
| flush_interval_sec | [uint64](#uint64) | optional | Interval between forced flushes. |
| max_optimization_threads | [uint64](#uint64) | optional | Max number of threads, which can be used for optimization. If 0 - `NUM_CPU - 1` will be used |






<a name="qdrant-PayloadIndexParams"></a>

### PayloadIndexParams



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| text_index_params | [TextIndexParams](#qdrant-TextIndexParams) |  | Parameters for text index |






<a name="qdrant-PayloadSchemaInfo"></a>

### PayloadSchemaInfo



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| data_type | [PayloadSchemaType](#qdrant-PayloadSchemaType) |  | Field data type |
| params | [PayloadIndexParams](#qdrant-PayloadIndexParams) | optional | Field index parameters |
| points | [uint64](#uint64) | optional | Number of points indexed within this field indexed |






<a name="qdrant-ProductQuantization"></a>

### ProductQuantization



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| compression | [CompressionRatio](#qdrant-CompressionRatio) |  | Compression ratio |
| always_ram | [bool](#bool) | optional | If true - quantized vectors always will be stored in RAM, ignoring the config of main storage |






<a name="qdrant-QuantizationConfig"></a>

### QuantizationConfig



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| scalar | [ScalarQuantization](#qdrant-ScalarQuantization) |  |  |
| product | [ProductQuantization](#qdrant-ProductQuantization) |  |  |






<a name="qdrant-RemoteShardInfo"></a>

### RemoteShardInfo



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| shard_id | [uint32](#uint32) |  | Local shard id |
| peer_id | [uint64](#uint64) |  | Remote peer id |
| state | [ReplicaState](#qdrant-ReplicaState) |  | Is replica active |






<a name="qdrant-RenameAlias"></a>

### RenameAlias



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| old_alias_name | [string](#string) |  | Name of the alias to rename |
| new_alias_name | [string](#string) |  | Name of the alias |






<a name="qdrant-Replica"></a>

### Replica



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| shard_id | [uint32](#uint32) |  |  |
| peer_id | [uint64](#uint64) |  |  |






<a name="qdrant-ScalarQuantization"></a>

### ScalarQuantization



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| type | [QuantizationType](#qdrant-QuantizationType) |  | Type of quantization |
| quantile | [float](#float) | optional | Number of bits to use for quantization |
| always_ram | [bool](#bool) | optional | If true - quantized vectors always will be stored in RAM, ignoring the config of main storage |






<a name="qdrant-ShardTransferInfo"></a>

### ShardTransferInfo



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| shard_id | [uint32](#uint32) |  | Local shard id |
| from | [uint64](#uint64) |  |  |
| to | [uint64](#uint64) |  |  |
| sync | [bool](#bool) |  | If `true` transfer is a synchronization of a replicas; If `false` transfer is a moving of a shard from one peer to another |






<a name="qdrant-TextIndexParams"></a>

### TextIndexParams



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| tokenizer | [TokenizerType](#qdrant-TokenizerType) |  | Tokenizer type |
| lowercase | [bool](#bool) | optional | If true - all tokens will be lowercase |
| min_token_len | [uint64](#uint64) | optional | Minimal token length |
| max_token_len | [uint64](#uint64) | optional | Maximal token length |






<a name="qdrant-UpdateCollection"></a>

### UpdateCollection



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| collection_name | [string](#string) |  | Name of the collection |
| optimizers_config | [OptimizersConfigDiff](#qdrant-OptimizersConfigDiff) | optional | New configuration parameters for the collection |
| timeout | [uint64](#uint64) | optional | Wait timeout for operation commit in seconds, if not specified - default value will be supplied |
| params | [CollectionParamsDiff](#qdrant-CollectionParamsDiff) | optional | New configuration parameters for the collection |
| hnsw_config | [HnswConfigDiff](#qdrant-HnswConfigDiff) | optional | New HNSW parameters for the collection index |






<a name="qdrant-UpdateCollectionClusterSetupRequest"></a>

### UpdateCollectionClusterSetupRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| collection_name | [string](#string) |  | Name of the collection |
| move_shard | [MoveShard](#qdrant-MoveShard) |  |  |
| replicate_shard | [MoveShard](#qdrant-MoveShard) |  |  |
| abort_transfer | [MoveShard](#qdrant-MoveShard) |  |  |
| drop_replica | [Replica](#qdrant-Replica) |  |  |
| timeout | [uint64](#uint64) | optional | Wait timeout for operation commit in seconds, if not specified - default value will be supplied |






<a name="qdrant-UpdateCollectionClusterSetupResponse"></a>

### UpdateCollectionClusterSetupResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| result | [bool](#bool) |  |  |






<a name="qdrant-VectorParams"></a>

### VectorParams



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| size | [uint64](#uint64) |  | Size of the vectors |
| distance | [Distance](#qdrant-Distance) |  | Distance function used for comparing vectors |
| hnsw_config | [HnswConfigDiff](#qdrant-HnswConfigDiff) | optional | Configuration of vector HNSW graph. If omitted - the collection configuration will be used |
| quantization_config | [QuantizationConfig](#qdrant-QuantizationConfig) | optional | Configuration of vector quantization config. If omitted - the collection configuration will be used |
| on_disk | [bool](#bool) | optional | If true - serve vectors from disk. If set to false, the vectors will be loaded in RAM. |






<a name="qdrant-VectorParamsMap"></a>

### VectorParamsMap



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| map | [VectorParamsMap.MapEntry](#qdrant-VectorParamsMap-MapEntry) | repeated |  |






<a name="qdrant-VectorParamsMap-MapEntry"></a>

### VectorParamsMap.MapEntry



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| key | [string](#string) |  |  |
| value | [VectorParams](#qdrant-VectorParams) |  |  |






<a name="qdrant-VectorsConfig"></a>

### VectorsConfig



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| params | [VectorParams](#qdrant-VectorParams) |  |  |
| params_map | [VectorParamsMap](#qdrant-VectorParamsMap) |  |  |






<a name="qdrant-WalConfigDiff"></a>

### WalConfigDiff



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| wal_capacity_mb | [uint64](#uint64) | optional | Size of a single WAL block file |
| wal_segments_ahead | [uint64](#uint64) | optional | Number of segments to create in advance |





 


<a name="qdrant-CollectionStatus"></a>

### CollectionStatus


| Name | Number | Description |
| ---- | ------ | ----------- |
| UnknownCollectionStatus | 0 |  |
| Green | 1 | All segments are ready |
| Yellow | 2 | Optimization in process |
| Red | 3 | Something went wrong |



<a name="qdrant-CompressionRatio"></a>

### CompressionRatio


| Name | Number | Description |
| ---- | ------ | ----------- |
| x4 | 0 |  |
| x8 | 1 |  |
| x16 | 2 |  |
| x32 | 3 |  |
| x64 | 4 |  |



<a name="qdrant-Distance"></a>

### Distance


| Name | Number | Description |
| ---- | ------ | ----------- |
| UnknownDistance | 0 |  |
| Cosine | 1 |  |
| Euclid | 2 |  |
| Dot | 3 |  |



<a name="qdrant-PayloadSchemaType"></a>

### PayloadSchemaType


| Name | Number | Description |
| ---- | ------ | ----------- |
| UnknownType | 0 |  |
| Keyword | 1 |  |
| Integer | 2 |  |
| Float | 3 |  |
| Geo | 4 |  |
| Text | 5 |  |



<a name="qdrant-QuantizationType"></a>

### QuantizationType


| Name | Number | Description |
| ---- | ------ | ----------- |
| UnknownQuantization | 0 |  |
| Int8 | 1 |  |



<a name="qdrant-ReplicaState"></a>

### ReplicaState


| Name | Number | Description |
| ---- | ------ | ----------- |
| Active | 0 | Active and sound |
| Dead | 1 | Failed for some reason |
| Partial | 2 | The shard is partially loaded and is currently receiving data from other shards |
| Initializing | 3 | Collection is being created |
| Listener | 4 | A shard which receives data, but is not used for search; Useful for backup shards |



<a name="qdrant-TokenizerType"></a>

### TokenizerType


| Name | Number | Description |
| ---- | ------ | ----------- |
| Unknown | 0 |  |
| Prefix | 1 |  |
| Whitespace | 2 |  |
| Word | 3 |  |


 

 

 



<a name="collections_service-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## collections_service.proto


 

 

 


<a name="qdrant-Collections"></a>

### Collections


| Method Name | Request Type | Response Type | Description |
| ----------- | ------------ | ------------- | ------------|
| Get | [GetCollectionInfoRequest](#qdrant-GetCollectionInfoRequest) | [GetCollectionInfoResponse](#qdrant-GetCollectionInfoResponse) | Get detailed information about specified existing collection |
| List | [ListCollectionsRequest](#qdrant-ListCollectionsRequest) | [ListCollectionsResponse](#qdrant-ListCollectionsResponse) | Get list name of all existing collections |
| Create | [CreateCollection](#qdrant-CreateCollection) | [CollectionOperationResponse](#qdrant-CollectionOperationResponse) | Create new collection with given parameters |
| Update | [UpdateCollection](#qdrant-UpdateCollection) | [CollectionOperationResponse](#qdrant-CollectionOperationResponse) | Update parameters of the existing collection |
| Delete | [DeleteCollection](#qdrant-DeleteCollection) | [CollectionOperationResponse](#qdrant-CollectionOperationResponse) | Drop collection and all associated data |
| UpdateAliases | [ChangeAliases](#qdrant-ChangeAliases) | [CollectionOperationResponse](#qdrant-CollectionOperationResponse) | Update Aliases of the existing collection |
| ListCollectionAliases | [ListCollectionAliasesRequest](#qdrant-ListCollectionAliasesRequest) | [ListAliasesResponse](#qdrant-ListAliasesResponse) | Get list of all aliases for a collection |
| ListAliases | [ListAliasesRequest](#qdrant-ListAliasesRequest) | [ListAliasesResponse](#qdrant-ListAliasesResponse) | Get list of all aliases for all existing collections |
| CollectionClusterInfo | [CollectionClusterInfoRequest](#qdrant-CollectionClusterInfoRequest) | [CollectionClusterInfoResponse](#qdrant-CollectionClusterInfoResponse) | Get cluster information for a collection |
| UpdateCollectionClusterSetup | [UpdateCollectionClusterSetupRequest](#qdrant-UpdateCollectionClusterSetupRequest) | [UpdateCollectionClusterSetupResponse](#qdrant-UpdateCollectionClusterSetupResponse) | Update cluster setup for a collection |

 



<a name="json_with_int-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## json_with_int.proto



<a name="qdrant-ListValue"></a>

### ListValue
`ListValue` is a wrapper around a repeated field of values.

The JSON representation for `ListValue` is a JSON array.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| values | [Value](#qdrant-Value) | repeated | Repeated field of dynamically typed values. |






<a name="qdrant-Struct"></a>

### Struct
`Struct` represents a structured data value, consisting of fields
which map to dynamically typed values. In some languages, `Struct`
might be supported by a native representation. For example, in
scripting languages like JS a struct is represented as an
object. The details of that representation are described together
with the proto support for the language.

The JSON representation for `Struct` is a JSON object.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| fields | [Struct.FieldsEntry](#qdrant-Struct-FieldsEntry) | repeated | Unordered map of dynamically typed values. |






<a name="qdrant-Struct-FieldsEntry"></a>

### Struct.FieldsEntry



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| key | [string](#string) |  |  |
| value | [Value](#qdrant-Value) |  |  |






<a name="qdrant-Value"></a>

### Value
`Value` represents a dynamically typed value which can be either
null, a number, a string, a boolean, a recursive struct value, or a
list of values. A producer of value is expected to set one of those
variants, absence of any variant indicates an error.

The JSON representation for `Value` is a JSON value.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| null_value | [NullValue](#qdrant-NullValue) |  | Represents a null value. |
| double_value | [double](#double) |  | Represents a double value. |
| integer_value | [int64](#int64) |  | Represents an integer value |
| string_value | [string](#string) |  | Represents a string value. |
| bool_value | [bool](#bool) |  | Represents a boolean value. |
| struct_value | [Struct](#qdrant-Struct) |  | Represents a structured value. |
| list_value | [ListValue](#qdrant-ListValue) |  | Represents a repeated `Value`. |





 


<a name="qdrant-NullValue"></a>

### NullValue
`NullValue` is a singleton enumeration to represent the null value for the
`Value` type union.

 The JSON representation for `NullValue` is JSON `null`.

| Name | Number | Description |
| ---- | ------ | ----------- |
| NULL_VALUE | 0 | Null value. |


 

 

 



<a name="points-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## points.proto



<a name="qdrant-BatchResult"></a>

### BatchResult



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| result | [ScoredPoint](#qdrant-ScoredPoint) | repeated |  |






<a name="qdrant-ClearPayloadPoints"></a>

### ClearPayloadPoints



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| collection_name | [string](#string) |  | name of the collection |
| wait | [bool](#bool) | optional | Wait until the changes have been applied? |
| points | [PointsSelector](#qdrant-PointsSelector) |  | Affected points |
| ordering | [WriteOrdering](#qdrant-WriteOrdering) | optional | Write ordering guarantees |






<a name="qdrant-Condition"></a>

### Condition



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| field | [FieldCondition](#qdrant-FieldCondition) |  |  |
| is_empty | [IsEmptyCondition](#qdrant-IsEmptyCondition) |  |  |
| has_id | [HasIdCondition](#qdrant-HasIdCondition) |  |  |
| filter | [Filter](#qdrant-Filter) |  |  |
| is_null | [IsNullCondition](#qdrant-IsNullCondition) |  |  |
| nested | [NestedCondition](#qdrant-NestedCondition) |  |  |






<a name="qdrant-CountPoints"></a>

### CountPoints



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| collection_name | [string](#string) |  | name of the collection |
| filter | [Filter](#qdrant-Filter) |  | Filter conditions - return only those points that satisfy the specified conditions |
| exact | [bool](#bool) | optional | If `true` - return exact count, if `false` - return approximate count |






<a name="qdrant-CountResponse"></a>

### CountResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| result | [CountResult](#qdrant-CountResult) |  |  |
| time | [double](#double) |  | Time spent to process |






<a name="qdrant-CountResult"></a>

### CountResult



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| count | [uint64](#uint64) |  |  |






<a name="qdrant-CreateFieldIndexCollection"></a>

### CreateFieldIndexCollection



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| collection_name | [string](#string) |  | name of the collection |
| wait | [bool](#bool) | optional | Wait until the changes have been applied? |
| field_name | [string](#string) |  | Field name to index |
| field_type | [FieldType](#qdrant-FieldType) | optional | Field type. |
| field_index_params | [PayloadIndexParams](#qdrant-PayloadIndexParams) | optional | Payload index params. |
| ordering | [WriteOrdering](#qdrant-WriteOrdering) | optional | Write ordering guarantees |






<a name="qdrant-DeleteFieldIndexCollection"></a>

### DeleteFieldIndexCollection



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| collection_name | [string](#string) |  | name of the collection |
| wait | [bool](#bool) | optional | Wait until the changes have been applied? |
| field_name | [string](#string) |  | Field name to delete |
| ordering | [WriteOrdering](#qdrant-WriteOrdering) | optional | Write ordering guarantees |






<a name="qdrant-DeletePayloadPoints"></a>

### DeletePayloadPoints



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| collection_name | [string](#string) |  | name of the collection |
| wait | [bool](#bool) | optional | Wait until the changes have been applied? |
| keys | [string](#string) | repeated | List of keys to delete |
| points_selector | [PointsSelector](#qdrant-PointsSelector) | optional | Affected points |
| ordering | [WriteOrdering](#qdrant-WriteOrdering) | optional | Write ordering guarantees |






<a name="qdrant-DeletePointVectors"></a>

### DeletePointVectors



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| collection_name | [string](#string) |  | name of the collection |
| wait | [bool](#bool) | optional | Wait until the changes have been applied? |
| points_selector | [PointsSelector](#qdrant-PointsSelector) |  | Affected points |
| vectors | [VectorsSelector](#qdrant-VectorsSelector) |  | List of vector names to delete |
| ordering | [WriteOrdering](#qdrant-WriteOrdering) | optional | Write ordering guarantees |






<a name="qdrant-DeletePoints"></a>

### DeletePoints



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| collection_name | [string](#string) |  | name of the collection |
| wait | [bool](#bool) | optional | Wait until the changes have been applied? |
| points | [PointsSelector](#qdrant-PointsSelector) |  | Affected points |
| ordering | [WriteOrdering](#qdrant-WriteOrdering) | optional | Write ordering guarantees |






<a name="qdrant-FieldCondition"></a>

### FieldCondition



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| key | [string](#string) |  |  |
| match | [Match](#qdrant-Match) |  | Check if point has field with a given value |
| range | [Range](#qdrant-Range) |  | Check if points value lies in a given range |
| geo_bounding_box | [GeoBoundingBox](#qdrant-GeoBoundingBox) |  | Check if points geolocation lies in a given area |
| geo_radius | [GeoRadius](#qdrant-GeoRadius) |  | Check if geo point is within a given radius |
| values_count | [ValuesCount](#qdrant-ValuesCount) |  | Check number of values for a specific field |






<a name="qdrant-Filter"></a>

### Filter



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| should | [Condition](#qdrant-Condition) | repeated | At least one of those conditions should match |
| must | [Condition](#qdrant-Condition) | repeated | All conditions must match |
| must_not | [Condition](#qdrant-Condition) | repeated | All conditions must NOT match |






<a name="qdrant-GeoBoundingBox"></a>

### GeoBoundingBox



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| top_left | [GeoPoint](#qdrant-GeoPoint) |  | north-west corner |
| bottom_right | [GeoPoint](#qdrant-GeoPoint) |  | south-east corner |






<a name="qdrant-GeoPoint"></a>

### GeoPoint



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| lon | [double](#double) |  |  |
| lat | [double](#double) |  |  |






<a name="qdrant-GeoRadius"></a>

### GeoRadius



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| center | [GeoPoint](#qdrant-GeoPoint) |  | Center of the circle |
| radius | [float](#float) |  | In meters |






<a name="qdrant-GetPoints"></a>

### GetPoints



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| collection_name | [string](#string) |  | name of the collection |
| ids | [PointId](#qdrant-PointId) | repeated | List of points to retrieve |
| with_payload | [WithPayloadSelector](#qdrant-WithPayloadSelector) |  | Options for specifying which payload to include or not |
| with_vectors | [WithVectorsSelector](#qdrant-WithVectorsSelector) | optional | Options for specifying which vectors to include into response |
| read_consistency | [ReadConsistency](#qdrant-ReadConsistency) | optional | Options for specifying read consistency guarantees |






<a name="qdrant-GetResponse"></a>

### GetResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| result | [RetrievedPoint](#qdrant-RetrievedPoint) | repeated |  |
| time | [double](#double) |  | Time spent to process |






<a name="qdrant-GroupId"></a>

### GroupId



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| unsigned_value | [uint64](#uint64) |  | Represents a double value. |
| integer_value | [int64](#int64) |  | Represents an integer value |
| string_value | [string](#string) |  | Represents a string value. |






<a name="qdrant-GroupsResult"></a>

### GroupsResult



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| groups | [PointGroup](#qdrant-PointGroup) | repeated | Groups |






<a name="qdrant-HasIdCondition"></a>

### HasIdCondition



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| has_id | [PointId](#qdrant-PointId) | repeated |  |






<a name="qdrant-IsEmptyCondition"></a>

### IsEmptyCondition



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| key | [string](#string) |  |  |






<a name="qdrant-IsNullCondition"></a>

### IsNullCondition



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| key | [string](#string) |  |  |






<a name="qdrant-LookupLocation"></a>

### LookupLocation



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| collection_name | [string](#string) |  |  |
| vector_name | [string](#string) | optional | Which vector to use for search, if not specified - use default vector |






<a name="qdrant-Match"></a>

### Match



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| keyword | [string](#string) |  | Match string keyword |
| integer | [int64](#int64) |  | Match integer |
| boolean | [bool](#bool) |  | Match boolean |
| text | [string](#string) |  | Match text |
| keywords | [RepeatedStrings](#qdrant-RepeatedStrings) |  | Match multiple keywords |
| integers | [RepeatedIntegers](#qdrant-RepeatedIntegers) |  | Match multiple integers |
| except_integers | [RepeatedIntegers](#qdrant-RepeatedIntegers) |  | Match any other value except those integers |
| except_keywords | [RepeatedStrings](#qdrant-RepeatedStrings) |  | Match any other value except those keywords |






<a name="qdrant-NamedVectors"></a>

### NamedVectors



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| vectors | [NamedVectors.VectorsEntry](#qdrant-NamedVectors-VectorsEntry) | repeated |  |






<a name="qdrant-NamedVectors-VectorsEntry"></a>

### NamedVectors.VectorsEntry



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| key | [string](#string) |  |  |
| value | [Vector](#qdrant-Vector) |  |  |






<a name="qdrant-NestedCondition"></a>

### NestedCondition



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| key | [string](#string) |  | Path to nested object |
| filter | [Filter](#qdrant-Filter) |  | Filter condition |






<a name="qdrant-PayloadExcludeSelector"></a>

### PayloadExcludeSelector



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| fields | [string](#string) | repeated | List of payload keys to exclude from the result |






<a name="qdrant-PayloadIncludeSelector"></a>

### PayloadIncludeSelector



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| fields | [string](#string) | repeated | List of payload keys to include into result |






<a name="qdrant-PointGroup"></a>

### PointGroup



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [GroupId](#qdrant-GroupId) |  | Group id |
| hits | [ScoredPoint](#qdrant-ScoredPoint) | repeated | Points in the group |
| lookup | [RetrievedPoint](#qdrant-RetrievedPoint) |  | Point(s) from the lookup collection that matches the group id |






<a name="qdrant-PointId"></a>

### PointId



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| num | [uint64](#uint64) |  | Numerical ID of the point |
| uuid | [string](#string) |  | UUID |






<a name="qdrant-PointStruct"></a>

### PointStruct



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [PointId](#qdrant-PointId) |  |  |
| payload | [PointStruct.PayloadEntry](#qdrant-PointStruct-PayloadEntry) | repeated |  |
| vectors | [Vectors](#qdrant-Vectors) | optional |  |






<a name="qdrant-PointStruct-PayloadEntry"></a>

### PointStruct.PayloadEntry



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| key | [string](#string) |  |  |
| value | [Value](#qdrant-Value) |  |  |






<a name="qdrant-PointVectors"></a>

### PointVectors



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [PointId](#qdrant-PointId) |  | ID to update vectors for |
| vectors | [Vectors](#qdrant-Vectors) |  | Named vectors to update, leave others intact |






<a name="qdrant-PointsIdsList"></a>

### PointsIdsList



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| ids | [PointId](#qdrant-PointId) | repeated |  |






<a name="qdrant-PointsOperationResponse"></a>

### PointsOperationResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| result | [UpdateResult](#qdrant-UpdateResult) |  |  |
| time | [double](#double) |  | Time spent to process |






<a name="qdrant-PointsSelector"></a>

### PointsSelector



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| points | [PointsIdsList](#qdrant-PointsIdsList) |  |  |
| filter | [Filter](#qdrant-Filter) |  |  |






<a name="qdrant-QuantizationSearchParams"></a>

### QuantizationSearchParams



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| ignore | [bool](#bool) | optional | If set to true, search will ignore quantized vector data |
| rescore | [bool](#bool) | optional | If true, use original vectors to re-score top-k results. Default is true. |
| oversampling | [double](#double) | optional | Oversampling factor for quantization.

Defines how many extra vectors should be pre-selected using quantized index, and then re-scored using original vectors.

For example, if `oversampling` is 2.4 and `limit` is 100, then 240 vectors will be pre-selected using quantized index, and then top-100 will be returned after re-scoring. |






<a name="qdrant-Range"></a>

### Range



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| lt | [double](#double) | optional |  |
| gt | [double](#double) | optional |  |
| gte | [double](#double) | optional |  |
| lte | [double](#double) | optional |  |






<a name="qdrant-ReadConsistency"></a>

### ReadConsistency



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| type | [ReadConsistencyType](#qdrant-ReadConsistencyType) |  | Common read consistency configurations |
| factor | [uint64](#uint64) |  | Send request to a specified number of nodes, and return points which are present on all of them |






<a name="qdrant-RecommendBatchPoints"></a>

### RecommendBatchPoints



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| collection_name | [string](#string) |  | Name of the collection |
| recommend_points | [RecommendPoints](#qdrant-RecommendPoints) | repeated |  |
| read_consistency | [ReadConsistency](#qdrant-ReadConsistency) | optional | Options for specifying read consistency guarantees |






<a name="qdrant-RecommendBatchResponse"></a>

### RecommendBatchResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| result | [BatchResult](#qdrant-BatchResult) | repeated |  |
| time | [double](#double) |  | Time spent to process |






<a name="qdrant-RecommendGroupsResponse"></a>

### RecommendGroupsResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| result | [GroupsResult](#qdrant-GroupsResult) |  |  |
| time | [double](#double) |  | Time spent to process |






<a name="qdrant-RecommendPointGroups"></a>

### RecommendPointGroups



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| collection_name | [string](#string) |  | Name of the collection |
| positive | [PointId](#qdrant-PointId) | repeated | Look for vectors closest to those |
| negative | [PointId](#qdrant-PointId) | repeated | Try to avoid vectors like this |
| filter | [Filter](#qdrant-Filter) |  | Filter conditions - return only those points that satisfy the specified conditions |
| limit | [uint32](#uint32) |  | Max number of groups in result |
| with_payload | [WithPayloadSelector](#qdrant-WithPayloadSelector) |  | Options for specifying which payload to include or not |
| params | [SearchParams](#qdrant-SearchParams) |  | Search config |
| score_threshold | [float](#float) | optional | If provided - cut off results with worse scores |
| using | [string](#string) | optional | Define which vector to use for recommendation, if not specified - default vector |
| with_vectors | [WithVectorsSelector](#qdrant-WithVectorsSelector) | optional | Options for specifying which vectors to include into response |
| lookup_from | [LookupLocation](#qdrant-LookupLocation) | optional | Name of the collection to use for points lookup, if not specified - use current collection |
| group_by | [string](#string) |  | Payload field to group by, must be a string or number field. If there are multiple values for the field, all of them will be used. One point can be in multiple groups. |
| group_size | [uint32](#uint32) |  | Maximum amount of points to return per group |
| read_consistency | [ReadConsistency](#qdrant-ReadConsistency) | optional | Options for specifying read consistency guarantees |
| with_lookup | [WithLookup](#qdrant-WithLookup) | optional | Options for specifying how to use the group id to lookup points in another collection |






<a name="qdrant-RecommendPoints"></a>

### RecommendPoints



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| collection_name | [string](#string) |  | name of the collection |
| positive | [PointId](#qdrant-PointId) | repeated | Look for vectors closest to those |
| negative | [PointId](#qdrant-PointId) | repeated | Try to avoid vectors like this |
| filter | [Filter](#qdrant-Filter) |  | Filter conditions - return only those points that satisfy the specified conditions |
| limit | [uint64](#uint64) |  | Max number of result |
| with_payload | [WithPayloadSelector](#qdrant-WithPayloadSelector) |  | Options for specifying which payload to include or not |
| params | [SearchParams](#qdrant-SearchParams) |  | Search config |
| score_threshold | [float](#float) | optional | If provided - cut off results with worse scores |
| offset | [uint64](#uint64) | optional | Offset of the result |
| using | [string](#string) | optional | Define which vector to use for recommendation, if not specified - default vector |
| with_vectors | [WithVectorsSelector](#qdrant-WithVectorsSelector) | optional | Options for specifying which vectors to include into response |
| lookup_from | [LookupLocation](#qdrant-LookupLocation) | optional | Name of the collection to use for points lookup, if not specified - use current collection |
| read_consistency | [ReadConsistency](#qdrant-ReadConsistency) | optional | Options for specifying read consistency guarantees |






<a name="qdrant-RecommendResponse"></a>

### RecommendResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| result | [ScoredPoint](#qdrant-ScoredPoint) | repeated |  |
| time | [double](#double) |  | Time spent to process |






<a name="qdrant-RepeatedIntegers"></a>

### RepeatedIntegers



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| integers | [int64](#int64) | repeated |  |






<a name="qdrant-RepeatedStrings"></a>

### RepeatedStrings



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| strings | [string](#string) | repeated |  |






<a name="qdrant-RetrievedPoint"></a>

### RetrievedPoint



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [PointId](#qdrant-PointId) |  |  |
| payload | [RetrievedPoint.PayloadEntry](#qdrant-RetrievedPoint-PayloadEntry) | repeated |  |
| vectors | [Vectors](#qdrant-Vectors) | optional |  |






<a name="qdrant-RetrievedPoint-PayloadEntry"></a>

### RetrievedPoint.PayloadEntry



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| key | [string](#string) |  |  |
| value | [Value](#qdrant-Value) |  |  |






<a name="qdrant-ScoredPoint"></a>

### ScoredPoint



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [PointId](#qdrant-PointId) |  | Point id |
| payload | [ScoredPoint.PayloadEntry](#qdrant-ScoredPoint-PayloadEntry) | repeated | Payload |
| score | [float](#float) |  | Similarity score |
| version | [uint64](#uint64) |  | Last update operation applied to this point |
| vectors | [Vectors](#qdrant-Vectors) | optional | Vectors to search |






<a name="qdrant-ScoredPoint-PayloadEntry"></a>

### ScoredPoint.PayloadEntry



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| key | [string](#string) |  |  |
| value | [Value](#qdrant-Value) |  |  |






<a name="qdrant-ScrollPoints"></a>

### ScrollPoints



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| collection_name | [string](#string) |  |  |
| filter | [Filter](#qdrant-Filter) |  | Filter conditions - return only those points that satisfy the specified conditions |
| offset | [PointId](#qdrant-PointId) | optional | Start with this ID |
| limit | [uint32](#uint32) | optional | Max number of result |
| with_payload | [WithPayloadSelector](#qdrant-WithPayloadSelector) |  | Options for specifying which payload to include or not |
| with_vectors | [WithVectorsSelector](#qdrant-WithVectorsSelector) | optional | Options for specifying which vectors to include into response |
| read_consistency | [ReadConsistency](#qdrant-ReadConsistency) | optional | Options for specifying read consistency guarantees |






<a name="qdrant-ScrollResponse"></a>

### ScrollResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| next_page_offset | [PointId](#qdrant-PointId) | optional | Use this offset for the next query |
| result | [RetrievedPoint](#qdrant-RetrievedPoint) | repeated |  |
| time | [double](#double) |  | Time spent to process |






<a name="qdrant-SearchBatchPoints"></a>

### SearchBatchPoints



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| collection_name | [string](#string) |  | Name of the collection |
| search_points | [SearchPoints](#qdrant-SearchPoints) | repeated |  |
| read_consistency | [ReadConsistency](#qdrant-ReadConsistency) | optional | Options for specifying read consistency guarantees |






<a name="qdrant-SearchBatchResponse"></a>

### SearchBatchResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| result | [BatchResult](#qdrant-BatchResult) | repeated |  |
| time | [double](#double) |  | Time spent to process |






<a name="qdrant-SearchGroupsResponse"></a>

### SearchGroupsResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| result | [GroupsResult](#qdrant-GroupsResult) |  |  |
| time | [double](#double) |  | Time spent to process |






<a name="qdrant-SearchParams"></a>

### SearchParams



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| hnsw_ef | [uint64](#uint64) | optional | Params relevant to HNSW index. Size of the beam in a beam-search. Larger the value - more accurate the result, more time required for search. |
| exact | [bool](#bool) | optional | Search without approximation. If set to true, search may run long but with exact results. |
| quantization | [QuantizationSearchParams](#qdrant-QuantizationSearchParams) | optional | If set to true, search will ignore quantized vector data |






<a name="qdrant-SearchPointGroups"></a>

### SearchPointGroups



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| collection_name | [string](#string) |  | Name of the collection |
| vector | [float](#float) | repeated | Vector to compare against |
| filter | [Filter](#qdrant-Filter) |  | Filter conditions - return only those points that satisfy the specified conditions |
| limit | [uint32](#uint32) |  | Max number of result |
| with_payload | [WithPayloadSelector](#qdrant-WithPayloadSelector) |  | Options for specifying which payload to include or not |
| params | [SearchParams](#qdrant-SearchParams) |  | Search config |
| score_threshold | [float](#float) | optional | If provided - cut off results with worse scores |
| vector_name | [string](#string) | optional | Which vector to use for search, if not specified - use default vector |
| with_vectors | [WithVectorsSelector](#qdrant-WithVectorsSelector) | optional | Options for specifying which vectors to include into response |
| group_by | [string](#string) |  | Payload field to group by, must be a string or number field. If there are multiple values for the field, all of them will be used. One point can be in multiple groups. |
| group_size | [uint32](#uint32) |  | Maximum amount of points to return per group |
| read_consistency | [ReadConsistency](#qdrant-ReadConsistency) | optional | Options for specifying read consistency guarantees |
| with_lookup | [WithLookup](#qdrant-WithLookup) | optional | Options for specifying how to use the group id to lookup points in another collection |






<a name="qdrant-SearchPoints"></a>

### SearchPoints



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| collection_name | [string](#string) |  | name of the collection |
| vector | [float](#float) | repeated | vector |
| filter | [Filter](#qdrant-Filter) |  | Filter conditions - return only those points that satisfy the specified conditions |
| limit | [uint64](#uint64) |  | Max number of result |
| with_payload | [WithPayloadSelector](#qdrant-WithPayloadSelector) |  | Options for specifying which payload to include or not |
| params | [SearchParams](#qdrant-SearchParams) |  | Search config |
| score_threshold | [float](#float) | optional | If provided - cut off results with worse scores |
| offset | [uint64](#uint64) | optional | Offset of the result |
| vector_name | [string](#string) | optional | Which vector to use for search, if not specified - use default vector |
| with_vectors | [WithVectorsSelector](#qdrant-WithVectorsSelector) | optional | Options for specifying which vectors to include into response |
| read_consistency | [ReadConsistency](#qdrant-ReadConsistency) | optional | Options for specifying read consistency guarantees |






<a name="qdrant-SearchResponse"></a>

### SearchResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| result | [ScoredPoint](#qdrant-ScoredPoint) | repeated |  |
| time | [double](#double) |  | Time spent to process |






<a name="qdrant-SetPayloadPoints"></a>

### SetPayloadPoints



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| collection_name | [string](#string) |  | name of the collection |
| wait | [bool](#bool) | optional | Wait until the changes have been applied? |
| payload | [SetPayloadPoints.PayloadEntry](#qdrant-SetPayloadPoints-PayloadEntry) | repeated | New payload values |
| points_selector | [PointsSelector](#qdrant-PointsSelector) | optional | Affected points |
| ordering | [WriteOrdering](#qdrant-WriteOrdering) | optional | Write ordering guarantees |






<a name="qdrant-SetPayloadPoints-PayloadEntry"></a>

### SetPayloadPoints.PayloadEntry



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| key | [string](#string) |  |  |
| value | [Value](#qdrant-Value) |  |  |






<a name="qdrant-UpdatePointVectors"></a>

### UpdatePointVectors



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| collection_name | [string](#string) |  | name of the collection |
| wait | [bool](#bool) | optional | Wait until the changes have been applied? |
| points | [PointVectors](#qdrant-PointVectors) | repeated | List of points and vectors to update |
| ordering | [WriteOrdering](#qdrant-WriteOrdering) | optional | Write ordering guarantees |






<a name="qdrant-UpdateResult"></a>

### UpdateResult



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| operation_id | [uint64](#uint64) |  | Number of operation |
| status | [UpdateStatus](#qdrant-UpdateStatus) |  | Operation status |






<a name="qdrant-UpsertPoints"></a>

### UpsertPoints



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| collection_name | [string](#string) |  | name of the collection |
| wait | [bool](#bool) | optional | Wait until the changes have been applied? |
| points | [PointStruct](#qdrant-PointStruct) | repeated |  |
| ordering | [WriteOrdering](#qdrant-WriteOrdering) | optional | Write ordering guarantees |






<a name="qdrant-ValuesCount"></a>

### ValuesCount



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| lt | [uint64](#uint64) | optional |  |
| gt | [uint64](#uint64) | optional |  |
| gte | [uint64](#uint64) | optional |  |
| lte | [uint64](#uint64) | optional |  |






<a name="qdrant-Vector"></a>

### Vector



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| data | [float](#float) | repeated |  |






<a name="qdrant-Vectors"></a>

### Vectors



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| vector | [Vector](#qdrant-Vector) |  |  |
| vectors | [NamedVectors](#qdrant-NamedVectors) |  |  |






<a name="qdrant-VectorsSelector"></a>

### VectorsSelector



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| names | [string](#string) | repeated | List of vectors to include into result |






<a name="qdrant-WithLookup"></a>

### WithLookup



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| collection | [string](#string) |  | Name of the collection to use for points lookup |
| with_payload | [WithPayloadSelector](#qdrant-WithPayloadSelector) | optional | Options for specifying which payload to include (or not) |
| with_vectors | [WithVectorsSelector](#qdrant-WithVectorsSelector) | optional | Options for specifying which vectors to include (or not) |






<a name="qdrant-WithPayloadSelector"></a>

### WithPayloadSelector



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| enable | [bool](#bool) |  | If `true` - return all payload, if `false` - none |
| include | [PayloadIncludeSelector](#qdrant-PayloadIncludeSelector) |  |  |
| exclude | [PayloadExcludeSelector](#qdrant-PayloadExcludeSelector) |  |  |






<a name="qdrant-WithVectorsSelector"></a>

### WithVectorsSelector



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| enable | [bool](#bool) |  | If `true` - return all vectors, if `false` - none |
| include | [VectorsSelector](#qdrant-VectorsSelector) |  | List of payload keys to include into result |






<a name="qdrant-WriteOrdering"></a>

### WriteOrdering



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| type | [WriteOrderingType](#qdrant-WriteOrderingType) |  | Write ordering guarantees |





 


<a name="qdrant-FieldType"></a>

### FieldType


| Name | Number | Description |
| ---- | ------ | ----------- |
| FieldTypeKeyword | 0 |  |
| FieldTypeInteger | 1 |  |
| FieldTypeFloat | 2 |  |
| FieldTypeGeo | 3 |  |
| FieldTypeText | 4 |  |



<a name="qdrant-ReadConsistencyType"></a>

### ReadConsistencyType


| Name | Number | Description |
| ---- | ------ | ----------- |
| All | 0 | Send request to all nodes and return points which are present on all of them |
| Majority | 1 | Send requests to all nodes and return points which are present on majority of them |
| Quorum | 2 | Send requests to half &#43; 1 nodes, return points which are present on all of them |



<a name="qdrant-UpdateStatus"></a>

### UpdateStatus


| Name | Number | Description |
| ---- | ------ | ----------- |
| UnknownUpdateStatus | 0 |  |
| Acknowledged | 1 | Update is received, but not processed yet |
| Completed | 2 | Update is applied and ready for search |



<a name="qdrant-WriteOrderingType"></a>

### WriteOrderingType


| Name | Number | Description |
| ---- | ------ | ----------- |
| Weak | 0 | Write operations may be reordered, works faster, default |
| Medium | 1 | Write operations go through dynamically selected leader, may be inconsistent for a short period of time in case of leader change |
| Strong | 2 | Write operations go through the permanent leader, consistent, but may be unavailable if leader is down |


 

 

 



<a name="points_service-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## points_service.proto


 

 

 


<a name="qdrant-Points"></a>

### Points


| Method Name | Request Type | Response Type | Description |
| ----------- | ------------ | ------------- | ------------|
| Upsert | [UpsertPoints](#qdrant-UpsertPoints) | [PointsOperationResponse](#qdrant-PointsOperationResponse) | Perform insert &#43; updates on points. If a point with a given ID already exists - it will be overwritten. |
| Delete | [DeletePoints](#qdrant-DeletePoints) | [PointsOperationResponse](#qdrant-PointsOperationResponse) | Delete points |
| Get | [GetPoints](#qdrant-GetPoints) | [GetResponse](#qdrant-GetResponse) | Retrieve points |
| UpdateVectors | [UpdatePointVectors](#qdrant-UpdatePointVectors) | [PointsOperationResponse](#qdrant-PointsOperationResponse) | Update named vectors for point |
| DeleteVectors | [DeletePointVectors](#qdrant-DeletePointVectors) | [PointsOperationResponse](#qdrant-PointsOperationResponse) | Delete named vectors for points |
| SetPayload | [SetPayloadPoints](#qdrant-SetPayloadPoints) | [PointsOperationResponse](#qdrant-PointsOperationResponse) | Set payload for points |
| OverwritePayload | [SetPayloadPoints](#qdrant-SetPayloadPoints) | [PointsOperationResponse](#qdrant-PointsOperationResponse) | Overwrite payload for points |
| DeletePayload | [DeletePayloadPoints](#qdrant-DeletePayloadPoints) | [PointsOperationResponse](#qdrant-PointsOperationResponse) | Delete specified key payload for points |
| ClearPayload | [ClearPayloadPoints](#qdrant-ClearPayloadPoints) | [PointsOperationResponse](#qdrant-PointsOperationResponse) | Remove all payload for specified points |
| CreateFieldIndex | [CreateFieldIndexCollection](#qdrant-CreateFieldIndexCollection) | [PointsOperationResponse](#qdrant-PointsOperationResponse) | Create index for field in collection |
| DeleteFieldIndex | [DeleteFieldIndexCollection](#qdrant-DeleteFieldIndexCollection) | [PointsOperationResponse](#qdrant-PointsOperationResponse) | Delete field index for collection |
| Search | [SearchPoints](#qdrant-SearchPoints) | [SearchResponse](#qdrant-SearchResponse) | Retrieve closest points based on vector similarity and given filtering conditions |
| SearchBatch | [SearchBatchPoints](#qdrant-SearchBatchPoints) | [SearchBatchResponse](#qdrant-SearchBatchResponse) | Retrieve closest points based on vector similarity and given filtering conditions |
| SearchGroups | [SearchPointGroups](#qdrant-SearchPointGroups) | [SearchGroupsResponse](#qdrant-SearchGroupsResponse) | Retrieve closest points based on vector similarity and given filtering conditions, grouped by a given field |
| Scroll | [ScrollPoints](#qdrant-ScrollPoints) | [ScrollResponse](#qdrant-ScrollResponse) | Iterate over all or filtered points points |
| Recommend | [RecommendPoints](#qdrant-RecommendPoints) | [RecommendResponse](#qdrant-RecommendResponse) | Look for the points which are closer to stored positive examples and at the same time further to negative examples. |
| RecommendBatch | [RecommendBatchPoints](#qdrant-RecommendBatchPoints) | [RecommendBatchResponse](#qdrant-RecommendBatchResponse) | Look for the points which are closer to stored positive examples and at the same time further to negative examples. |
| RecommendGroups | [RecommendPointGroups](#qdrant-RecommendPointGroups) | [RecommendGroupsResponse](#qdrant-RecommendGroupsResponse) | Look for the points which are closer to stored positive examples and at the same time further to negative examples, grouped by a given field |
| Count | [CountPoints](#qdrant-CountPoints) | [CountResponse](#qdrant-CountResponse) | Count points in collection with given filtering conditions |

 



<a name="qdrant-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## qdrant.proto



<a name="qdrant-HealthCheckReply"></a>

### HealthCheckReply



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| title | [string](#string) |  |  |
| version | [string](#string) |  |  |






<a name="qdrant-HealthCheckRequest"></a>

### HealthCheckRequest






 

 

 


<a name="qdrant-Qdrant"></a>

### Qdrant


| Method Name | Request Type | Response Type | Description |
| ----------- | ------------ | ------------- | ------------|
| HealthCheck | [HealthCheckRequest](#qdrant-HealthCheckRequest) | [HealthCheckReply](#qdrant-HealthCheckReply) |  |

 



<a name="snapshots_service-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## snapshots_service.proto



<a name="qdrant-CreateFullSnapshotRequest"></a>

### CreateFullSnapshotRequest







<a name="qdrant-CreateSnapshotRequest"></a>

### CreateSnapshotRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| collection_name | [string](#string) |  | Name of the collection |






<a name="qdrant-CreateSnapshotResponse"></a>

### CreateSnapshotResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| snapshot_description | [SnapshotDescription](#qdrant-SnapshotDescription) |  |  |
| time | [double](#double) |  | Time spent to process |






<a name="qdrant-DeleteFullSnapshotRequest"></a>

### DeleteFullSnapshotRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| snapshot_name | [string](#string) |  | Name of the full snapshot |






<a name="qdrant-DeleteSnapshotRequest"></a>

### DeleteSnapshotRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| collection_name | [string](#string) |  | Name of the collection |
| snapshot_name | [string](#string) |  | Name of the collection snapshot |






<a name="qdrant-DeleteSnapshotResponse"></a>

### DeleteSnapshotResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| time | [double](#double) |  | Time spent to process |






<a name="qdrant-ListFullSnapshotsRequest"></a>

### ListFullSnapshotsRequest







<a name="qdrant-ListSnapshotsRequest"></a>

### ListSnapshotsRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| collection_name | [string](#string) |  | Name of the collection |






<a name="qdrant-ListSnapshotsResponse"></a>

### ListSnapshotsResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| snapshot_descriptions | [SnapshotDescription](#qdrant-SnapshotDescription) | repeated |  |
| time | [double](#double) |  | Time spent to process |






<a name="qdrant-SnapshotDescription"></a>

### SnapshotDescription



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| name | [string](#string) |  | Name of the snapshot |
| creation_time | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | Creation time of the snapshot |
| size | [int64](#int64) |  | Size of the snapshot in bytes |





 

 

 


<a name="qdrant-Snapshots"></a>

### Snapshots


| Method Name | Request Type | Response Type | Description |
| ----------- | ------------ | ------------- | ------------|
| Create | [CreateSnapshotRequest](#qdrant-CreateSnapshotRequest) | [CreateSnapshotResponse](#qdrant-CreateSnapshotResponse) | Create collection snapshot |
| List | [ListSnapshotsRequest](#qdrant-ListSnapshotsRequest) | [ListSnapshotsResponse](#qdrant-ListSnapshotsResponse) | List collection snapshots |
| Delete | [DeleteSnapshotRequest](#qdrant-DeleteSnapshotRequest) | [DeleteSnapshotResponse](#qdrant-DeleteSnapshotResponse) | Delete collection snapshots |
| CreateFull | [CreateFullSnapshotRequest](#qdrant-CreateFullSnapshotRequest) | [CreateSnapshotResponse](#qdrant-CreateSnapshotResponse) | Create full storage snapshot |
| ListFull | [ListFullSnapshotsRequest](#qdrant-ListFullSnapshotsRequest) | [ListSnapshotsResponse](#qdrant-ListSnapshotsResponse) | List full storage snapshots |
| DeleteFull | [DeleteFullSnapshotRequest](#qdrant-DeleteFullSnapshotRequest) | [DeleteSnapshotResponse](#qdrant-DeleteSnapshotResponse) | List full storage snapshots |

 



## Scalar Value Types

| .proto Type | Notes | C++ | Java | Python | Go | C# | PHP | Ruby |
| ----------- | ----- | --- | ---- | ------ | -- | -- | --- | ---- |
| <a name="double" /> double |  | double | double | float | float64 | double | float | Float |
| <a name="float" /> float |  | float | float | float | float32 | float | float | Float |
| <a name="int32" /> int32 | Uses variable-length encoding. Inefficient for encoding negative numbers – if your field is likely to have negative values, use sint32 instead. | int32 | int | int | int32 | int | integer | Bignum or Fixnum (as required) |
| <a name="int64" /> int64 | Uses variable-length encoding. Inefficient for encoding negative numbers – if your field is likely to have negative values, use sint64 instead. | int64 | long | int/long | int64 | long | integer/string | Bignum |
| <a name="uint32" /> uint32 | Uses variable-length encoding. | uint32 | int | int/long | uint32 | uint | integer | Bignum or Fixnum (as required) |
| <a name="uint64" /> uint64 | Uses variable-length encoding. | uint64 | long | int/long | uint64 | ulong | integer/string | Bignum or Fixnum (as required) |
| <a name="sint32" /> sint32 | Uses variable-length encoding. Signed int value. These more efficiently encode negative numbers than regular int32s. | int32 | int | int | int32 | int | integer | Bignum or Fixnum (as required) |
| <a name="sint64" /> sint64 | Uses variable-length encoding. Signed int value. These more efficiently encode negative numbers than regular int64s. | int64 | long | int/long | int64 | long | integer/string | Bignum |
| <a name="fixed32" /> fixed32 | Always four bytes. More efficient than uint32 if values are often greater than 2^28. | uint32 | int | int | uint32 | uint | integer | Bignum or Fixnum (as required) |
| <a name="fixed64" /> fixed64 | Always eight bytes. More efficient than uint64 if values are often greater than 2^56. | uint64 | long | int/long | uint64 | ulong | integer/string | Bignum |
| <a name="sfixed32" /> sfixed32 | Always four bytes. | int32 | int | int | int32 | int | integer | Bignum or Fixnum (as required) |
| <a name="sfixed64" /> sfixed64 | Always eight bytes. | int64 | long | int/long | int64 | long | integer/string | Bignum |
| <a name="bool" /> bool |  | bool | boolean | boolean | bool | bool | boolean | TrueClass/FalseClass |
| <a name="string" /> string | A string must always contain UTF-8 encoded or 7-bit ASCII text. | string | String | str/unicode | string | string | string | String (UTF-8) |
| <a name="bytes" /> bytes | May contain any arbitrary sequence of bytes. | string | ByteString | str | []byte | ByteString | string | String (ASCII-8BIT) |

