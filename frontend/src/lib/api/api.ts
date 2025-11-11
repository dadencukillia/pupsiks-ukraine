export type CallbacksSet<T> = {
  onSuccess: (data: T) => void,
  onError: (codeError: string, message: string, data: any) => void,
  onFatal: (error: string) => void,
};

export const emptyRequest = async <T>(
  uri: string, 
  method: "GET"|"DELETE",
  callbacks: CallbacksSet<T>
) => {
  return fetch(uri, {
    method: method,
  }).then(async resp => {
    const json = await resp.json();

    if (Math.floor(resp.status / 100) === 2) {
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
}

export const jsonRequest = async <T>(
  uri: string, 
  method: "POST"|"DELETE"|"PUT",
  body: any, 
  callbacks: CallbacksSet<T>
) => {
  return fetch(uri, {
    method: method,
    body: JSON.stringify(body),
    headers: {
      "Content-Type": "application/json",
    },
  }).then(async resp => {
    const json = await resp.json();

    if (Math.floor(resp.status / 100) === 2) {
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
}
