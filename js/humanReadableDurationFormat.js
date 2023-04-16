function formatDuration (seconds) {
  if (seconds === 0) {
    return 'now';
  }
  let out = [];
  let outp = ``;
  const units = [
    { name: 'year', seconds: 31536000 },
    { name: 'day', seconds: 86400 },
    { name: 'hour', seconds: 3600 },
    { name: 'minute', seconds: 60 },
    { name: 'second', seconds: 1 },
  ];
  
  for (let i = 0; i < units.length; i++) {
    const unit = units[i];
    const value = Math.floor(seconds / unit.seconds);
    if (value !== 0) {
      out.push(value === 1 ? `${value} ${unit.name}` : `${value} ${unit.name}s`);
      seconds %= unit.seconds;
    }
  }
  
  if (out.length === 1) {
    outp += `${out[0]}`;
  } else if (out.length === 2) {
    outp += `${out[0]} and ${out[1]}`;
  } else {
    outp += `${out.slice(0, -1).join(', ')} and ${out.slice(-1)}`;
  }
  
  console.log(outp);
  return outp;
}
