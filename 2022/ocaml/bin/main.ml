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

let answer2string = function Solution.Integer i -> Int.to_string i

let solve (module S : Solution.Solve) input =
  let s = S.parse input in
  if !part <> Some "2" then
    s |> S.part1 |> answer2string |> Stdio.printf "Part 1: %s\n";
  if !part <> Some "1" then
    s |> S.part2 |> answer2string |> Stdio.printf "Part 2: %s\n"

let () =
  Caml.Arg.parse speclist (fun _ -> ()) usage_msg;
  let s =
    match !day with
    | 1 -> (module Day01.Solution : Solution.Solve)
    | 2 -> (module Day02.Solution : Solution.Solve)
    | 3 -> (module Day03.Solution : Solution.Solve)
    | n -> failwith (Printf.sprintf "Day %d not implemented" n)
  in
  solve s
    (if !infile <> "" then Stdio.In_channel.create !infile else Stdio.stdin)
