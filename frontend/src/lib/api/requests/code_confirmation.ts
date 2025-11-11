import { jsonRequest, type CallbacksSet } from "../api";
import { API_SEND_CODE } from "../configs";

type SendCodeResponse = {
  email: string,
  token: string,
  expires_at: number
};

// Send code creation certificate
export const sendCodeCertCreation = (
  email: string,
  callbacks: CallbacksSet<SendCodeResponse>
) => jsonRequest(API_SEND_CODE, "POST", { 
  purpose: { 
    type: "create"
  },
  email: email
}, callbacks);


// Send code deletion certificate
export const sendCodeCertDeletion = (
  email: string,
  certId: string,
  callbacks: CallbacksSet<SendCodeResponse>
) => jsonRequest(API_SEND_CODE, "POST", { 
  purpose: { 
    type: "delete",
    id: certId
  },
  email: email
}, callbacks);
