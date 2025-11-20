/**
 * Joins a base URL fragment and a subsequent path segment, ensuring a single
 * forward slash (/) acts as a separator between them, regardless of whether
 * the base fragment ends or the segment begins with a slash.
 *
 * @param left - The base URL or the left fragment.
 * @param right - The path segment to append or the right fragment.
 * @returns The combined, correctly separated URL string.
 */
export const joinURL = (left: string, right: string): string => {
  return (left.endsWith("/") ? left.substring(0, left.length - 1) : left) + "/" + (right.startsWith("/") ? right.substring(1) : right);
};

/**
 * Joins multiple URL fragments into a single, cohesive URL string.
 * It iteratively uses `joinURL` to safely combine an arbitrary number of path fragments,
 * preventing duplicate slashes between segments.
 *
 * @param fragments - An array of URL or path fragments to be joined.
 * @returns The final, concatenated URL string, or an empty string if no fragments are provided.
 */
export const joinURLs = (...fragments: string[]): string => {
  if (fragments.length === 0) return "";

  return fragments.slice(1).reduce((acc, cur) => joinURL(acc, cur), fragments[0]);
};
