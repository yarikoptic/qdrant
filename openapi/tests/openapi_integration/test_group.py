import pytest

from .helpers.helpers import request_with_validation
from .helpers.collection_setup import basic_collection_setup, drop_collection

collection_name = 'test_collection_groups'

def upsert_chunked_docs(collection_name, docs=50, chunks=5):
    points = []
    for doc in range(docs):
        for chunk in range(chunks):
            doc_id = f"doc_{doc}"
            i = doc * chunks + chunk
            p = {"id": i, "vector": [0.5, 0.5, 0.5, 0.5], "payload": {"docId": doc_id}} 
            points.append(p)
            
    response = request_with_validation(
        api='/collections/{collection_name}/points',
        method="PUT",
        path_params={'collection_name': collection_name},
        query_params={'wait': 'true'},
        body={ "points": points }
    )
    
    assert response.ok

def upsert_points_with_array_fields(collection_name, docs=5, chunks=5, id_offset=500000):
    points = []
    for doc in range(docs):
        for chunk in range(chunks):
            doc_ids = [f"valid_{doc}", "unused"]
            i = doc * chunks + chunk + id_offset
            p = {"id": i, "vector": [0.5, 0.5, 0.5, 0.5], "payload": {"compoundId": doc_ids}} 
            points.append(p)
    print(points)
            
    response = request_with_validation(
        api='/collections/{collection_name}/points',
        method="PUT",
        path_params={'collection_name': collection_name},
        query_params={'wait': 'true'},
        body={ "points": points }
    )
    
    assert response.ok

def upsert_with_heterogenous_fields(collection_name):
    pass

@pytest.fixture(autouse=True, scope="module")
def setup():
    basic_collection_setup(collection_name=collection_name)
    upsert_chunked_docs(collection_name=collection_name)
    upsert_points_with_array_fields(collection_name=collection_name)
    upsert_with_heterogenous_fields(collection_name=collection_name)
    yield
    drop_collection(collection_name=collection_name)


def test_search():
    response = request_with_validation(
        api='/collections/{collection_name}/points/search/groups',
        method="POST",
        path_params={'collection_name': collection_name},
        body={
            "vector": [1.0, 0.0, 0.0, 0.0],
            "limit": 10,
            "with_payload": True,
            "group_by": "docId",
            "per_group": 3,
        }   
    )
    assert response.ok
    
    groups = response.json()["result"]["groups"]
    
    assert len(groups) == 10
    for g in groups:
        assert len(g["hits"]) == 3
        for h in g["hits"]:
            assert h["payload"]["docId"] == g["group_id"]["docId"]
            
def test_recommend():
    response = request_with_validation(
        api='/collections/{collection_name}/points/recommend/groups',
        method="POST",
        path_params={'collection_name': collection_name},
        body={
            "positive": [10, 20, 30],
            "negative": [4, 5, 6],
            "limit": 10,
            "with_payload": True,
            "group_by": "docId",
            "per_group": 3,
        }   
    )
    assert response.ok
    
    groups = response.json()["result"]["groups"]
    
    assert len(groups) == 10
    for g in groups:
        assert len(g["hits"]) == 3
        for h in g["hits"]:
            assert h["payload"]["docId"] == g["group_id"]["docId"]
            
def test_with_vectors():
    response = request_with_validation(
        api='/collections/{collection_name}/points/search/groups',
        method="POST",
        path_params={'collection_name': collection_name},
        body={
            "vector": [1.0, 0.0, 0.0, 0.0],
            "limit": 5,
            "with_payload": True,
            "with_vector": True,
            "group_by": "docId",
            "per_group": 3,
        }   
    )
    assert response.ok
    
    groups = response.json()["result"]["groups"]
    
    assert len(groups) == 5
    for g in groups:
        assert len(g["hits"]) == 3
        for h in g["hits"]:
            assert h["payload"]["docId"] == g["group_id"]["docId"]
            assert h["vector"] == [0.5, 0.5, 0.5, 0.5]

def test_array_fields_uses_first_value_only():
    response = request_with_validation(
        api='/collections/{collection_name}/points/search/groups',
        method="POST",
        path_params={'collection_name': collection_name},
        body={
            "vector": [1.0, 0.5, 0.0, 0.0],
            "limit": 5,
            "with_payload": True,
            "group_by": "compoundId",
            "per_group": 3,
        }   
    )
    assert response.ok
    
    groups = response.json()["result"]["groups"]
    
    assert len(groups) == 5
    for g in groups:
        assert len(g["hits"]) == 3
        assert g["group_id"]["compoundId"].starts_with("valid_")
        assert g["group_id"]["compoundId"] != "unused"
        
def test_groups_by_heterogenous_fields():
    assert False