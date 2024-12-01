package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"strconv"
	"strings"
)

func bubbleSort(arr []int32) {
    n := len(arr)
    for i := 0; i < n-1; i++ {
        for j := 0; j < n-i-1; j++ {
            if arr[j] > arr[j+1] {
                arr[j], arr[j+1] = arr[j+1], arr[j]
            }
        }
    }
}

func main(){

  file, err := os.Open("day1inp.txt")
  if err != nil{
    fmt.Printf("err: %v\n", err)
  }
  defer file.Close()

  var l1 []int32
  var l2 []int32

  scanner := bufio.NewScanner(file)
  for scanner.Scan(){
    line := scanner.Text()
    pairsStr := strings.Split(line, "\n")

    for _, pair := range pairsStr{
      pair = strings.TrimSpace(pair)
      nums := strings.Split(pair, "   ")
      num1, _ := strconv.Atoi(nums[0])
      num2, _ := strconv.Atoi(nums[1])
      l1 = append(l1, int32(num1))
      l2 = append(l2, int32(num2))
    }
  }

  if err := scanner.Err(); err != nil{
    fmt.Printf("error reading: %v\n", err)
    return
  }

  bubbleSort(l1)
  bubbleSort(l2)

  var total int32;
  for i := range(len(l1)){
    total = total + int32(math.Abs(float64(l2[i] - l1[i]))) 
  }

  fmt.Printf("total: %v\n", total)

}
