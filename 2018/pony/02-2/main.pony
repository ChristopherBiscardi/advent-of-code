use "debug"
use "collections"
use "promises"
use "files"

actor Main
  let _env: Env
  let _agg: Aggregator = Aggregator
  let file_content: Array[String] = []

  new create(env: Env) =>
    _env = env
    try
      let path = FilePath(_env.root as AmbientAuth, "./input.txt")?
      match OpenFile(path)
        | let file: File =>
          for line in file.lines() do
            file_content.push(line.clone())
          end
          Promises[String]
          .join(process_input(file_content).values())
          .next[None](_agg~print_output())
      else halt_and_catch_fire("Error opening file")
      end
    else halt_and_catch_fire("env does not contain auth root")
    end

  fun ref process_input(input: Array[String]): Array[Promise[String]] =>
    let promises: Array[Promise[String]] = []
    for a in input.clone().values() do
      for b in input.clone().values() do
        let p = Promise[String]
        let cs = CompareStrings(a.clone(), b.clone())
        cs.process(p)
        promises.push(p)
      end
    end
    promises

  fun halt_and_catch_fire(error_message: String) =>
    _env.err.print(error_message)
    _env.exitcode(-1)

actor Aggregator
  be print_output(arr: Array[String] val) =>
    for str in arr.values() do
      if str.size() > 0 then
        Debug.out(str)
      end
    end

actor CompareStrings
  var _a: String
  var _b: String
  
  new create(a: String, b: String) =>
    _a = a
    _b = b

  be process(p: Promise[String]) =>
    """
    returns the common letters in matching strings if they are one char apart
    """
    var str_position: USize = 0
    var num_different_letters: I8 = 0
    var common_letters: Array[U8] iso = []

    while str_position < _a.size() do
      try
        let a_char: U8 = _a.at_offset(str_position.isize())?
        let b_char: U8 = _b.at_offset(str_position.isize())?
        
        if a_char == b_char then
          common_letters.push(a_char)
        else
          num_different_letters = num_different_letters + 1
        end
        
        str_position = str_position + 1
        end
    end

    if num_different_letters == 1 then
      let return_val = String.from_iso_array(consume common_letters)
      p(consume return_val)
    else
      p("")
    end


