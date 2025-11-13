export type CallbacksSet<T> = {
  onSuccess: (data: T) => void,
  onError: (codeError: string, message: string, data: any) => void,
  onFatal: (error: string) => void,
};

const handleResponse = async <T>(
  fetchPromise: Promise<Response>,
  callbacks: CallbacksSet<T>
) => {
  return fetchPromise.then(async resp => {
    const json = await resp.json();

    if (resp.ok) {
      callbacks.onSuccess(json);
    } else {
      const data = JSON.parse(JSON.stringify(json));
      delete data["code_error"];
      delete data["message"];

      callbacks.onError(
        json["code_error"]!,
        json["message"]!,
        data
      );
    }
  }).catch(err => {
    callbacks.onFatal(err.toString());
  });
};

export const emptyRequest = async <T>(
  uri: string, 
  method: "GET"|"DELETE",
  callbacks: CallbacksSet<T>
) => {
  const promise = fetch(uri, {
    method: method,
  });

  return handleResponse(promise, callbacks);
}

export const jsonRequest = async <T>(
  uri: string, 
  method: "POST"|"DELETE"|"PUT",
  body: any, 
  callbacks: CallbacksSet<T>
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
