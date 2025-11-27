import { jsonRequest, type CallbacksSet } from "../api";
import { API_SEND_CODE } from "../configs";

type SendCodeResponse = {
  /**
   * The email address to which the code was sent.
   */
  email: string,
  /**
   * The temporary token required to proceed to the next step.
   */
  token: string,
  /**
   * Timestamp (in seconds) indicating when the token expires.
   */
  expires_at: number
};


/* ------------------------------ *
 * Send code creation certificate *
 * ------------------------------ */ 

/**
 * Sends a verification code to the specified email address
 * to initiate the certificate creation process.
 *
 * @param email - The user's email address to send the code to.
 * @param callbacks - The set of success and error callbacks.
 */
export const sendCodeCertCreation = async (
  email: string,
  callbacks: CallbacksSet<SendCodeResponse, [
  "FATAL_ERROR",
  "BAD_REQUEST",
  "INTERNAL_SERVER_ERROR",
  "RESOURCE_NOT_FOUND",
  "ALREADY_EXISTS",
  "INVALID_EMAIL",
  "IP_RATE_LIMIT",
  "EMAIL_RATE_LIMIT"
]>
) => jsonRequest(API_SEND_CODE, "POST", { 
  purpose: { 
    type: "create"
  },
  email: email
}, callbacks);


/* ------------------------------ *
 * Send code deletion certificate *
 * ------------------------------ */ 

/**
 * Sends a verification code to the specified email address
 * to confirm the deletion of an existing certificate.
 *
 * @param email - The user's email address to send the code to.
 * @param certId - The ID of the certificate to be deleted.
 * @param callbacks - The set of success and error callbacks.
 */
export const sendCodeCertDeletion = async (
  email: string,
  certId: string,
  callbacks: CallbacksSet<SendCodeResponse, [
  "FATAL_ERROR",
  "BAD_REQUEST",
  "INTERNAL_SERVER_ERROR",
  "INVALID_EMAIL",
  "IP_RATE_LIMIT",
  "EMAIL_RATE_LIMIT"
]>
) => jsonRequest(API_SEND_CODE, "POST", { 
  purpose: { 
    type: "delete",
    id: certId
  },
  email: email
}, callbacks);
