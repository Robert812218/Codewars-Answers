function newSort(arr) {
  let cond = false;
  let init = arr[0];
  let iter = 0;
  while (cond === false) {
    if (arr[iter + 1] - arr[iter] !== 1 && arr[iter + 1] - arr[iter] !== -1) {
       cond = true;
       break;
    }
    iter++;
  }
  return arr[0] !== arr[iter] ? arr[iter] : false;
}


function solution(list){
  let sortedList = list.sort((a, b) => a - b);
  let newString = "";
  console.log("sorted list: \n" + sortedList + "\n");
  let last = sortedList[0];
  for (let i = 0; i < sortedList.length; i++) {
    let s = sortedList[i];
    console.log(`last: ${last}, this: ${s}`);
    if (last + 1 !== s) {
      newString += ` ${s}`;
      last = s;
    } else {
      let next = newSort(sortedList);
      newString += next ? `-${next}` : '';
      last = s;
    }
  }
  
  console.log("newString: " + newString);
  return newString;
}
