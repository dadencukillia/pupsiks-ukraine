import { emptyRequest, jsonRequest, type CallbacksSet } from "../api";
import { API_CREATE_CERT, API_DELETE_CERT, API_FORGOT_CERT, API_GET_CERT } from "../configs";


// Get certificate
type GetCertResponse = {
  id: string,
  name: string,
  title: string
};

export const getCert = (
  certId: string,
  callbacks: CallbacksSet<GetCertResponse, [
  "FATAL_ERROR",
  "BAD_REQUEST",
  "INTERNAL_SERVER_ERROR",
  "RESOURCE_NOT_FOUND"
]>
) => emptyRequest(API_GET_CERT(certId), "GET", callbacks);


// Create certificate
type CreateCertRequest = {
    email: string,
    title: string,
    name: string,
    code: string,
    token: string
};

type CreateCertResponse = {
  id: string,
  name: string,
  title: string
};

export const createCert = (
  data: CreateCertRequest,
  callbacks: CallbacksSet<CreateCertResponse, [
  "FATAL_ERROR",
  "BAD_REQUEST",
  "INVALID_ROUTE",
  "INTERNAL_SERVER_ERROR",
  "ALREADY_EXISTS",
  "INVALID_TOKEN",
  "RESOURCE_NOT_FOUND",
  "INVALID_CODE",
  "TRIES_OUT"
]>
) => jsonRequest(API_CREATE_CERT, "POST", data, callbacks);


// Delete certificate
type DeleteCertRequest = {
  email: string,
  code: string,
  token: string
};

type DeleteCertResponse = {
  id: string
};

export const deleteCert = (
  data: DeleteCertRequest,
  callbacks: CallbacksSet<DeleteCertResponse, [
  "FATAL_ERROR",
  "BAD_REQUEST",
  "INVALID_ROUTE",
  "INTERNAL_SERVER_ERROR",
  "RESOURCE_NOT_FOUND",
  "INVALID_TOKEN",
  "INVALID_CODE",
  "TRIES_OUT"
]>
) => jsonRequest(API_DELETE_CERT, "DELETE", data, callbacks);


// Forgot certificate
type ForgotCertResponse = {
  email: string
};

export const forgotCert = (
  email: string,
  callbacks: CallbacksSet<ForgotCertResponse, [
  "FATAL_ERROR",
  "INTERNAL_SERVER_ERROR",
  "IP_RATE_LIMIT",
  "EMAIL_RATE_LIMIT",
  "RESOURCE_NOT_FOUND",
  "BAD_REQUEST"
]>
) => jsonRequest(API_FORGOT_CERT, "POST", { email: email }, callbacks);
