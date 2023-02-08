open Aoc2022

let usage_msg = "main.exe -day <day number> -part <1|2> -input <input file>"
let day = ref 0
let part = ref None
let infile = ref ""

let speclist =
  [
    ("-day", Caml.Arg.Set_int day, "Set day number to solve");
    ("-part", Caml.Arg.Symbol ([ "1"; "2" ], fun s -> part := Some s), " ");
    ("-input", Caml.Arg.Set_string infile, "Set input file (default: stdin)");
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
  Caml.Arg.parse speclist (fun _ -> ()) usage_msg;
  let s =
    match !day with
    | 1 -> (module Day01.Solution : Solution.Solve)
    | 2 -> (module Day02.Solution : Solution.Solve)
    | 3 -> (module Day03.Solution : Solution.Solve)
    | 4 -> (module Day04.Solution : Solution.Solve)
    | 5 -> (module Day05.Solution : Solution.Solve)
    | 6 -> (module Day06.Solution : Solution.Solve)
    | 7 -> (module Day07.Solution : Solution.Solve)
    | n -> failwith (Printf.sprintf "Day %d not implemented" n)
  in
  solve s
    (if !infile <> "" then Stdio.In_channel.create !infile else Stdio.stdin)
