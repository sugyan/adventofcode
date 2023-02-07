open Base

module Solution : Solution.Solve = struct
  type t = ((int * int) * (int * int) -> bool) -> int

  let fully_contains ((a, b), (c, d)) = (a <= c && d <= b) || (c <= a && b <= d)
  let overlaps ((a, b), (c, d)) = a <= d && c <= b

  let parse input =
    let parse_range s =
      let left, right = String.lsplit2_exn s ~on:'-' in
      (Int.of_string left, Int.of_string right)
    in
    let parse_assignments s =
      let left, right = String.lsplit2_exn s ~on:',' in
      (parse_range left, parse_range right)
    in
    let pairs =
      Stdio.In_channel.input_lines input |> List.map ~f:parse_assignments
    in
    fun f -> List.count pairs ~f

  let part1 count_pairs = Solution.Integer (count_pairs fully_contains)
  let part2 count_pairs = Solution.Integer (count_pairs overlaps)
end
