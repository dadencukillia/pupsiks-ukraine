import { error } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';
import { errors } from '$lib/api/errors';
import { joinURL } from '$lib/utils/joinURL';
import { API_GET_CERT } from '$lib/api/configs';

export const load: PageServerLoad = async ({ params, request, fetch }) => {
  const certId = params.id;
  const clientIp = request.headers.get("Forwarded")!;
  const websiteOrigin = `${request.headers.get("X-Forwarded-Proto")}://${request.headers.get("Host")}`;

  const apiCertResponse = await fetch(joinURL("http://backend:8080", API_GET_CERT(certId)), {
    method: "GET",
    headers: {
      "X-Forwarded-Proto": request.headers.get("X-Forwarded-Proto")!,
      "Host": request.headers.get("Host")!,
      "Forwarded": clientIp,
      "X-Forwarded-For": clientIp,
      "X-Real-IP": clientIp
    }
  });

  const json = await apiCertResponse.json();

  if (apiCertResponse.ok) {
    const cert = json as {
      id: string,
      name: string,
      title: string
    };

    return { cert, websiteOrigin };
  }

  const {code_error, message, ...data} = json;
  console.error(code_error, message);

  const errorText = function() {
    switch (code_error) {
    case errors.RESOURCE_NOT_FOUND.code: return "Сертифікат не знайдено";
    case errors.BAD_REQUEST.code: return "Неправильний серійний номер";
    case errors.FATAL_ERROR.code: return "Помилка з'єднання";
    default: return "Невідома помилка";
    }
  }();

  error(500, { message: errorText })
};
