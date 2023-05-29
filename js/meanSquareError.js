var solution = function(firstArray, secondArray) {
  let diffs = [];
  for (let i = 0; i < firstArray.length; i++) {
    let diff = Math.abs(firstArray[i] - secondArray[i]);
    let squaredDiff = Math.pow(diff, 2);
    diffs.push(squaredDiff);
  }
  let div = diffs.length;
  const sum = diffs.reduce((accumulator, currentValue) => accumulator + currentValue, 0);
  return sum / div;
}
