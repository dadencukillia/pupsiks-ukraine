const connectUrl = (left: string, right: string): string => {
  return (left.endsWith("/") ? left.substring(0, left.length - 1) : left) + "/" + (right.startsWith("/") ? right.substring(1) : right);
};

export const API_HOST = "http://127.0.0.1:3000/";

export const API_CREATE_CERT = connectUrl(API_HOST, "/123");
export const API_CREATE_CERT_MAIL_CODE = connectUrl(API_HOST, "/123");

export const API_GET_CERT = connectUrl(API_HOST, "/123");

export const API_DELETE_CERT = connectUrl(API_HOST, "/123");
export const API_DELETE_CERT_MAIL_CODE = connectUrl(API_HOST, "/123");
