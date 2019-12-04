use "collections"
use "files"
use "debug"

actor Main
  let input: Range[USize] = Range(245182, 790572+1)
  let possiblePasswords: Array[USize] = []

  new create(env: Env) =>
    try
      for num in input do
        let numArray: Array[USize] = []
        let numAsStrings = num.string()
        for iStr in (consume numAsStrings).values() do
          let int = String.from_array([iStr])
                          .read_int[USize](0, 10)?
          numArray.push(int._1)
        end

        //
        if isIncreasing(numArray) and hasTwoDigits(numArray) then
          possiblePasswords.push(num)
        end

      end

      for num in possiblePasswords.values() do
        env.out.print("pass: " + num.string())
      end
    end
    env.out.print("end " + possiblePasswords.size().string())
  
  fun ref isIncreasing(nums: Array[USize]): Bool =>
    var lastNum: USize = 0
    var result = true
    for num in nums.values() do
      if result == false then break end
      let truthy = num >= lastNum
      // Debug.out(num.string() + ">=" + lastNum.string() + ": " + truthy.string())
      result = truthy
      lastNum = num
    end

    result
  
  fun ref hasTwoDigits(nums: Array[USize]): Bool =>
    var result = false
    try

      var lastNum = nums.shift()?

      for num in nums.values() do
        if lastNum == num then
          result = true
          break
        else
          lastNum = num
        end
      end
    end
    result