open Base

module Solution : Solution.Solve = struct
  type t = int list list

  let parse input =
    Stdio.In_channel.input_lines input
    |> List.map ~f:(fun line ->
           String.to_list line
           |> List.map ~f:(fun c ->
                  match c with
                  | 'a' .. 'z' -> Char.to_int c - 96
                  | 'A' .. 'Z' -> Char.to_int c - 38
                  | _ -> failwith "invalid char"))

  let part1 t =
    let find_common l =
      let first, second = List.split_n l (List.length l / 2) in
      Set.inter
        (Set.of_list (module Int) first)
        (Set.of_list (module Int) second)
      |> Set.choose_exn
    in
    Solution.Integer
      (List.map t ~f:find_common |> List.sum (module Int) ~f:Fn.id)

  let part2 t =
    Solution.Integer
      (List.chunks_of t ~length:3
      |> List.map ~f:(fun l ->
             List.map l ~f:(Set.of_list (module Int))
             |> List.reduce_exn ~f:Set.inter
             |> Set.choose_exn)
      |> List.sum (module Int) ~f:Fn.id)
end
