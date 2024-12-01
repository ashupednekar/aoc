package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

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



}
