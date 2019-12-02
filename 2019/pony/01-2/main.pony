use "collections"

actor Main
  let input: List[ILong]
  let output: ILong

//   fun processInt(theInt: ILong): ILong =>
//     theInt * 4

  new create(env: Env) =>
    input = List[ILong].from([
130762
108691
131618
138163
59967
130453
117515
115776
134083
86966
128075
55569
112843
97878
92330
70917
143903
81171
148506
141379
131161
88719
69654
82141
55265
75623
97408
105269
147378
126054
133962
60304
130503
138350
93164
69661
69271
100054
138295
142865
64142
123466
80101
149696
102510
129988
87742
106785
133039
59192
86544
124950
64242
80128
109287
129634
140335
118220
106819
97296
111003
103222
54192
103548
63861
140571
50476
100570
114065
110279
64720
91941
62312
80834
132969
51973
115887
68662
138266
107234
75795
81409
78610
112587
92384
111804
138861
79393
81285
131307
68815
54976
127529
103359
138537
79663
128097
56085
96504
119501
    ])
    output = input
      .map[List[ILong]]({ (n) =>
        let results = List[ILong]()
        var thing: ILong = n 
        while (thing > 0) do
          thing = (thing).fld(3) - 2
          if thing > 0 then
            results.push(thing)
            // env.out.print(thing.string())
          end
        end
        results
      })
      .fold[List[ILong]]({
        (b, a) => 
        b.push(
          a.fold[ILong]({
            (b', a') => b' + a'
          }, 0))
        b
      }, List[ILong]())
      .fold[ILong]({
        (b, a) => b + a
      }, 0)
    env.out.print(output.string())

