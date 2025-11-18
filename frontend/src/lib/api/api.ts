import { APIError, errors, ErrorsMatch, type ErrorKeys } from "./errors";

export type CallbacksSet<JsonResult, PossibleErrors extends ErrorKeys[]> = {
  onSuccess: (data: JsonResult) => void,
  onError: (matcher: ErrorsMatch<PossibleErrors>, message: string, data: Record<string, unknown>) => void,
};

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
