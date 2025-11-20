/**
 * Trims a string, removes all control characters, and collapses multiple
 * spaces into a single space, resulting in a clean and standardized string.
 *
 * @param input - The original string to be cleaned and trimmed.
 * @returns The resulting string after all cleaning operations are applied.
 */
export const smartTrim = (input: string): string => {
  // Trim and clear all control symbols
  let trimmedAndCleaned = input
      .trim()
      .replace(/[\p{C}]/gu, ''); 
     
  // Collapse multiple whitespace characters into a single space.
  let singleSpaced = trimmedAndCleaned.replace(/\s+/g, ' ');

  // Final trim in case the input started or ended with multiple spaces/control characters.
  return singleSpaced.trim();
}
