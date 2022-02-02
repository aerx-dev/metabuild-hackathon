"""Test the uupload to IPFS"""
from scripts.meta2ipfs import upload2ipfs

import json
import requests

def test_ipfs():
    """Test the upload to IPFS.
    """
    # Assign
    meta_path = "res/meta/metadata_template.json"
    url = upload2ipfs(meta_path)
    # Arrange
    answer = requests.get(url)
    answer_json = json.load(answer)
    print(answer)
    # Assert
    with open(meta_path, 'r') as f:
        meta_json = json.load(f)
        assert meta_json == answer_json
