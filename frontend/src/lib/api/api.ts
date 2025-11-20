import { APIError, errors, ErrorsMatch, type ErrorKeys } from "./errors";

/**
 * Set of two required callbacks: onSuccess and onError.
 *
 * @typeParam JsonResult The expected data shape on success.
 * @typeParam PossibleErrors A list of all possible error keys.
 */
export type CallbacksSet<JsonResult, PossibleErrors extends ErrorKeys[]> = {
  /**
   * A callback that is invoked on a successful request.
   *
   * @param data - The JSON body data **received from the server**.
   */
  onSuccess: (data: JsonResult) => void,
  /**
   * A callback that is invoked when something went wrong with the request.
   *
   * @param matcher - A convenient object that has the `match` method.
   * @param message - The error message.
   * @param data - Any additional information the server might send, such as `timestamp`.
   */
  onError: (matcher: ErrorsMatch<PossibleErrors>, message: string, data: Record<string, unknown>) => void,
};

/**
 * Handles the server response and any potential errors.
 * Invokes the corresponding callback (`onSuccess` or `onError`).
 *
 * @param fetchPromise - The Promise returned by the `fetch` function call.
 * @param callbacks - The set of success and error callbacks.
 */
const handleResponse = async <JsonResult, PossibleErrors extends ErrorKeys[]>(
  fetchPromise: Promise<Response>,
  callbacks: CallbacksSet<JsonResult, PossibleErrors>
) => {
  return fetchPromise.then(async resp => {
    const json = await resp.json();

    if (resp.ok) {
      callbacks.onSuccess(json);
    } else {
      const {code_error, message, ...data} = json;
      console.error(code_error, message);

      callbacks.onError(
        new ErrorsMatch<PossibleErrors>(new APIError(code_error??"" as string)),
        message??"" as string,
        data
      );
    }
  }).catch(err => {
    const errorText = err instanceof Error? err.message : String(err);
    console.error(errors.FATAL_ERROR.code, errorText);

    callbacks.onError(
        new ErrorsMatch<PossibleErrors>(errors.FATAL_ERROR),
        errorText,
        {}
      );
  });
};

/**
 * Sends a request and invokes the corresponding callback.
 *
 * @param uri - The URL the request is sent to.
 * @param method - The HTTP method of the request (e.g., "GET", "DELETE").
 * @param callbacks - The set of success and error callbacks.
 */
export const emptyRequest = async <JsonResult, PossibleErrors extends ErrorKeys[]>(
  uri: string, 
  method: "GET"|"DELETE",
  callbacks: CallbacksSet<JsonResult, PossibleErrors>
) => {
  const promise = fetch(uri, {
    method: method,
  });

  return handleResponse(promise, callbacks);
}

/**
 * Sends a request with a JSON object as a body and invokes the corresponding callback.
 *
 * @param uri - The URL the request is sent to.
 * @param method - The HTTP method of the request (e.g., "POST", "DELETE", "PUT").
 * @param body - The JSON body object to send the server.
 * @param callbacks - The set of success and error callbacks.
 */
export const jsonRequest = async <JsonResult, PossibleErrors extends ErrorKeys[]>(
  uri: string, 
  method: "POST"|"DELETE"|"PUT",
  body: Record<string, any>,
  callbacks: CallbacksSet<JsonResult, PossibleErrors>
) => {
  const promise = fetch(uri, {
    method: method,
    body: JSON.stringify(body),
    headers: {
      "Content-Type": "application/json",
    },
  });

  return handleResponse(promise, callbacks);
}
