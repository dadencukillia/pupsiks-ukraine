/**
 * Converts a total number of seconds into a human-readable duration string,
 * displaying the two largest relevant time units (e.g., "1 д 5 год" or "4 хв 30 с").
 *
 * @param seconds - The total duration in seconds.
 * @returns The formatted duration string, localized (in Ukrainian) with two units,
 * or "0 с" if the input is zero or negative.
 */
export const durationFormat = (seconds: number) => {
  const daysWord = " д";
  const hoursWord = " год";
  const minutesWord = " хв";
  const secondsWord = " с";

  if (seconds <= 0) {
    return `0${secondsWord}`;
  }

  let minutes = Math.floor(seconds / 60);
  seconds -= minutes * 60;

  let hours = Math.floor(minutes / 60);
  minutes -= hours * 60;

  let days = Math.floor(hours / 24);
  hours -= days * 24;

  if (days !== 0) {
    return `${days}${daysWord} ${hours}${hoursWord}`;
  }

  if (hours !== 0) {
    return `${hours}${hoursWord} ${minutes}${minutesWord}`;
  }

  if (minutes !== 0) {
    return `${minutes}${minutesWord} ${seconds}${secondsWord}`;
  }

  return `${seconds}${secondsWord}`;
}
