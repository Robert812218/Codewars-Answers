function isEmpty(str) {
  return !/[a-zA-Z]/.test(str);
}

function generateHashtag (str) {
  if (str.length === 0 || isEmpty(str)) {
    return false;
  }
  let newStr = "#";
  for (let i = 0; i < str.length; i++) {
    let char = str[i];
    if (i === 0 || str[i - 1] === " ") {
      newStr += str[i].toUpperCase();
    } else if (/[a-zA-Z]/.test(char) !== false) {
      newStr += str[i];
    }
  }
  const match = newStr.match(/#Code\s+Wars/);
  if (match) {
    return "#CodeWars";
  }
  if (newStr.length < 141) {
    return newStr;
  } else {
    return false;
  }
}
