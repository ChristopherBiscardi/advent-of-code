use "collections"
use "files"

actor Main
  let input: Array[Array[String]] = []

  new create(env: Env) =>
    let caps = recover val FileCaps.>set(FileRead).>set(FileStat) end

    try
      with file = OpenFile(
        FilePath(env.root as AmbientAuth, env.args(1)?, caps)?) as File
      do
        env.out.print(file.path.path)
        for line in file.lines() do
          input.push(line.clone().split_by(","))
          env.out.print(consume line)
        end
      end
    else
      try
        env.out.print("Couldn't open " + env.args(1)?)
        env.exitcode(-1)
      end
    end

    try
      let arrowPoints: Array[Array[(ILong, ILong)]] = []
      for arr in input.values() do
        // (x, y)
        let positions: Array[(ILong, ILong)] = [(0,0)]
        for instruction in arr.values() do
          let distance = instruction.read_int[ILong](1, 10)?
          let direction = instruction.substring(0,1)
          // env.out.print("Moving " +  direction + " " + distance._1.string())
          // env.out.print(distance._1.string() + "," + distance._2.string())
          var i: ILong = 1
          match consume direction
          | let x: String if x == "U" => 
              while i <= distance._1 do
                let currentPosition = positions.apply(positions.size()-1)?
                positions.push((currentPosition._1,currentPosition._2 + 1))
                i = i+1
              end
          | let x: String if x == "D" => 
              while i <= distance._1 do
                let currentPosition = positions.apply(positions.size()-1)?
                positions.push((currentPosition._1,currentPosition._2 - 1))
                i = i+1
              end
          | let x: String if x == "L" => 
              while i <= distance._1 do
                let currentPosition = positions.apply(positions.size()-1)?
                positions.push((currentPosition._1 - 1,currentPosition._2))
                i = i+1
              end
          | let x: String if x == "R" => 
              while i <= distance._1 do
                let currentPosition = positions.apply(positions.size()-1)?
                positions.push((currentPosition._1 + 1,currentPosition._2))
                i = i+1
              end
          end
        end
        arrowPoints.push(positions)
      end

      let arrowOne = arrowPoints.apply(0)?
      let arrowTwo = arrowPoints.apply(1)?
      let commonPoints: Array[(ILong, ILong)] = []
      var minPoint: (ILong, ILong) = (0,0)
      var minDistance: ULong = 1000000000000
      
      for point in arrowOne.values() do
        if arrowTwo.contains(point) and ((point._1 != 0) and (point._2 !=0)) then
          commonPoints.push(point)
          let newMin = minDistance.min(point._1.abs() + point._2.abs())
          if newMin != minDistance then 
            minPoint = point
            minDistance = newMin
          end
          // env.out.print(point._1.string() + "," + point._2.string() + ":" + (point._1.abs() + point._2.abs()).string() )
        end
      end
      env.out.print(minDistance.string())

      var commonPointDistance: USize = 100000000000
      var commonPoint: (ILong,ILong) = (0,0)
      // find common points wire distances
      for point in commonPoints.values() do
        let a1Index = arrowOne.find(point)?
        let a2Index = arrowTwo.find(point)?
        if (a1Index + a2Index) < commonPointDistance then
          commonPointDistance = a1Index + a2Index
          commonPoint = point
        end
      end

      env.out.print(commonPointDistance.string())
    end

    env.out.print("end")
    