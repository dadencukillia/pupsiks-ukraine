const connectUrl = (left: string, right: string): string => {
  return (left.endsWith("/") ? left.substring(0, left.length - 1) : left) + "/" + (right.startsWith("/") ? right.substring(1) : right);
};

export const API_HOST = "http://127.0.0.1:8080/api/v1";

// Certificate CRUD URLs
export const API_CREATE_CERT = connectUrl(API_HOST, "/cert");
export const API_GET_CERT = (certId: string) => connectUrl(connectUrl(API_HOST, "/cert"), certId);
export const API_DELETE_CERT = connectUrl(API_HOST, "/cert");
export const API_FORGOT_CERT = connectUrl(API_HOST, "/cert/forgot");
export const API_SEND_CODE = connectUrl(API_HOST, "/send_code");

// Statistics
export const API_STATS_USERS_COUNT = connectUrl(API_HOST, "/stats/users_count");

// Errors
export const ERROR_PAGE_NOT_FOUND = "page_not_found";
export const ERROR_BAD_REQUEST = "bad_request";
export const ERROR_RESOURCE_NOT_FOUND = "resource_not_found";
export const ERROR_INTERNAL_SERVER_ERROR = "internal_server_error";
export const ERROR_EMAIL_RATE_LIMIT = "email_rate_limit";
export const ERROR_IP_RATE_LIMIT = "ip_rate_limit";
export const ERROR_INVALID_ROUTE = "invalid_route";
export const ERROR_INVALID_CODE = "invalid_code";
export const ERROR_INVALID_TOKEN = "invalid_token";
export const ERROR_ALREADY_EXISTS = "already_exists";
export const ERROR_TRIES_OUT = "tries_out";
export const ERROR_INVALID_EMAIL = "invalid_email";
