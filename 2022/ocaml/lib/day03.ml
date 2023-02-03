open Base

module Solution : Solution.Solve = struct
  let solution input =
    List.map (Stdio.In_channel.input_lines input) ~f:(fun line ->
        List.map (String.to_list line) ~f:(fun c ->
            match c with
            | 'a' .. 'z' -> Char.to_int c - 96
            | 'A' .. 'Z' -> Char.to_int c - 38
            | _ -> failwith "invalid char"))

  let to_set = Set.of_list (module Int)

  let part1 items =
    let find_common l =
      let first, second = List.split_n l (List.length l / 2) in
      Set.inter (to_set first) (to_set second) |> Set.choose_exn
    in
    List.map items ~f:find_common |> List.sum (module Int) ~f:Fn.id

  let part2 items =
    List.chunks_of items ~length:3
    |> List.map ~f:(fun l ->
           List.map l ~f:to_set
           |> List.reduce_exn ~f:Set.inter
           |> Set.choose_exn)
    |> List.sum (module Int) ~f:Fn.id

  let solve input =
    let s = solution input in
    (Solution.Integer (part1 s), Solution.Integer (part2 s))
end
