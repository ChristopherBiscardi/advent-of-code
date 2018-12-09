use "debug"
use "collections"
use "promises"

actor Main
  let _env: Env
  var input: Array[String]
  let _db: Db = Db

  new create(env: Env) =>
    _env = env
    input = env.args.clone()
    try
      input.shift()?
    else 
      env.exitcode(-1)  // something is totally fucked because the name of the program should always be able to be shifted off
    then
      if input.size() == 0
      then env.exitcode(-1)
      end
      // _db = Db(input.size())
      Promises[Replications]
        .join(process_input().values())
        .next[None](_db~print_output())

    end

  fun ref process_input(): Array[Promise[Replications]] => 
    let promises: Array[Promise[Replications]] = []
    for raw_input in input.values() do
      let p = Promise[Replications]
      let check = Checksum(raw_input, _db)
      check.process(p)
      promises.push(p)
    end
    promises

type Replications is Map[I8, Bool] val

actor Db
  be print_output(arr: Array[Replications] val) =>
    var num_two_reps: I64 = 0
    var num_three_reps: I64 = 0

    for reps in arr.values() do
      for (count_id, has_count) in reps.pairs() do
        if has_count then
          match count_id
            | 2 => num_two_reps = num_two_reps + 1
            | 3 => num_three_reps = num_three_reps + 1
          end
        end
      end
    end
    Debug.out(num_two_reps.string() + " " + num_three_reps.string() + " " + (num_two_reps*num_three_reps).string())


actor Checksum
  let _counts: Map[String, I8] = _counts.create(6)
  var input: String
  var _db: Db

  new create(str: String, db: Db) =>
    input = str
    _db = db

  be process(p: Promise[Replications]) => 
    for char in input.values() do
      try 
        _counts.upsert(String.from_array([char]), 1, {(v',v'') => v' + v''})?
      end
    end

    var reps: Map[I8, Bool] iso = recover reps.create(2) end
    let has_two_reps = get_repeated(2)
    let has_three_reps = get_repeated(3)
    reps(2) = has_two_reps
    reps(3) = has_three_reps
    recover reps end
    p(consume reps)

  fun get_repeated(n: I8): Bool =>
    """
    figure out if there's a character that has `n` repetitions in the string
    """
    var has_n_reps = false
    for (_, occurrences) in _counts.pairs() do
      match occurrences
      | n => has_n_reps = true
      end
    end
    has_n_reps


