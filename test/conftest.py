import pytest
import requests

BASE_URL: str = 'https://raw.githubusercontent.com/Cryptex-github/ril-py/main/test/images/'

@pytest.fixture
def fetch_file():
    def inner(filename: str) -> bytes:
        return requests.get(BASE_URL + filename).content
    
    return inner
