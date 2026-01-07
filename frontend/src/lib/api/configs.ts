import { joinURL, joinURLs } from "../utils/joinURL";

export const API_PATH: string = "/api/v1";

// Certificate CRUD URLs
export const API_CREATE_CERT = joinURL(API_PATH, "/cert");
export const API_GET_CERT = (certId: string) => joinURLs(API_PATH, "/cert", certId);
export const API_DELETE_CERT = joinURL(API_PATH, "/cert");
export const API_FORGOT_CERT = joinURL(API_PATH, "/cert/forgot");
export const API_SEND_CODE = joinURL(API_PATH, "/send_code");

// Statistics
export const API_STATS_USERS_COUNT = joinURL(API_PATH, "/stats/users_count");
