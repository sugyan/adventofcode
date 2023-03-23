open Aoc2022

let usage_msg = "main.exe -day <day number> -part <1|2>"
let day = ref 0
let part = ref None

let speclist =
  [
    ("-day", Arg.Set_int day, "Set day number to solve");
    ("-part", Arg.Symbol ([ "1"; "2" ], fun s -> part := Some s), " ");
  ]

let answer2string = function
  | Solution.Integer i -> Int.to_string i
  | Solution.String s -> s

let solve (module S : Solution.Solve) input =
  let s = S.parse input in
  if !part <> Some "2" then
    s |> S.part1 |> answer2string
    |> Printf.sprintf "Part 1: %s"
    |> print_endline;
  if !part <> Some "1" then
    s |> S.part2 |> answer2string
    |> Printf.sprintf "Part 2: %s"
    |> print_endline

let () =
  Arg.parse speclist ignore usage_msg;
  let s =
    match !day with
    | 1 -> (module Day01.Solution : Solution.Solve)
    | 2 -> (module Day02.Solution : Solution.Solve)
    | 3 -> (module Day03.Solution : Solution.Solve)
    | 4 -> (module Day04.Solution : Solution.Solve)
    | 5 -> (module Day05.Solution : Solution.Solve)
    | 6 -> (module Day06.Solution : Solution.Solve)
    | 7 -> (module Day07.Solution : Solution.Solve)
    | 8 -> (module Day08.Solution : Solution.Solve)
    | 9 -> (module Day09.Solution : Solution.Solve)
    | 10 -> (module Day10.Solution : Solution.Solve)
    | 11 -> (module Day11.Solution : Solution.Solve)
    | 12 -> (module Day12.Solution : Solution.Solve)
    | 13 -> (module Day13.Solution : Solution.Solve)
    | 14 -> (module Day14.Solution : Solution.Solve)
    | 15 -> (module Day15.Solution : Solution.Solve)
    | 16 -> (module Day16.Solution : Solution.Solve)
    | 17 -> (module Day17.Solution : Solution.Solve)
    | n -> failwith (Printf.sprintf "day %d not implemented" n)
  in
  solve s Stdio.stdin
