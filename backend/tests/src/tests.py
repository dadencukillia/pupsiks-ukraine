import pytest
import requests
import uuid
import time

from .configs import BASE_URL, TEST_EMAIL, VALID_CODE

states = {}
ratelimit_requests_per_second = 3


def sleep():
    """
    To be sure that rate limit will not affect on the test
    """

    time.sleep(1/ratelimit_requests_per_second)

# =====
# TESTS
# =====


def test_healthcheck():
    """
    Check GET /healthcheck
    """

    sleep()
    res = requests.get(BASE_URL + "/healthcheck")
    assert res.status_code == 200


def test_stats_certs_count_empty():
    """
    Check GET /api/v1/stats/users_count when there is no certificates in data base
    """

    sleep()
    res = requests.get(BASE_URL + "/api/v1/stats/users_count")
    assert res.status_code == 200
    assert res.json()["count"] == 0


def test_send_code_empty():
    """
    Check POST /api/v1/send_code when we don't pass body
    """

    sleep()
    res = requests.post(BASE_URL + "/api/v1/send_code")
    assert res.status_code == 400 # Bad request


def test_send_code_invalid_json():
    """
    Check POST /api/v1/send_code when we pass invalid json format
    """

    sleep()
    res = requests.post(BASE_URL + "/api/v1/send_code", json={
        "purpose": {
            "type": "hah"
        },
        "email": "invalidemail"
    })
    assert res.status_code == 400


def test_send_code_creation():
    """
    Check POST /api/v1/send_code
    """

    sleep()
    res = requests.post(BASE_URL + "/api/v1/send_code", json={
        "purpose": {
            "type": "create"
        },
        "email": TEST_EMAIL
    })
    assert res.status_code == 200
    states["token"] = res.json()["token"]


def test_create_cert_invalid_code():
    """
    Check POST /api/v1/cert when we try invalid code
    """

    sleep()
    res = requests.post(BASE_URL + "/api/v1/cert", json={
        "token": states["token"],
        "code": "QWE000RTY",
        "email": TEST_EMAIL,
        "title": "The King",
        "name": "Peter"
    })
    assert res.status_code == 400 # Bad request


def test_create_cert_invalid_token():
    """
    Check POST /api/v1/cert when we try invalid token
    """

    sleep()
    res = requests.post(BASE_URL + "/api/v1/cert", json={
        "token": "adadkaodkaodkaokdoakdokaod",
        "code": VALID_CODE,
        "email": TEST_EMAIL,
        "title": "The King",
        "name": "Peter"
    })
    assert res.status_code == 400 # Bad request


def test_create_cert_invalid_title():
    """
    Check POST /api/v1/cert when we try invalid title (< 5 characters)
    """

    sleep()
    res = requests.post(BASE_URL + "/api/v1/cert", json={
        "token": states["token"],
        "code": VALID_CODE,
        "email": TEST_EMAIL,
        "title": "      foo     ",
        "name": "Peter"
    })
    assert res.status_code == 400 # Bad request


def test_create_cert_invalid_name():
    """
    Check POST /api/v1/cert when we try invalid name (< 1 characters)
    """

    sleep()
    res = requests.post(BASE_URL + "/api/v1/cert", json={
        "token": states["token"],
        "code": VALID_CODE,
        "email": TEST_EMAIL,
        "title": "foo",
        "name": "       "
    })
    assert res.status_code == 400 # Bad request


def test_create_cert():
    """
    Check POST /api/v1/cert
    """

    sleep()
    res = requests.post(BASE_URL + "/api/v1/cert", json={
        "token": states["token"],
        "code": VALID_CODE,
        "email": TEST_EMAIL,
        "title": "  The    King",
        "name": "  Peter "
    })

    states["created_id"] = res.json()["id"]
    assert res.status_code == 200


def test_stats_certs_count_after_creation():
    """
    Check GET /api/v1/stats/users_count when there is recently created certificate
    """

    sleep()
    res = requests.get(BASE_URL + "/api/v1/stats/users_count")
    assert res.status_code == 200
    assert res.json()["count"] == 1


def test_get_cert_invalid_uuid():
    """
    Check GET /api/v1/cert/{uuid} when we pass wrong {uuid}
    """

    sleep()
    res = requests.get(BASE_URL + "/api/v1/cert/ababagalamaga")
    assert res.status_code == 400 # Bad request


def test_get_unknown_cert_uuid():
    """
    Check GET /api/v1/cert/{uuid} when we pass valid long {uuid}
    """

    sleep()
    generatedUuid = str(uuid.uuid4())

    res = requests.get(BASE_URL + "/api/v1/cert/" + generatedUuid)
    assert res.status_code == 404


def test_get_cert():
    """
    Check GET /api/v1/cert/{uuid} when we pass valid long {uuid} that belongs recently created certificate
    """

    sleep()
    res = requests.get(BASE_URL + "/api/v1/cert/" + states["created_id"])
    assert res.status_code == 200
    assert res.json()["title"] == "The King"
    assert res.json()["name"] == "Peter"


def test_send_code_creation_already_exist():
    """
    Check POST /api/v1/send_code if a certificate by this email is already exist
    """

    sleep()
    res = requests.post(BASE_URL + "/api/v1/send_code", json={
        "purpose": {
            "type": "create"
        },
        "email": TEST_EMAIL
    })
    assert res.status_code == 409 # Conflict


def test_send_code_deletion_invalid_json():
    """
    Check POST /api/v1/send_code when we pass invalid deletion json format
    Field "id" miss
    """

    sleep()
    res = requests.post(BASE_URL + "/api/v1/send_code", json={
        "purpose": {
            "type": "delete"
        },
        "email": TEST_EMAIL
    })
    assert res.status_code == 400 # Bad request


def test_send_code_deletion_invalid_email():
    """
    Check POST /api/v1/send_code when we pass certificate id that that doesn't belong to email
    """

    sleep()
    res = requests.post(BASE_URL + "/api/v1/send_code", json={
        "purpose": {
            "type": "delete",
            "id": states["created_id"]
        },
        "email": "a" + TEST_EMAIL
    })
    assert res.status_code == 400


def test_send_code_deletion():
    """
    Check POST /api/v1/send_code
    """

    sleep()
    res = requests.post(BASE_URL + "/api/v1/send_code", json={
        "purpose": {
            "type": "delete",
            "id": states["created_id"]
        },
        "email": TEST_EMAIL
    })
    assert res.status_code == 200
    states["token"] = res.json()["token"]


def test_send_code_deletion_ratelimit():
    """
    Check POST /api/v1/send_code when we try to send a code again
    Rate limit will hit
    """

    sleep()
    res = requests.post(BASE_URL + "/api/v1/send_code", json={
        "purpose": {
            "type": "delete",
            "id": states["created_id"]
        },
        "email": TEST_EMAIL
    })
    assert res.status_code == 429 # Too many requests


def test_code_invalid_method():
    """
    Check a case when we use deletion code and token for creation of new certificate
    """

    sleep()
    res = requests.post(BASE_URL + "/api/v1/cert", json={
        "token": states["token"],
        "code": VALID_CODE,
        "email": TEST_EMAIL,
        "title": "The King",
        "name": "Peter "
    })
    assert res.status_code == 409 # Conflict


def test_delete_cert_invalid_code():
    """
    Check DELETE /api/v1/cert when we pass invalid code
    """

    sleep()
    res = requests.delete(BASE_URL + "/api/v1/cert", json={
        "email": TEST_EMAIL,
        "code": "QWE000RTY",
        "token": states["token"]
    })
    assert res.status_code == 400 # Bad requests


def test_delete_cert_invalid_token():
    """
    Check DELETE /api/v1/cert when we pass invalid token
    """

    sleep()
    res = requests.delete(BASE_URL + "/api/v1/cert", json={
        "email": TEST_EMAIL,
        "code": VALID_CODE,
        "token": "odkaodkoakdoakdo"
    })
    assert res.status_code == 400 # Bad request


def test_delete_cert_invalid_email():
    """
    Check DELETE /api/v1/cert when we pass invalid email
    """

    sleep()
    res = requests.delete(BASE_URL + "/api/v1/cert", json={
        "email": "a" + TEST_EMAIL,
        "code": VALID_CODE,
        "token": states["token"]
    })
    assert res.status_code in [400, 404] # Bad request


def test_delete_cert():
    """
    Check DELETE /api/v1/cert
    """

    sleep()
    res = requests.delete(BASE_URL + "/api/v1/cert", json={
        "email": TEST_EMAIL,
        "code": VALID_CODE,
        "token": states["token"]
    })
    assert res.status_code == 200


def test_stats_certs_count_after_deletion():
    """
    Check GET /api/v1/stats/users_count when there is recently deleted certificate
    """

    sleep()
    res = requests.get(BASE_URL + "/api/v1/stats/users_count")
    assert res.status_code == 200
    assert res.json()["count"] == 0


def test_requests_spam():
    """
    Check if rate limiter is working by spaming requests
    """

    ratelimit_counter = 0
    for _ in range(50):
        res1 = requests.get(BASE_URL + "/api/v1/stats/users_count")
        res2 = requests.get(BASE_URL + "/api/v1/cert/aodaokdoakod")
        res3 = requests.get(BASE_URL + "/api/v1/")
        res4 = requests.get(BASE_URL + "/api/v1/stats")

        ratelimit_counter += len([1 for i in [res1, res2, res3, res4] if i.status_code == 429])

    assert ratelimit_counter > 0
    time.sleep(1)


def test_send_code_email_invalid_code_block():
    """
    Check if email sending code will be blocked by some invalid codes
    """

    sleep()
    res = requests.post(BASE_URL + "/api/v1/send_code", json={
        "purpose": {
            "type": "create"
        },
        "email": TEST_EMAIL
    })
    assert res.status_code == 200

    token = res.json()["token"]
    tries_counter = 0
    for _ in range(10):
        sleep()
        res = requests.post(BASE_URL + "/api/v1/cert", json={
            "token": token,
            "code": "QWE000RTY",
            "email": TEST_EMAIL,
            "title": "The King",
            "name": "Peter"
        })
        if res.status_code == 429:
            assert res.json()["code_error"] == "tries_out"
            break

        assert res.status_code == 400
        tries_counter += 1

    assert tries_counter > 0 and tries_counter < 10


def test_blocked_email():
    """
    Check if email sending code is blocked
    """

    sleep()
    res = requests.post(BASE_URL + "/api/v1/send_code", json={
        "purpose": {
            "type": "create"
        },
        "email": TEST_EMAIL
    })
    assert res.status_code == 429
    assert res.json()["code_error"] == "email_rate_limit"


def test_blocked_ip():
    """
    Check if IP being blocked due too frequent code sending
    """

    for email in ["a@gmail.com", "b@example.com", "c@outlook.com", "d@proton.me", "e@gmail.com", "f@gmail.com"]:
        sleep()
        res = requests.post(BASE_URL + "/api/v1/send_code", json={
            "purpose": {
                "type": "create"
            },
            "email": email
        })
        if res.status_code == 429:
            assert res.json()["code_error"] == "ip_rate_limit"
            return

    assert False
