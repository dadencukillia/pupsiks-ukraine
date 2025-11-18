export class APIError {
  private readonly _code: string;

  public constructor(code: string) {
    this._code = code;
  }

  public get code(): string {
    return this._code;
  }
}

export const errors = {
  PAGE_NOT_FOUND: new APIError("page_not_found"),
  BAD_REQUEST: new APIError("bad_request"),
  RESOURCE_NOT_FOUND: new APIError("resource_not_found"),
  INTERNAL_SERVER_ERROR: new APIError("internal_server_error"),
  EMAIL_RATE_LIMIT: new APIError("email_rate_limit"),
  IP_RATE_LIMIT: new APIError("ip_rate_limit"),
  INVALID_ROUTE: new APIError("invalid_route"),
  INVALID_CODE: new APIError("invalid_code"),
  INVALID_TOKEN: new APIError("invalid_token"),
  ALREADY_EXISTS: new APIError("already_exists"),
  TRIES_OUT: new APIError("tries_out"),
  INVALID_EMAIL: new APIError("invalid_email"),
  FATAL_ERROR: new APIError("fatal")
} as const;

export type ErrorKeys = keyof typeof errors;

export class ErrorsMatch<T extends ErrorKeys[]> {
  private readonly _error: APIError;

  public constructor(error: APIError) {
    this._error = error;
  }

  public get error(): APIError {
    return this._error;
  }

  public match(handles: { [K in T[number]]?: () => void } & { default?: () => void }) {
    let defaultHandler = () => {};

    for (const error in handles) {
      if (error === "default") {
        defaultHandler = handles.default!;
      } else {
        const k = error as T[number];

        if (errors[k].code === this._error.code) {
          handles[k]!();
          return;
        }
      }
    }

    defaultHandler();
  }
}
