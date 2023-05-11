use std::future::Future;

use segment::types::{
    AnyVariants, Condition, FieldCondition, Filter, IsNullCondition, Match, PointGroup,
    ScoredPoint, WithPayloadInterface, WithVector,
};
use serde_json::Value;
use tokio::sync::RwLockReadGuard;
use validator::Validate;

use super::aggregator::GroupsAggregator;
use crate::collection::Collection;
use crate::grouping::types::HashablePoint;
use crate::operations::consistency_params::ReadConsistency;
use crate::operations::types::{CollectionResult, RecommendRequest, SearchRequest, SearchGroupsRequest, BaseGroupRequest, RecommendGroupsRequest};
use crate::recommendations::recommend_by;
use crate::shards::shard::ShardId;

const MAX_GET_GROUPS_REQUESTS: usize = 5;
const MAX_GROUP_FILLING_REQUESTS: usize = 5;

#[derive(Clone, Debug)]
pub enum SourceRequest {
    Search(SearchRequest),
    Recommend(RecommendRequest),
}

impl SourceRequest {
    async fn r#do<'a, F, Fut>(
        &self,
        collection: &Collection,
        // only used for recommend
        collection_by_name: F,
        read_consistency: Option<ReadConsistency>,
        shard_selection: Option<ShardId>,
        include_key: String,
        limit: usize,
        per_group: usize,
    ) -> CollectionResult<Vec<ScoredPoint>>
    where
        F: Fn(String) -> Fut,
        Fut: Future<Output = Option<RwLockReadGuard<'a, Collection>>>,
    {
        let only_group_by_key = Some(WithPayloadInterface::Fields(vec![include_key.clone()]));

        let key_not_null = Filter::new_must_not(Condition::IsNull(IsNullCondition::from(
            include_key.clone(),
        )));

        match self {
            SourceRequest::Search(request) => {
                let mut request = request.clone();

                request.limit = limit * per_group;

                request.filter = Some(request.filter.unwrap_or_default().merge(&key_not_null));

                // We're enriching the final results at the end, so we'll keep this minimal
                request.with_payload = only_group_by_key;
                request.with_vector = None;

                collection
                    .search(request, read_consistency, shard_selection)
                    .await
            }
            SourceRequest::Recommend(request) => {
                let mut request = request.clone();

                request.limit = limit * per_group;

                request.filter = Some(request.filter.unwrap_or_default().merge(&key_not_null));

                // We're enriching the final results at the end, so we'll keep this minimal
                request.with_payload = only_group_by_key;
                request.with_vector = None;

                recommend_by(request, collection, collection_by_name, read_consistency).await
            }
        }
    }

    fn merge_filter(&mut self, filter: &Filter) {
        match self {
            SourceRequest::Search(request) => {
                request.filter = Some(request.filter.clone().unwrap_or_default().merge(filter))
            }
            SourceRequest::Recommend(request) => {
                request.filter = Some(request.filter.clone().unwrap_or_default().merge(filter))
            }
        }
    }

    fn with_payload(&self) -> Option<WithPayloadInterface> {
        match self {
            SourceRequest::Search(request) => request.with_payload.clone(),
            SourceRequest::Recommend(request) => request.with_payload.clone(),
        }
    }

    fn with_vector(&self) -> Option<WithVector> {
        match self {
            SourceRequest::Search(request) => request.with_vector.clone(),
            SourceRequest::Recommend(request) => request.with_vector.clone(),
        }
    }
}

#[derive(Clone, Validate)]
pub struct GroupRequest {
    /// Request to use (search or recommend)
    pub request: SourceRequest,

    /// Path to the field to group by
    pub group_by: String,

    /// Limit of points to return per group
    #[validate(range(min = 1))]
    pub per_group: usize,

    /// Limit of groups to return
    #[validate(range(min = 1))]
    pub limit: usize,
}

impl GroupRequest {
    pub fn new(request: SourceRequest, group_by: String, per_group: usize) -> Self {
        let limit = match &request {
            SourceRequest::Search(request) => request.limit,
            SourceRequest::Recommend(request) => request.limit,
        };
        Self {
            request,
            group_by,
            per_group,
            limit,
        }
    }
}

impl From<SearchGroupsRequest> for GroupRequest {
    fn from(request: SearchGroupsRequest) -> Self {
        let SearchGroupsRequest {
            vector,
            filter,
            params,
            with_payload,
            with_vector,
            score_threshold,
            group_request: BaseGroupRequest {
                group_by,
                per_group,
                limit
            }
        } = request;       

        let search = SearchRequest {
            vector,
            filter,
            params,
            limit: 0,
            offset: 0,
            with_payload,
            with_vector,
            score_threshold,
        };

        GroupRequest {
            request: SourceRequest::Search(search),
            group_by,
            per_group: per_group as usize,
            limit: limit as usize,
        }
    }
}

impl From<RecommendGroupsRequest> for GroupRequest {
    fn from(request: RecommendGroupsRequest) -> Self {
        let RecommendGroupsRequest {
            positive,
            negative,
            filter,
            params,
            with_payload,
            with_vector,
            score_threshold,
            using,
            lookup_from,
            group_request: BaseGroupRequest {
                group_by,
                per_group,
                limit
            }
        } = request;

        let recommend = RecommendRequest {
            positive,
            negative,
            filter,
            params,
            limit: 0,
            offset: 0,
            with_payload,
            with_vector,
            score_threshold,
            using,
            lookup_from,
        };

        GroupRequest {
            request: SourceRequest::Recommend(recommend),
            group_by,
            per_group: per_group as usize,
            limit: limit as usize,
        }
    }
}

/// Uses the request to fill up groups of points.
pub async fn group_by<'a, F, Fut>(
    request: GroupRequest,
    collection: &Collection,
    // Obligatory for recommend
    collection_by_name: F,
    read_consistency: Option<ReadConsistency>,
    shard_selection: Option<ShardId>,
) -> CollectionResult<Vec<PointGroup>>
where
    F: Fn(String) -> Fut + Clone,
    Fut: Future<Output = Option<RwLockReadGuard<'a, Collection>>>,
{
    let mut aggregator =
        GroupsAggregator::new(request.limit, request.per_group, request.group_by.clone());

    // Try to complete amount of groups
    for _ in 0..MAX_GET_GROUPS_REQUESTS {
        // TODO: should we break early if we have some amount of "enough" groups?
        if aggregator.len_of_filled_best_groups() >= request.limit {
            break;
        }

        let mut req = request.request.clone();

        // construct filter to exclude already found groups
        let full_groups = aggregator.keys_of_filled_groups();
        if !full_groups.is_empty() {
            if let Some(match_any) = match_on(request.group_by.clone(), full_groups) {
                let exclude_groups = Filter::new_must_not(match_any);
                req.merge_filter(&exclude_groups);
            }
        }

        // exclude already aggregated points
        let ids = aggregator.ids();
        if !ids.is_empty() {
            let exclude_ids = Filter::new_must_not(Condition::HasId(ids.into()));
            req.merge_filter(&exclude_ids);
        }

        let points = req
            .r#do(
                collection,
                collection_by_name.clone(),
                read_consistency,
                shard_selection,
                request.group_by.clone(),
                request.limit,
                request.per_group,
            )
            .await?;

        if points.is_empty() {
            break;
        }

        aggregator.add_points(&points)
    }

    // Try to fill up groups
    for _ in 0..MAX_GROUP_FILLING_REQUESTS {
        if aggregator.len_of_filled_best_groups() >= request.limit {
            break;
        }

        let mut req = request.request.clone();

        // construct filter to only include unsatisfied groups
        let unsatisfied_groups = aggregator.keys_of_unfilled_best_groups();
        if let Some(match_any) = match_on(request.group_by.clone(), unsatisfied_groups) {
            let include_groups = Filter::new_must(match_any);
            req.merge_filter(&include_groups);
        }

        // exclude already aggregated points
        let ids = aggregator.ids();
        if !ids.is_empty() {
            let exclude_ids = Filter::new_must_not(Condition::HasId(ids.into()));
            req.merge_filter(&exclude_ids);
        }

        let points = req
            .r#do(
                collection,
                collection_by_name.clone(),
                read_consistency,
                shard_selection,
                request.group_by.clone(),
                request.limit,
                request.per_group,
            )
            .await?;

        if points.is_empty() {
            break;
        }

        aggregator.add_points(&points);
    }

    let mut groups = aggregator.distill();

    // flatten results
    let bare_points = groups
        .iter()
        .cloned()
        .flat_map(|group| group.hits)
        .map(ScoredPoint::from)
        .collect();

    // enrich with payload and vector
    let enriched_points = collection
        .fill_search_result_with_payload(
            bare_points,
            request.request.with_payload(),
            request.request.with_vector().unwrap_or_default(),
            read_consistency,
            None,
        )
        .await?
        .into_iter()
        .map(HashablePoint::from)
        .collect();

    // hydrate groups with enriched points
    groups
        .iter_mut()
        .for_each(|group| group.hydrate_from(&enriched_points));

    // turn into output form
    let groups = groups.into_iter().map(PointGroup::from).collect();

    Ok(groups)
}

/// Uses the set of values to create a Match::Any, if possible
fn match_on(path: String, values: Vec<Value>) -> Option<Condition> {
    match values.first() {
        Some(Value::Number(_)) => Some(Match::new_any(AnyVariants::Integers(
            values.into_iter().filter_map(|v| v.as_i64()).collect(),
        ))),
        Some(Value::String(_)) => Some(Match::new_any(AnyVariants::Keywords(
            values
                .into_iter()
                .filter_map(|v| v.as_str().map(|s| s.to_owned()))
                .collect(),
        ))),
        _ => None, // also considers the case of empty values
    }
    .map(|m| Condition::Field(FieldCondition::new_match(path, m)))
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use segment::types::{Payload, ScoredPoint};

    use crate::grouping::types::{Group, GroupKey, HashablePoint};

    #[test]
    fn test_hydrated_from() {
        // arrange
        let mut groups: Vec<Group> = Vec::new();
        [
            (
                "a",
                [
                    ScoredPoint {
                        id: 1.into(),
                        version: 0,
                        score: 1.0,
                        payload: None,
                        vector: None,
                    },
                    ScoredPoint {
                        id: 2.into(),
                        version: 0,
                        score: 1.0,
                        payload: None,
                        vector: None,
                    },
                ],
            ),
            (
                "b",
                [
                    ScoredPoint {
                        id: 3.into(),
                        version: 0,
                        score: 1.0,
                        payload: None,
                        vector: None,
                    },
                    ScoredPoint {
                        id: 4.into(),
                        version: 0,
                        score: 1.0,
                        payload: None,
                        vector: None,
                    },
                ],
            ),
        ]
        .into_iter()
        .for_each(|(key, points)| {
            let group = Group {
                key: GroupKey::from(key),
                hits: points.into_iter().map(HashablePoint::from).collect(),
                group_by: "docId".to_string(),
            };
            groups.push(group);
        });

        let payload_a = Payload::from(serde_json::json!({"some_key": "some value a"}));
        let payload_b = Payload::from(serde_json::json!({"some_key": "some value b"}));

        let hydrated = vec![
            ScoredPoint {
                id: 1.into(),
                version: 0,
                score: 1.0,
                payload: Some(payload_a.clone()),
                vector: None,
            },
            ScoredPoint {
                id: 2.into(),
                version: 0,
                score: 1.0,
                payload: Some(payload_a.clone()),
                vector: None,
            },
            ScoredPoint {
                id: 3.into(),
                version: 0,
                score: 1.0,
                payload: Some(payload_b.clone()),
                vector: None,
            },
            ScoredPoint {
                id: 4.into(),
                version: 0,
                score: 1.0,
                payload: Some(payload_b.clone()),
                vector: None,
            },
        ];

        let set: HashSet<_> = hydrated.iter().cloned().map(HashablePoint::from).collect();

        // act
        groups.iter_mut().for_each(|group| group.hydrate_from(&set));

        // assert
        assert_eq!(groups.len(), 2);
        assert_eq!(groups.get(0).unwrap().hits.len(), 2);
        assert_eq!(groups.get(1).unwrap().hits.len(), 2);

        let a = groups.get(0).unwrap();
        let b = groups.get(1).unwrap();

        assert!(a.hits.iter().all(|x| x.payload() == Some(&payload_a)));
        assert!(b.hits.iter().all(|x| x.payload() == Some(&payload_b)));
    }
}