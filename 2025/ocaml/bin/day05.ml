open Aoc2025
open Base

module Solution : Aoc.Day = struct
  type input = (int * int) list * int list

  let parse channel =
    let ranges, ids =
      In_channel.input_lines channel
      |> List.split_while ~f:(String.is_empty |> Fn.non)
    in
    let parse_range s =
      String.split_on_chars s ~on:[ '-' ] |> List.map ~f:Int.of_string
      |> function
      | [ lo; hi ] -> (lo, hi)
      | _ -> failwith "invalid range"
    in
    ( List.map ranges ~f:parse_range,
      List.tl_exn ids |> List.map ~f:Int.of_string )

  let part1 input =
    let f n = List.exists (fst input) ~f:(fun (lo, hi) -> lo <= n && n <= hi) in
    snd input |> List.count ~f |> Answer.of_int

  let part2 input =
    let rec aux = function
      | (lo0, hi0) :: (lo1, hi1) :: tl when lo1 <= hi0 ->
          aux ((lo0, max hi0 hi1) :: tl)
      | hd :: tl -> hd :: aux tl
      | [] -> []
    in
    fst input
    |> List.sort ~compare:Poly.compare
    |> aux
    |> List.sum (module Int) ~f:(fun (lo, hi) -> hi - lo + 1)
    |> Answer.of_int
end

let () =
  let part =
    Sys.get_argv () |> Array.last |> function
    | "part1" -> Aoc.Part1
    | "part2" -> Aoc.Part2
    | _ -> Aoc.Both
  in
  Aoc.run ~part (module Solution) In_channel.stdin
