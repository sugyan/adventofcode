open Base

module Solution : Solution.Solve = struct
  type t = int * int -> int

  let parse input =
    let parse_monkey chunk =
      let items =
        chunk |> Fn.flip List.nth_exn 1
        |> String.chop_prefix_exn ~prefix:"Starting items:"
        |> String.split ~on:',' |> List.map ~f:String.lstrip
        |> List.map ~f:Int.of_string
      in
      let operation =
        chunk |> Fn.flip List.nth_exn 2
        |> String.chop_prefix_exn ~prefix:"Operation:"
        |> String.split ~on:' ' |> Fn.flip List.drop 4
        |> function
        | [ "*"; "old" ] -> fun x -> x * x
        | [ "*"; s ] -> fun x -> x * Int.of_string s
        | [ "+"; s ] -> fun x -> x + Int.of_string s
        | _ -> failwith "unknown operation"
      in
      let test =
        let f s = s |> String.split ~on:' ' |> List.last_exn |> Int.of_string in
        chunk |> Fn.flip List.drop 3 |> Fn.flip List.take 3 |> List.map ~f
        |> function
        | [ div; t; f ] -> (div, t, f)
        | _ -> failwith "invalid test"
      in
      (items, (operation, test))
    in
    let monkeys =
      Stdio.In_channel.input_lines input
      |> List.chunks_of ~length:7
      |> List.map ~f:(List.map ~f:String.lstrip)
      |> List.map ~f:parse_monkey
    in
    fun (n, d) ->
      let items = monkeys |> List.map ~f:fst |> Array.of_list in
      let lcm =
        let f (div, _, _) = div in
        monkeys |> List.map ~f:snd |> List.map ~f:snd |> List.map ~f
        |> List.fold ~init:1 ~f:( * )
      in
      let inspect counts =
        let f i (_, (operation, (div, t, f))) =
          let test x = x |> Fn.flip ( % ) div |> ( = ) 0 in
          let ts, fs =
            items.(i) |> List.map ~f:operation
            |> List.map ~f:(Fn.flip ( / ) d)
            |> List.map ~f:(Fn.flip ( % ) lcm)
            |> List.partition_tf ~f:test
          in
          items.(i) <- [];
          items.(t) <- ts @ items.(t);
          items.(f) <- fs @ items.(f);
          List.length ts + List.length fs
        in
        monkeys |> List.mapi ~f |> List.map2_exn counts ~f:( + )
      in
      Fn.apply_n_times ~n inspect (List.map monkeys ~f:(Fn.const 0))
      |> List.sort ~compare:Int.descending
      |> Fn.flip List.take 2 |> List.fold ~init:1 ~f:( * )

  let part1 monkey_business = monkey_business (20, 3) |> Solution.answer_of_int

  let part2 monkey_business =
    monkey_business (10000, 1) |> Solution.answer_of_int
end
