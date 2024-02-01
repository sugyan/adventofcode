open Aoc2023
open Solution

let usage_msg = "main.exe -day <day number> -part <1|2>"
let day = ref 0
let part = ref None

let speclist =
  [
    ("-day", Arg.Set_int day, "Set day number to solve");
    ("-part", Arg.Symbol ([ "1"; "2" ], fun s -> part := Some s), "");
  ]

let solve (module S : Solve) input =
  let s = S.parse input in
  if !part <> Some "2" then
    s |> S.part1 |> Answer.to_string
    |> Printf.sprintf "Part 1: %s"
    |> print_endline;
  if !part <> Some "1" then
    s |> S.part2 |> Answer.to_string
    |> Printf.sprintf "Part 2: %s"
    |> print_endline

let () =
  Arg.parse speclist ignore usage_msg;
  let s =
    match !day with
    | 1 -> (module Day01.Solution : Solve)
    | 2 -> (module Day02.Solution : Solve)
    | 3 -> (module Day03.Solution : Solve)
    | n -> failwith (Printf.sprintf "day %d not implemented" n)
  in
  solve s Stdlib.stdin
