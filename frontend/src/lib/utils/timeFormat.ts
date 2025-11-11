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
