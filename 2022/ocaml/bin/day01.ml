open Base
open Stdio

let solution input =
  let sorted_calories =
    In_channel.fold_lines input ~init:(0, []) ~f:(fun (sum, l) line ->
        match line with
        | "" -> (0, sum :: l)
        | _ -> (sum + Int.of_string line, l))
    |> (fun (sum, l) -> sum :: l)
    |> List.sort ~compare:Int.compare
    |> List.rev |> List.take
  in
  fun n -> sorted_calories n |> List.fold ~init:0 ~f:( + )

let _ =
  let solve = solution Stdio.stdin in
  printf "Part 1: %d\n" (solve 1);
  printf "Part 2: %d\n" (solve 3)
