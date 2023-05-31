var maxSequence = function(arr){
  if (arr.length === 0) {
    return 0;
  }
  let maxSum = 0;
  let currentSum = 0;
  
  for (let i = 0; i < arr.length; i++) {
    currentSum = Math.max(0, currentSum + arr[i]);
    maxSum = Math.max(maxSum, currentSum);
  }
  
  return maxSum;
}
