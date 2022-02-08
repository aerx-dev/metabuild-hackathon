from brownie import ETH_ADDRESS, accounts


def test_brownie():
    """Test if the brownie module is working.
    """
    acc = accounts[0]
    assert type(acc.address) == str
