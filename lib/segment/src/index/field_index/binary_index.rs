use std::sync::Arc;

use bitvec::prelude::*;
use bitflags::bitflags;
use parking_lot::RwLock;
use rocksdb::DB;

use super::{CardinalityEstimation, PayloadFieldIndex, PrimaryCondition, ValueIndexer};
use crate::common::rocksdb_wrapper::DatabaseColumnWrapper;
use crate::entry::entry_point::{OperationError, OperationResult};
use crate::telemetry::PayloadIndexTelemetry;
use crate::types::{
    FieldCondition, Match, MatchValue, PayloadKeyType, PointOffsetType, ValueVariants,
};

struct BinaryMemory {
    trues: BitVec,
    falses: BitVec,
}

bitflags! {
    /// Due to being able to store multi-values, the binary index is not a simple
    /// bitset, but rather a pair of bitsets, one for true values and one for false values.
    pub struct BinaryItem: u8 {
        const TRUE = 0b00000001;
        const FALSE = 0b00000010;
    }
}

impl BinaryItem {
    pub fn from_bools(has_true: bool, has_false: bool) -> Self {
        let mut item = Self::empty();
        item.set(Self::TRUE, has_true);
        item.set(Self::FALSE, has_false);
        item
    }
}

impl BinaryMemory {
    pub fn new() -> Self {
        Self {
            trues: BitVec::new(),
            falses: BitVec::new(),
        }
    }

    pub fn get(&self, id: PointOffsetType) -> BinaryItem {
        debug_assert!(self.trues.len() == self.falses.len());
        if (id as usize) >= self.trues.len() {
            return BinaryItem::empty();
        }

        unsafe {
            // SAFETY: we just checked that the id is within bounds
            let has_true = *self.trues.get_unchecked(id as usize).as_ref();
            let has_false = *self.falses.get_unchecked(id as usize).as_ref();
            BinaryItem::from_bools(has_true, has_false)
        }
    }

    pub fn set_or_insert(&mut self, id: PointOffsetType, item: BinaryItem) {
        if item.is_empty() {
            return;
        }

        if (id as usize) >= self.trues.len() {
            self.trues.resize(id as usize + 1, false);
            self.falses.resize(id as usize + 1, false);
        }

        debug_assert!(self.trues.len() == self.falses.len());

        unsafe {
            // SAFETY: we just resized the vectors to be at least as long as the id
            self.trues.set_unchecked(id as usize, item.contains(BinaryItem::TRUE));
            self.falses.set_unchecked(id as usize, item.contains(BinaryItem::FALSE));
        }
    }

    /// Removes the point from the index and tries to shrink the vectors if possible. If the index is not within bounds, does nothing
    pub fn remove(&mut self, id: PointOffsetType) {
        if (id as usize) < self.trues.len() {
            self.trues.set(id as usize, false);
            self.falses.set(id as usize, false);
        }

        // TODO: should we avoid shrinking the vecs on every remove?
        self.shrink();
    }

    /// Shrinks the vectors to the last populated index
    fn shrink(&mut self) {
        let last_populated_index = self.trues.last_one().max(self.falses.last_one());
        match last_populated_index {
            Some(index) if index < self.trues.len() - 1 => {
                self.trues.truncate(index + 1);
                self.falses.truncate(index + 1);
            }
            None => {
                self.trues.clear();
                self.falses.clear();
            }
            _ => {}
        }
    }

    pub fn count_trues(&self) -> usize {
        self.trues.count_ones()
    }

    pub fn count_falses(&self) -> usize {
        self.falses.count_ones()
    }

    pub fn indexed_count(&self) -> usize {
        self.trues.count_ones().max(self.falses.count_ones())
    }

    pub fn iter(&self) -> BinaryMemoryIterator {
        let last_false = self.falses.last_one();
        let last_true = self.trues.last_one();
        let end = last_false.max(last_true).unwrap_or(0) + 1;
        BinaryMemoryIterator {
            memory: self,
            ptr: 0,
            end,
        }
    }
}

struct BinaryMemoryIterator<'a> {
    memory: &'a BinaryMemory,
    ptr: usize,
    end: usize,
}

impl<'a> Iterator for BinaryMemoryIterator<'a> {
    type Item = BinaryItem;

    fn next(&mut self) -> Option<Self::Item> {
        if self.ptr == self.end {
            return None;
        }

        let item = self.memory.get(self.ptr as PointOffsetType);
        self.ptr += 1;

        Some(item)
    }
}

pub struct BinaryIndex {
    memory: BinaryMemory,
    db_wrapper: DatabaseColumnWrapper,
}

impl BinaryIndex {
    pub fn new(db: Arc<RwLock<DB>>, field_name: &str) -> BinaryIndex {
        let store_cf_name = Self::storage_cf_name(field_name);
        let db_wrapper = DatabaseColumnWrapper::new(db, &store_cf_name);
        Self {
            memory: BinaryMemory::new(),
            db_wrapper,
        }
    }

    fn storage_cf_name(field: &str) -> String {
        format!("{}_binary", field)
    }

    pub fn recreate(&self) -> OperationResult<()> {
        self.db_wrapper.recreate_column_family()
    }

    pub fn get_telemetry_data(&self) -> PayloadIndexTelemetry {
        PayloadIndexTelemetry {
            field_name: None,
            points_count: self.memory.indexed_count(),
            points_values_count: self.memory.count_falses() + self.memory.count_falses(),
            histogram_bucket_size: None,
        }
    }

    pub fn values_count(&self, point_id: PointOffsetType) -> usize {
        self.memory.get(point_id).iter().count()
    }

    pub fn values_is_empty(&self, point_id: PointOffsetType) -> bool {
        self.memory.get(point_id).is_empty()
    }
}

impl PayloadFieldIndex for BinaryIndex {
    fn indexed_points(&self) -> usize {
        self.memory.indexed_count()
    }

    fn load(&mut self) -> crate::entry::entry_point::OperationResult<bool> {
        if !self.db_wrapper.has_column_family()? {
            return Ok(false);
        }

        for (key, value) in self.db_wrapper.lock_db().iter()? {
            let idx = PointOffsetType::from_be_bytes(key.as_ref().try_into().unwrap());
            let value = value.as_ref().first().ok_or(OperationError::service_error(
                "Expected a value in binary index",
            ))?;

            let item = BinaryItem::from_bits_truncate(*value);

            self.memory.set_or_insert(idx, item);
        }
        Ok(true)
    }

    fn clear(self) -> crate::entry::entry_point::OperationResult<()> {
        self.db_wrapper.remove_column_family()
    }

    fn flusher(&self) -> crate::common::Flusher {
        self.db_wrapper.flusher()
    }

    fn filter<'a>(
        &'a self,
        condition: &'a crate::types::FieldCondition,
    ) -> Option<Box<dyn Iterator<Item = PointOffsetType> + 'a>> {
        match &condition.r#match {
            Some(Match::Value(MatchValue {
                value: ValueVariants::Bool(value),
            })) => {
                let iter = self
                    .memory
                    .iter()
                    .zip(0u32..) // enumerate but with u32
                    .filter_map(|(stored, point_id)| 
                        match *value {
                            true => stored.contains(BinaryItem::TRUE).then_some(point_id),
                            false => stored.contains(BinaryItem::FALSE).then_some(point_id),
                        }
                    );

                Some(Box::new(iter))
            }
            _ => None,
        }
    }

    fn estimate_cardinality(&self, condition: &FieldCondition) -> Option<CardinalityEstimation> {
        match &condition.r#match {
            Some(Match::Value(MatchValue {
                value: ValueVariants::Bool(value),
            })) => {
                let count = if *value {
                    self.memory.count_trues()
                } else {
                    self.memory.count_falses()
                };

                let estimation = CardinalityEstimation::exact(count)
                    .with_primary_clause(PrimaryCondition::Condition(condition.clone()));

                Some(estimation)
            }
            _ => None,
        }
    }

    fn payload_blocks(
        &self,
        threshold: usize,
        key: PayloadKeyType,
    ) -> Box<dyn Iterator<Item = super::PayloadBlockCondition> + '_> {
        let make_block = |count, value, key: PayloadKeyType| {
            if count > threshold {
                Some(super::PayloadBlockCondition {
                    condition: FieldCondition::new_match(
                        key,
                        Match::Value(MatchValue {
                            value: ValueVariants::Bool(value),
                        }),
                    ),
                    cardinality: count,
                })
            } else {
                None
            }
        };

        // just two possible blocks: true and false
        let iter = [
            make_block(self.memory.count_trues(), true, key.clone()),
            make_block(self.memory.count_falses(), false, key),
        ]
        .into_iter()
        .flatten();

        Box::new(iter)
    }

    fn count_indexed_points(&self) -> usize {
        self.memory.indexed_count()
    }
}

impl ValueIndexer<bool> for BinaryIndex {
    fn add_many(
        &mut self,
        id: PointOffsetType,
        values: Vec<bool>,
    ) -> crate::entry::entry_point::OperationResult<()> {
        if values.is_empty() {
            return Ok(());
        }

        let has_true = values.iter().any(|v| *v);
        let has_false = values.iter().any(|v| !v);

        let item = BinaryItem::from_bools(has_true, has_false);

        self.memory.set_or_insert(id, item);

        let record = BinaryItem::from_bools(has_true, has_false).bits();

        self.db_wrapper.put(id.to_be_bytes(), [record])?;

        Ok(())
    }

    fn get_value(&self, value: &serde_json::Value) -> Option<bool> {
        value.as_bool()
    }

    fn remove_point(
        &mut self,
        id: PointOffsetType,
    ) -> crate::entry::entry_point::OperationResult<()> {
        self.memory.remove(id);
        self.db_wrapper.remove(id.to_be_bytes())?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;
    use rstest::rstest;
    use serde_json::json;
    use tempfile::{Builder, TempDir};

    use super::BinaryIndex;
    use crate::common::rocksdb_wrapper::open_db_with_existing_cf;
    use crate::common::utils::MultiValue;
    use crate::index::field_index::{PayloadFieldIndex, ValueIndexer};

    const FIELD_NAME: &str = "bool_field";
    const DB_NAME: &str = "test_db";

    fn new_binary_index() -> (TempDir, BinaryIndex) {
        let tmp_dir = Builder::new().prefix(DB_NAME).tempdir().unwrap();
        let db = open_db_with_existing_cf(tmp_dir.path()).unwrap();
        let index = BinaryIndex::new(db, FIELD_NAME);
        index.recreate().unwrap();
        (tmp_dir, index)
    }

    fn match_bool(value: bool) -> crate::types::FieldCondition {
        crate::types::FieldCondition::new_match(
            FIELD_NAME.to_string(),
            crate::types::Match::Value(crate::types::MatchValue {
                value: crate::types::ValueVariants::Bool(value),
            }),
        )
    }

    fn filter(given: serde_json::Value, match_on: bool, expected_count: usize) {
        let (_tmp_dir, mut index) = new_binary_index();

        index.add_point(0, &MultiValue::one(&given)).unwrap();

        let count = index.filter(&match_bool(match_on)).unwrap().count();

        assert_eq!(count, expected_count);
    }

    #[rstest]
    #[case(json!(true), 1)]
    #[case(json!(false), 0)]
    #[case(json!([true]), 1)]
    #[case(json!([false]), 0)]
    #[case(json!([true, false]), 1)]
    #[case(json!([false, true]), 1)]
    #[case(json!([false, false]), 0)]
    #[case(json!([true, true]), 1)]
    fn filter_true(#[case] given: serde_json::Value, #[case] expected_count: usize) {
        filter(given, true, expected_count)
    }

    #[rstest]
    #[case(json!(true), 0)]
    #[case(json!(false), 1)]
    #[case(json!([true]), 0)]
    #[case(json!([false]), 1)]
    #[case(json!([true, false]), 1)]
    #[case(json!([false, true]), 1)]
    #[case(json!([false, false]), 1)]
    #[case(json!([true, true]), 0)]
    fn filter_false(#[case] given: serde_json::Value, #[case] expected_count: usize) {
        filter(given, false, expected_count)
    }

    #[test]
    fn load_from_disk() {
        let (_tmp_dir, mut index) = new_binary_index();

        [
            json!(true),
            json!(false),
            json!([true, false]),
            json!([false, true]),
            json!([true, true]),
            json!([false, false]),
            json!([true, false, true]),
            serde_json::Value::Null,
            json!(1),
            json!("test"),
            json!([false]),
            json!([true]),
        ]
        .into_iter()
        .enumerate()
        .for_each(|(i, value)| {
            let payload = MultiValue::one(&value);
            index.add_point(i as u32, &payload).unwrap();
        });

        index.flusher()().unwrap();
        let db = index.db_wrapper.database;

        let mut new_index = BinaryIndex::new(db, FIELD_NAME);
        assert!(new_index.load().unwrap());

        let point_offsets = new_index.filter(&match_bool(false)).unwrap().collect_vec();
        assert_eq!(point_offsets, vec![1, 2, 3, 5, 6, 10]);

        let point_offsets = new_index.filter(&match_bool(true)).unwrap().collect_vec();
        assert_eq!(point_offsets, vec![0, 2, 3, 4, 6, 11]);
    }

    #[rstest]
    #[case(json!(false), json!(true))]
    #[case(json!(false), json!([true, false]))]
    #[case(json!([false, true]), json!([true, false]))]
    #[case(json!([false, true]), json!(true))]
    fn modify_value(#[case] before: serde_json::Value, #[case] after: serde_json::Value) {
        let (_tmp_dir, mut index) = new_binary_index();

        let idx = 1000;
        index.add_point(idx, &MultiValue::one(&before)).unwrap();

        let point_offsets = index.filter(&match_bool(false)).unwrap().collect_vec();
        assert_eq!(point_offsets, vec![idx]);

        index.add_point(idx, &MultiValue::one(&after)).unwrap();

        let point_offsets = index.filter(&match_bool(true)).unwrap().collect_vec();
        assert_eq!(point_offsets, vec![idx]);
    }
}