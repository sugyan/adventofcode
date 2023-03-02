open Base
open Solution

module Solution : Solve = struct
  type t = ((int * int) * (int * int) -> bool) -> int

  let parse input =
    let parse_line line =
      Caml.Scanf.sscanf line "%d-%d,%d-%d" (fun a b c d -> ((a, b), (c, d)))
    in
    let pairs = Stdio.In_channel.input_lines input |> List.map ~f:parse_line in
    fun f -> List.count pairs ~f

  let part1 count_pairs =
    count_pairs (fun ((a, b), (c, d)) ->
        (a <= c && d <= b) || (c <= a && b <= d))
    |> answer_of_int

  let part2 count_pairs =
    count_pairs (fun ((a, b), (c, d)) -> a <= d && c <= b) |> answer_of_int
end
