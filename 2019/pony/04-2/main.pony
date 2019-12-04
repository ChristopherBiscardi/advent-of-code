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
    let hashMap: HashMap[USize, USize, HashIs[USize]] = HashMap[USize, USize, HashIs[USize]]

    for num in nums.values() do
      hashMap.upsert(num, 1, {(current, provided) => current + provided })
    end

    var result = false
    for value in hashMap.values() do
      if value == 2 then
        result = true
      end
    end
    result