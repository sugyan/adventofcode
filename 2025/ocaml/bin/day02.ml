open Aoc2025
open Base

module Solution : Aoc.Day = struct
  type input = (int * int) list

  let parse channel =
    let parse_range s =
      String.split s ~on:'-' |> function
      | [ lo; hi ] -> (Int.of_string lo, Int.of_string hi)
      | _ -> failwith "invalid range"
    in
    In_channel.input_lines channel
    |> String.concat |> String.split ~on:',' |> List.map ~f:parse_range

  let sum_of_invalid_ids input re =
    List.map input ~f:(fun (lo, hi) -> List.range lo hi ~stop:`inclusive)
    |> List.concat
    |> List.filter_map ~f:(fun i ->
        if Str.string_match re (Int.to_string i) 0 then Some i else None)
    |> List.sum (module Int) ~f:Fn.id

  let part1 input =
    sum_of_invalid_ids input (Str.regexp "^\\(.+\\)\\1$") |> Answer.of_int

  let part2 input =
    sum_of_invalid_ids input (Str.regexp "^\\(.+\\)\\1+$") |> Answer.of_int
end

let () =
  let part =
    Sys.get_argv () |> Array.last |> function
    | "part1" -> Aoc.Part1
    | "part2" -> Aoc.Part2
    | _ -> Aoc.Both
  in
  Aoc.run ~part (module Solution) In_channel.stdin
