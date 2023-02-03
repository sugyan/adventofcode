open Base

module Solution : Solution.Solve = struct
  let solution input =
    let counts =
      Stdio.In_channel.fold_lines input
        ~init:(Map.empty (module String))
        ~f:(fun counter line ->
          let c = Map.find counter line |> Option.value ~default:0 in
          Map.set counter ~key:line ~data:(c + 1))
    in
    fun score ->
      Map.fold counts ~init:0 ~f:(fun ~key ~data acc ->
          acc + (score key * data))

  let part1 = function
    | "A X" -> 4 (* 1 + 3 *)
    | "A Y" -> 8 (* 2 + 6 *)
    | "A Z" -> 3 (* 3 + 0 *)
    | "B X" -> 1 (* 1 + 0 *)
    | "B Y" -> 5 (* 2 + 3 *)
    | "B Z" -> 9 (* 3 + 6 *)
    | "C X" -> 7 (* 1 + 6 *)
    | "C Y" -> 2 (* 2 + 0 *)
    | "C Z" -> 6 (* 3 + 3 *)
    | _ -> failwith "Invalid input"

  let part2 = function
    | "A X" -> 3 (* 3 + 0 *)
    | "A Y" -> 4 (* 1 + 3 *)
    | "A Z" -> 8 (* 2 + 6 *)
    | "B X" -> 1 (* 1 + 0 *)
    | "B Y" -> 5 (* 2 + 3 *)
    | "B Z" -> 9 (* 3 + 6 *)
    | "C X" -> 2 (* 2 + 0 *)
    | "C Y" -> 6 (* 3 + 3 *)
    | "C Z" -> 7 (* 1 + 6 *)
    | _ -> failwith "Invalid input"

  let solve input =
    let total_score = solution input in
    (Solution.Integer (total_score part1), Solution.Integer (total_score part2))
end
