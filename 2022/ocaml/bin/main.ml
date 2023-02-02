open Aoc2022

let usage_msg = "main.exe -day <day number>"
let day = ref 0
let speclist = [ ("-day", Caml.Arg.Set_int day, "Set day number to solve") ]
let answer2string = function Solution.Integer i -> Int.to_string i

let solve (module S : Solution.Solve) input =
  let part1, part2 = S.solve input in
  Printf.printf "Part 1: %s\n" (answer2string part1);
  Printf.printf "Part 2: %s\n" (answer2string part2)

let () =
  Caml.Arg.parse speclist (fun _ -> ()) usage_msg;
  let s =
    match !day with
    | 1 -> (module Day01.Solution : Solution.Solve)
    | n -> failwith (Printf.sprintf "Day %d not implemented" n)
  in
  solve s Stdio.stdin
