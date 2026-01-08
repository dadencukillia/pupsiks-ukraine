/**
 * Downloads the file by the URL.
 *
 * @param url - the URI the file stored by.
 * @param filename - the name of the downloaded file suggested to user.
 * 
 * @example
 * Downloads the website icon of Google:
 * ```ts
 * downloadFile("https://google.com/favicon.ico", "favicon.ico");
 * ```
 */
export async function downloadFile(url: URL|string, filename: string = 'myfile') {
  const response = await fetch(url);
  const blob = await response.blob();
  const blobUrl = URL.createObjectURL(blob);

  const link = document.createElement('a');
  link.href = blobUrl;
  link.download = filename;
  document.body.appendChild(link);
  link.click();
  
  document.body.removeChild(link);
  URL.revokeObjectURL(blobUrl);
}
