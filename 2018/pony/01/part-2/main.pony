actor Main
  let _env: Env
  var freq: I32 = 0
  var input: Array[String]
  var seen_freqs: Array[I32] = []
  var first_repeated_freq_established: Bool = false

  new create(env: Env) =>
    _env = env
    input = env.args.clone()
    try
      input.shift()?
    else 
      usage()
      env.exitcode(-1)  // something is totally fucked because the name of the program should always be able to be shifted off
    then
      if input.size() == 0
      then usage(); env.exitcode(-1)
      end
      while first_repeated_freq_established != true do
        process_input()
      end
      _env.out.print("repeated freq = " + freq.string())
    end

  fun ref process_input() => 
    for raw_input in input.values() do
      if seen_freqs.contains(freq)
      then
        first_repeated_freq_established = true
      else
        seen_freqs.push(freq)
        let op = raw_input.clone()
        op.strip(", ")
        let tuple: (String, String) = op.clone().chop(1)
        match tuple
          | ("+", let u: String) => 
            try 
              freq = freq + u.i32()?
            end
          | ("-", let u: String) => 
            try
              freq = freq - u.i32()?
            end
          | (_,_) => _env.out.print("malformed input"); _env.exitcode(-1)
        end
      end
    end

  fun ref usage() =>
    _env.out.print(
      """
      program INPUT
      INPUT   a comma space separated list of operations "-1, +2, -3"
      """
      )