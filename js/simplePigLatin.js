function leftSide(inp) {
  return inp.substring(1);
}

function rightSide(inp) {
  if (!/[A-Za-z]/.test(inp)) {
    return false;
  }
  return `${inp[0]}ay`;
}

function pigIt(str){
  let words = str.split(" ");
  let fin = "";
  for (const w of words) {
    let ls = leftSide(w);
    let rs = rightSide(w) ? rightSide(w) : "";
    let outp = `${ls}${rs}`;
    fin += outp + (rs ? " " : "");
  }
  console.log(fin.trim());
  return fin.trim();
}
