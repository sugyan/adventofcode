open Base

module Solution : Solution.Solve = struct
  type t = ((int * int) * (int * int)) list

  let part1 ((a, b), (c, d)) = (a <= c && d <= b) || (c <= a && b <= d)
  let part2 ((a, b), (c, d)) = a <= d && c <= b

  let parse input =
    let parse_range s =
      let left, right = String.lsplit2_exn s ~on:'-' in
      (Int.of_string left, Int.of_string right)
    in
    let parse_assignments line =
      let left, right = String.lsplit2_exn line ~on:',' in
      (parse_range left, parse_range right)
    in
    Stdio.In_channel.input_lines input |> List.map ~f:parse_assignments

  let part1 assignments = Solution.Integer (List.count assignments ~f:part1)
  let part2 assignments = Solution.Integer (List.count assignments ~f:part2)
end
