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
  callbacks: CallbacksSet<GetCertResponse>
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
  callbacks: CallbacksSet<CreateCertResponse>
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
  callbacks: CallbacksSet<DeleteCertResponse>
) => jsonRequest(API_DELETE_CERT, "DELETE", data, callbacks);


// Forgot certificate
type ForgotCertResponse = {
  email: string
};

export const forgotCert = (
  email: string,
  callbacks: CallbacksSet<ForgotCertResponse>
) => jsonRequest(API_FORGOT_CERT, "POST", { email: email }, callbacks);
