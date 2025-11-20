import { joinURL, joinURLs } from "../utils/joinURL";

export const API_HOST: string = import.meta.env.VITE_API_URL;

// Certificate CRUD URLs
export const API_CREATE_CERT = joinURL(API_HOST, "/cert");
export const API_GET_CERT = (certId: string) => joinURLs(API_HOST, "/cert", certId);
export const API_DELETE_CERT = joinURL(API_HOST, "/cert");
export const API_FORGOT_CERT = joinURL(API_HOST, "/cert/forgot");
export const API_SEND_CODE = joinURL(API_HOST, "/send_code");

// Statistics
export const API_STATS_USERS_COUNT = joinURL(API_HOST, "/stats/users_count");
