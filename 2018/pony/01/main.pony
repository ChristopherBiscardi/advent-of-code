use "options"

actor Main
  let _env: Env
  var _cli_input: String = ""

  new create(env: Env) =>
    _env = env
    try
      arguments()?
    end

    var freq: I32 = 0
    env.out.print(_cli_input)
    let ops = _cli_input.split_by(", ").values()

    for op in ops do
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
        | (_,_) => env.out.print("nuthin")
      end
    end
    _env.out.print("output: " + freq.string())

  fun ref arguments() ? =>
    var options = Options(_env.args)

    options
      .add("input", "i", StringArgument)

    for option in options do
      match option
      | ("input", let arg: String) => _cli_input = arg
      | let err: ParseError => err.report(_env.out) ; usage() ; error
      end
    end

  fun ref usage() =>
    _env.out.print(
      """
      program [OPTIONS]\n
        --string      N   a string argument. Defaults to 'default'.
      """
      )