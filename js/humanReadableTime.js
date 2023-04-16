function humanReadable (seconds) {
  let outp = ``;
  let hours = Math.floor(seconds / 3600);
  let mins = Math.floor((seconds % 3600) / 60);
  let secs = Math.floor(seconds % 60);
  outp += hours < 10 ? `0${hours}:` : `${hours}:`;
  outp += mins < 10 ? `0${mins}:` : `${mins}:`;
  outp += secs < 10 ? `0${secs}` : `${secs}`;
  return outp;
}
