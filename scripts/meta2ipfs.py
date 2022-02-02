"""deploy Metadata on IPFS"""

# TODO write test!!

import requests
import argparse


def upload2ipfs(file_path: str) -> str:
    """Upload to ipfs using local port. 
    IPFS node must be running locally. Run:
    $ ipfs daemon

    Args:
        nft_path (str): path to metadata file
    """
    with open(file_path, "rb") as f:
        nft_binary = f.read()
    ipfs_endpoint = "http://127.0.0.1:5001/"
    api = "api/v0/add"
    nft_name = file_path.split("/")[-1:][0]
    ipfs_hash = requests.post(ipfs_endpoint + api, files={"from": nft_binary}).json()[
        "Hash"
    ]
    nft_uri = f"https://ipfs.io/ipfs/{ipfs_hash}?filename={nft_name}"
    print("IPFS address: ", nft_uri)
    return nft_uri


if __name__=="__main__":
    """Main App.
    Runs upload2ipfs function. 
    """
    parser = argparse.ArgumentParser(description='Get Metadata path.')
    parser.add_argument('meta-path', metavar='meta', type=str, help='an integer for the accumulator')
    args = parser.parse_args()
    meta_path = args['meta-pass']
    url = upload2ipfs(meta_path)
    print(url)
