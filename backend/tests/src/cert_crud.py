from .configs import BASE_URL

import pytest
import requests
import uuid

def test_get_cert_invalid_uuid():
    """
    Check /api/v1/cert/{uuid} when we pass wrong {uuid}
    """

    res = requests.get(BASE_URL + "/api/v1/cert/ababagalamaga")
    assert res.status_code == 400 # Bad Request


def test_get_cert_valid_uuid():
    """
    Check /api/v1/cert/{uuid} when we pass valid long {uuid}
    """

    generatedUuid = str(uuid.uuid4())

    res = requests.get(BASE_URL + "/api/v1/cert/" + generatedUuid)
    assert res.status_code in [200, 404]
