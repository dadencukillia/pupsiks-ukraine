from .configs import BASE_URL

import pytest
import requests
import json

def test_send_code_empty():
    """
    Check /api/v1/send_code when we don't pass body
    """

    res = requests.get(BASE_URL + "/api/v1/send_code")
    assert res.status_code == 404 # Bad Request


def test_send_code_invalid_json():
    """
    Check /api/v1/send_code when we pass invalid json format
    """

    res = requests.get(BASE_URL + "/api/v1/send_code", data=json.dumps({
        "purpose": "hah",
        "email": "invalidemail"
    }))
    assert res.status_code == 400
