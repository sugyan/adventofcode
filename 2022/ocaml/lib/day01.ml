open Base

module Solution : Solution.Solve = struct
  type t = int list

  let top_n_sum t n = List.take t n |> List.sum (module Int) ~f:Fn.id

  let parse input =
    Stdio.In_channel.fold_lines input ~init:(0, []) ~f:(fun (sum, l) -> function
      | "" -> (0, sum :: l) | s -> (sum + Int.of_string s, l))
    |> (fun (sum, l) -> sum :: l)
    |> List.sort ~compare:descending

  let part1 t = Solution.Integer (top_n_sum t 1)
  let part2 t = Solution.Integer (top_n_sum t 3)
end
