open Base

module Solution : Solution.Solve = struct
  type t = int list list

  let parse input =
    let priority c =
      match c with
      | 'a' .. 'z' -> Char.to_int c - 96
      | 'A' .. 'Z' -> Char.to_int c - 38
      | _ -> failwith "invalid char"
    in
    Stdio.In_channel.input_lines input
    |> List.map ~f:String.to_list
    |> List.map ~f:(List.map ~f:priority)

  let part1 items =
    let f l =
      let first, second = List.split_n l (List.length l / 2) in
      Set.inter
        (Set.of_list (module Int) first)
        (Set.of_list (module Int) second)
      |> Set.choose_exn
    in
    Solution.Integer (items |> List.sum (module Int) ~f)

  let part2 items =
    let f l =
      List.map l ~f:(Set.of_list (module Int))
      |> List.reduce_exn ~f:Set.inter
      |> Set.choose_exn
    in
    Solution.Integer (List.chunks_of items ~length:3 |> List.sum (module Int) ~f)
end
