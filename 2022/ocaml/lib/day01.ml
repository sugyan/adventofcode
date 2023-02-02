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
    let s = solution input in
    (Solution.Integer (s 1), Solution.Integer (s 3))
end
