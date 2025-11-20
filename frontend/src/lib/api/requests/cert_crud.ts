import { emptyRequest, jsonRequest, type CallbacksSet } from "../api";
import { API_CREATE_CERT, API_DELETE_CERT, API_FORGOT_CERT, API_GET_CERT } from "../configs";


/* --------------- *
 * Get certificate *
 * --------------- */

type GetCertResponse = {
  /**
   * The unique identifier of the certificate.
   */
  id: string,
  /**
   * The name of the person specified in the certificate.
   */
  name: string,
  /**
   * The additional title of the person specified in the certificate.
   */
  title: string
};

/**
 * Fetches the details of a specific certificate by its ID.
 *
 * @param certId - The ID of the certificate to fetch.
 * @param callbacks - The set of success and error callbacks.
 */
export const getCert = (
  certId: string,
  callbacks: CallbacksSet<GetCertResponse, [
  "FATAL_ERROR",
  "BAD_REQUEST",
  "INTERNAL_SERVER_ERROR",
  "RESOURCE_NOT_FOUND"
]>
) => emptyRequest(API_GET_CERT(certId), "GET", callbacks);


/* ------------------ *
 * Create certificate *
 * ------------------ */

type CreateCertRequest = {
  /**
   * The user's email, used for verification.
   */
  email: string,
  /**
   * The additional title of the person to be specified in the certificate.
   */
  title: string,
  /**
   * The name of the person to be specified in the certificate.
   */
  name: string,
  /**
   * The verification code received via email.
   */
  code: string,
  /**
   * The temporary token received after the code request.
   */
  token: string
};

type CreateCertResponse = {
  /**
   * The unique identifier of the newly created certificate.
   */
  id: string,
  /**
   * The name of the person specified in the certificate.
   */
  name: string,
  /**
   * The additional title of the person to be specified in the certificate.
   */
  title: string
};

/**
 * Creates a new certificate using the provided user data and verification code.
 *
 * @param data - The request body containing user data, code, and token.
 * @param callbacks - The set of success and error callbacks.
 */
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


/* ------------------ *
 * Delete certificate *
 * ------------------ */

type DeleteCertRequest = {
  /**
   * The email address associated with the certificate.
   */
  email: string,
  /**
   * The verification code received via email.
   */
  code: string,
  /**
   * The temporary token received after the code request.
   */
  token: string
};

type DeleteCertResponse = {
  /**
   * The ID of the certificate that was successfully deleted.
   */
  id: string
};

/**
 * Deletes a certificate using the provided verification data.
 *
 * @param data - The request body containing the email, code, and token.
 * @param callbacks - The set of success and error callbacks.
 */
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


/* ------------------ *
 * Forgot certificate *
 * ------------------ */

type ForgotCertResponse = {
  /**
   * The email address where the certificate ID was sent.
   */
  email: string
};

/**
 * Sends the ID of certificate to the specified email.
 *
 * @param email - The email address to send the certificate ID to.
 * @param callbacks - The set of success and error callbacks.
 */
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
