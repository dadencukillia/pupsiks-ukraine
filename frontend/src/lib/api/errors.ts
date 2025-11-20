/**
 * Custom error object designed to represent API-specific errors.
 * This class primarily holds the standardized error code returned by the server.
 *
 * It can also represent `fatal` error not originating from the server.
 */
export class APIError {
  private readonly _code: string;

  /**
   * Creates an instance of APIError.
   *
   * @param code - The standardized error code (e.g., 'page_not_found', 'bad_request').
   */
  public constructor(code: string) {
    this._code = code;
  }

  /**
   * Gets the standardized error code.
   */
  public get code(): string {
    return this._code;
  }
}

/**
 * A collection of standardized API error constants.
 * These constants are instances of the APIError class,
 * representing specific server or internal error conditions.
 *
 * @remarks The 'FATAL_ERROR' is an internal application error, not an error returned by the backend server.
 */
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

/**
 * A union type consisting of all possible standardized error keys
 * defined in the `errors` constant object.
 */
export type ErrorKeys = keyof typeof errors;

/**
 * A utility class designed to match an occurred API error against a set of
 * handled error keys, executing the corresponding handler function.
 *
 * @typeParam T - A literal array type defining the specific error keys (from `ErrorKeys`)
 * that this matcher instance is capable of handling.
 */
export class ErrorsMatch<T extends ErrorKeys[]> {
  private readonly _error: APIError;

  /**
   * Creates an instance of ErrorsMatch.
   *
   * @param error - The specific APIError object that occurred and needs to be handled.
   */
  public constructor(error: APIError) {
    this._error = error;
  }

  /**
   * Returns the underlying APIError object.
   *
   * @returns The APIError instance.
   */
  public get error(): APIError {
    return this._error;
  }

  /**
   * Attempts to match the occurred error code with a handler defined in the 'handles' object.
   *
   * @param handles - An object containing handler functions keyed by error name (e.g., 'PAGE_NOT_FOUND').
   * It may optionally include a 'default' handler invoked when no other matches are found.
   */
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
