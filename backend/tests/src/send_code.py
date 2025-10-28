from .configs import BASE_URL

import pytest
import requests

def test_send_code_empty():
    """
    Check /api/v1/send_code when we don't pass body
    """

    res = requests.post(BASE_URL + "/api/v1/send_code")
    assert res.status_code == 400 # Bad Request


def test_send_code_invalid_json():
    """
    Check /api/v1/send_code when we pass invalid json format
    """

    res = requests.post(BASE_URL + "/api/v1/send_code", json={
        "purpose": "hah",
        "email": "invalidemail"
    })
    assert res.status_code == 400


def test_send_code_valid_json():
    """
    Check /api/v1/send_code when we pass valid json format
    """

    res = requests.post(BASE_URL + "/api/v1/send_code", json={
        "purpose": "create",
        "email": "dadencukillia@gmail.com"
    })
    print(res.json())
    assert res.status_code == 200
