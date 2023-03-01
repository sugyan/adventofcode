open Base
open Solution

module Solution : Solve = struct
  type t = ((int * int) * (int * int) -> bool) -> int

  let parse input =
    let parse_range s =
      let l, r = String.lsplit2_exn s ~on:'-' in
      (Int.of_string l, Int.of_string r)
    in
    let parse_assignments s =
      let l, r = String.lsplit2_exn s ~on:',' in
      (parse_range l, parse_range r)
    in
    let pairs =
      Stdio.In_channel.input_lines input |> List.map ~f:parse_assignments
    in
    fun f -> List.count pairs ~f

  let part1 count_pairs =
    count_pairs (fun ((a, b), (c, d)) ->
        (a <= c && d <= b) || (c <= a && b <= d))
    |> answer_of_int

  let part2 count_pairs =
    count_pairs (fun ((a, b), (c, d)) -> a <= d && c <= b) |> answer_of_int
end
