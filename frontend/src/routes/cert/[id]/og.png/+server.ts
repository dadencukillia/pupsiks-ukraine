import { error } from "@sveltejs/kit";
import { ImageResponse } from '@ethercorps/sveltekit-og';
import OpenGraphImage from "$lib/components/opengraph/cert.svelte";
import type { RequestHandler } from "./$types";
import { CustomFont, resolveFonts } from "@ethercorps/sveltekit-og/fonts";
import { joinURL } from "$lib/utils/joinURL";
import { API_GET_CERT } from "$lib/api/configs";
import { read } from "$app/server";
import UnboundedBold from "$lib/assets/Unbounded-Bold.ttf";
import OpenSansRegular from "$lib/assets/OpenSans-Regular.ttf";

const unboundedRegular = new CustomFont('Unbounded', 
  () => read(UnboundedBold).arrayBuffer(),
  {
	  weight: 700,
  }
);

const opensansRegular = new CustomFont('Open Sans', 
  () => read(OpenSansRegular).arrayBuffer(),
  {
    weight: 400,
  }
);

const cacheStorage = new Map<string, [Uint8Array<ArrayBuffer>, number]>();

export const GET: RequestHandler = async ({ params, request, fetch }) => {
  const certId = params.id;

  let cachedImage = cacheStorage.get(certId);

  const now = performance.now();
  for (const [key, [_, exp]] of cacheStorage) {
    if (now > exp) {
      cacheStorage.delete(key);
    }
  }

  if (!cachedImage) {
    const clientIp = request.headers.get("Forwarded")!;

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

    if (!apiCertResponse.ok) {
      return new Response("Something went wrong with certificate", {
        status: 500,
        headers: {
          "Content-Type": "text/plain"
        }
      });
    }

    const cert = json as {
      id: string,
      name: string,
      title: string
    };

    const fonts = await resolveFonts([ opensansRegular, unboundedRegular ]);

    const image = new ImageResponse(
      OpenGraphImage,
      {
        width: 1200,
        height: 630,
        fonts
      },
      cert
    );

    const bytes = await image.bytes();
    cacheStorage.set(cert.id, [bytes, performance.now() + 15 * 1_000]);
    cachedImage = [bytes, 0];
  }

  return new Response(cachedImage[0], {
    status: 200,
    headers: {
      "Content-Type": "image/png",
      "Cache-Control": "public, max-age=900"
    }
  });
};
