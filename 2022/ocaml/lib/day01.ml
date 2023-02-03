open Base
open Stdio

module Solution : Solution.Solve = struct
  let solution input =
    let sorted_calories =
      In_channel.fold_lines input ~init:(0, []) ~f:(fun (sum, l) -> function
        | "" -> (0, sum :: l) | s -> (sum + Int.of_string s, l))
      |> (fun (sum, l) -> sum :: l)
      |> List.sort ~compare:descending
      |> List.take
    in
    fun n -> sorted_calories n |> List.fold ~init:0 ~f:( + )

  let solve input =
    let sum_top_n = solution input in
    (Solution.Integer (sum_top_n 1), Solution.Integer (sum_top_n 3))
end
