use "collections"
use "debug"

actor Main
  let input: List[ILong]
  let opInput: ILong = 1
  var opModes: Array[USize] = []
  var position: USize = 0
  var parameter: USize = 0
  var opcode: ILong = 0 // initial value. invalid opcode

  new create(env: Env) =>
    Debug.out("start")
    input = List[ILong].from([
      3;225;1;225;6;6;1100;1;238;225;104;0;101;71;150;224;101;-123;224;224;4;224;102;8;223;223;101;2;224;224;1;224;223;223;2;205;209;224;1001;224;-3403;224;4;224;1002;223;8;223;101;1;224;224;1;223;224;223;1101;55;24;224;1001;224;-79;224;4;224;1002;223;8;223;101;1;224;224;1;223;224;223;1;153;218;224;1001;224;-109;224;4;224;1002;223;8;223;101;5;224;224;1;224;223;223;1002;201;72;224;1001;224;-2088;224;4;224;102;8;223;223;101;3;224;224;1;223;224;223;1102;70;29;225;102;5;214;224;101;-250;224;224;4;224;1002;223;8;223;1001;224;3;224;1;223;224;223;1101;12;52;225;1101;60;71;225;1001;123;41;224;1001;224;-111;224;4;224;102;8;223;223;1001;224;2;224;1;223;224;223;1102;78;66;224;1001;224;-5148;224;4;224;1002;223;8;223;1001;224;2;224;1;223;224;223;1101;29;77;225;1102;41;67;225;1102;83;32;225;1101;93;50;225;1102;53;49;225;4;223;99;0;0;0;677;0;0;0;0;0;0;0;0;0;0;0;1105;0;99999;1105;227;247;1105;1;99999;1005;227;99999;1005;0;256;1105;1;99999;1106;227;99999;1106;0;265;1105;1;99999;1006;0;99999;1006;227;274;1105;1;99999;1105;1;280;1105;1;99999;1;225;225;225;1101;294;0;0;105;1;0;1105;1;99999;1106;0;300;1105;1;99999;1;225;225;225;1101;314;0;0;106;0;0;1105;1;99999;1107;677;677;224;1002;223;2;223;1005;224;329;101;1;223;223;7;677;677;224;1002;223;2;223;1005;224;344;1001;223;1;223;7;226;677;224;102;2;223;223;1006;224;359;101;1;223;223;1108;226;226;224;1002;223;2;223;1005;224;374;1001;223;1;223;8;226;677;224;1002;223;2;223;1006;224;389;1001;223;1;223;1108;226;677;224;1002;223;2;223;1006;224;404;101;1;223;223;1107;677;226;224;102;2;223;223;1006;224;419;101;1;223;223;1007;677;677;224;1002;223;2;223;1005;224;434;101;1;223;223;7;677;226;224;102;2;223;223;1006;224;449;1001;223;1;223;1008;226;677;224;1002;223;2;223;1006;224;464;101;1;223;223;8;677;677;224;1002;223;2;223;1006;224;479;101;1;223;223;108;226;226;224;102;2;223;223;1005;224;494;101;1;223;223;1107;226;677;224;1002;223;2;223;1006;224;509;101;1;223;223;107;226;226;224;1002;223;2;223;1006;224;524;1001;223;1;223;107;677;677;224;1002;223;2;223;1005;224;539;101;1;223;223;1007;226;226;224;102;2;223;223;1006;224;554;101;1;223;223;108;677;677;224;102;2;223;223;1005;224;569;101;1;223;223;107;677;226;224;102;2;223;223;1005;224;584;101;1;223;223;1008;226;226;224;102;2;223;223;1006;224;599;101;1;223;223;1108;677;226;224;1002;223;2;223;1006;224;614;101;1;223;223;8;677;226;224;102;2;223;223;1005;224;629;1001;223;1;223;1008;677;677;224;102;2;223;223;1006;224;644;101;1;223;223;1007;226;677;224;102;2;223;223;1005;224;659;101;1;223;223;108;226;677;224;102;2;223;223;1006;224;674;101;1;223;223;4;223;99;226
      // 1002;4;3;4;99
      // 3;0;4;0;99
    ])

    try
      opcode = getOpcode(0)?
    else
      env.exitcode(-1)
      return
    end

    while (opcode != 99) do
      Debug.out("opcode " + opcode.string())
      // var i: USize = 0
      // while i <= 20 do
      // Debug.out("listSpot " + getValue(i, 1)?.string())
      // i = i+1
      // end
      match opcode
      | let op: ILong if ((op == 1) or (op == 2)) =>
        var valueA: ILong = -1 // bad init
        var valueB: ILong = -1 // bad init
        var positionToReplace: ILong = -1 // bad init
        try 
          valueA = getValue(position+1, getParameterValue(parameter))?
        else
          Debug.out("[1/2]: failed to get init valueA at " + (position+1).string() + " with mode " + parameter.string())
          Debug.out("getParameterValue " + getParameterValue(parameter).string())
          env.exitcode(1)
          return
        end

        try
          valueB = getValue(position+2, getParameterValue(parameter+1))?
        else
          Debug.out("[1/2]: failed to get init valueB")
          env.exitcode(1)
          return
        end
        try
          positionToReplace = getValue(position+3, 1)?
        else
          Debug.out("[1/2]: failed to get init valueC")
          env.exitcode(1)
          return
        end

        var finalValue: ILong = 0

        // Debug.out((getParameterValue(parameter+1)).string() + " " + (position+1).string())
        if opcode == 1 then
          Debug("[" + opcode.string() + "]: " + valueA.string() + " + " + valueB.string())
          finalValue = valueA + valueB
        elseif opcode == 2 then
          Debug("[" + opcode.string() + "]: " + valueA.string() + " * " + valueB.string())
          finalValue = valueA * valueB
        else
          Debug.out("opcode not 1 or 2")
          env.exitcode(-1)
          return
        end
        Debug("1/2: inserting " + finalValue.string() + " at " + positionToReplace.string())
        try
          input.update(positionToReplace.usize(), finalValue)?
        else
          Debug.out("[1/2]: failed to insert " + finalValue.string() + " at " + positionToReplace.usize().string())
          env.exitcode(-1)
          return
        end
        position = position + 4
        // parameter = parameter + 3
        try
          opcode = getOpcode(position)?
        else 
          Debug.out("failed at opcode " + opcode.string())
          env.exitcode(-1)
          return
        end
      | let op: ILong if op == 3 => 
        try
          let positionToReplace = getValue(position+1, 1)?
          Debug("3: inserting " + opInput.string() + " at " + positionToReplace.string())
          input.update(positionToReplace.usize(), opInput)?

          position = position + 2
          // parameter = parameter + 1
          opcode = getOpcode(position)?
        else 
          Debug.out("failed at opcode " + opcode.string())
          return
        end
      | let op: ILong if op == 4 => 
        try
          let finalValue = getValue(position+1, getParameterValue(parameter))?
          Debug.out("finalValue: " + finalValue.string())
          position = position + 2
          // parameter = parameter + 1
          opcode = getOpcode(position)?
        else 
          Debug.out("failed at opcode " + opcode.string())
          return
        end
      // else 
        // Debug.out("opcode")
        // Debug.out(opcode)
      end
      parameter = 0
      opModes = []
    end
    env.out.print("end")
    try
      env.out.print(getValue(0,0)?.string())
    end

  fun ref getValue(pos: USize, mode: USize): ILong? =>
    let positionOrValue = input.index(pos)?.apply()?
    Debug.out("positionOrValue " + positionOrValue.string())
    var result: ILong = 0
    match mode
    | let m: USize if m == 0 =>
      // Debug.out("postiion " + positionOrValue.string() + " " + positionOrValue.usize().string())
      let res = input.index(positionOrValue.usize())?.apply()?
      // Debug.out("res" + res.string())
      result = res
    | let m: USize if m == 1 =>
      result = positionOrValue
    end
    // Debug.out("get-value result: at position"
    //  + position.string()
    //  + " result: " + result.string())
    result

  fun ref getParameterValue(pos: USize): USize =>
    var result: USize = 0
    try
      result = opModes.apply(pos)?.usize()
    else
      Debug.out("overflowed pos with " + pos.string())
    end
    // Debug.out("parameter " + result.string())
    result
  
  fun ref getOpcode(pos: USize): ILong? =>
    var rawOpcode: ILong = getValue(pos,1)?
    let validOps: Array[ILong] = [1;2;3;4]
    Debug.out("getOpcode " + rawOpcode.string())
    if validOps.contains(rawOpcode) then
      return rawOpcode
    end

    let ops = rawOpcode.string()
    let one = String.from_array([ops.pop()?])
    let two = String.from_array([ops.pop()?])
    let opTuple = (two+one).read_int[ILong](0, 10)?
    var opcode': ILong = opTuple._1
    for op in (consume ops).values() do
      let tuple = String.from_array([consume op]).read_int[USize](0, 10)?
      opModes.unshift(tuple._1)
    end

    opcode'